#!/usr/bin/env bash
# Checks that releases replay identically: with a fixed seed and a fixed
# number of search iterations (CG_FIXED_ITERS), a release plays the same
# games twice (ADR 0025, decision 5). It also measures each release's
# iteration rate.
#
# Usage: scripts/check-determinism.sh GAME [ITERS] [RELEASE.rs...]
#
# ITERS defaults to 500. Without RELEASE.rs, every release of the game is
# checked, oldest first. Each release is compiled the way CodinGame compiles
# it, then plays itself twice, 2 pairs of games with seed 7, and the arena
# records every answer. The two runs must agree on every game: the lines
# each answer sent, the winner and how the game ended (times, which vary,
# are ignored). The iteration rate is ITERS divided by the median time of
# the answers of run 1, leaving out the first answer of each seat. Rates
# depend on the machine, and on ITERS: each answer has a fixed cost, which
# weighs more at small counts, so rates are comparable only at the same ITERS
# and are rough below a few thousand iterations. Records and binaries are
# kept in target/determinism/.
#
# Needs jq (preinstalled on GitHub's Ubuntu runners).
#
# Exit status: 0 when every release replays identically, 1 when one differs,
# 2 on usage errors, another non-zero status on other errors.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -lt 1 ]]; then
  echo "usage: $0 GAME [ITERS] [RELEASE.rs...]" >&2
  exit 2
fi
game="$1"
iters="${2:-500}"
shift $(($# > 1 ? 2 : 1))
if [[ ! $iters =~ ^[1-9][0-9]*$ ]]; then
  echo "error: ITERS must be a positive number, got: $iters" >&2
  exit 2
fi
if ! command -v jq >/dev/null; then
  echo "error: jq is needed" >&2
  exit 2
fi
if (($# > 0)); then
  files=("$@")
else
  mapfile -t files < <(releases "$game")
fi
if ((${#files[@]} == 0)); then
  echo "error: no release of $game" >&2
  exit 2
fi

dir=target/determinism
rm -rf "$dir"
mkdir -p "$dir/bin"
build_tools "$game"
CG_BIN_DIR="$dir/bin" scripts/cg-check.sh "${files[@]}" >&2
arena="target/release/$game-arena"

# What a record must keep from run to run: the lines of every answer, by
# turn, the winner and how the game ended.
# A record without these fields is an error, never "identical".
readonly canon='
  if .format != 1 then error("unexpected record format")
  elif (has("winner") | not) then error("record without winner")
  else
    {turns: [.turns[] | [.[] | (.lines // error("record without lines"))]],
     winner,
     end: (.end.kind // error("record without end kind"))}
  end'

# Where records $1 and $2 first differ: "turn N", "the winner" or "the end".
first_difference() {
  jq -n -r --slurpfile a <(jq -c "$canon" "$1") --slurpfile b <(jq -c "$canon" "$2") '
    ($a[0].turns) as $x | ($b[0].turns) as $y
    | ([range(0; [($x | length), ($y | length)] | max)]
       | map(select($x[.] != $y[.])) | first) as $turn
    | if $turn != null then "turn \($turn + 1)"
      elif $a[0].winner != $b[0].winner then "the winner"
      else "the end" end'
}

status=0
for file in "${files[@]}"; do
  name="$(basename "$file" .rs)"
  for run in 1 2; do
    mkdir -p "$dir/$name/run$run"
    if ! CG_FIXED_ITERS="$iters" "$arena" match --bot "a=$dir/bin/$name" \
      --bot "b=$dir/bin/$name" --pairs 2 --seed 7 --time-scale 50 \
      --records "$dir/$name/run$run" --expect-no-faults \
      >"$dir/$name/run$run.log" 2>&1; then
      echo "FAIL $name: run $run has faults or failed; see $dir/$name/run$run.log"
      status=1
      continue 2
    fi
  done

  games=0
  problem=""
  for first in "$dir/$name/run1"/*.json; do
    games=$((games + 1))
    base="$(basename "$first")"
    second="$dir/$name/run2/$base"
    if [[ ! -f $second ]]; then
      problem="run 2 has no $base"
      break
    fi
    canon_first="$(jq -c "$canon" "$first")"
    canon_second="$(jq -c "$canon" "$second")"
    if [[ $canon_first != "$canon_second" ]]; then
      problem="run 2 differs from run 1 in $base at $(first_difference "$first" "$second")"
      break
    fi
  done
  if [[ -z $problem && $(find "$dir/$name/run2" -name '*.json' | wc -l) -ne $games ]]; then
    problem="run 2 has other games than run 1"
  fi
  if [[ -n $problem ]]; then
    echo "FAIL $name: $problem"
    status=1
    continue
  fi

  # The median time of the answers but each seat's first, which includes the
  # start of the process.
  median="$(jq -s '
    [.[] | [.turns[][]]
     | (map(select(.seat == 0))[1:] + map(select(.seat == 1))[1:])
     | .[].ms]
    | sort
    | if length == 0 then 0
      else (.[(length / 2) | floor] + .[((length - 1) / 2) | floor]) / 2 end' \
    "$dir/$name/run1"/*.json)"
  rate="$(awk -v n="$iters" -v m="$median" 'BEGIN {
    if (m > 0) printf "%.1f", n / m; else printf "n/a" }')"
  median_text="$(awk -v m="$median" 'BEGIN { printf "%.2f", m }')"
  echo "ok   $name: identical in 2 runs of $games games; $rate iterations/ms (median $median_text ms at $iters iterations)"
done
exit "$status"
