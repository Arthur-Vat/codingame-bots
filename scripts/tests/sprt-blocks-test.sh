#!/usr/bin/env bash
# Tests scripts/sprt-blocks.sh without a network or a build: which games play
# their SPRT over runners, the plan of a test, the shard's wait for its blocks,
# and the coordinator's rounds, with a stand-in `gh` that serves artifacts
# from a folder and a stand-in arena whose `sprt-check` decides by the number
# of blocks. It also checks that the workflow has as many upload steps as the
# script's MAX_BLOCKS.
# Usage: scripts/tests/sprt-blocks-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_OUTPUT GITHUB_ENV SPRT_WORK SPRT_RESULT RECORDS_DIR
unset "${!SPRT_@}" TIME_SCALE TIME_TOLERANCE_MS

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}
expect() { # expect <description> <actual> <expected>
  checks=$((checks + 1))
  if [[ $2 != "$3" ]]; then fail "$1: expected '$3', got '$2'"; fi
}
contains() { # contains <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 != *"$3"* ]]; then fail "$1: '$3' not found in: $2"; fi
}

repo="$tmp_dir/repo"
mkdir -p "$repo/scripts/lib" "$repo/bin" "$repo/target/release"
cp "$repo_root/scripts/sprt-blocks.sh" "$repo/scripts/"
cp "$repo_root/scripts/lib/evaluation.sh" "$repo/scripts/lib/"
for game in toy solo; do
  mkdir -p "$repo/games/$game/releases"
  touch "$repo/games/$game/releases/$game-v001.rs" "$repo/games/$game/releases/$game-v002.rs"
done
cat >"$repo/games/toy/evaluation.env" <<'ENV'
TIME_SCALE=auto
TIME_TOLERANCE_MS=auto
SEED=1
OPENING_PLIES=0
MAX_TIMEOUT_RATE=0.01
SPRT_ELO0=0
SPRT_ELO1=10
SPRT_ALPHA=0.05
SPRT_BETA=0.05
SPRT_MAX_PAIRS=400
SPRT_RUNNERS=2
SPRT_BLOCK_PAIRS=10
BASELINE_BOT=greedy
ENV
sed -e 's/^TIME_SCALE=.*/TIME_SCALE=1/' -e 's/^TIME_TOLERANCE_MS=.*/TIME_TOLERANCE_MS=0/' -e 's/^SPRT_RUNNERS=.*/SPRT_RUNNERS=1/' \
  "$repo/games/toy/evaluation.env" >"$repo/games/solo/evaluation.env"
(
  cd "$repo"
  git init -q
  git add -A
  git -c user.name=t -c user.email=t@example.com commit -q -m init
)
script="$repo/scripts/sprt-blocks.sh"
run() { # run <args>...: the script in the stand-in repository; sets output, status
  status=0
  output="$("$script" "$@" 2>&1)" || status=$?
}

# ---- classify ----
run classify '["solo","toy"]' games/toy/releases/toy-v002.rs games/solo/releases/solo-v002.rs
expect "classify status" "$status" 0
contains "classify" "$output" 'serial_games=["solo"]'
contains "classify" "$output" 'parallel_games=["toy"]'
contains "classify" "$output" 'parallel_matrix={"include":[{"game":"toy","candidate":"games/toy/releases/toy-v002.rs"}]}'
run classify '["solo"]' games/solo/releases/solo-v002.rs
contains "classify serial only" "$output" 'parallel_matrix={"include":[]}'
run classify '[]'
contains "classify nothing" "$output" 'serial_games=[]'
run classify '["toy"]' games/toy/releases/toy-v001.rs games/toy/releases/toy-v002.rs
expect "two releases" "$status" 1
contains "two releases" "$output" "one experiment per pull request"
run classify '["toy"]' games/solo/releases/solo-v002.rs
expect "no release of the game" "$status" 1
sed -i 's/^TIME_SCALE=auto/TIME_SCALE=1/' "$repo/games/toy/evaluation.env"
run classify '["toy"]' games/toy/releases/toy-v002.rs
expect "not calibrated" "$status" 2
contains "not calibrated" "$output" "needs TIME_SCALE=auto"
sed -i 's/^TIME_SCALE=1/TIME_SCALE=auto/' "$repo/games/toy/evaluation.env"

