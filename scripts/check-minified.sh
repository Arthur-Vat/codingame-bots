#!/usr/bin/env bash
# Checks that a minified release plays exactly like its readable copy
# (ADR 0045, decision 4).
#
# Usage: scripts/check-minified.sh GAME [ITERS] RELEASE.rs...
#
# ITERS defaults to 500. For each RELEASE.rs, games/GAME/releases/X.rs, its
# readable copy is games/GAME/readable/X.rs; a missing copy fails. Both are
# compiled the way CodinGame compiles them (scripts/cg-check.sh; the readable
# copy may be over the size limit). Then each plays itself, 2 pairs of games
# with seed 7 and a fixed number of search iterations (CG_FIXED_ITERS), as in
# scripts/check-determinism.sh: the readable copy first, then the minified
# release under the same bot names, which fix each bot's CG_SEED. The records
# must agree on every game: the lines each answer sent, the winner and how
# the game ended (times, which vary, are ignored). Records and binaries are
# kept in target/minified/.
#
# Needs jq (preinstalled on GitHub's Ubuntu runners).
#
# Exit status: 0 when every release plays like its readable copy, 1 when one
# differs or has no readable copy, 2 on usage errors, another non-zero status
# on other errors.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh
# shellcheck source=scripts/lib/records.sh
source scripts/lib/records.sh

if [[ $# -lt 1 ]]; then
  echo "usage: $0 GAME [ITERS] RELEASE.rs..." >&2
  exit 2
fi
game="$1"
iters="${2:-500}"
shift $(($# > 1 ? 2 : 1))
if [[ ! $game =~ ^[A-Za-z0-9_-]+$ ]]; then
  echo "error: GAME must be a game's folder name, got: $game" >&2
  exit 2
fi
if [[ ! $iters =~ ^[1-9][0-9]*$ ]]; then
  echo "error: ITERS must be a positive number, got: $iters" >&2
  exit 2
fi
if ! command -v jq >/dev/null; then
  echo "error: jq is needed" >&2
  exit 2
fi
if (($# == 0)); then
  echo "usage: $0 GAME [ITERS] RELEASE.rs..." >&2
  exit 2
fi
files=("$@")

dir=target/minified
rm -rf "$dir"
mkdir -p "$dir/bin/minified" "$dir/bin/readable"
build_tools "$game"
arena="target/release/$game-arena"

status=0
for file in "${files[@]}"; do
  name="$(basename "$file" .rs)"
  readable="$(dirname "$file")/../readable/$name.rs"
  if [[ ! -f $file ]]; then
    echo "FAIL $name: no file $file"
    status=1
    continue
  fi
  if [[ ! -f $readable ]]; then
    echo "FAIL $name: no readable copy $readable"
    status=1
    continue
  fi
  # The two binaries have the same name: one folder each.
  if ! CG_BIN_DIR="$dir/bin/minified" scripts/cg-check.sh "$file" >&2 ||
    ! CG_BIN_DIR="$dir/bin/readable" scripts/cg-check.sh --any-size "$readable" >&2; then
    echo "FAIL $name: the release or its readable copy does not build as CodinGame builds it"
    status=1
    continue
  fi

  # Same bot names in both runs: the arena derives each bot's CG_SEED from
  # its name.
  ran=true
  for run in readable minified; do
    mkdir -p "$dir/$name/$run"
    if ! CG_FIXED_ITERS="$iters" "$arena" match --bot "a=$dir/bin/$run/$name" \
      --bot "b=$dir/bin/$run/$name" --pairs 2 --seed 7 --time-scale 50 \
      --records "$dir/$name/$run" --expect-no-faults \
      >"$dir/$name/$run.log" 2>&1; then
      echo "FAIL $name: the $run run has faults or failed; see $dir/$name/$run.log"
      status=1
      ran=false
      break
    fi
  done
  if ! $ran; then
    continue
  fi

  problem="$(compare_runs "$dir/$name/readable" "$dir/$name/minified" "the readable copy" "the minified release")" || true
  if [[ -n $problem ]]; then
    echo "FAIL $name: $problem"
    status=1
    continue
  fi
  games="$(find "$dir/$name/readable" -name '*.json' | wc -l)"
  echo "ok   $name: same answers as its readable copy in $games games"
done
exit "$status"
