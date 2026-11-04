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
uploads="$(grep -c '^      - \*upload$' "$repo_root/.github/workflows/sprt-parallel.yml")"
max="$(sed -n 's/^readonly MAX_UPLOADS=//p' "$repo_root/scripts/sprt-blocks.sh")"
expect "upload steps (anchor included)" "$((uploads + 1))" "$max"

# ---- the shard uploads its blocks in batches ----
cat >"$repo/bin/gh" <<'GH'
#!/usr/bin/env bash
# Stand-in gh api: serves the folder $FAKE.
url="$2"
case $url in
  *"name=sprt-verdict-"*) cat "$FAKE/verdict-count" ;;
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
echo 0 >"$FAKE/verdict-count"
work="$tmp_dir/work"
mkdir -p "$work/blocks" "$work/upload"
echo 0 >"$work/next"
echo 0 >"$work/uploads"
echo 0 >"$work/last-upload"
echo old0 >"$work/blocks/block-0.jsonl"
echo old1 >"$work/blocks/block-1.jsonl"
sleep 300 &
sleeper=$!
echo "$sleeper" >"$work/player.pid"
envfile="$tmp_dir/env"
: >"$envfile"
last_env() { grep "^$1=" "$envfile" | tail -n 1; }
export SPRT_WORK="$work" GITHUB_ENV="$envfile" REPO=o/r RUN_ID=5 RUN_ATTEMPT=1 WAIT_SECONDS=0 UPLOAD_SECONDS=0
run shard-wait toy 1
expect "wait status" "$status" 0
expect "batch name" "$(last_env SPRT_BLOCK)" "SPRT_BLOCK=sprt-blocks-toy-1-1-0"
expect "batch folder" "$(last_env SPRT_BLOCK_FILE)" "SPRT_BLOCK_FILE=$work/upload/batch-0"
expect "batch holds both blocks" "$(find "$work/upload/batch-0" -type f -printf '%f ' | tr ' ' '\n' | sort | paste -sd' ' -)" "1-0.jsonl 1-1.jsonl"
expect "block copied" "$(cat "$work/upload/batch-0/1-1.jsonl")" old1
# Nothing new: no upload, not over.
run shard-wait toy 1
expect "nothing new" "$(last_env SPRT_UPLOAD)" "SPRT_UPLOAD=false"
checks=$((checks + 1))
if grep -q SPRT_DONE "$envfile"; then fail "the shard is not over"; fi
# A block more: the next batch holds only it.
echo old2 >"$work/blocks/block-2.jsonl"
run shard-wait toy 1
expect "second batch" "$(find "$work/upload/batch-1" -type f -printf '%f')" "1-2.jsonl"
# Before the interval is up the step waits (a long interval, a dead tournament ends it).
kill "$sleeper"
wait "$sleeper" 2>/dev/null || true
: >"$work/player.log"
run shard-wait toy 1
expect "dead tournament still exits 0" "$status" 0
contains "dead tournament" "$(cat "$envfile")" "SPRT_DONE=true"
run shard-finish
expect "finish fails after a death" "$status" 1
# The last blocks end the shard; later steps do nothing.
rm -f "$work/died" "$work/finished"
for n in $(seq 3 19); do echo "b$n" >"$work/blocks/block-$n.jsonl"; done
sleep 300 &
sleeper=$!
echo "$sleeper" >"$work/player.pid"
run shard-wait toy 1
contains "all blocks" "$output" "played its 20 blocks"
expect "last batch" "$(find "$work/upload/batch-2" -type f | wc -l | tr -d ' ')" 17
run shard-wait toy 1
expect "after the end" "$(last_env SPRT_UPLOAD)" "SPRT_UPLOAD=false"
# A verdict stops it, and the tournament with it.
rm -f "$work/finished"
echo 1 >"$FAKE/verdict-count"
run shard-wait toy 1
contains "verdict" "$output" "The verdict is in"
checks=$((checks + 1))
if kill -0 "$sleeper" 2>/dev/null; then
  fail "the tournament still runs after the verdict"
  kill "$sleeper"
fi
unset SPRT_WORK UPLOAD_SECONDS

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
export SPRT_WORK="$cwork" SPRT_RESULT="$cres" POLL_SECONDS=0
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

if ((failures > 0)); then
  echo "FAIL sprt-blocks: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   sprt-blocks: $checks checks"