# ---- plan ----
run plan toy games/toy/releases/toy-v002.rs
expect "plan status" "$status" 0
contains "plan" "$output" "baseline=games/toy/releases/toy-v001.rs"
contains "plan" "$output" "shards=[0,1]"
contains "plan" "$output" "blocks=20"
contains "plan" "$output" "key=sprt-blocks-toy-toy-v002-toy-v001-"
run plan toy games/toy/releases/toy-v001.rs
contains "plan first release" "$output" "baseline=bot:greedy"
run plan 'toy/../x' games/toy/releases/toy-v002.rs
expect "bad game" "$status" 2
run plan toy 'games/toy/releases/../../../etc/passwd'
expect "bad candidate" "$status" 2
run plan toy games/toy/releases/toy-v002.rs games/toy/releases/toy-v002.rs
expect "same bot" "$status" 2

# ---- the workflow ----
# One upload loop, not a repeated pair of steps.
expect "wait steps in the workflow" "$(grep -c 'shard-wait\|^      - \*' "$repo_root/.github/workflows/sprt-parallel.yml" || true)" 0

# ---- the shard uploads its blocks in batches ----
cat >"$repo/bin/gh" <<'GH'
#!/usr/bin/env bash
# Stand-in gh api: serves the folder $FAKE.
url="$2"
case $url in
  *"name=sprt-calibration-"*)
    shard="${url##*-}"; shard="${shard%%&*}"
    echo "$((100 + shard))" ;;
  *"/zip") id="${url%/zip}"; cat "$FAKE/zips/${id##*/}.zip" ;;
  *"/jobs"*) cat "$FAKE/jobs.tsv" ;;
  *"/artifacts?per_page"*) cat "$FAKE/artifacts.tsv" ;;
  *) exit 1 ;;
esac
GH
chmod +x "$repo/bin/gh"
export PATH="$repo/bin:$PATH" FAKE="$tmp_dir/fake"
mkdir -p "$FAKE/zips"
loop="$repo_root/scripts/sprt-upload/loop.cjs"
node --check "$loop"
work="$tmp_dir/work"
dry="$tmp_dir/dry"
loop_env() { # loop_env <command>...: the loop in dry-run mode; sets status
  status=0
  env SPRT_WORK="$work" SPRT_DRY_RUN_DIR="$dry" SPRT_DRY_VERDICT_FILE="$tmp_dir/verdict" \
    GAME=toy RUN_ATTEMPT=1 SHARD=1 UPLOAD_SECONDS=0 POLL_MS=10 "$@" >"$tmp_dir/loop.out" 2>&1 || status=$?
  output="$(cat "$tmp_dir/loop.out")"
}
new_shard() { # new_shard <blocks total>
  rm -rf "$work" "$dry" "$tmp_dir/verdict"
  mkdir -p "$work/blocks"
  echo "$1" >"$work/blocks-total"
  # Started from a subshell that ends: the arena of a shard is nobody's child
  # either, and a killed child would stay a zombie that still looks alive.
  (sleep 300 >/dev/null 2>&1 & echo $! >"$work/player.pid")
  sleeper="$(cat "$work/player.pid")"
}
running() { [[ -d /proc/$1 && $(awk '{print $3}' "/proc/$1/stat") != Z ]]; }
files_of() { find "$dry/$1" -type f -printf '%f\n' 2>/dev/null | sort | paste -sd' ' -; }

