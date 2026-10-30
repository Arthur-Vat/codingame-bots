#!/usr/bin/env bash
# Checks the release files a branch adds or changes, against BASE (default
# origin/main):
#
# - files already released never change and are never deleted, the readable
#   copies in games/<game>/readable/ included (ADR 0045);
# - a game gets at most one new release, numbered right after the last one;
# - a new release is the minified bundle of the bot named in its header, as
#   the branch's sources build it (the bundler's default, the minifier,
#   which needs rustc 1.90.0; whitespace aside: rustfmt versions differ), and
#   passes scripts/cg-check.sh;
# - a new release comes with its readable copy, added in the same branch, and
#   a readable copy comes with its release: the copy is the bundle of the same
#   bot with its comments (--keep-comments, whitespace aside). The two do not
#   have the same tokens, since the minifier removes the items never used
#   and renames the rest (ADR 0046);
#   scripts/check-minified.sh shows that they play the same. Releases made
#   before ADR 0045 are untouched and need no copy;
# - a journal entry of the game has the line `release: <game>-vNNN`.
#
# With GAME, only the releases (and readable copies) of games/GAME/ are
# considered; without, those of every game.
#
# Usage: scripts/check-releases.sh [BASE [GAME]]
set -euo pipefail

cd "$(dirname "$0")/.."
game_filter="${2:-*}"
if [[ $game_filter != '*' && ! $game_filter =~ ^[A-Za-z0-9_-]+$ ]]; then
  echo "usage: $0 [BASE [GAME]]" >&2
  exit 2
fi
base="$(git merge-base "${1:-origin/main}" HEAD)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

status=0
fail() {
  echo "FAIL $*" >&2
  status=1
}

declare -A added=()
while IFS=$'\t' read -r change file; do
  [[ -z $file ]] && continue
  if [[ $change != A ]]; then
    fail "$file: released files are frozen, but this branch changes or deletes it ($change)"
    continue
  fi
  game="$(cut -d/ -f2 <<<"$file")"
  if [[ -n ${added[$game]:-} ]]; then
    fail "$file: one new release per game and pull request (also ${added[$game]})"
    continue
  fi
  added[$game]="$file"
done < <(git diff --name-status --no-renames "$base" HEAD -- "games/$game_filter/releases/*")

# The readable copies, frozen like the releases; one is added only with the
# release of the same name.
declare -A added_readable=()
while IFS=$'\t' read -r change file; do
  [[ -z $file ]] && continue
  if [[ $change != A ]]; then
    fail "$file: readable copies are frozen, but this branch changes or deletes it ($change)"
    continue
  fi
  game="$(cut -d/ -f2 <<<"$file")"
  name="$(basename "$file" .rs)"
  if [[ $file != "games/$game/readable/$name.rs" || ${added[$game]:-} != "games/$game/releases/$name.rs" ]]; then
    fail "$file: a readable copy needs the release games/$game/releases/$name.rs, added in the same branch"
    continue
  fi
  added_readable[$file]=1
done < <(git diff --name-status --no-renames "$base" HEAD -- "games/$game_filter/readable/*")

if ((${#added[@]} > 0)); then
  cargo build --quiet --release --locked -p cg-bundler
fi
for game in "${!added[@]}"; do
  file="${added[$game]}"
  name="$(basename "$file" .rs)"
  last="$(git ls-tree --name-only "$base" "games/$game/releases/" |
    sed -n "s|.*/$game-v\([0-9]\{3\}\)\.rs$|\1|p" | sort -n | tail -n 1)"
  expected="$(printf '%s-v%03d' "$game" $((10#${last:-0} + 1)))"
  if [[ $name != "$expected" ]]; then
    fail "$file: the next release of $game is $expected"
  fi

  # shellcheck disable=SC2016 # the backquotes are literal
  package="$(sed -n '1s/^\/\/ Bundled by cg-bundler from the `\(.*\)` package\..*/\1/p' "$file")"
  if [[ -z $package ]]; then
    fail "$file: the first line does not name the bundled package"
  else
    if ! target/release/cg-bundler "$package" --output "$tmp_dir/fresh.rs" >"$tmp_dir/log" 2>&1; then
      cat "$tmp_dir/log" >&2
      fail "$file: cannot bundle $package"
    elif [[ "$(tr -d '[:space:]' <"$file")" != "$(tr -d '[:space:]' <"$tmp_dir/fresh.rs")" ]]; then
      fail "$file: differs from a fresh minified bundle of $package; run scripts/new-release.sh again"
    fi
  fi

  readable="games/$game/readable/$name.rs"
  if [[ -z ${added_readable[$readable]:-} ]]; then
    fail "$file: the readable copy $readable is not added in this branch (ADR 0045)"
  elif [[ -n $package ]]; then
    # shellcheck disable=SC2016 # the backquotes are literal
    readable_package="$(sed -n '1s/^\/\/ Bundled by cg-bundler from the `\(.*\)` package\..*/\1/p' "$readable")"
    if [[ $readable_package != "$package" ]]; then
      fail "$readable: bundled from '$readable_package', but $file is bundled from '$package'"
    elif ! target/release/cg-bundler "$package" --keep-comments --output "$tmp_dir/fresh-readable.rs" >"$tmp_dir/log" 2>&1; then
      cat "$tmp_dir/log" >&2
      fail "$readable: cannot bundle $package with comments"
    elif [[ "$(tr -d '[:space:]' <"$readable")" != "$(tr -d '[:space:]' <"$tmp_dir/fresh-readable.rs")" ]]; then
      fail "$readable: differs from a fresh bundle of $package with comments; run scripts/new-release.sh again"
    fi
  fi

  scripts/cg-check.sh "$file" || fail "$file: not paste-ready"

  if ! grep -rqsx -- "release: $name" "games/$game/journal/"; then
    fail "$file: no entry in games/$game/journal/ has the line 'release: $name'"
  fi
  [[ $status -eq 0 ]] && echo "ok   $file (minified bundle of $package, with its readable copy)"
done
if ((${#added[@]} == 0)); then
  echo "no new release"
fi
exit "$status"
