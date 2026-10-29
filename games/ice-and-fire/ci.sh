#!/usr/bin/env bash
# The CI steps that only A Code of Ice and Fire needs (ADR 0029, decision 4:
# every game provides a games/<game>/ci.sh). The bot checks job of ci.yml runs
# it after bundling the bots and building them standalone:
#
#   scripts/bundle-bots.sh ice-and-fire
#   CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/ice-and-fire-*.rs
#   games/ice-and-fire/ci.sh
#
# It expects the compiled bundles in target/cg/bin/ice-and-fire-* and runs, in
# order, each step printing a `== <step>` line first and stopping the script at
# the first failure:
#   - smoke tests the rules-check bundle on an empty input;
#   - builds the arena;
#   - plays 1,000 games between the bundled random bots;
#   - plays every other bundled bot against random, 100 pairs each; the
#     rules-check bot runs in strict mode, so it fails on any difference
#     between the referee and the engine's rules over real arena games;
#   - the evaluation controls and the A/A test (fixed seeds), and a small
#     league whose first place must be the bronze bot;
#   - tests the plan bot's analysis mode (`--features analysis`), which the
#     workspace's tests do not build;
#   - measures the engine's speed: a report, not a check (ADR 0029, decision 7),
#     which fails only if the measurement itself crashes; with
#     GITHUB_STEP_SUMMARY set, the table goes to the job summary.
# A failing step prints an error annotation (scripts/annotate.sh).
#
# Usage: games/ice-and-fire/ci.sh   (from any directory; no arguments)
set -euo pipefail

cd "$(dirname "$0")/../.."

for bot in random wait bronze rules-check; do
  if [[ ! -x target/cg/bin/ice-and-fire-$bot ]]; then
    echo "games/ice-and-fire/ci.sh: target/cg/bin/ice-and-fire-$bot is missing; first run scripts/bundle-bots.sh ice-and-fire and" >&2
    echo "  CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/ice-and-fire-*.rs" >&2
    exit 2
  fi
done

bin=target/cg/bin
arena=target/release/ice-and-fire-arena

echo "== The rules-check bundle starts and stops cleanly"
# An empty input is a game that never started: strict mode must not fail.
if ! CG_RULES_CHECK_STRICT=1 "$bin/ice-and-fire-rules-check" </dev/null 2>/dev/null; then
  echo "::error title=Rules-check bundle::$bin/ice-and-fire-rules-check fails on an empty input"
  exit 1
fi

echo "== Build the arena"
scripts/annotate.sh run "Build the arena" cargo build --release --locked -p ice-and-fire-arena

echo "== 1,000 games between the bundled random bots"
scripts/annotate.sh run "1,000 random games" "$arena" match \
  --bot "random=$bin/ice-and-fire-random" \
  --bot "random-again=$bin/ice-and-fire-random" \
  --pairs 500 --expect-no-faults --out target/cg/ice-and-fire-smoke.jsonl

echo "== Every other bundled bot against random"
# Bots that search may lose a few games on time (ADR 0015).
rate="$(sed -n 's/^MAX_TIMEOUT_RATE=//p' games/ice-and-fire/evaluation.env)"
# One bundle per bot folder: a glob on target/cg/bin/ice-and-fire-* would also
# match a game whose id starts with ice-and-fire-.
for dir in games/ice-and-fire/bots/*/; do
  name="ice-and-fire-$(basename "$dir")"
  bot="$bin/$name"
  [ "$name" = ice-and-fire-random ] && continue
  if [[ ! -x $bot ]]; then
    echo "::error title=$name against random::$bot is missing"
    exit 1
  fi
  # The rules-check bot exits on any disagreement with the referee.
  if ! out="$(CG_RULES_CHECK_STRICT=1 "$arena" match \
    --bot "$name=$bot" --bot "random=$bin/ice-and-fire-random" \
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
# The bots never look at the clock, so the outcome is reproducible. The
# referee ignores opening plies, so none are passed.
expect() {
  local verdict="$1" out
  shift
  out="$("$arena" sprt "$@" --seed 1)" || true
  echo "$out"
  if ! grep -q "^Verdict: $verdict" <<<"$out" ||
    [ "$(grep -c '0 timeouts, 0 crashes, 0 invalid answers' <<<"$out")" != 2 ]; then
    echo "::error title=Evaluation control::expected $verdict; got $(grep -E '^Verdict|faults' <<<"$out" | tr '\n' ' ')"
    return 1
  fi
}
expect ACCEPTED --candidate "bronze=$bin/ice-and-fire-bronze" --baseline "random=$bin/ice-and-fire-random"
expect REJECTED --candidate "random=$bin/ice-and-fire-random" --baseline "bronze=$bin/ice-and-fire-bronze"
expect REJECTED --candidate "bronze=$bin/ice-and-fire-bronze" --baseline "bronze-again=$bin/ice-and-fire-bronze"
"$arena" league --pairs 50 --expect-no-faults \
  --bot "wait=$bin/ice-and-fire-wait" --bot "random=$bin/ice-and-fire-random" \
  --bot "bronze=$bin/ice-and-fire-bronze" --anchor random | tee target/cg/ice-and-fire-league.txt
grep -q '^  1\. bronze ' target/cg/ice-and-fire-league.txt

echo "== The plan bot's analysis mode"
# Bots never carry it, so only this step builds and tests it.
scripts/annotate.sh run "Plan bot analysis mode" \
  cargo test --release --locked -p ice-and-fire-bot-plan --features analysis analysis::

echo "== Engine speed (a report)"
scripts/annotate.sh run "Engine speed" \
  cargo run --release --locked -p ice-and-fire-engine --example speed -- 3 \
  | tee target/ice-and-fire-speed.txt
if [[ -n ${GITHUB_STEP_SUMMARY:-} ]]; then
  # The annotation of a failure stays out of the step summary.
  { grep -v '^::error title=' target/ice-and-fire-speed.txt || true; } >>"$GITHUB_STEP_SUMMARY"
fi
