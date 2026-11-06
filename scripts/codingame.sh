#!/usr/bin/env bash
# Works with CodinGame's public API (ADR 0047) for the game analysis
# (docs/GAME_ANALYSIS.md, ADR 0056). One subcommand:
#
#   fetch GAME [--submission ID] [--data DIR] [--push]
#
# Downloads our own ranking games ("battles") of the game's puzzle and a
# snapshot of its leaderboard (top 100) into a data directory:
#
#   <game>/logs/<game_id>.json                   the API's log without "limits"
#   <game>/index.jsonl                           one line a game, by game_id
#   <game>/leaderboard/<YYYY-MM-DDTHHMMSSZ>.json {"puzzle","fetched_at","rows"}
#
# The puzzle comes from CODINGAME_PUZZLE in games/GAME/evaluation.env. The
# token is read from ${CODINGAME_TOKEN_FILE:-$HOME/.config/codingame/token}
# and reaches curl through a header file (curl -H @file), never an argument.
# A rerun fetches only the games the index lacks; a log stored without its
# index line (an interrupted run) is indexed again without being fetched.
#
# Without --data the directory is a detached worktree of the branch
# claude/codingame-games (an orphan one if the branch does not exist yet).
# Nothing is pushed unless --push is given (a commit, then a push without
# force, then the worktree is removed).
#
# Environment: CODINGAME_API (base URL), CODINGAME_CURL (curl program),
# CODINGAME_PACE (seconds between requests, 0.6), CODINGAME_TOKEN_FILE.
# Needs bash, curl and jq. Exit status: 0 done, 1 a request failed, 2 usage
# or setup error. Runs in Claude's Linux sessions only (not Windows).
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/codingame.sh fetch GAME [--submission ID] [--data DIR] [--push]

  GAME             a folder of games/ whose evaluation.env sets CODINGAME_PUZZLE
  --submission ID  list the battles of this submission, not the current one
  --data DIR       write into this plain directory (no git, no push)
  --push           commit and push the data branch claude/codingame-games
                   (without --data only); without it nothing is pushed
USAGE
}
die() { # die <status> <message>
  echo "codingame.sh: $2" >&2
  exit "$1"
}
bad_usage() {
  usage >&2
  exit 2
}

cd "$(dirname "$0")/.."

if [[ ${1:-} == -h || ${1:-} == --help ]]; then
  usage
  exit 0
fi
[[ ${1:-} == fetch ]] || bad_usage
shift
game=""
submission=""
data_dir=""
push=false
while (($#)); do
  case "$1" in
    -h | --help)
      usage
      exit 0
      ;;
    --submission)
      [[ ${2:-} =~ ^[0-9]+$ ]] || bad_usage
      submission="$2"
      shift 2
      ;;
    --data)
      [[ -n ${2:-} ]] || bad_usage
      data_dir="$2"
      shift 2
      ;;
    --push)
      push=true
      shift
      ;;
    -*) bad_usage ;;
    *)
      [[ -z $game ]] || bad_usage
      game="$1"
      shift
      ;;
  esac
done
[[ $game =~ ^[a-z0-9][a-z0-9-]*$ ]] || bad_usage
if [[ $push == true && -n $data_dir ]]; then
  die 2 "--push works without --data only"
fi

for tool in jq git; do
  command -v "$tool" >/dev/null || die 2 "$tool is required"
done
curl_bin="${CODINGAME_CURL:-curl}"
command -v "$curl_bin" >/dev/null || die 2 "$curl_bin is required"

env_file="games/$game/evaluation.env"
[[ -f $env_file ]] || die 2 "no $env_file"
puzzle="$(sed -n 's/^CODINGAME_PUZZLE=//p' "$env_file" | tail -n 1)"
[[ -n $puzzle ]] || die 2 "CODINGAME_PUZZLE is not set in $env_file"

token_file="${CODINGAME_TOKEN_FILE:-$HOME/.config/codingame/token}"
[[ -s $token_file ]] || die 2 "no token at $token_file; the device flow that makes one is in docs/CODINGAME.md, section API"