# All blocks are there: one batch, then the end.
new_shard 3
for n in 0 1 2; do echo "b$n" >"$work/blocks/block-$n.jsonl"; done
loop_env node "$loop"
expect "loop status" "$status" 0
expect "batch files" "$(files_of sprt-blocks-toy-1-1-0)" "1-0.jsonl 1-1.jsonl 1-2.jsonl"
contains "loop line" "$output" "uploaded sprt-blocks-toy-1-1-0, 3 blocks, 3 of 3 in all"
kill "$sleeper" 2>/dev/null || true
# Blocks come in two moments: two batches with the blocks since the last.
new_shard 3
echo b0 >"$work/blocks/block-0.jsonl"
(sleep 1; echo b1 >"$work/blocks/block-1.jsonl"; echo b2 >"$work/blocks/block-2.jsonl") &
later=$!
loop_env node "$loop"
expect "two batches status" "$status" 0
expect "first batch" "$(files_of sprt-blocks-toy-1-1-0)" "1-0.jsonl"
expect "second batch" "$(files_of sprt-blocks-toy-1-1-1)" "1-1.jsonl 1-2.jsonl"
wait "$later"
kill "$sleeper" 2>/dev/null || true
# A block missing in the middle waits for it: batches hold consecutive blocks.
new_shard 3
echo b0 >"$work/blocks/block-0.jsonl"
echo b2 >"$work/blocks/block-2.jsonl"
(sleep 1; echo b1 >"$work/blocks/block-1.jsonl") &
later=$!
loop_env node "$loop"
expect "gap first batch" "$(files_of sprt-blocks-toy-1-1-0)" "1-0.jsonl"
expect "gap second batch" "$(files_of sprt-blocks-toy-1-1-1)" "1-1.jsonl 1-2.jsonl"
wait "$later"
kill "$sleeper" 2>/dev/null || true
# A failed upload is retried under the next number; 3 failures fail the loop.
new_shard 2
for n in 0 1; do echo "b$n" >"$work/blocks/block-$n.jsonl"; done
loop_env env SPRT_DRY_FAIL=1 RETRY_MS=1 node "$loop"
expect "retry status" "$status" 0
expect "retried batch" "$(files_of sprt-blocks-toy-1-1-1)" "1-0.jsonl 1-1.jsonl"
expect "failed batch" "$(files_of sprt-blocks-toy-1-1-0)" ""
contains "retry line" "$output" "try 1 of 3"
new_shard 2
echo b0 >"$work/blocks/block-0.jsonl"
loop_env env SPRT_DRY_FAIL=3 RETRY_MS=1 node "$loop"
expect "three failures" "$status" 1
contains "three failures" "$output" "failed 3 times"
kill "$sleeper" 2>/dev/null || true
# A verdict stops the arena and uploads nothing.
new_shard 3
touch "$tmp_dir/verdict"
echo b0 >"$work/blocks/block-0.jsonl"
loop_env node "$loop"
expect "verdict status" "$status" 0
contains "verdict" "$output" "The verdict is in"
expect "nothing uploaded after the verdict" "$(files_of sprt-blocks-toy-1-1-0)" ""
sleep 0.3
checks=$((checks + 1))
if running "$sleeper"; then
  fail "the tournament still runs after the verdict"
  kill "$sleeper"
fi
# An arena that died: what exists is uploaded, and the loop fails.
new_shard 3
echo b0 >"$work/blocks/block-0.jsonl"
kill "$sleeper"
sleep 0.3
loop_env node "$loop"
expect "died status" "$status" 1
contains "died" "$output" "ended before block 1"
expect "died batch" "$(files_of sprt-blocks-toy-1-1-0)" "1-0.jsonl"
# An arena that hangs is killed and fails the loop.
new_shard 3
loop_env env HUNG_SECONDS=1 node "$loop"
expect "hung status" "$status" 1
contains "hung" "$output" "finished no block"
sleep 0.3
checks=$((checks + 1))
if running "$sleeper"; then
  fail "the hung tournament still runs"
  kill "$sleeper"
fi
# A partial re-run is refused.
run_attempt() { status=0; output="$(PREPARE_ATTEMPT=1 RUN_ATTEMPT=2 "$script" shard-start toy games/toy/releases/toy-v002.rs games/toy/releases/toy-v001.rs 0 2>&1)" || status=$?; }
run_attempt
expect "partial re-run" "$status" 2
contains "partial re-run" "$output" "re-run all jobs"

# ---- the coordinator ----
cwork="$tmp_dir/cwork"
cres="$tmp_dir/cres"
mkdir -p "$cwork"
printf 'Candidate: toy-v002. Baseline: toy-v001.\n\nSmoke test against toy-random:\n  ok\n' >"$cwork/head.txt"
echo 0 >"$cwork/smoke-status"
# A stand-in arena: 4 blocks (2 rounds of 2) accept, fewer have no verdict.
cat >"$repo/target/release/toy-arena" <<'ARENA'
#!/usr/bin/env bash
blocks=0
for arg in "$@"; do [[ $arg == --block ]] && blocks=$((blocks + 1)); done
echo "SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 1.00 (bounds -2.94, 2.94) after $((blocks * 10)) pairs"
if ((blocks >= 4)); then
  echo "Verdict: ACCEPTED: toy-v002 is stronger than toy-v001"
  exit 0
