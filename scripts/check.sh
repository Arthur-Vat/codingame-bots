#!/usr/bin/env bash
# Runs the local checks that CI would run for the files a branch changes
# (ADR 0054). scripts/ci-scopes.sh maps the files to areas, as in CI
# (ADR 0029), and this script runs each area's local checks:
#
#   always       scripts/check-docs.sh, shellcheck and every
#                scripts/tests/*-test.sh (CI's docs and scripts job)
#   rust         cargo fmt --check; cargo clippy on the workspace, and CI's
#                two other lints (cg-search without analysis, the plan bot's
#                analysis mode)
#   framework    cargo test --workspace; every bot bundled and checked
#                (scripts/bundle-bots.sh, scripts/cg-check.sh)
#   a game       cargo test of the shared crates (the bundler's tests bundle
#                the game's bots), then of the game's packages, each run on
#                its own as CI's jobs are (cargo would otherwise build each
#                package with the features of all); the game's bots bundled
#                and checked
#   studio       cargo test of the packages under studio/ and games/*/studio/
#   studio or framework
#                the Ultimate Tic-Tac-Toe adapter's tests built alone,
#                without its feature "analysis"; the WebAssembly build of the
#                rules, releases and baseline bots (scripts/build-studio-wasm.sh
#                --strict, cached in target/studio-wasm-cache), when the
#                target wasm32-wasip1 is installed
#   licenses     cargo deny check, when cargo-deny is installed
#   web          a note only: CI runs the front end's checks (npm, browser
#                tests); see studio/README.md to run them locally
#   .github/     actionlint in Docker, when Docker runs
#
# A framework change, a workflow, the workspace's configuration or an unknown
# path select everything, as in CI. Not run here: games/<game>/ci.sh (arena
# games), the release checks (scripts/check-releases.sh, determinism, the
# readable copies), the front end's checks and the SPRT; play bots with the
# game's arena for bot or engine changes (CLAUDE.md).
#
# The files are those changed since the merge base with BASE (committed,
# staged, unstaged and untracked; both paths of a rename, as in CI), or the
# FILEs given after --.
#
# Usage: scripts/check.sh [--all] [--base REF] [--dry-run] [-- FILE...]
#   --all       every check, as for a push to main
#   --base REF  compare with REF (default origin/main)
#   --dry-run   print the areas and the steps, run nothing
#
# Every step runs even when an earlier one fails. The script ends with one
# line per step and a last line `ok   check: ...` or `FAIL check: ...`; it
# exits 1 if a step failed, 2 on wrong usage. Rust is the one that
# rust-toolchain.toml pins, 1.90.0 as on CI (ADR 0055). Needs git, and jq for
# cargo test of chosen packages.
set -euo pipefail

usage() {
  sed -n '/^# Usage:/,/^#   --dry-run/p' "$0" | sed 's/^# \{0,1\}//' >&2
  exit 2
}

all=false
base=origin/main
dry_run=false
given=false
files=()
while (($# > 0)); do
  case $1 in
    --all) all=true ;;
    --base)
      if (($# < 2)) || [[ -z $2 || $2 == -* ]]; then usage; fi
      base=$2
      shift
      ;;
    --dry-run) dry_run=true ;;
    --)
      shift
      given=true
      files=("$@")
      break
      ;;
    *) usage ;;
  esac
  shift
done

cd "$(dirname "$0")/.."

if [[ $given == false ]]; then
  if ! merge_base="$(git merge-base "$base" HEAD 2>/dev/null)"; then
    echo "check: no merge base with $base (fetch it, or pass --base REF)" >&2
    exit 2
  fi
  # --no-renames: a moved file counts at its old path too, as in CI.
  mapfile -d '' -t files < <(
    {
      git diff --name-only --no-renames -z "$merge_base"
      git ls-files --others --exclude-standard -z
    } | LC_ALL=C sort -z -u
  )
fi

scope_args=()
[[ $all == true ]] && scope_args+=(--all)
scopes="$(scripts/ci-scopes.sh ci "${scope_args[@]}" -- ${files[@]+"${files[@]}"})"
scope() { sed -n "s/^$1=//p" <<<"$scopes"; }
rust="$(scope rust)"
framework="$(scope framework)"
studio="$(scope studio)"
web="$(scope web)"
licenses="$(scope licenses)"
mapfile -t games < <(scope games | tr -d '[]"' | tr ',' '\n' | sed '/^$/d')
workflows=false
if [[ $all == true ]]; then
  workflows=true
