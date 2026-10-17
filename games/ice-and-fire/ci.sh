#!/usr/bin/env bash
# The CI steps that only A Code of Ice and Fire needs (ADR 0029, decision 4:
# every game provides a games/<game>/ci.sh). The bot checks job of ci.yml runs
# it after bundling the bots and building them standalone:
#
#   scripts/bundle-bots.sh ice-and-fire
#   CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/ice-and-fire-*.rs
#   games/ice-and-fire/ci.sh
#
# It expects the compiled bundles in target/cg/bin/ (ice-and-fire-wait and
# ice-and-fire-rules-check). The game has no arena yet, so for now it smoke
# tests the rules-check bundle (below; the bot's agreement with the referee is
# checked by its tests, which the game's test job runs) and measures the
# engine's speed (a report, not a check: ADR 0029, decision 7; it fails only if the
# measurement itself crashes; with GITHUB_STEP_SUMMARY set, the table goes to
# the job summary); stage 6 adds the arena's games here.
#
# Usage: games/ice-and-fire/ci.sh   (from any directory; no arguments)
set -euo pipefail

cd "$(dirname "$0")/../.."

for bot in wait rules-check; do
  if [[ ! -x target/cg/bin/ice-and-fire-$bot ]]; then
    echo "games/ice-and-fire/ci.sh: target/cg/bin/ice-and-fire-$bot is missing; first run scripts/bundle-bots.sh ice-and-fire and" >&2
    echo "  CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/ice-and-fire-*.rs" >&2
    exit 2
  fi
done

echo "== The rules-check bundle starts and stops cleanly"
# An empty input is a game that never started: strict mode must not fail.
if ! CG_RULES_CHECK_STRICT=1 target/cg/bin/ice-and-fire-rules-check </dev/null 2>/dev/null; then
  echo "::error title=Rules-check bundle::target/cg/bin/ice-and-fire-rules-check fails on an empty input"
  exit 1
fi

echo "== No arena yet: stage 6 adds the arena's games"

echo "== Engine speed (a report)"
scripts/annotate.sh run "Engine speed" \
  cargo run --release --locked -p ice-and-fire-engine --example speed -- 3 \
  | tee target/ice-and-fire-speed.txt
if [[ -n ${GITHUB_STEP_SUMMARY:-} ]]; then
  # The annotation of a failure stays out of the step summary.
  { grep -v '^::error title=' target/ice-and-fire-speed.txt || true; } >>"$GITHUB_STEP_SUMMARY"
fi