api="${CODINGAME_API:-https://www.codingame.com/api/v1}"
pace="${CODINGAME_PACE:-0.6}"

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
(
  umask 077
  token="$(tr -d '[:space:]' <"$token_file")"
  printf 'Authorization: Bearer %s\n' "$token" >"$scratch/auth"
)

# The status of the last request is left in $status, its body in the file $2.
status=""
request() { # request <path with query> <body file>
  local url="$api$1" attempt wait
  for attempt in 1 2 3 4 5 6; do
    sleep "$pace"
    status="$("$curl_bin" -sS -D "$scratch/headers" -o "$2" -w '%{http_code}' \
      -H "@$scratch/auth" "$url" 2>"$scratch/curl-err")" || status=000
    if [[ $status != 429 ]]; then
      return 0
    fi
    ((attempt < 6)) || break
    wait="$(tr -d '\r' <"$scratch/headers" | sed -n 's/^[Rr]etry-[Aa]fter: *//p' | tail -n 1)"
    [[ $wait =~ ^[0-9]+$ ]] || wait=5
    echo "rate limited on $url, waiting ${wait}s" >&2
    sleep "$wait"
  done
  die 1 "still rate limited after 5 retries: $url"
}

# A request that must answer 200 with JSON that satisfies a jq test.
must_get() { # must_get <path> <body file> <jq test>
  request "$1" "$2"
  [[ $status == 200 ]] || die 1 "$api$1 answered $status"
  jq -e "$3" "$2" >/dev/null 2>&1 || die 1 "$api$1 answered something unexpected (not JSON, or without the expected members)"
}

# The data directory.
worktree=""
if [[ -n $data_dir ]]; then
  mkdir -p "$data_dir"
else
  git fetch -q origin +refs/heads/claude/codingame-games:refs/remotes/origin/claude/codingame-games 2>/dev/null || true
  data_dir="$(mktemp -d)/codingame-games"
  if git rev-parse -q --verify refs/remotes/origin/claude/codingame-games >/dev/null; then
    git worktree add -q --detach "$data_dir" refs/remotes/origin/claude/codingame-games
  else
    git worktree add -q --detach "$data_dir" HEAD
    git -C "$data_dir" checkout -q --orphan codingame-games
    git -C "$data_dir" rm -rq --cached . >/dev/null
    git -C "$data_dir" clean -fdxq
  fi
  worktree="$data_dir"
fi
echo "data directory: $data_dir"
out="$data_dir/$game"
mkdir -p "$out/logs" "$out/leaderboard"
index="$out/index.jsonl"
touch "$index"

# 1. Open the puzzle's session.
must_get "/puzzles/$puzzle" "$scratch/puzzle.json" 'type == "object"'

# 2. Snapshot the leaderboard.
now="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
snapshot="$out/leaderboard/$(date -u +%Y-%m-%dT%H%M%SZ).json"
must_get "/puzzles/$puzzle/leaderboard?limit=100" "$scratch/leaderboard.json" '.rows | type == "array"'
jq -c --arg puzzle "$puzzle" --arg now "$now" \
  '{puzzle: $puzzle, fetched_at: $now, rows: .rows}' "$scratch/leaderboard.json" >"$scratch/snapshot.json"
mv "$scratch/snapshot.json" "$snapshot"

# 3. List the battles, page by page.
: >"$scratch/battles.jsonl"
before=""
submission_id="$submission"
page=0
while :; do
  query="limit=100"
  [[ -z $before ]] || query+="&before=$before"
  [[ -z $submission ]] || query+="&submission=$submission"
  must_get "/puzzles/$puzzle/battles?$query" "$scratch/page.json" '.battles | type == "array"'
  jq -c '.battles[]' "$scratch/page.json" >>"$scratch/battles.jsonl"
  [[ -n $submission_id ]] || submission_id="$(jq -r '.submission_id // empty' "$scratch/page.json")"
  before="$(jq -r '.next_before // empty' "$scratch/page.json")"
  page=$((page + 1))
  [[ -n $before ]] || break
  ((page < 1000)) || die 1 "more than 1000 battle pages; stopping"
