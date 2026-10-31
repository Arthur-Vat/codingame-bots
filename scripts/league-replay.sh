#!/usr/bin/env bash
# Replays a game's stored league pairings at its current evaluation settings
# (ADR 0048, decision 5): each pairing of games/GAME/league.jsonl is played
# again by the same two bots, with its stored seed, LEAGUE_PAIRS pairs, and
# written as a new line. The pairings are split into shards that run in
# parallel jobs (the League replay workflow), then merged in the stored order.
#
# Usage:
#   scripts/league-replay.sh GAME SHARD SHARDS
#   scripts/league-replay.sh --merge GAME FILE...
#
# The first form plays the pairings whose 0-based line number N, among the
# pairings of the file, has N % SHARDS == SHARD, with the game's settings
# (calibrated on this machine when TIME_SCALE is auto). It writes them to
# target/league/GAME-replay-SHARD.jsonl and prints them between the lines
# `---- replayed results of shard SHARD ----` and `---- end ----`. A fault
# loses its game and is counted in the line, as in the league; it does not
# stop the replay.
#
# The second form prints, in the order of games/GAME/league.jsonl, the
# replayed line of each stored pairing found in the FILEs. It fails if a
# stored pairing has no replayed line, or more than one, or a FILE holds a
# line of no stored pairing.
#
# Environment: LEAGUE_COMMIT, the commit written to each line (HEAD by
# default).
#
# Exit status: 0 on success; 2 on errors.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

usage() {
  echo "usage: $0 GAME SHARD SHARDS" >&2
  echo "       $0 --merge GAME FILE..." >&2
  exit 2
}

# Prints "A B" for each line of the results file $1, in order.
pairings_of() {
  sed -n 's/^{"a":"\([A-Za-z0-9_-]*\)","b":"\([A-Za-z0-9_-]*\)",.*/\1 \2/p' "$1"
}

# Checks that every non-blank line of $1 was read by pairings_of.
check_lines() {
  local lines pairs
  lines="$(grep -c . "$1" || true)"
  pairs="$(pairings_of "$1" | grep -c . || true)"
  if [[ $lines != "$pairs" ]]; then
    echo "error: $1 has $lines lines but $pairs start like a pairing ({\"a\":\"A\",\"b\":\"B\",...)" >&2
    exit 2
  fi
}

if [[ ${1:-} == --merge ]]; then
  [[ $# -ge 3 ]] || usage
  game="$2"
  shift 2
  [[ $game =~ ^[A-Za-z0-9_-]+$ ]] || usage
  results="games/$game/league.jsonl"
  check_lines "$results"
  replayed="$(mktemp)"
  trap 'rm -f "$replayed"' EXIT
  for file in "$@"; do
    check_lines "$file"
    cat "$file" >>"$replayed"
  done
  merged=0
  while read -r a b; do
    prefix="{\"a\":\"$a\",\"b\":\"$b\","
    count="$(grep -cF "$prefix" "$replayed" || true)"
    if [[ $count != 1 ]]; then
      echo "error: the stored pairing $a,$b has $count replayed lines, not 1" >&2
      exit 2
    fi
    grep -F "$prefix" "$replayed"
    merged=$((merged + 1))
  done < <(pairings_of "$results")
  total="$(grep -c . "$replayed" || true)"
  if [[ $total != "$merged" ]]; then
    echo "error: the files hold $total lines, $merged of them stored pairings" >&2
    exit 2
  fi
  exit 0
fi

[[ $# -eq 3 ]] || usage
game="$1"
shard="$2"
shards="$3"
if [[ ! $game =~ ^[A-Za-z0-9_-]+$ || ! $shard =~ ^[0-9]+$ || ! $shards =~ ^[1-9][0-9]*$ ]] ||
  ((shard >= shards)); then
  usage
fi
results="games/$game/league.jsonl"
if [[ ! -f $results ]]; then
  echo "error: $results does not exist: nothing to replay" >&2
  exit 2
fi
check_lines "$results"
load_settings "$game"
TIME_SCALE="${SPRT_TIME_SCALE:-$TIME_SCALE}"
TIME_TOLERANCE_MS="${SPRT_TIME_TOLERANCE_MS:-$TIME_TOLERANCE_MS}"
build_tools "$game"

dir=target/league
mkdir -p "$dir/bin"
calibrate_settings "$game" "$dir/calibration"

# This shard's pairings, and the bots they need.
chosen=()
needed=()
add_needed() { # add_needed <name>
  local known
  for known in ${needed[@]+"${needed[@]}"}; do
    [[ $known == "$1" ]] && return 0
  done
  needed+=("$1")
}
index=0
while read -r a b; do
  if ((index % shards == shard)); then
    chosen+=("$a,$b")
    add_needed "$a"
    add_needed "$b"
  fi
  index=$((index + 1))
done < <(pairings_of "$results")

new_results="$dir/$game-replay-$shard.jsonl"
echo "Settings: $(describe_settings), $LEAGUE_PAIRS pairs per pairing."
echo "Shard $shard of $shards: ${#chosen[@]} of the $index stored pairings."
if ((${#chosen[@]} == 0)); then
  : >"$new_results"
  echo "---- replayed results of shard $shard ----"
  echo "---- end ----"
  exit 0
fi

IFS=',' read -r -a baselines <<<"$LEAGUE_BOTS"
is_baseline() { # is_baseline <name>
  local bot
  for bot in "${baselines[@]}"; do
    [[ $bot == "$1" ]] && return 0
  done
  return 1
}
files=()
args=()
for bot in "${needed[@]}"; do
  if is_baseline "$bot"; then
    bundle_bot "$game" "$bot" "$dir"
    files+=("$dir/$game-$bot.rs")
  elif [[ -f games/$game/releases/$game-$bot.rs ]]; then
    files+=("games/$game/releases/$game-$bot.rs")
  else
    echo "error: '$bot' is neither a baseline bot of LEAGUE_BOTS ($LEAGUE_BOTS) nor a release of games/$game/releases" >&2
    exit 2
  fi
  args+=(--bot "$bot=$dir/bin/$game-$bot")
done
CG_BIN_DIR="$dir/bin" scripts/cg-check.sh "${files[@]}" >&2

pairing_args=()
for pairing in "${chosen[@]}"; do
  pairing_args+=(--pairing "$pairing")
done
speed=()
if [[ -n $MACHINE_SPEED ]]; then
  speed=(--speed "$MACHINE_SPEED")
fi
read -r -a options <<<"$(arena_options)"
status=0
"target/release/$game-arena" replay --results "$results" --new-results "$new_results" \
  --pairs "$LEAGUE_PAIRS" --commit "${LEAGUE_COMMIT:-$(git rev-parse HEAD)}" \
  ${speed[@]+"${speed[@]}"} "${options[@]}" "${args[@]}" "${pairing_args[@]}" || status=$?
if ((status != 0)); then
  echo "error: the replay of shard $shard failed (arena status $status)" >&2
  exit 2
fi
echo "---- replayed results of shard $shard ----"
cat "$new_results"
echo "---- end ----"
