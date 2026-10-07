#!/usr/bin/env bash
# Tests whether a candidate bot is stronger than a baseline, with the game's
# evaluation settings (games/<game>/evaluation.env, ADR 0012).
#
# Usage: scripts/sprt.sh GAME CANDIDATE.rs [BASELINE.rs]
#
# Both files are paste-ready bots, compiled the way CodinGame compiles them
# and named after their file. Without BASELINE.rs, the baseline is the
# newest release older than the candidate, or else the bundle of the game's
# BASELINE_BOT. The candidate first plays the random bot (smoke test), then
# the SPRT runs against the baseline. Games are written to target/sprt/.
#
# Exit status: 0 when the candidate is accepted; 1 when the smoke test
# fails or the SPRT does not accept the candidate; 2 on errors.
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

echo "Candidate: $cand. Baseline: $base."
echo "Settings: time scale $TIME_SCALE, opening plies $OPENING_PLIES, seed $SEED."
echo
if ((SMOKE_PAIRS > 0)); then
  echo "Smoke test against $game-random:"
  if ! "$arena" match "${options[@]}" --expect-no-faults \
    --bot "$cand=$dir/bin/$cand" --bot "random=$dir/bin/$game-random" \
    --pairs "$SMOKE_PAIRS" --min-score "$SMOKE_MIN_SCORE" \
    --out "$dir/$cand-smoke.jsonl"; then
    echo
    echo "Verdict: SMOKE TEST FAILED (needs a score of $SMOKE_MIN_SCORE and no faults)"
    exit 1
  fi
  echo
fi
echo "SPRT against $base:"
status=0
"$arena" sprt "${options[@]}" --expect-no-faults \
  --candidate "$cand=$dir/bin/$cand" --baseline "$base=$dir/bin/$base" \
  --elo0 "$SPRT_ELO0" --elo1 "$SPRT_ELO1" --alpha "$SPRT_ALPHA" --beta "$SPRT_BETA" \
  --max-pairs "$SPRT_MAX_PAIRS" --out "$dir/$cand-sprt.jsonl" || status=$?
exit "$status"