done
[[ $submission_id =~ ^[0-9]+$ ]] || submission_id=null
# Every battle once (pages may overlap), by game_id.
jq -cs 'unique_by(.game_id)[]' "$scratch/battles.jsonl" >"$scratch/battles-unique.jsonl" ||
  die 1 "a battle of $api/puzzles/$puzzle/battles is malformed"

# 4. Fetch the logs of the finished games the index lacks.
jq -r '.game_id' "$index" | sort -u >"$scratch/known"
listed=0 stored=0 fetched=0 recovered=0 failed=0 unfinished=0
while IFS= read -r battle; do
  listed=$((listed + 1))
  id="$(jq -r '.game_id' <<<"$battle")"
  if [[ $(jq -r '.done' <<<"$battle") != true ]]; then
    unfinished=$((unfinished + 1))
    continue
  fi
  if grep -qx -- "$id" "$scratch/known"; then
    stored=$((stored + 1))
    continue
  fi
  log="$out/logs/$id.json"
  if [[ -f $log ]]; then
    origin="stored"
  else
    origin="fetched"
    request "/games/$id?format=json" "$scratch/log.json"
    if [[ $status != 200 ]]; then
      echo "failed: game $id answered $status" >&2
      failed=$((failed + 1))
      continue
    fi
    if ! jq -e '.agents | type == "array"' "$scratch/log.json" >/dev/null 2>&1; then
      echo "failed: game $id: $api/games/$id?format=json answered something unexpected" >&2
      failed=$((failed + 1))
      continue
    fi
    jq -c 'del(.limits)' "$scratch/log.json" >"$scratch/log-clean.json"
    mv "$scratch/log-clean.json" "$log"
  fi
  if ! jq -c --argjson battle "$battle" --arg puzzle "$puzzle" --argjson sub "$submission_id" \
    --arg now "$now" '
    (.agents | map(select(.me == true))) as $me
    | if ($me | length) != 1 then error("no single agent with me") else . end
    | {game_id: $battle.game_id, kind: "ranking", puzzle: $puzzle, submission_id: $sub,
       ended_at: $battle.ended_at, fetched_at: $now, our_seat: $me[0].index,
       players: [.agents[] | {seat: .index, name: .name, agent_id: .agent_id}
                 + (if .me == true then {me: true} else {} end)],
       ranking: .ranking, game_options: (.game_options // null)}' \
    "$log" >"$scratch/line.json" 2>"$scratch/jq-err"; then
    echo "failed: game $id: log not usable ($(head -n 1 "$scratch/jq-err"))" >&2
    [[ $origin == stored ]] || rm -f "$log"
    failed=$((failed + 1))
    continue
  fi
  cat "$scratch/line.json" >>"$index"
  if [[ $origin == fetched ]]; then fetched=$((fetched + 1)); else recovered=$((recovered + 1)); fi
done <"$scratch/battles-unique.jsonl"

# 5. The index again: sorted, one line a game.
jq -cs 'unique_by(.game_id)[]' "$index" >"$scratch/index.sorted"
mv "$scratch/index.sorted" "$index"

echo "games listed: $listed"
echo "already stored: $stored"
echo "fetched: $fetched"
echo "indexed from stored logs: $recovered"
echo "failed: $failed"
echo "not finished: $unfinished"
echo "leaderboard snapshot: $snapshot"

new=$((fetched + recovered))
if [[ -n $worktree ]]; then
  if [[ $push == true ]]; then
    git -C "$worktree" add -A
    if git -C "$worktree" diff --cached --quiet; then
      echo "nothing to commit"
    else
      git -C "$worktree" commit -q -m "chore(repo): codingame games: $game, $new new games" \
        -m "Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>"
      git -C "$worktree" push origin HEAD:refs/heads/claude/codingame-games
    fi
    git worktree remove --force "$worktree"
  else
    echo "nothing was pushed. Inspect it with: git -C $worktree status"
    echo "--push publishes it to claude/codingame-games (commit and push, no force)."
  fi
fi
((failed == 0)) || exit 1
