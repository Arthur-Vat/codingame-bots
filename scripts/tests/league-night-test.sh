#!/usr/bin/env bash
# Tests scripts/league-night.sh (ADR 0049, 0050) in a repository of its own
# with a stand-in arena and cargo: the arguments, the arena's options built
# from the settings for the two waves, a shard without a line in the plan,
# the merge with a drift line and with no file, and the leaderboard.
# Usage: scripts/tests/league-night-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_OUTPUT LEAGUE_NIGHT LEAGUE_COMMIT RECORDS_SAMPLE
unset "${!LEAGUE_@}" TIME_SCALE TIME_TOLERANCE_MS SPRT_TIME_SCALE SPRT_TIME_TOLERANCE_MS

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# A repository of its own: the script, the settings library, a game "toy"
# with two releases and one baseline.
repo="$tmp_dir/repo"
mkdir -p "$repo/scripts/lib" "$repo/games/toy/releases" "$repo/games/toy/journal" "$repo/target/release" "$repo/bin"
cp "$repo_root/scripts/league-night.sh" "$repo/scripts/"
cp "$repo_root/scripts/lib/evaluation.sh" "$repo/scripts/lib/"
touch "$repo/games/toy/releases/toy-v001.rs" "$repo/games/toy/releases/toy-v002.rs" "$repo/games/toy/journal/README.md"
cat >"$repo/games/toy/evaluation.env" <<'ENV'
TIME_SCALE=1
TIME_TOLERANCE_MS=0
SEED=1
OPENING_PLIES=0
MAX_TIMEOUT_RATE=0.01
LEAGUE_BOTS=random,greedy
LEAGUE_ANCHOR=v001
LEAGUE_PAIRS=800
LEAGUE_FIXED=greedy,v009
LEAGUE_NEAREST=5
LEAGUE_NIGHTLY=true
LEAGUE_NIGHTLY_RUNNERS=2
LEAGUE_NIGHTLY_MINUTES=180
LEAGUE_NIGHTLY_PAIRS=3000
LEAGUE_TARGET_MARGIN=10
LEAGUE_TOP=10
LEAGUE_BLOCK_PAIRS=200
LEAGUE_LOW_SHARE=0.2
ENV

# Stand-ins: cargo builds nothing; the arena logs its arguments, `plan`
# writes two lines (one new), `merge` reports a drift and copies the files,
# `leaderboard` writes a line.
printf '#!/usr/bin/env bash\nexit 0\n' >"$repo/bin/cargo"
cat >"$repo/target/release/toy-arena" <<'STUB'
#!/usr/bin/env bash
echo "$*" >>"$STUB_LOG"
mode="$1"
shift
out=""
results=""
adds=()
while (($#)); do
  case $1 in
    --out) out=$2; shift ;;
    --results) results=$2; shift ;;
    --add) adds+=("$2"); shift ;;
  esac
  shift
done
case $mode in
  plan)
    {
      echo '{"a":"v002","b":"v001","pairs":800,"seed":1,"kind":"new","shard":0}'
      echo '{"a":"v002","b":"greedy","pairs":200,"seed":2,"kind":"top","shard":0}'
    } >"$out"
    echo "plan for the night: new: 1 lines"
    ;;
  merge)
    cat "$results" "${adds[@]}" >"$out"
    echo "drift v002 vs v001: scored 60.0% against a stored 50.0% (interval 47.0% to 53.0%)"
    echo "warning: drift v002 vs v001 scored outside its stored interval"
    echo "merged ${#adds[@]} new lines into $out"
    ;;
  leaderboard) echo "# Leaderboard" >"$out" ;;
  *) exit 2 ;;
esac
STUB
chmod +x "$repo/bin/cargo" "$repo/target/release/toy-arena"
export STUB_LOG="$tmp_dir/arena.log"
export PATH="$repo/bin:$PATH"

# run <args>...: runs the script in the repository; sets status and output.
status=0
output=""
run() {
  status=0
  : >"$STUB_LOG"
  output="$(cd "$repo" && scripts/league-night.sh "$@" 2>&1)" || status=$?
}

expect_status() { # expect_status <what> <status>
  checks=$((checks + 1))
  if ((status != $2)); then fail "$1: expected status $2, got $status: $output"; fi
}
expect_output() { # expect_output <what> <text>
  checks=$((checks + 1))
  if [[ $output != *"$2"* ]]; then fail "$1: the output lacks '$2': $output"; fi
}
expect_log() { # expect_log <what> <text>
  checks=$((checks + 1))
  if ! grep -qF -- "$2" "$STUB_LOG"; then fail "$1: the arena's arguments lack '$2': $(cat "$STUB_LOG")"; fi
}

# The arguments.
run
expect_status "no mode" 2
run nothing
expect_status "unknown mode" 2
run plan toy 3 "$tmp_dir/none" "$tmp_dir/plan"
expect_status "wave 3" 2
expect_output "wave 3" "WAVE must be 1 or 2"
run plan toy 1 "$tmp_dir/none" "$tmp_dir/plan"
expect_status "no results file" 2
run plan '../toy' 1 "$tmp_dir/none" "$tmp_dir/plan"
expect_status "bad game" 2
run play toy "$tmp_dir/none" 0 95
expect_status "no plan file" 2
run play toy "$tmp_dir/plan" x 95
expect_status "bad shard" 2
run merge toy "$tmp_dir/none" "$tmp_dir/out" "$tmp_dir/a"
expect_status "merge: no results file" 2
run publish toy
expect_status "publish: no arguments" 2

