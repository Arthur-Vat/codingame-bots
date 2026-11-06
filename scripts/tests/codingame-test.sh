#!/usr/bin/env bash
# Tests scripts/codingame.sh without the network (fetch, batch, replay and
# fetch-ids): a stand-in for curl answers from synthetic JSON files (made-up
# players and games) or from queues of answers, can answer 429 once and fails
# chosen requests; a stand-in for sleep logs the waits. Needs jq. Usage: scripts/tests/codingame-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset CODINGAME_API CODINGAME_PACE

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
lacks() { # lacks <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 == *"$3"* ]]; then fail "$1: '$3' found in: $2"; fi
}

# A stand-in repository with a game that has a puzzle and one without.
repo="$tmp_dir/repo"
mkdir -p "$repo/scripts" "$repo/games/toy" "$repo/games/nopuzzle"
cp "$repo_root/scripts/codingame.sh" "$repo/scripts/"
printf 'SEED=1\nCODINGAME_PUZZLE=toy-puzzle\n' >"$repo/games/toy/evaluation.env"
printf 'SEED=1\n' >"$repo/games/nopuzzle/evaluation.env"
script="$repo/scripts/codingame.sh"

# The stand-in curl: the URL (after the base) with / ? & = turned into _ names
# a file of $STUB_DIR; <name>.status forces a status; <name>.429once answers
# 429 with Retry-After: 0 the first time. It logs the arguments and the
# headers it was given.
stub="$tmp_dir/curl-stub"
cat >"$stub" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
all="$*"
headers="" body="" format="" hfile="" url="" postfile=""
while (($#)); do
  case "$1" in
    -D) headers="$2"; shift 2 ;;
    -o) body="$2"; shift 2 ;;
    -w) format="$2"; shift 2 ;;
    -H) [[ $2 == @* ]] && hfile="${2#@}"; shift 2 ;;
    -X) shift 2 ;;
    --data-binary) postfile="${2#@}"; shift 2 ;;
    -sS) shift ;;
    *) url="$1"; shift ;;
  esac
done
echo "$all" >>"$STUB_DIR/requests.log"
cat "$hfile" >>"$STUB_DIR/auth.log"
if [[ -n $postfile ]]; then
  n=$(( $(cat "$STUB_DIR/posts.count" 2>/dev/null || echo 0) + 1 ))
  echo "$n" >"$STUB_DIR/posts.count"
  cp "$postfile" "$STUB_DIR/post-$n.json"
fi
name="$(sed 's|^http://stub||; s|[/?&=]|_|g; s|^_||' <<<"$url")"
reply() { : >"$headers"; [[ ${3:-} == retry ]] && echo "Retry-After: 0" >"$headers"; printf '%s' "$2" >"$body"; printf '%s' "${format/\%\{http_code\}/$1}"; }
first=""
if [[ -d $STUB_DIR/$name.queue ]]; then first="$(ls "$STUB_DIR/$name.queue" | sort | head -n 1)"; fi
if [[ -n $first ]]; then
  # A queue of answers: the first line of a file is the status, the rest the body.
  qf="$STUB_DIR/$name.queue/$first"
  qstatus="$(head -n 1 "$qf")"
  qbody="$(tail -n +2 "$qf")"
  rm "$qf"
  reply "$qstatus" "$qbody"
elif [[ -f $STUB_DIR/$name.429once ]]; then
  rm "$STUB_DIR/$name.429once"
  reply 429 '{"error":"rate-limited"}' retry
elif [[ -f $STUB_DIR/$name.status ]]; then
  reply "$(cat "$STUB_DIR/$name.status")" "$(cat "$STUB_DIR/$name.body" 2>/dev/null || echo '{"error":"x"}')"
elif [[ -f $STUB_DIR/$name.json ]]; then
  reply 200 "$(cat "$STUB_DIR/$name.json")"
else
  reply 404 '{"error":"not-found"}'
fi
STUB
chmod +x "$stub"

token="tok-SECRET-0123456789"
printf '%s\n' "$token" >"$tmp_dir/token"
export CODINGAME_TOKEN_FILE="$tmp_dir/token" CODINGAME_CURL="$stub" CODINGAME_PACE=0
export CODINGAME_API="http://stub"
export STUB_DIR="$tmp_dir/stub"
mkdir -p "$STUB_DIR"

puzzle_url="puzzles_toy-puzzle"
echo '{"title":"Toy"}' >"$STUB_DIR/$puzzle_url.json"
echo '{"total":2,"rows":[{"rank":1,"nickname":"Alpha","agent_id":11,"score":9.5},{"rank":2,"nickname":"Beta","agent_id":22,"score":8.5}]}' \
  >"$STUB_DIR/${puzzle_url}_leaderboard_limit_100.json"
battle() { # battle <id> <done> <ended_at>
  echo "{\"game_id\":$1,\"done\":$2,\"ended_at\":\"$3\",\"my_position\":0,\"players\":[]}"
}
echo "{\"submission_id\":77,\"battles\":[$(battle 300 true 2026-01-03T10:00:00.000Z),$(battle 250 false null)],\"next_before\":250}" \
  >"$STUB_DIR/${puzzle_url}_battles_limit_100.json"
echo "{\"submission_id\":77,\"battles\":[$(battle 200 true 2026-01-02T10:00:00.000Z),$(battle 150 true 2026-01-01T10:00:00.000Z)]}" \
  >"$STUB_DIR/${puzzle_url}_battles_limit_100_before_250.json"