fi
echo "Verdict: NO VERDICT YET"
exit 4
ARENA
chmod +x "$repo/target/release/toy-arena"
# Artifacts: calibrations, and batches with the blocks of rounds 0 and 1
# (round 2 is partial: only shard 0's block).
python3 -I - "$FAKE" <<'PY'
import sys, zipfile
fake = sys.argv[1]
rows = []
def add(id, name, files):
    rows.append(f"{id}\t{name}")
    with zipfile.ZipFile(f"{fake}/zips/{id}.zip", "w") as z:
        for f in files:
            z.writestr(f, "{}\n")
add(1004, "sprt-blocks-toy-1-0-1", ["0-2.jsonl"])
add(1003, "sprt-blocks-toy-1-1-0", ["1-0.jsonl", "1-1.jsonl", "../evil.txt"])
add(1002, "sprt-blocks-toy-1-0-0", ["0-0.jsonl", "0-1.jsonl"])
add(1, "sprt-blocks-toy-2-0-0", ["0-0.jsonl"])
add(2, "game-records-toy", ["x.json"])
open(f"{fake}/artifacts.tsv", "w").write("\n".join(rows) + "\n")
for shard in (0, 1):
    with zipfile.ZipFile(f"{fake}/zips/{100 + shard}.zip", "w") as z:
        z.writestr("c.txt", f"shard {shard}\n  shard {shard}: calibration line {shard}\n")
PY
printf 'ice / Shard 0 (toy)\tin_progress\nice / Shard 1 (toy)\tin_progress\n' >"$FAKE/jobs.tsv"
export SPRT_WORK="$cwork" SPRT_RESULT="$cres" POLL_SECONDS=0 REPO=o/r RUN_ID=5 RUN_ATTEMPT=1
run coordinate toy games/toy/releases/toy-v002.rs games/toy/releases/toy-v001.rs
expect "coordinate status" "$status" 0
expect "accepted" "$(cat "$cres/status")" 0
report="$(cat "$cres/report.txt")"
contains "report head" "$report" "Candidate: toy-v002. Baseline: toy-v001."
contains "report calibration" "$report" "shard 1: calibration line 1"
contains "report sprt" "$report" "Verdict: ACCEPTED"
checks=$((checks + 1))
if [[ ! -e $cwork/coordinator/blocks/0-2.jsonl || -e $cwork/coordinator/blocks/1-2.jsonl || ! -e $cwork/coordinator/blocks/1-1.jsonl ]]; then
  fail "the coordinator should hold rounds 0 and 1 and the partial round's block of shard 0 only"
fi
expect "origin" "$(cat "$cres/origin")" "5-1"
# Shards that ended with no verdict: an error, after one more look.
rm -rf "$cwork/coordinator" "$cres"
grep -v -e 'sprt-blocks-toy-1-1-0' "$FAKE/artifacts.tsv" >"$FAKE/a.tsv"
mv "$FAKE/a.tsv" "$FAKE/artifacts.tsv"
printf 'ice / Shard 0 (toy)\tcompleted\nice / Shard 1 (toy)\tcompleted\n' >"$FAKE/jobs.tsv"
run coordinate toy games/toy/releases/toy-v002.rs games/toy/releases/toy-v001.rs
expect "ended status" "$status" 0
expect "error status" "$(cat "$cres/status")" 2
contains "ended report" "$(cat "$cres/report.txt")" "every shard ended before a verdict"
# A failed smoke test is the result.
rm -rf "$cres"
echo 1 >"$cwork/smoke-status"
run coordinate toy games/toy/releases/toy-v002.rs games/toy/releases/toy-v001.rs
expect "smoke failed" "$(cat "$cres/status")" 1

# The tournament is started detached: through `annotate.sh run`, which copies
# stderr through a pipe on a higher descriptor and waits for it, the call must
# return at once while the command runs on (2026-11-04: it waited until the
# tournament ended, so the uploads came only at the end).
started="$(date +%s)"
"$repo_root/scripts/annotate.sh" run "launch" "$script" launch-detached "$tmp_dir/launch.log" "$tmp_dir/launch.pid" sleep 8 >/dev/null 2>&1
took=$(($(date +%s) - started))
expect "launch-detached returns at once through annotate.sh" "$((took < 4))" 1
pid="$(cat "$tmp_dir/launch.pid")"
if kill -0 "$pid" 2>/dev/null; then alive=1; else alive=0; fi
expect "launch-detached leaves the command running" "$alive" 1
kill -- "-$pid" 2>/dev/null || kill "$pid" 2>/dev/null || true

if ((failures > 0)); then
  echo "FAIL sprt-blocks: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   sprt-blocks: $checks checks"
