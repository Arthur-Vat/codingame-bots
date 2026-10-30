#!/usr/bin/env bash
# Builds what the studio's browser engine needs (ADR 0030), for the target
# wasm32-wasip1:
#
# - OUT/rules.wasm: the rules program, the crate studio-rules;
# - OUT/<game>/<release>.wasm: every release, games/<game>/releases/<release>.rs,
#   compiled unchanged with rustc (edition 2021, opt-level 3, symbols
#   stripped);
# - OUT/<game>/<game>-<folder>.wasm: every baseline bot of the game
#   (ADR 0040), games/<game>/bots/<folder>, bundled from the current code by
#   cg-bundler (no rustfmt, the readable bundle: no need for rustc 1.90.0
#   here) and
#   compiled like a release. The adapters name
#   the baseline bots: the script asks the rules program (run natively), so
#   that no second list exists;
# - OUT/<game>/analysis.wasm: the game's analysis program, the crate
#   <game>-analysis (ADR 0032), for each game with a games/<game>/analysis
#   folder, built with cargo like the rules program (symbols stripped);
# - OUT/manifest.json:
#   {"games":[{"id":"uttt","releases":["uttt-v010",...],
#   "baselines":["uttt-greedy",...],"analysis":true}]}, for each game with a
#   release or a baseline bot: the releases that compiled, newest first (the
#   order of the server's list of releases), the baseline bots that compiled,
#   in the adapter's order, and whether the game has an analysis program.
#
# A release or a baseline bot that does not compile to WebAssembly is skipped
# with a warning on stderr and left out of the manifest: it is not offered in
# the browser. The release files are never changed. An analysis program that
# does not build is an error, not skipped.
#
# Needs Node, which `npm run wasm` has, to read the rules program's answer,
# and builds cg-bundler.
#
# Usage: scripts/build-studio-wasm.sh [OUT] [--strict]
#   OUT       the output folder, relative to the current folder (default:
#             studio/web/public/wasm in the repository, not committed)
#   --strict  also fail when a release or a baseline bot could not be built,
#             so that nothing is skipped without notice (used by CI)
set -euo pipefail

caller=$PWD
out=studio/web/public/wasm
strict=0
positional=0
for arg in "$@"; do
  case $arg in
    --strict) strict=1 ;;
    -*)
      echo "usage: $0 [OUT] [--strict]" >&2
      exit 2
      ;;
    *)
      positional=$((positional + 1))
      out=$arg
      ;;
  esac
done
if ((positional > 1)); then
  echo "usage: $0 [OUT] [--strict]" >&2
  exit 2
