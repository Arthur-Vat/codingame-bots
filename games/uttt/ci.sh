#!/usr/bin/env bash
# The CI steps that only Ultimate Tic-Tac-Toe needs (ADR 0029, decision 4: every
# game provides a games/<game>/ci.sh). The bot checks job of ci.yml runs it
# after bundling the bots and building them standalone:
#
#   scripts/bundle-bots.sh uttt
#   CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/uttt-*.rs
#   games/uttt/ci.sh
#
# It expects the compiled bundles in target/cg/bin/uttt-* and runs, in order,
# each step printing a `== <step>` line first and stopping the script at the
# first failure:
#   - builds the arena;
#   - plays 1,000 games between the bundled random bots;
#   - plays every other bundled bot against random, 100 pairs each;
#   - the evaluation controls and the A/A test (fixed seeds);
#   - measures the engine's speed: a report, not a check (ADR 0029, decision 7),
#     which fails only if the measurement itself crashes; with
#     GITHUB_STEP_SUMMARY set, the table goes to the job summary.
# A failing step prints an error annotation (scripts/annotate.sh).
#
# Usage: games/uttt/ci.sh   (from any directory; no arguments)
set -euo pipefail

cd "$(dirname "$0")/../.."

if [[ ! -x target/cg/bin/uttt-random ]]; then
  echo "games/uttt/ci.sh: target/cg/bin/uttt-random is missing; first run scripts/bundle-bots.sh uttt and" >&2
  echo "  CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/uttt-*.rs" >&2
  exit 2
fi

bin=target/cg/bin
arena=target/release/uttt-arena

echo "== Build the arena"
scripts/annotate.sh run "Build the arena" cargo build --release --locked -p uttt-arena

echo "== 1,000 games between the bundled random bots"
scripts/annotate.sh run "1,000 random games" "$arena" match \
  --bot "random=$bin/uttt-random" \
  --bot "random-again=$bin/uttt-random" \
  --pairs 500 --expect-no-faults --out target/cg/uttt-smoke.jsonl

echo "== Every other bundled bot against random"
# Bots that search may lose a few games on time (ADR 0015).
rate="$(sed -n 's/^MAX_TIMEOUT_RATE=//p' games/uttt/evaluation.env)"
for bot in "$bin"/uttt-*; do
  name="$(basename "$bot")"
  [ "$name" = uttt-random ] && continue
  # The rules-check bot exits on any disagreement with the referee.
  if ! out="$(CG_RULES_CHECK_STRICT=1 "$arena" match \
    --bot "$name=$bot" --bot "random=$bin/uttt-random" \
    --pairs 100 --expect-no-faults --max-timeout-rate "$rate" 2>&1)"; then
    echo "$out"
    # Job logs are hard to read from outside GitHub; an annotation
    # carries the summary to the check run.
    echo "::error title=$name against random::$(grep -E 'score|faults|slowest|error' <<<"$out" | tr '\n' ' ')"
    exit 1
  fi
  echo "$out"
done

echo "== The evaluation tells bots apart (controls and A/A test)"
# Runs an SPRT with fixed seeds and checks its verdict and faults.
# The bots never look at the clock, so the outcome is reproducible.
expect() {
  local verdict="$1" out
  shift
  out="$("$arena" sprt "$@" --opening-plies 4 --seed 1)" || true
  echo "$out"
  if ! grep -q "^Verdict: $verdict" <<<"$out" ||
    [ "$(grep -c '0 timeouts, 0 crashes, 0 invalid answers' <<<"$out")" != 2 ]; then
    echo "::error title=Evaluation control::expected $verdict; got $(grep -E '^Verdict|faults' <<<"$out" | tr '\n' ' ')"
    return 1
  fi
}
expect ACCEPTED --candidate "greedy=$bin/uttt-greedy" --baseline "random=$bin/uttt-random"
expect REJECTED --candidate "random=$bin/uttt-random" --baseline "greedy=$bin/uttt-greedy"
expect REJECTED --candidate "greedy=$bin/uttt-greedy" --baseline "greedy-again=$bin/uttt-greedy"
"$arena" league --pairs 50 --opening-plies 4 --expect-no-faults \
  --bot "first-valid=$bin/uttt-first-valid" --bot "random=$bin/uttt-random" \
  --bot "greedy=$bin/uttt-greedy" --anchor random | tee target/cg/league.txt
grep -q '^  1\. greedy ' target/cg/league.txt

echo "== Engine speed (a report)"
scripts/annotate.sh run "Engine speed" \
  cargo run --release --locked -p uttt-engine --example speed -- 3 \
  | tee target/speed.txt
if [[ -n ${GITHUB_STEP_SUMMARY:-} ]]; then
  # The annotation of a failure stays out of the step summary.
  { grep -v '^::error title=' target/speed.txt || true; } >>"$GITHUB_STEP_SUMMARY"
fi