# The plan: the pool, the options and the output for each wave.
: >"$tmp_dir/results.jsonl"
export GITHUB_OUTPUT="$tmp_dir/github-output"
: >"$GITHUB_OUTPUT"
LEAGUE_NIGHT=2026-11-01 run plan toy 1 "$tmp_dir/results.jsonl" "$tmp_dir/plan1.jsonl"
expect_status "plan, wave 1" 0
expect_log "wave 1" "--pool random,greedy,v001,v002 --releases v001,v002 --anchor v001 --fixed-opponents greedy --nearest 5 --new-pairs 800 --budget-pairs 3000"
expect_log "wave 1" "--block-pairs 200 --top 10 --target-margin 10 --low-share 0.2 --night 2026-11-01 --shards 2 --out $tmp_dir/plan1.jsonl"
expect_output "wave 1" "plan for the night: new: 1 lines"
expect_output "wave 1" "lines=2"
expect_output "wave 1" "new=1"
checks=$((checks + 1))
if [[ "$(cat "$GITHUB_OUTPUT")" != $'lines=2\nnew=1' ]]; then fail "GITHUB_OUTPUT: $(cat "$GITHUB_OUTPUT")"; fi
LEAGUE_NIGHT=2026-11-01 run plan toy 2 "$tmp_dir/results.jsonl" "$tmp_dir/plan2.jsonl"
expect_status "plan, wave 2" 0
expect_log "wave 2" "--budget-pairs 0"
expect_log "wave 2" "--night 2026-11-01-2"
LEAGUE_NIGHT=tomorrow run plan toy 1 "$tmp_dir/results.jsonl" "$tmp_dir/plan1.jsonl"
expect_status "a night that is not a date" 2
unset GITHUB_OUTPUT
run plan toy 1 "$tmp_dir/results.jsonl" "$tmp_dir/plan1.jsonl"
expect_status "plan, today" 0
expect_log "today" "--night $(date -u +%F) "
LEAGUE_NIGHTLY_RUNNERS=0 run plan toy 1 "$tmp_dir/results.jsonl" "$tmp_dir/plan1.jsonl"
expect_status "no runner" 2
expect_output "no runner" "LEAGUE_NIGHTLY_RUNNERS must be a positive number"

# A shard without a line in the plan plays nothing and succeeds, without a
# build, an arena or a bot.
: >"$STUB_LOG"
run play toy "$tmp_dir/plan1.jsonl" 1 95
expect_status "an empty shard" 0
expect_output "an empty shard" "Shard 1 has no line"
checks=$((checks + 1))
if [[ -s $STUB_LOG ]] || [[ ! -f $repo/target/league/toy-night-1.jsonl || -s $repo/target/league/toy-night-1.jsonl ]]; then
  fail "an empty shard: the arena was called or the output is not an empty file"
fi
: >"$tmp_dir/empty-plan.jsonl"
run play toy "$tmp_dir/empty-plan.jsonl" 0 95
expect_status "an empty plan" 0

# The merge: the lines are added, a drift line becomes an annotation.
line() { printf '{"a":"%s","b":"%s","pairs":1}\n' "$1" "$2"; }
line v001 greedy >"$tmp_dir/stored.jsonl"
line v002 v001 >"$tmp_dir/shard-0.jsonl"
line v002 greedy >"$tmp_dir/shard-1.jsonl"
run merge toy "$tmp_dir/stored.jsonl" "$tmp_dir/merged.jsonl" "$tmp_dir/shard-0.jsonl" "$tmp_dir/shard-1.jsonl"
expect_status "merge" 0
expect_log "merge" "merge --results $tmp_dir/stored.jsonl --add $tmp_dir/shard-0.jsonl --add $tmp_dir/shard-1.jsonl --out $tmp_dir/merged.jsonl"
expect_output "merge" "::warning title=League drift::drift v002 vs v001 scored outside its stored interval"
expect_output "merge" "merged 2 new lines"
checks=$((checks + 1))
if [[ "$(wc -l <"$tmp_dir/merged.jsonl")" != 3 ]]; then fail "merge: the merged file: $(cat "$tmp_dir/merged.jsonl")"; fi
# No file to add: a copy.
run merge toy "$tmp_dir/stored.jsonl" "$tmp_dir/copy.jsonl"
expect_status "merge with no file" 0
checks=$((checks + 1))
if ! cmp -s "$tmp_dir/stored.jsonl" "$tmp_dir/copy.jsonl"; then fail "merge with no file: not a copy"; fi
run merge toy "$tmp_dir/stored.jsonl" "$tmp_dir/stored.jsonl"
expect_status "merge with no file, in place" 0
run merge toy "$tmp_dir/stored.jsonl" "$tmp_dir/out.jsonl" "$tmp_dir/missing.jsonl"
expect_status "merge: a missing file" 2

# The leaderboard.
run publish toy "$tmp_dir/stored.jsonl" "$tmp_dir/LEADERBOARD.md"
expect_status "publish" 0
expect_log "publish" "leaderboard --results $tmp_dir/stored.jsonl --anchor v001 --journal games/toy/journal/README.md --out $tmp_dir/LEADERBOARD.md"

if ((failures > 0)); then
  echo "FAIL league-night: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   league-night: $checks checks"
