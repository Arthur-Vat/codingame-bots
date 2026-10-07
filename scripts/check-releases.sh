#!/usr/bin/env bash
# Checks the release files a branch adds or changes, against BASE (default
# origin/main):
#
# - files already released never change and are never deleted;
# - a game gets at most one new release, numbered right after the last one;
# - a new release is the bundle of the bot named in its header, as the
#   branch's sources build it (whitespace aside: rustfmt versions differ),
#   and passes scripts/cg-check.sh;
# - a journal entry of the game has the line `release: <game>-vNNN`.
#
# Usage: scripts/check-releases.sh [BASE]
set -euo pipefail

cd "$(dirname "$0")/.."
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
done < <(git diff --name-status --no-renames "$base" HEAD -- 'games/*/releases/*')

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
      fail "$file: differs from a fresh bundle of $package; run scripts/new-release.sh again"
    fi
  fi

  scripts/cg-check.sh "$file" || fail "$file: not paste-ready"

  if ! grep -rqsx -- "release: $name" "games/$game/journal/"; then
    fail "$file: no entry in games/$game/journal/ has the line 'release: $name'"
  fi
  [[ $status -eq 0 ]] && echo "ok   $file (bundle of $package)"
done
if ((${#added[@]} == 0)); then
  echo "no new release"
fi
exit "$status"