else
  for file in ${files[@]+"${files[@]}"}; do
    [[ ${file#./} == .github/* ]] && workflows=true
  done
fi
echo "check: ${#files[@]} changed file(s); rust=$rust framework=$framework games=${games[*]:-none} studio=$studio web=$web licenses=$licenses"

# ---- steps ---------------------------------------------------------------

results=()
failed=0

step() { # step <name> <command> [args...]
  local name=$1 start status=0
  shift
  if [[ $dry_run == true ]]; then
    echo "step: $name"
    return 0
  fi
  echo "== $name"
  start=$SECONDS
  "$@" || status=$?
  if ((status == 0)); then
    results+=("ok   $name ($((SECONDS - start)) s)")
  else
    results+=("FAIL $name ($((SECONDS - start)) s)")
    failed=$((failed + 1))
  fi
}

note() {
  if [[ $dry_run == true ]]; then
    echo "note: $1"
  else
    results+=("note $1")
  fi
}

script_tests() {
  local test status=0
  for test in scripts/tests/*-test.sh; do
    "$test" || status=1
  done
  return "$status"
}

# cargo test of every package whose manifest's path, from the repository
# root, matches one of the regular expressions, as CI's jobs choose them.
test_packages() {
  local root package path pattern
  local -a args=()
  root="$PWD/"
  while IFS=$'\t' read -r package path; do
    path=${path#"$root"}
    for pattern in "$@"; do
      if [[ $path =~ $pattern ]]; then
        args+=(-p "$package")
        break
      fi
    done
  done < <(cargo metadata --no-deps --format-version 1 --locked | jq -r '.packages[] | "\(.name)\t\(.manifest_path)"')
  if ((${#args[@]} == 0)); then
    echo "check: no package matches $*" >&2
    return 1
  fi
  cargo test --locked "${args[@]}"
}

wasm_target_installed() {
  rustup target list --installed 2>/dev/null | grep -qx wasm32-wasip1
}

studio_wasm() {
  local out status=0
  out="$(mktemp -d)"
  scripts/build-studio-wasm.sh "$out" --strict --cache target/studio-wasm-cache || status=$?
  rm -rf "$out"
  return "$status"
}

bundle_game() { # bundle_game <game>
  local dir
  local -a bundles=()
  scripts/bundle-bots.sh "$1" || return
  for dir in "games/$1"/bots/*/; do
    bundles+=("target/cg/$1-$(basename "$dir").rs")
  done
  scripts/cg-check.sh "${bundles[@]}"
}

bundle_all() {
  scripts/bundle-bots.sh && scripts/cg-check.sh target/cg/*.rs
}

step "docs check" scripts/check-docs.sh
if command -v shellcheck >/dev/null; then
  step "shellcheck" shellcheck scripts/*.sh scripts/lib/*.sh scripts/tests/*.sh games/*/ci.sh
else
  note "shellcheck is not installed: not run"
fi
step "script tests" script_tests

if [[ $rust == true ]]; then
  step "cargo fmt" cargo fmt --all -- --check
  step "cargo clippy" cargo clippy --workspace --all-targets --locked -- -D warnings
  step "cargo clippy without analysis" cargo clippy -p cg-search --lib --locked -- -D warnings
  step "cargo clippy, plan bot analysis" cargo clippy -p ice-and-fire-bot-plan --all-targets --features analysis --locked -- -D warnings
fi

if [[ $framework == true ]]; then
  step "cargo test --workspace" cargo test --workspace --locked
  step "bundles of every game" bundle_all
else
  # One cargo test per CI job.
  if ((${#games[@]} > 0)); then
    step "cargo test of crates/" test_packages '^crates/'
  fi
  for game in ${games[@]+"${games[@]}"}; do
    step "cargo test of games/$game/" test_packages "^games/$game/"
  done
  if [[ $studio == true ]]; then
    step "cargo test of studio/ and games/*/studio/" test_packages '^studio/' '^games/[^/]+/studio/'
  fi
  for game in ${games[@]+"${games[@]}"}; do
    step "bundles of $game" bundle_game "$game"
  done
fi

if [[ $studio == true || $framework == true ]]; then
  # Built with the others, the adapter has its feature "analysis" on; one of
  # its tests runs only without it.
  step "cargo test of uttt-studio alone" cargo test --locked -p uttt-studio
  if wasm_target_installed; then
    step "WebAssembly build" studio_wasm
  else
    note "the target wasm32-wasip1 is not installed: the WebAssembly build runs on CI only (rustup target add wasm32-wasip1)"
  fi
fi

if [[ $licenses == true ]]; then
  if command -v cargo-deny >/dev/null; then
    step "cargo deny" cargo deny --log-level warn --all-features check
  else
    note "cargo-deny is not installed: the dependency policy runs on CI only"
  fi
fi

if [[ $workflows == true ]]; then
  if command -v docker >/dev/null && docker info >/dev/null 2>&1; then
    step "actionlint" docker run --rm -v "$PWD:/repo" -w /repo rhysd/actionlint:1.7.12 -color
  else
    note "Docker does not run: actionlint runs on CI only (in a cloud session, start it with sudo dockerd)"
  fi
fi

if [[ $web == true ]]; then
  note "the front end changed: its checks run on CI (to run them here, see studio/README.md)"
fi

[[ $dry_run == true ]] && exit 0

echo
for line in "${results[@]}"; do
  echo "$line"
done
steps=$(printf '%s\n' "${results[@]}" | grep -c -v '^note' || true)
if ((failed > 0)); then
  echo "FAIL check: $failed of $steps steps failed"
  exit 1
fi
echo "ok   check: $steps steps passed"