fi
if ((positional == 1)) && [[ $out != /* ]]; then
  out=$caller/$out
fi
cd "$(dirname "$0")/.."

target=wasm32-wasip1
installed=0
if command -v rustup >/dev/null 2>&1; then
  if rustup target list --installed 2>/dev/null | grep -qx "$target"; then
    installed=1
  fi
elif [[ -d "$(rustc --print target-libdir --target "$target" 2>/dev/null || true)" ]]; then
  installed=1
fi
if ((installed == 0)); then
  echo "build-studio-wasm: the target $target is not installed; run: rustup target add $target" >&2
  exit 1
fi

if ! command -v node >/dev/null; then
  echo "build-studio-wasm: node is needed" >&2
  exit 1
fi

mkdir -p "$out"
# Without symbols the file is about a fifth smaller.
CARGO_PROFILE_RELEASE_STRIP=symbols cargo build --locked --release --target "$target" -p studio-rules
cp "${CARGO_TARGET_DIR:-target}/$target/release/studio-rules.wasm" "$out/rules.wasm"

# The baseline bots of each game, from the adapters.
games_json="$(echo '{"op":"games"}' | cargo run -q --locked -p studio-rules)"
# One `<game> <baseline bot>` line per baseline bot. Node rather than jq: Git
# Bash on Windows has no jq. The script is JavaScript: no shell expansion.
# shellcheck disable=SC2016
baseline_lines="$(node -e '
  const answer = JSON.parse(require("fs").readFileSync(0, "utf8"));
  if (!Array.isArray(answer.ok)) process.exit(1);
  for (const game of answer.ok) for (const bot of game.baselines) console.log(`${game.id} ${bot}`);
' <<<"$games_json")" || {
  echo "build-studio-wasm: the rules program cannot list the games: $games_json" >&2
  exit 1
}
cargo build -q --locked --release -p cg-bundler
bundler="${CARGO_TARGET_DIR:-target}/release/cg-bundler"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

shopt -s nullglob
total=0
built=0
baseline_total=0
baseline_built=0
analyses=0
manifest='{"games":['
separator=
for game_dir in games/*/; do
  game="$(basename "$game_dir")"
  [[ $game =~ ^[A-Za-z0-9_-]+$ ]] || continue
  # The releases, newest first: by number, then by name, both descending, as
  # studio/server/src/releases.rs sorts them.
  names=()
  while IFS= read -r name; do
    names+=("$name")
  done < <(
    for file in "games/$game/releases/$game"-v*.rs; do
      name="$(basename "$file" .rs)"
      number="${name#"$game"-v}"
      [[ $number =~ ^[0-9]+$ ]] && printf '%s %s\n' "$((10#$number))" "$name"
    done | sort -k1,1nr -k2,2r | cut -d' ' -f2
  )
  baselines=()
  while IFS= read -r name; do
    baselines+=("$name")
  done < <(awk -v game="$game" '$1 == game { print $2 }' <<<"$baseline_lines")
  ((${#names[@]} + ${#baselines[@]} > 0)) || continue
  mkdir -p "$out/$game"
  # A game without an analysis program must not keep an old file.
  rm -f "$out/$game/analysis.wasm"
  analysis=false
  if [[ -f games/$game/analysis/Cargo.toml ]]; then
    CARGO_PROFILE_RELEASE_STRIP=symbols cargo build --locked --release --target "$target" -p "$game-analysis"
    cp "${CARGO_TARGET_DIR:-target}/$target/release/$game-analysis.wasm" "$out/$game/analysis.wasm"
    analysis=true
    analyses=$((analyses + 1))
  fi
  compiled=()
  for name in "${names[@]}"; do
    total=$((total + 1))
    wasm="$out/$game/$name.wasm"
    rm -f "$wasm"
    if rustc --edition 2021 -C opt-level=3 -C strip=symbols --target "$target" \
      -o "$wasm" "games/$game/releases/$name.rs"; then
      compiled+=("$name")
      built=$((built + 1))
    else
      rm -f "$wasm"
      echo "build-studio-wasm: warning: $name does not compile to $target; it is left out of the manifest" >&2
    fi
  done
  list=
  for name in ${compiled[@]+"${compiled[@]}"}; do
    list+="${list:+,}\"$name\""
  done
  compiled=()
  for name in ${baselines[@]+"${baselines[@]}"}; do
    baseline_total=$((baseline_total + 1))
    wasm="$out/$game/$name.wasm"
    rm -f "$wasm"
    folder="${name#"$game"-}"
    manifest_file="games/$game/bots/$folder/Cargo.toml"
    package=
    [[ -f $manifest_file ]] && package="$(tr -d '\r' <"$manifest_file" | sed -n 's/^name = "\(.*\)"$/\1/p' | head -n 1)"
    if [[ -n $package ]] &&
      "$bundler" "$package" --no-format --keep-comments --output "$work/$name.rs" &&
      rustc --edition 2021 -C opt-level=3 -C strip=symbols --target "$target" \
        --crate-name baseline_bot -o "$wasm" "$work/$name.rs"; then
      compiled+=("$name")
      baseline_built=$((baseline_built + 1))
    else
      rm -f "$wasm"
      echo "build-studio-wasm: warning: the baseline bot $name does not compile to $target; it is left out of the manifest" >&2
    fi
  done
  baseline_list=
  for name in ${compiled[@]+"${compiled[@]}"}; do
    baseline_list+="${baseline_list:+,}\"$name\""
  done
  manifest+="$separator{\"id\":\"$game\",\"releases\":[$list],\"baselines\":[$baseline_list],\"analysis\":$analysis}"
  separator=,
done
manifest+=']}'
printf '%s\n' "$manifest" >"$out/manifest.json"

if ((strict == 1 && (built < total || baseline_built < baseline_total))); then
  echo "build-studio-wasm: only $built of $total releases and $baseline_built of $baseline_total baseline bots are in the manifest (--strict)" >&2
  exit 1
fi
echo "ok   studio wasm: rules.wasm, analysis.wasm for $analyses game(s), $built of $total releases and $baseline_built of $baseline_total baseline bots in $out"
