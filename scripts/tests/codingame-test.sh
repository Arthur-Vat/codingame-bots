#!/usr/bin/env bash
# Tests scripts/codingame.sh fetch without the network: a stand-in for curl
# answers from synthetic JSON files (made-up players and games), can answer 429
# once and fails chosen requests. Needs jq. Usage: scripts/tests/codingame-test.sh
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
headers="" body="" format="" hfile="" url=""
while (($#)); do
  case "$1" in
    -D) headers="$2"; shift 2 ;;
    -o) body="$2"; shift 2 ;;
    -w) format="$2"; shift 2 ;;
    -H) hfile="${2#@}"; shift 2 ;;
    -sS) shift ;;
    *) url="$1"; shift ;;
  esac
done
echo "$url" >>"$STUB_DIR/requests.log"
cat "$hfile" >>"$STUB_DIR/auth.log"
name="$(sed 's|^http://stub||; s|[/?&=]|_|g; s|^_||' <<<"$url")"
reply() { : >"$headers"; [[ ${3:-} == retry ]] && echo "Retry-After: 0" >"$headers"; printf '%s' "$2" >"$body"; printf '%s' "${format/\%\{http_code\}/$1}"; }
if [[ -f $STUB_DIR/$name.429once ]]; then
  rm "$STUB_DIR/$name.429once"
  reply 429 '{"error":"rate-limited"}' retry
elif [[ -f $STUB_DIR/$name.status ]]; then
  reply "$(cat "$STUB_DIR/$name.status")" '{"error":"x"}'
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
expect "snapshot" "$(jq -c '[.puzzle, (.fetched_at | test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:]{8}Z$")), (.rows | length), .rows[0].nickname]' "${snapshots[0]}")" '["toy-puzzle",true,2,"Alpha"]'
contains "paging asked for the second page" "$(cat "$STUB_DIR/requests.log")" "battles?limit=100&before=250"

# Neither the token's text nor its place in an argument shows anywhere.
lacks "token in output" "$out" "$token"
lacks "token in arguments" "$(cat "$STUB_DIR/requests.log")" "$token"
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
expect "worktree is orphan" "$(git -C "$wt" rev-parse -q --verify HEAD >/dev/null 2>&1 && echo has-head || echo no-head)" no-head
contains "push hint" "$out" "--push"
expect "nothing pushed" "$(git -C "$bare" rev-parse -q --verify "$branch" >/dev/null && echo pushed || echo none)" none
git -C "$clone" worktree remove --force "$wt"

# (2) --push: the branch holds only the game's files.
rc=0
out="$("$cscript" fetch toy --push 2>&1)" || rc=$?
expect "push exit" "$rc" 0
expect "subject" "$(git -C "$bare" log -1 --format=%s "$branch")" "chore(repo): codingame games: toy, 2 new games"
contains "trailer" "$(git -C "$bare" log -1 --format=%B "$branch")" "Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>"
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
expect "second subject" "$(git -C "$bare" log -1 --format=%s "$branch")" "chore(repo): codingame games: toy, 1 new games"
changed="$(git -C "$bare" diff --name-only "$first" "$branch" | grep -v 'leaderboard/' | tr '\n' ' ')"
expect "only the new log and the index changed" "$changed" "toy/index.jsonl toy/logs/400.json "
expect "index has three" "$(git -C "$bare" show "$branch:toy/index.jsonl" | wc -l | tr -d ' ')" 3
expect "worktree removed again" "$(git -C "$clone" worktree list | wc -l | tr -d ' ')" 1

if ((failures > 0)); then
  echo "$failures of $checks codingame checks failed" >&2
  exit 1
fi
echo "ok   codingame.sh: $checks checks"
