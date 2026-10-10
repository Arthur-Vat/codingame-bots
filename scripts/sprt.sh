#!/usr/bin/env bash
# Tests whether a candidate bot is stronger than a baseline, with the game's
# evaluation settings (games/<game>/evaluation.env, ADR 0012 to 0015, 0020).
# The smoke test and the SPRT play at SPRT_TIME_SCALE and
# SPRT_TIME_TOLERANCE_MS when the game sets them (CodinGame's exact limits
# once its bot is in Legend), else at TIME_SCALE and TIME_TOLERANCE_MS.
#
# Usage: scripts/sprt.sh GAME CANDIDATE.rs [BASELINE.rs]
#
# Both files are paste-ready bots, compiled the way CodinGame compiles them
# and named after their file. Without BASELINE.rs, the baseline is the
# newest release older than the candidate, or else the bundle of the game's
# BASELINE_BOT. The candidate first plays the random bot (smoke test), then
# the SPRT runs against the baseline. If the SPRT accepts the candidate, it
# plays CONFIRM_PAIRS pairs against the baseline at CodinGame's exact time
# limits (confirmation), and is rejected if it is clearly weaker there or
# faults. Only the candidate's faults fail a step, and its timeouts only
# beyond MAX_TIMEOUT_RATE of the step's games: below that, like any fault of
# its opponent, a timeout just loses its game. Games are written to
# target/sprt/.
#
# Environment: when RECORDS_DIR is set and not empty, each step also keeps a
# sample of its games as game records (ADR 0027), in its own folder of
# RECORDS_DIR: CANDIDATE-smoke, CANDIDATE-sprt and CANDIDATE-confirmation
# (CANDIDATE is the candidate's name). RECORDS_SAMPLE is the number of games
# kept per step, 10 by default. The arena never overwrites a record: give an
# empty RECORDS_DIR. Without RECORDS_DIR, nothing changes.
#
# Exit status: 0 when the candidate is accepted and confirmed; 1 when the
# smoke test fails, the SPRT does not accept the candidate or the
# confirmation fails; 2 on errors.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -lt 2 || $# -gt 3 ]]; then
  echo "usage: $0 GAME CANDIDATE.rs [BASELINE.rs]" >&2
  exit 2
fi
game="$1"
candidate="$2"
baseline="${3:-}"
load_settings "$game"
TIME_SCALE="${SPRT_TIME_SCALE:-$TIME_SCALE}"
TIME_TOLERANCE_MS="${SPRT_TIME_TOLERANCE_MS:-$TIME_TOLERANCE_MS}"
build_tools "$game"

dir=target/sprt
mkdir -p "$dir/bin"
if [[ -z $baseline ]]; then
  candidate_name="$(basename "$candidate" .rs)"
  baseline="$(releases "$game" | awk -v c="$candidate_name" '
    { n = $0; sub(".*/", "", n); sub("\\.rs$", "", n) }
    n == c { exit }
    { last = $0 }
    END { print last }')"
  if [[ -z $baseline ]]; then
    bundle_bot "$game" "$BASELINE_BOT" "$dir"
    baseline="$dir/$game-$BASELINE_BOT.rs"
  fi
fi
bundle_bot "$game" random "$dir"
CG_BIN_DIR="$dir/bin" scripts/cg-check.sh "$candidate" "$baseline" "$dir/$game-random.rs" >&2

arena="target/release/$game-arena"
name() { basename "$1" .rs; }
cand="$(name "$candidate")"
base="$(name "$baseline")"
read -r -a options <<<"$(arena_options)"
# Sets "records" to the arena's options that keep a sample of a step's games,
# or to nothing when RECORDS_DIR is unset or empty.
records=()
records_for() {
  records=()
  if [[ -n ${RECORDS_DIR:-} ]]; then
    records=(--records "$RECORDS_DIR/$cand-$1" --records-sample "${RECORDS_SAMPLE:-10}")
  fi
}

echo "Candidate: $cand. Baseline: $base."
echo "Settings: $(describe_settings)."
echo
if ((SMOKE_PAIRS > 0)); then
  echo "Smoke test against $game-random:"
  records_for smoke
  if ! "$arena" match "${options[@]}" ${records[@]+"${records[@]}"} --expect-no-faults-from "$cand" \
    --bot "$cand=$dir/bin/$cand" --bot "random=$dir/bin/$game-random" \
    --pairs "$SMOKE_PAIRS" --min-score "$SMOKE_MIN_SCORE" \
    --out "$dir/$cand-smoke.jsonl"; then
    echo
    echo "Verdict: SMOKE TEST FAILED (needs a score of $SMOKE_MIN_SCORE, no crash or invalid answer, and timeouts within MAX_TIMEOUT_RATE=$MAX_TIMEOUT_RATE)"
    exit 1
  fi
  echo
fi
echo "SPRT against $base:"
records_for sprt
status=0
"$arena" sprt "${options[@]}" ${records[@]+"${records[@]}"} --expect-no-faults-from "$cand" \
  --candidate "$cand=$dir/bin/$cand" --baseline "$base=$dir/bin/$base" \
  --elo0 "$SPRT_ELO0" --elo1 "$SPRT_ELO1" --alpha "$SPRT_ALPHA" --beta "$SPRT_BETA" \
  --max-pairs "$SPRT_MAX_PAIRS" --out "$dir/$cand-sprt.jsonl" || status=$?
if ((status != 0 || CONFIRM_PAIRS == 0)); then
  exit "$status"
fi

echo
echo "Confirmation against $base at CodinGame's limits (time scale 1, no tolerance):"
records_for confirmation
if ! "$arena" match --seed "$SEED" --opening-plies "$OPENING_PLIES" \
  --time-scale 1 --time-tolerance-ms 0 --max-timeout-rate "$MAX_TIMEOUT_RATE" \
  ${records[@]+"${records[@]}"} \
  --expect-no-faults-from "$cand" --expect-not-worse \
  --bot "$cand=$dir/bin/$cand" --bot "$base=$dir/bin/$base" \
  --pairs "$CONFIRM_PAIRS" --out "$dir/$cand-confirmation.jsonl"; then
  echo
  echo "Final verdict: REJECTED at full time ($cand is clearly weaker than $base there, or faulted)"
  exit 1
fi
echo
echo "Final verdict: ACCEPTED (SPRT at time scale $TIME_SCALE, confirmed at full time)"