log() { # log <id> <our seat> <options json>
  local other=$((1 - $2))
  echo "{\"game_id\":$1,\"game_options\":$3,\"agents\":[{\"index\":$2,\"name\":\"Me\",\"agent_id\":1,\"me\":true},{\"index\":$other,\"name\":\"Rival$1\",\"agent_id\":$((1000 + $1))}],\"frames\":[{\"x\":1}],\"scores\":[1,0],\"ranking\":[1,0],\"limits\":{\"requests\":{\"remaining\":5}}}"
}
log 300 1 '"seed=42"' >"$STUB_DIR/games_300_format_json.json"
log 200 0 null >"$STUB_DIR/games_200_format_json.json"
log 150 0 '"seed=7"' >"$STUB_DIR/games_150_format_json.json"
touch "$STUB_DIR/games_200_format_json.429once"
echo 500 >"$STUB_DIR/games_150_format_json.status"
mv "$STUB_DIR/games_150_format_json.json" "$tmp_dir/log150.json"

run() { # run <args...>: sets out (stdout and stderr) and rc
  rc=0
  out="$("$script" "$@" 2>&1)" || rc=$?
}
data="$tmp_dir/data"
out_dir="$data/toy"

# First run: two games stored, one fails, one unfinished; exit 1.
run fetch toy --data "$data"
expect "first run exit" "$rc" 1
contains "summary listed" "$out" "games listed: 4"
contains "summary fetched" "$out" "fetched: 2"
contains "summary failed" "$out" "failed: 1"
contains "summary unfinished" "$out" "not finished: 1"
contains "failure reported" "$out" "failed: game 150 answered 500"
contains "429 waited" "$out" "rate limited"
expect "logs stored" "$(find "$out_dir/logs" -name "*.json" -printf "%f\n" | sort | tr "\n" " ")" "200.json 300.json "
expect "limits removed" "$(jq -c 'has("limits")' "$out_dir/logs/300.json")" "false"
expect "log kept whole" "$(jq -c '.frames' "$out_dir/logs/300.json")" '[{"x":1}]'
expect "log is one line" "$(wc -l <"$out_dir/logs/300.json" | tr -d ' ')" "1"
expect "index sorted, one line a game" "$(jq -r '.game_id' "$out_dir/index.jsonl" | tr '\n' ' ')" "200 300 "
line300='{"game_id":300,"kind":"ranking","puzzle":"toy-puzzle","submission_id":77,"ended_at":"2026-01-03T10:00:00.000Z","fetched_at":"FETCHED","our_seat":1,"players":[{"seat":1,"name":"Me","agent_id":1,"me":true},{"seat":0,"name":"Rival300","agent_id":1300}],"ranking":[1,0],"game_options":"seed=42"}'
actual="$(sed -n 2p "$out_dir/index.jsonl")"
fetched_at="$(jq -r '.fetched_at' <<<"$actual")"
expect "index line of 300" "$actual" "${line300/FETCHED/$fetched_at}"
expect "null game options" "$(sed -n 1p "$out_dir/index.jsonl" | jq -c '.game_options')" "null"
snapshots=("$out_dir"/leaderboard/*.json)
expect "one snapshot" "${#snapshots[@]}" 1
expect "snapshot named after fetched_at" "$(basename "${snapshots[0]}" .json)" "$(jq -r '.fetched_at | gsub(":"; "")' "${snapshots[0]}")"
expect "snapshot" "$(jq -c '[.puzzle, (.fetched_at | test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:]{8}Z$")), (.rows | length), .rows[0].nickname]' "${snapshots[0]}")" '["toy-puzzle",true,2,"Alpha"]'
contains "paging asked for the second page" "$(cat "$STUB_DIR/requests.log")" "battles?limit=100&before=250"

# Neither the token's text nor its place in an argument shows anywhere.
lacks "token in output" "$out" "$token"
lacks "token in arguments" "$(cat "$STUB_DIR/requests.log")" "$token"
contains "arguments are logged" "$(cat "$STUB_DIR/requests.log")" "-H @"
expect "token in the data directory" "$(grep -rlF -- "$token" "$data" || true)" ""
contains "token sent in the header" "$(cat "$STUB_DIR/auth.log")" "Authorization: Bearer $token"

# Second run with the failing game fixed: only that game is fetched.
rm "$STUB_DIR/games_150_format_json.status"
mv "$tmp_dir/log150.json" "$STUB_DIR/games_150_format_json.json"
: >"$STUB_DIR/requests.log"
run fetch toy --data "$data"
expect "second run exit" "$rc" 0
contains "second run fetched" "$out" "fetched: 1"
contains "second run stored" "$out" "already stored: 2"
expect "logs fetched" "$(grep -c 'games/' "$STUB_DIR/requests.log")" 1
expect "index has three" "$(jq -r '.game_id' "$out_dir/index.jsonl" | tr '\n' ' ')" "150 200 300 "

# A rerun fetches nothing and keeps the index.
cp "$out_dir/index.jsonl" "$tmp_dir/index.before"
: >"$STUB_DIR/requests.log"
run fetch toy --data "$data"
expect "rerun exit" "$rc" 0
contains "rerun fetched none" "$out" "fetched: 0"
expect "rerun fetched no log" "$(grep -c 'games/' "$STUB_DIR/requests.log" || true)" 0
expect "index unchanged" "$(cmp "$out_dir/index.jsonl" "$tmp_dir/index.before" && echo same)" same

# An interrupted run: a log without its index line is indexed, not fetched.
grep -v '"game_id":300,' "$tmp_dir/index.before" >"$out_dir/index.jsonl"
: >"$STUB_DIR/requests.log"
run fetch toy --data "$data"
expect "recovery exit" "$rc" 0
contains "recovered" "$out" "indexed from stored logs: 1"
expect "recovery fetched no log" "$(grep -c 'games/' "$STUB_DIR/requests.log" || true)" 0
expect "recovered line" "$(jq -c 'del(.fetched_at)' "$out_dir/index.jsonl" | sed -n 3p)" "$(jq -c 'del(.fetched_at)' <<<"${line300/FETCHED/x}")"
expect "index again three" "$(wc -l <"$out_dir/index.jsonl" | tr -d ' ')" 3

# A truncated stored log (a run cut short) is fetched again and indexed.
printf '{"game_id":200,"agents":[{"ind' >"$out_dir/logs/200.json"
grep -v '"game_id":200,' "$out_dir/index.jsonl" >"$tmp_dir/index.cut"
cp "$tmp_dir/index.cut" "$out_dir/index.jsonl"
: >"$STUB_DIR/requests.log"
run fetch toy --data "$data"
expect "truncated log exit" "$rc" 0
contains "damaged log reported" "$out" "damaged"
expect "truncated log fetched again" "$(grep -c 'games/200' "$STUB_DIR/requests.log")" 1
expect "truncated log fixed" "$(jq -c 'has("agents")' "$out_dir/logs/200.json")" "true"
expect "index complete again" "$(jq -r '.game_id' "$out_dir/index.jsonl" | tr '\n' ' ')" "150 200 300 "
expect "no temporary file left" "$(find "$data" -name '*.tmp' | wc -l | tr -d ' ')" 0

# The API's hint and curl's own words reach the error.
echo 503 >"$STUB_DIR/${puzzle_url}_leaderboard_limit_100.status"
echo '{"error":"unavailable","hint":"try again later"}' >"$STUB_DIR/${puzzle_url}_leaderboard_limit_100.body"
run fetch toy --data "$tmp_dir/data5"
expect "503 exit" "$rc" 1
contains "hint shown" "$out" "answered 503 (try again later)"
rm "$STUB_DIR/${puzzle_url}_leaderboard_limit_100.status" "$STUB_DIR/${puzzle_url}_leaderboard_limit_100.body"

# --submission is passed on and recorded.
: >"$STUB_DIR/requests.log"
echo "{\"submission_id\":5,\"battles\":[]}" >"$STUB_DIR/${puzzle_url}_battles_limit_100_submission_5.json"
run fetch toy --data "$tmp_dir/data2" --submission 5
expect "submission exit" "$rc" 0
contains "submission summary" "$out" "games listed: 0"
contains "submission requested" "$(cat "$STUB_DIR/requests.log")" "battles?limit=100&submission=5"
expect "empty index" "$(wc -c <"$tmp_dir/data2/toy/index.jsonl" | tr -d ' ')" 0

# Malformed answers name the URL and write no index line.
echo 'not json' >"$STUB_DIR/${puzzle_url}_battles_limit_100.json"
run fetch toy --data "$tmp_dir/data3"
expect "malformed exit" "$rc" 1
contains "malformed names the URL" "$out" "http://stub/puzzles/toy-puzzle/battles?limit=100"
expect "malformed index empty" "$(wc -c <"$tmp_dir/data3/toy/index.jsonl" | tr -d ' ')" 0

# Setup errors exit 2.
run fetch nopuzzle --data "$tmp_dir/data4"
expect "no puzzle exit" "$rc" 2
contains "no puzzle message" "$out" "CODINGAME_PUZZLE"
CODINGAME_TOKEN_FILE="$tmp_dir/missing" run fetch toy --data "$tmp_dir/data4"
expect "no token exit" "$rc" 2
contains "no token message" "$out" "docs/CODINGAME.md"
run fetch
expect "no subcommand" "$rc" 2
run fetch toy --bogus
expect "bad option" "$rc" 2
run fetch toy --data "$tmp_dir/data4" --push
expect "push with data" "$rc" 2
run -h
expect "help" "$rc" 0
contains "usage text" "$out" "Usage: scripts/codingame.sh fetch"

# Without --data: a worktree of the data branch, in a clone with a bare
# repository as its origin (offline).
export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@example.com GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@example.com
bare="$tmp_dir/origin.git"
clone="$tmp_dir/clone"
git init -q --bare "$bare"
git clone -q "$bare" "$clone" 2>/dev/null
mkdir -p "$clone/scripts" "$clone/games/toy"
cp "$repo/scripts/codingame.sh" "$clone/scripts/"
cp "$repo/games/toy/evaluation.env" "$clone/games/toy/"
git -C "$clone" add -A
git -C "$clone" commit -q -m "chore(repo): stand-in"
branch=refs/heads/claude/codingame-games
cscript="$clone/scripts/codingame.sh"
echo "{\"submission_id\":77,\"battles\":[$(battle 300 true 2026-01-03T10:00:00.000Z),$(battle 200 true 2026-01-02T10:00:00.000Z)]}" \
  >"$STUB_DIR/${puzzle_url}_battles_limit_100.json"

# (1) No --push, no remote branch: an orphan worktree, nothing pushed.
rc=0
out="$("$cscript" fetch toy 2>&1)" || rc=$?
expect "worktree run exit" "$rc" 0
wt="$(sed -n 's/^data directory: //p' <<<"$out")"
contains "worktree path printed" "$out" "data directory: $wt"
expect "worktree has the index" "$(wc -l <"$wt/toy/index.jsonl" | tr -d ' ')" 2
expect "worktree holds only the data" "$(find "$wt" -mindepth 1 -maxdepth 1 -not -name .git | sed 's|.*/||' | tr '\n' ' ')" "toy "
contains "push hint" "$out" "--push"
expect "nothing pushed" "$(git -C "$bare" rev-parse -q --verify "$branch" >/dev/null && echo pushed || echo none)" none
git -C "$clone" worktree remove --force "$wt"
expect "no local branch after a run" "$(git -C "$clone" for-each-ref --format='%(refname)' refs/heads | grep -c codingame || true)" 0

# A run that dies with --push leaves no worktree.
echo 503 >"$STUB_DIR/${puzzle_url}_leaderboard_limit_100.status"
rc=0
out="$("$cscript" fetch toy --push 2>&1)" || rc=$?
rm "$STUB_DIR/${puzzle_url}_leaderboard_limit_100.status"
expect "failed run exit" "$rc" 1
expect "worktree removed after a failed run" "$(git -C "$clone" worktree list | wc -l | tr -d ' ')" 1

# A rejected push: the worktree is removed, no local branch is left, a rerun works.
mkdir -p "$bare/hooks"
printf '#!/bin/sh\nexit 1\n' >"$bare/hooks/pre-receive"
chmod +x "$bare/hooks/pre-receive"
rc=0
out="$("$cscript" fetch toy --push 2>&1)" || rc=$?
expect "rejected push exit" "$rc" 1
expect "worktree removed after a rejected push" "$(git -C "$clone" worktree list | wc -l | tr -d ' ')" 1
expect "no local branch after a rejected push" "$(git -C "$clone" for-each-ref --format='%(refname)' refs/heads | grep -c codingame || true)" 0
rm "$bare/hooks/pre-receive"

# (2) --push: the branch holds only the game's files.
rc=0
out="$(CODINGAME_COMMIT_TRAILERS='Co-Authored-By: Test <t@example.com>' "$cscript" fetch toy --push 2>&1)" || rc=$?
expect "push exit" "$rc" 0
expect "subject" "$(git -C "$bare" log -1 --format=%s "$branch")" "chore(repo): codingame games: toy, 2 new games"
expect "trailer" "$(git -C "$bare" log -1 --format=%b "$branch" | head -n 1)" "Co-Authored-By: Test <t@example.com>"
expect "no local branch after a push" "$(git -C "$clone" for-each-ref --format='%(refname)' refs/heads | grep -c codingame || true)" 0
expect "only game files" "$(git -C "$bare" ls-tree -r --name-only "$branch" | grep -vc '^toy/' || true)" 0
expect "files pushed" "$(git -C "$bare" ls-tree -r --name-only "$branch" | grep -c 'logs/\|index.jsonl')" 3
expect "one commit" "$(git -C "$bare" rev-list --count "$branch")" 1
expect "worktree removed" "$(git -C "$clone" worktree list | wc -l | tr -d ' ')" 1

# (3) A new battle: a second commit on top, the old logs untouched.
log 400 0 '"seed=9"' >"$STUB_DIR/games_400_format_json.json"
echo "{\"submission_id\":77,\"battles\":[$(battle 400 true 2026-01-04T10:00:00.000Z),$(battle 300 true 2026-01-03T10:00:00.000Z),$(battle 200 true 2026-01-02T10:00:00.000Z)]}" \
  >"$STUB_DIR/${puzzle_url}_battles_limit_100.json"
first="$(git -C "$bare" rev-parse "$branch")"
rc=0
out="$("$cscript" fetch toy --push 2>&1)" || rc=$?
expect "second push exit" "$rc" 0
expect "second commit on top" "$(git -C "$bare" rev-parse "$branch^")" "$first"
expect "second subject" "$(git -C "$bare" log -1 --format=%s "$branch")" "chore(repo): codingame games: toy, 1 new game"
changed="$(git -C "$bare" diff --name-only "$first" "$branch" | grep -v 'leaderboard/' | tr '\n' ' ')"
expect "only the new log and the index changed" "$changed" "toy/index.jsonl toy/logs/400.json "
expect "index has three" "$(git -C "$bare" show "$branch:toy/index.jsonl" | wc -l | tr -d ' ')" 3
expect "worktree removed again" "$(git -C "$clone" worktree list | wc -l | tr -d ' ')" 1

# --- batch, replay and fetch-ids ---------------------------------------------

cd "$repo"
mkdir -p games/toy/releases
release_file="games/toy/releases/toy-v1.rs"
printf 'fn main() {\n\tprintln!("h\\u00e9llo \\"x\\" \303\251 \\\\");\n}\n' >"$release_file"
printf '# comment\n\nseed=1\n  seed=2  \n' >games/toy/codingame-seeds.txt
sleep_stub="$tmp_dir/sleep-stub"
# shellcheck disable=SC2016
printf '#!/usr/bin/env bash\necho "$*" >>"$STUB_DIR/sleeps.log"\n' >"$sleep_stub"
chmod +x "$sleep_stub"
export CODINGAME_SLEEP="$sleep_stub" CODINGAME_RETRY_WAIT=0
: >"$STUB_DIR/sleeps.log"

# The leaderboard has us (Mine) at rank 2.
echo '{"total":4,"rows":[{"rank":1,"nickname":"Alpha","agent_id":11},{"rank":2,"nickname":"Mine","agent_id":99,"me":true},{"rank":3,"nickname":"Beta","agent_id":22},{"rank":4,"nickname":"Gamma","agent_id":33}]}' \
  >"$STUB_DIR/${puzzle_url}_leaderboard_limit_100.json"
post_name="${puzzle_url}_test-games_wait_30_format_json"
posts() { cat "$STUB_DIR/posts.count" 2>/dev/null || echo 0; }
tglog() { # tglog <game id> <seed line> <opponent name> <opponent agent> <winner seat> <scores>
  local other=$((1 - $5))
  echo "{\"game_id\":$1,\"game_options\":\"$2\",\"agents\":[{\"index\":0,\"name\":\"Mine\",\"agent_id\":-1,\"me\":true},{\"index\":1,\"name\":\"$3\",\"agent_id\":$4}],\"frames\":[],\"scores\":$6,\"ranking\":[$5,$other]}"
}
limits() { # limits <remaining> <resets in>
  echo "{\"test_games\":{\"name\":\"test_games_1m\",\"limit\":10,\"used\":1,\"remaining\":$1,\"window_seconds\":60,\"resets_in_seconds\":$2},\"requests\":{\"remaining\":50}}"
}
enqueue() { # enqueue <queue name> <file name> <status> <body>
  mkdir -p "$STUB_DIR/$1.queue"
  printf '%s\n%s' "$3" "$4" >"$STUB_DIR/$1.queue/$2"
}
done_reply() { # done_reply <test game id> <log json> <remaining> <resets in>
  echo "{\"test_game_id\":$1,\"status\":\"done\",\"poll_url\":\"/api/v1/test-games/$1\",\"log\":$2,\"limits\":$(limits "$3" "$4")}"
}
bdata="$tmp_dir/bdata"

# A dry run prints the plan and writes nothing, and posts nothing.
run batch toy "$release_file" --top 2 --games-per-opponent 3 --name tdry --data "$tmp_dir/dry"
expect "dry run exit" "$rc" 0
contains "dry run plan" "$out" "rank 1  Alpha"
contains "dry run second opponent" "$out" "rank 3  Beta"
lacks "dry run skips us" "$out" "Mine"
lacks "dry run skips the rest" "$out" "Gamma"
contains "dry run maps cycle" "$out" "maps: seed=1 seed=2 seed=1"
contains "dry run total" "$out" "games: 6 to play of 6"
contains "dry run quota" "$out" "quota: 6 test games"
contains "dry run time" "$out" "about 3 minutes"
contains "dry run says so" "$out" "dry run: no game was played"
expect "dry run posts nothing" "$(posts)" 0
expect "dry run writes nothing" "$(find "$tmp_dir/dry" -mindepth 1 2>/dev/null | wc -l | tr -d ' ')" 0
run batch toy "$release_file" --top 2 --name tdry --push
expect "push without play" "$rc" 2

# 2 opponents x 2 maps: a done game at once, a 202 then a poll, a 409 then
# success, a failed game then success; the first answer used up the quota.
enqueue "$post_name" 01 200 "$(done_reply 5001 "$(tglog 9001 seed=1 Alpha 11 0 '[300,100]')" 0 37)"
enqueue "$post_name" 02 202 '{"test_game_id":5002,"status":"running","poll_url":"/api/v1/test-games/5002","limits":'"$(limits 9 59)"'}'
enqueue "test-games_5002_wait_30_format_json" 01 200 "$(done_reply 5002 "$(tglog 9002 seed=1 Beta 22 1 '[100,300]')" 8 50)"
enqueue "$post_name" 03 409 '{"error":"test-game-running","test_game_id":5003,"poll_url":"/api/v1/test-games/5003"}'
enqueue "test-games_5003_wait_30_format_json" 01 200 '{"test_game_id":5003,"status":"done","log":{"game_id":1,"agents":[]}}'
enqueue "$post_name" 04 200 "$(done_reply 5004 "$(tglog 9003 seed=2 Alpha 11 0 '[200,200]')" 7 40)"
enqueue "$post_name" 05 200 '{"test_game_id":5005,"status":"failed","failure":"No code machine was available to run the game in time.","hint":"wait"}'
enqueue "$post_name" 06 200 "$(done_reply 5006 "$(tglog 9004 seed=2 Beta 22 0 '[300,100]')" 6 30)"
run batch toy "$release_file" --top 2 --games-per-opponent 2 --name tb1 --play --data "$bdata"
expect "batch exit" "$rc" 0
expect "six posts" "$(posts)" 6
expect "interleaved order, retries in place" \
  "$(for n in 1 2 3 4 5 6; do jq -r '[.opponents[0], .game_options] | join(":")' "$STUB_DIR/post-$n.json"; done | tr '\n' ' ')" \
  "11:seed=1 22:seed=1 11:seed=2 11:seed=2 22:seed=2 22:seed=2 "
contains "quota wait" "$(cat "$STUB_DIR/sleeps.log")" "37"
contains "quota message" "$out" "test game quota used up, waiting 37s"
contains "failed attempt reported" "$out" "attempt 1 failed (No code machine was available to run the game in time.)"
expect "no IDE flags, no replay" "$(jq -c '[.show_in_ide, .overwrite_ide_code, .overwrite_ide_game_parameters, .language, has("replay_game_id")]' "$STUB_DIR/post-1.json")" '[false,false,false,"Rust",false]'
expect "release sent unchanged" "$(jq -j '.code' "$STUB_DIR/post-1.json" | cmp - "$release_file" && echo same)" same
expect "release sent unchanged, last post" "$(jq -j '.code' "$STUB_DIR/post-6.json" | cmp - "$release_file" && echo same)" same
expect "logs stored" "$(find "$bdata/toy/logs" -name '*.json' -printf '%f\n' | sort | tr '\n' ' ')" "9001.json 9002.json 9003.json 9004.json "
expect "log without limits" "$(jq -c 'has("limits")' "$bdata/toy/logs/9001.json")" false
line9001='{"game_id":9001,"kind":"test","puzzle":"toy-puzzle","submission_id":null,"ended_at":"NOW","fetched_at":"NOW","our_seat":0,"players":[{"seat":0,"name":"Mine","agent_id":-1,"me":true},{"seat":1,"name":"Alpha","agent_id":11}],"ranking":[0,1],"game_options":"seed=1","batch":"tb1","release":"toy-v1"}'
actual="$(sed -n 1p "$bdata/toy/index.jsonl")"
fetched_at="$(jq -r '.fetched_at' <<<"$actual")"
expect "index line of 9001" "$actual" "${line9001//NOW/$fetched_at}"
expect "ended_at is when it was seen" "$(jq -r '.ended_at == .fetched_at' <<<"$actual")" true
expect "index has four" "$(wc -l <"$bdata/toy/index.jsonl" | tr -d ' ')" 4
batch_json="$bdata/toy/batches/tb1.json"
expect "batch keys" "$(jq -c 'keys_unsorted' "$batch_json")" '["name","game","puzzle","release","release_sha256","created_at","leaderboard","opponents","seeds","games"]'
expect "batch header" "$(jq -c '[.name, .game, .puzzle, .release, .release_sha256, .seeds, .opponents]' "$batch_json")" \
  "[\"tb1\",\"toy\",\"toy-puzzle\",\"toy-v1\",\"$(sha256sum "$release_file" | cut -d' ' -f1)\",[\"seed=1\",\"seed=2\"],[{\"name\":\"Alpha\",\"agent_id\":11,\"rank\":1},{\"name\":\"Beta\",\"agent_id\":22,\"rank\":3}]]"
expect "batch games" "$(jq -c '[.games[] | [.opponent, .game_options, .status, .attempts, .test_game_id, .game_id, .failure]]' "$batch_json")" \
  '[["Alpha","seed=1","done",1,5001,9001,null],["Beta","seed=1","done",1,5002,9002,null],["Alpha","seed=2","done",1,5004,9003,null],["Beta","seed=2","done",2,5006,9004,null]]'
expect "game keys" "$(jq -c '.games[0] | keys_unsorted' "$batch_json")" '["opponent","agent_id","game_options","status","attempts","test_game_id","game_id","seconds","failure"]'
expect "seconds are numbers" "$(jq -c '[.games[].seconds | type] | unique' "$batch_json")" '["number"]'
snapshot_file="$bdata/toy/leaderboard/$(jq -r '.leaderboard' "$batch_json")"
expect "snapshot written, named in the batch" "$(jq -c '[.puzzle, (.rows | length)]' "$snapshot_file")" '["toy-puzzle",4]'
contains "result lines" "$out" "game 1/4: vs Alpha, seed=1, won,"
contains "lost line" "$out" "game 2/4: vs Beta, seed=1, lost,"
contains "draw line" "$out" "game 3/4: vs Alpha, seed=2, draw,"
contains "quota left shown" "$out" "quota left 9"
contains "totals" "$out" "Alpha: played 2, won 1, lost 0, drawn 1, failed 0"
contains "totals second" "$out" "Beta: played 2, won 1, lost 1, drawn 0, failed 0"
expect "no temporary file" "$(find "$bdata" -name '*.tmp' | wc -l | tr -d ' ')" 0

# Resume after an interruption: only the planned games are played.
jq -c '.games[2] |= (.status = "planned" | .attempts = 0 | .test_game_id = null | .game_id = null | .seconds = null)
  | .games[3] |= (.status = "planned" | .attempts = 0 | .test_game_id = null | .game_id = null | .seconds = null)' \
  "$batch_json" >"$tmp_dir/interrupted.json"
cp "$tmp_dir/interrupted.json" "$batch_json"
: >"$STUB_DIR/requests.log"
before="$(posts)"
enqueue "$post_name" 01 200 "$(done_reply 5011 "$(tglog 9011 seed=2 Alpha 11 1 '[100,300]')" 5 20)"
enqueue "$post_name" 02 200 "$(done_reply 5012 "$(tglog 9012 seed=2 Beta 22 0 '[300,100]')" 4 10)"
run batch toy "$release_file" --top 5 --name tb1 --play --data "$bdata"
expect "resume exit" "$rc" 0
contains "resumed" "$out" "batch tb1 (resumed)"
expect "resume plays two" "$(($(posts) - before))" 2
expect "resume plays the planned ones" "$(for n in $((before + 1)) $((before + 2)); do jq -r '[.opponents[0], .game_options] | join(":")' "$STUB_DIR/post-$n.json"; done | tr '\n' ' ')" "11:seed=2 22:seed=2 "
expect "resume keeps its opponents" "$(jq -r '.opponents | length' "$batch_json")" 2
expect "resume statuses" "$(jq -r '[.games[].status] | join(",")' "$batch_json")" "done,done,done,done"
expect "resume ids" "$(jq -r '[.games[].game_id] | join(",")' "$batch_json")" "9001,9002,9011,9012"
expect "resume takes no new snapshot" "$(grep -c leaderboard "$STUB_DIR/requests.log" || true)" 0
expect "resume index" "$(jq -r '.game_id' "$bdata/toy/index.jsonl" | tr '\n' ' ')" "9001 9002 9003 9004 9011 9012 "

# A game that fails 3 times fails; a refused request fails at once; the rest are played.
enqueue "$post_name" 01 200 '{"test_game_id":5021,"status":"failed","failure":"The code machine failed to run the game.","hint":"again"}'
enqueue "$post_name" 02 200 '{"test_game_id":5022,"status":"failed","failure":"The code machine failed to run the game.","hint":"again"}'
enqueue "$post_name" 03 200 '{"test_game_id":5023,"status":"failed","failure":"The code machine failed to run the game.","hint":"again"}'
enqueue "$post_name" 04 200 "$(done_reply 5024 "$(tglog 9021 seed=2 Beta 22 0 '[300,100]')" 5 20)"
enqueue "$post_name" 05 400 '{"error":"request-rejected","code":"INVALID_OPPONENT"}'
before="$(posts)"
run batch toy "$release_file" --opponents Beta --games-per-opponent 3 --name tfail --play --data "$bdata"
expect "failing batch exit" "$rc" 1
expect "failing batch posts" "$(($(posts) - before))" 5
expect "failing batch statuses" "$(jq -c '[.games[] | [.status, .attempts, .failure, .game_id]]' "$bdata/toy/batches/tfail.json")" \
  '[["failed",3,"The code machine failed to run the game.",null],["done",1,null,9021],["failed",1,"INVALID_OPPONENT",null]]'
contains "failure printed" "$out" "FAILED after 3 attempts: The code machine failed to run the game."
contains "rejection printed" "$out" "FAILED after 1 attempts: INVALID_OPPONENT"
contains "failure totals" "$out" "Beta: played 1, won 1, lost 0, drawn 0, failed 2"
expect "named opponent" "$(jq -r '.opponents[0] | "\(.name) \(.rank)"' "$bdata/toy/batches/tfail.json")" "Beta 3"
expect "failed game not stored" "$(find "$bdata/toy/logs" -name '*.json' | wc -l | tr -d ' ')" 7

# A malformed answer fails the game with a message; the batch goes on.
enqueue "$post_name" 01 200 'not json'
enqueue "$post_name" 02 200 'not json'
enqueue "$post_name" 03 200 '{"test_game_id":5031,"status":"done","log":"text"}'
run batch toy "$release_file" --opponents Gamma --games-per-opponent 1 --name tbad --play --data "$bdata"
expect "malformed exit" "$rc" 1
contains "malformed message" "$out" "FAILED after 3 attempts: the finished test game has no usable log"
expect "malformed stores nothing" "$(find "$bdata/toy/logs" -name '*.json' | wc -l | tr -d ' ')" 7

# Setup errors: exit 2, nothing written.
run batch toy "$release_file" --opponents Nobody --name tno --play --data "$tmp_dir/no1"
expect "unknown player exit" "$rc" 2
contains "unknown player message" "$out" "Nobody is not in the leaderboard's top 100"
expect "unknown player writes nothing" "$(find "$tmp_dir/no1" -mindepth 1 2>/dev/null | wc -l | tr -d ' ')" 0
run batch toy "$release_file" --opponents Mine --name tno --data "$tmp_dir/no1"
expect "ourselves exit" "$rc" 2
run batch toy games/toy/releases/missing.rs --data "$tmp_dir/no1"
expect "missing release exit" "$rc" 2
printf 'fn main() {}\n' >"$tmp_dir/outside.rs"
run batch toy "$tmp_dir/outside.rs" --data "$tmp_dir/no1"
expect "release outside releases exit" "$rc" 2
contains "release outside message" "$out" "is not a .rs file of games/toy/releases/"
run batch toy "$release_file" --seeds "$tmp_dir/missing-seeds" --data "$tmp_dir/no1"
expect "missing seeds exit" "$rc" 2
run batch toy "$release_file" --top 0
expect "bad top" "$rc" 2
run replay toy "$release_file" --top 3
expect "option of another command" "$rc" 2
run batch toy
expect "batch without release" "$rc" 2

# replay: lost games against the named opponent, newest first, not yet replayed.
rdata="$tmp_dir/rdata"
mkdir -p "$rdata/toy"
rline() { # rline <id> <ended_at> <opponent> <ranking>
  echo "{\"game_id\":$1,\"kind\":\"ranking\",\"puzzle\":\"toy-puzzle\",\"submission_id\":7,\"ended_at\":\"$2\",\"fetched_at\":\"2026-02-01T00:00:00Z\",\"our_seat\":0,\"players\":[{\"seat\":0,\"name\":\"Mine\",\"agent_id\":-1,\"me\":true},{\"seat\":1,\"name\":\"$3\",\"agent_id\":1}],\"ranking\":$4,\"game_options\":\"seed=5\"}"
}
{
  rline 101 2026-01-05T00:00:00Z Alpha '[1,0]'
  rline 102 2026-01-04T00:00:00Z Alpha '[0,1]'
  rline 103 2026-01-03T00:00:00Z Beta '[1,0]'
  rline 104 2026-01-02T00:00:00Z Alpha '[1,0]'
  rline 105 2026-01-01T00:00:00Z Alpha '[1,0]'
  echo '{"game_id":8104,"kind":"replay","puzzle":"toy-puzzle","submission_id":null,"ended_at":null,"fetched_at":"x","our_seat":0,"players":[],"ranking":[0,1],"game_options":null,"replay_of":104,"release":"toy-v1"}'
  echo '{"game_id":8105,"kind":"replay","puzzle":"toy-puzzle","submission_id":null,"ended_at":null,"fetched_at":"x","our_seat":0,"players":[],"ranking":[0,1],"game_options":null,"replay_of":105,"release":"toy-v0"}'
} >"$rdata/toy/index.jsonl"
cp "$rdata/toy/index.jsonl" "$tmp_dir/rindex.before"
before="$(posts)"
run replay toy "$release_file" --opponents Alpha --data "$rdata"
expect "replay dry exit" "$rc" 0
expect "replay picks lost games against Alpha, not replayed" "$(grep -o '^  [0-9]*' <<<"$out" | tr -d ' ' | tr '\n' ' ')" "101 105 "
contains "replay dry says so" "$out" "dry run: no game was played"
expect "replay dry posts nothing" "$(posts)" "$before"
expect "replay dry writes nothing" "$(cmp "$rdata/toy/index.jsonl" "$tmp_dir/rindex.before" && echo same)" same
run replay toy "$release_file" --all --limit 3 --data "$rdata"
expect "replay --all, limit" "$(grep -o '^  [0-9]*' <<<"$out" | tr -d ' ' | tr '\n' ' ')" "101 102 103 "
run replay toy "$release_file" --opponents Alpha,Beta --limit 1 --data "$rdata"
expect "replay limit" "$(grep -o '^  [0-9]*' <<<"$out" | tr -d ' ' | tr '\n' ' ')" "101 "
enqueue "$post_name" 01 200 "$(done_reply 5101 "$(tglog 9101 seed=5 Alpha 11 0 '[300,100]')" 3 5)"
enqueue "$post_name" 02 200 "$(done_reply 5102 "$(tglog 9102 seed=5 Alpha 11 1 '[100,300]')" 2 4)"
run replay toy "$release_file" --opponents Alpha --play --data "$rdata"
expect "replay play exit" "$rc" 0
expect "replay posts" "$(($(posts) - before))" 2
expect "replay body" "$(jq -c '[.replay_game_id, has("opponents"), has("game_options"), .show_in_ide, .overwrite_ide_code, .overwrite_ide_game_parameters]' "$STUB_DIR/post-$((before + 1)).json")" '[101,false,false,false,false,false]'
expect "replay second body" "$(jq -c '.replay_game_id' "$STUB_DIR/post-$((before + 2)).json")" 105
expect "replay code unchanged" "$(jq -j '.code' "$STUB_DIR/post-$((before + 1)).json" | cmp - "$release_file" && echo same)" same
rep="$(grep '"game_id":9101' "$rdata/toy/index.jsonl")"
expect "replay index line" "$(jq -c 'del(.ended_at, .fetched_at)' <<<"$rep")" \
  '{"game_id":9101,"kind":"replay","puzzle":"toy-puzzle","submission_id":null,"our_seat":0,"players":[{"seat":0,"name":"Mine","agent_id":-1,"me":true},{"seat":1,"name":"Alpha","agent_id":11}],"ranking":[0,1],"game_options":"seed=5","replay_of":101,"release":"toy-v1"}'
expect "replay keys at the end" "$(jq -c 'keys_unsorted | .[-2:]' <<<"$rep")" '["replay_of","release"]'
expect "replay logs" "$(find "$rdata/toy/logs" -name '*.json' -printf '%f\n' | sort | tr '\n' ' ')" "9101.json 9102.json "
run replay toy "$release_file" --opponents Alpha --data "$rdata"
contains "replayed games are skipped" "$out" "0 games to play"

# fetch-ids: the logs of given ids, ours or not.
echo '{"game_id":8001,"game_options":"seed=3","agents":[{"index":0,"name":"Top1","agent_id":5},{"index":1,"name":"Top2","agent_id":6}],"frames":[],"scores":[1,0],"ranking":[0,1],"limits":{"x":1}}' \
  >"$STUB_DIR/games_8001_format_json.json"
tglog 8002 seed=4 Alpha 11 1 '[1,2]' >"$STUB_DIR/games_8002_format_json.json"
fdata="$tmp_dir/fdata"
run fetch-ids toy --kind test 8001 8002 8003 --data "$fdata"
expect "fetch-ids exit (8003 is unknown)" "$rc" 1
contains "fetch-ids summary" "$out" "stored: 2"
contains "fetch-ids failure" "$out" "failed: game 8003 answered 404"
expect "fetch-ids line of a game we did not play" "$(sed -n 1p "$fdata/toy/index.jsonl" | jq -c 'del(.fetched_at)')" \
  '{"game_id":8001,"kind":"test","puzzle":"toy-puzzle","submission_id":null,"ended_at":null,"our_seat":null,"players":[{"seat":0,"name":"Top1","agent_id":5},{"seat":1,"name":"Top2","agent_id":6}],"ranking":[0,1],"game_options":"seed=3"}'
expect "fetch-ids our game" "$(sed -n 2p "$fdata/toy/index.jsonl" | jq -c '[.game_id, .our_seat, .kind, .ended_at]')" '[8002,0,"test",null]'
expect "fetch-ids log without limits" "$(jq -c 'has("limits")' "$fdata/toy/logs/8001.json")" false
: >"$STUB_DIR/requests.log"
run fetch-ids toy 8001 8002 --data "$fdata"
expect "fetch-ids rerun exit" "$rc" 0
contains "fetch-ids rerun" "$out" "already stored: 2"
expect "fetch-ids rerun fetched nothing" "$(grep -c 'games/' "$STUB_DIR/requests.log" || true)" 0
run fetch-ids toy 8001 --kind bogus
expect "fetch-ids bad kind" "$rc" 2
run fetch-ids toy abc
expect "fetch-ids bad id" "$rc" 2
run fetch-ids toy
expect "fetch-ids without ids" "$rc" 2
echo '{"game_id":8099,"agents":[{"index":0,"name":"A","agent_id":1}],"ranking":[0],"scores":[1]}' >"$STUB_DIR/games_8098_format_json.json"
run fetch-ids toy 8098 --data "$fdata"
expect "fetch-ids refuses a log of another id" "$rc" 1

# The token never shows, and no submission is ever made.
lacks "token in output" "$out" "$token"
lacks "token in arguments" "$(cat "$STUB_DIR/requests.log")" "$token"
expect "token in the data" "$(grep -rlF -- "$token" "$bdata" "$rdata" "$fdata" "$STUB_DIR"/post-*.json || true)" ""
expect "no submissions endpoint in the script" "$(grep -c submissions "$repo_root/scripts/codingame.sh" || true)" 0

# --push of a batch: a commit of the batch's files only.
clone2="$tmp_dir/clone2"
bare2="$tmp_dir/origin2.git"
git init -q --bare "$bare2"
git clone -q "$bare2" "$clone2" 2>/dev/null
mkdir -p "$clone2/scripts" "$clone2/games/toy/releases"
cp "$repo/scripts/codingame.sh" "$clone2/scripts/"
cp "$repo/games/toy/evaluation.env" "$clone2/games/toy/"
cp "$repo/games/toy/codingame-seeds.txt" "$clone2/games/toy/"
cp "$repo/$release_file" "$clone2/$release_file"
git -C "$clone2" add -A
git -C "$clone2" commit -q -m "chore(repo): stand-in"
enqueue "$post_name" 01 200 "$(done_reply 5201 "$(tglog 9201 seed=1 Beta 22 0 '[300,100]')" 5 20)"
rc=0
out="$(cd "$clone2" && CODINGAME_COMMIT_TRAILERS='Co-Authored-By: Test <t@example.com>' scripts/codingame.sh batch toy "$release_file" --opponents Beta --games-per-opponent 1 --name tp --play --push 2>&1)" || rc=$?
expect "batch push exit" "$rc" 0
expect "batch push subject" "$(git -C "$bare2" log -1 --format=%s refs/heads/claude/codingame-games)" "chore(repo): codingame games: toy, batch tp, 1 game"
expect "batch push files" "$(git -C "$bare2" ls-tree -r --name-only refs/heads/claude/codingame-games | sed 's|leaderboard/.*|leaderboard/X|' | tr '\n' ' ')" "toy/batches/tp.json toy/index.jsonl toy/leaderboard/X toy/logs/9201.json "
expect "batch push trailer" "$(git -C "$bare2" log -1 --format=%b refs/heads/claude/codingame-games | head -n 1)" "Co-Authored-By: Test <t@example.com>"
expect "batch push leaves no worktree" "$(git -C "$clone2" worktree list | wc -l | tr -d ' ')" 1

if ((failures > 0)); then
  echo "$failures of $checks codingame checks failed" >&2
  exit 1
fi
echo "ok   codingame.sh: $checks checks"
