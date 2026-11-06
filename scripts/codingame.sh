#!/usr/bin/env bash
# Works with CodinGame's public API (ADR 0047) for the game analysis
# (docs/GAME_ANALYSIS.md, ADR 0056). Subcommands:
#
#   fetch GAME [--submission ID] [--data DIR] [--push]
#   batch GAME RELEASE.rs [--top N] [--opponents NAME,...] [--games-per-opponent K]
#         [--seeds FILE] [--name NAME] [--play] [--data DIR] [--push]
#   replay GAME RELEASE.rs [--opponents NAME,...] [--all] [--limit N] [--play]
#         [--data DIR] [--push]
#   fetch-ids GAME [--kind ranking|test|replay] ID... [--data DIR] [--push]
#
# fetch downloads our own ranking games ("battles") of the game's puzzle and a
# snapshot of its leaderboard (top 100) into a data directory:
#
#   <game>/logs/<game_id>.json                   the API's log without "limits"
#   <game>/index.jsonl                           one line a game, by game_id
#   <game>/leaderboard/<YYYY-MM-DDTHHMMSSZ>.json {"puzzle","fetched_at","rows"}
#   <game>/batches/<name>.json                   a batch of test games (below)
#
# batch plays test games of a frozen release (a file of games/GAME/releases/)
# against the leaderboard's top players, on the fixed maps of
# games/GAME/codingame-seeds.txt, one game at a time; replay plays our lost
# ranking games again with a release; fetch-ids stores the logs of given game
# ids. batch and replay play nothing without --play (the only commands that
# write to CodinGame); a test game never touches the owner's IDE (show_in_ide,
# overwrite_ide_code and overwrite_ide_game_parameters are false) and no
# submission is ever made. The batch file is rewritten after every game, so an
# interrupted batch resumes: only its planned games are played.
#
# The puzzle comes from CODINGAME_PUZZLE in games/GAME/evaluation.env. The
# token is read from ${CODINGAME_TOKEN_FILE:-$HOME/.config/codingame/token}
# and reaches curl through a header file (curl -H @file), never an argument.
# A rerun of fetch fetches only the games the index lacks; a log stored
# without its index line (an interrupted run) is indexed again without being
# fetched.
#
# Without --data the directory is a detached worktree of the branch
# claude/codingame-games (an orphan one if the branch does not exist yet).
# Nothing is pushed unless --push is given (a commit, then a push without
# force, then the worktree is removed).
#
# Environment: CODINGAME_API (base URL), CODINGAME_CURL (curl program),
# CODINGAME_PACE (seconds between requests, 0.6), CODINGAME_TOKEN_FILE,
# CODINGAME_SLEEP (program that waits, for the waits of test games, sleep),
# CODINGAME_RETRY_WAIT (seconds before a failed test game is tried again, 60).
# Needs bash, curl and jq. Exit status: 0 done, 1 a request or a game failed,
# 2 usage or setup error. Runs in Claude's Linux sessions only (not Windows).
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/codingame.sh fetch GAME [--submission ID] [--data DIR] [--push]
       scripts/codingame.sh batch GAME RELEASE.rs [--top N] [--opponents NAME,...]
                [--games-per-opponent K] [--seeds FILE] [--name NAME] [--play]
                [--data DIR] [--push]
       scripts/codingame.sh replay GAME RELEASE.rs [--opponents NAME,...] [--all]
                [--limit N] [--play] [--data DIR] [--push]
       scripts/codingame.sh fetch-ids GAME [--kind ranking|test|replay] ID...
                [--data DIR] [--push]

  GAME             a folder of games/ whose evaluation.env sets CODINGAME_PUZZLE
  RELEASE.rs       a file of games/GAME/releases/, sent unchanged
  --submission ID  fetch: list the battles of this submission, not the current one
  --top N          batch: the leaderboard's top N players but us (10)
  --opponents L    batch: these players (nicknames, comma separated, in the top
                   100) instead of the top N; replay: only games against them
  --games-per-opponent K
                   batch: games against each opponent, on the first K maps of
                   the seeds file, cycled if it has fewer (10)
  --seeds FILE     batch: game options, one line a map (games/GAME/codingame-seeds.txt)
  --name NAME      batch: name of the batch (DATE-RELEASE-topN); a batch of
                   that name that exists is resumed, only planned games played
  --all            replay: the games we won too, not only the lost ones
  --limit N        replay: at most N games, newest first (20)
  --kind K         fetch-ids: the kind of the games in the index (ranking)
  --play           batch, replay: really play the test games; without it they
                   only print what they would play, and write nothing
  --data DIR       write into this plain directory (no git, no push)
  --push           commit and push the data branch claude/codingame-games
                   (without --data only; with --play for batch and replay);
                   without it nothing is pushed

Environment: CODINGAME_COMMIT_TRAILERS, lines (for example Co-Authored-By:)
appended to the body of the --push commit; none when unset.
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

orig_pwd="$PWD"
cd "$(dirname "$0")/.."

if [[ ${1:-} == -h || ${1:-} == --help ]]; then
  usage
  exit 0
fi
cmd="${1:-}"
case "$cmd" in
  fetch | batch | replay | fetch-ids) ;;
  *) bad_usage ;;
esac
shift
only() { # only <commands>: the option just read belongs to one of them
  case " $1 " in *" $cmd "*) ;; *) bad_usage ;; esac
}
positional=()
submission=""
data_dir=""
push=false
top=10 per_opp=10 limit=20
opp_names="" seeds_file="" batch_name="" kind="ranking"
play=false all=false
while (($#)); do
  case "$1" in
    -h | --help)
      usage
      exit 0
      ;;
    --submission)
      only fetch
      [[ ${2:-} =~ ^[0-9]+$ ]] || bad_usage
      submission="$2"
      shift 2
      ;;
    --top)
      only batch
      [[ ${2:-} =~ ^[1-9][0-9]*$ ]] || bad_usage
      top="$2"
      shift 2
      ;;
    --games-per-opponent)
      only batch
      [[ ${2:-} =~ ^[1-9][0-9]*$ ]] || bad_usage
      per_opp="$2"
      shift 2
      ;;
    --limit)
      only replay
      [[ ${2:-} =~ ^[1-9][0-9]*$ ]] || bad_usage
      limit="$2"
      shift 2
      ;;
    --opponents)
      only "batch replay"
      [[ -n ${2:-} ]] || bad_usage
      opp_names="$2"
      shift 2
      ;;
    --seeds)
      only batch
      [[ -n ${2:-} ]] || bad_usage
      seeds_file="$2"
      shift 2
      ;;
    --name)
      only batch
      [[ ${2:-} =~ ^[A-Za-z0-9][A-Za-z0-9._+-]*$ ]] || bad_usage
      batch_name="$2"
      shift 2
      ;;
    --kind)
      only fetch-ids
      [[ ${2:-} =~ ^(ranking|test|replay)$ ]] || bad_usage
      kind="$2"
      shift 2
      ;;
    --play)
      only "batch replay"
      play=true
      shift
      ;;
    --all)
      only replay
      all=true
      shift
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
      positional+=("$1")
      shift
      ;;
  esac
done
game="${positional[0]:-}"
[[ $game =~ ^[a-z0-9][a-z0-9-]*$ ]] || bad_usage
release_arg=""
ids=()
case "$cmd" in
  fetch) ((${#positional[@]} == 1)) || bad_usage ;;
  batch | replay)
    ((${#positional[@]} == 2)) || bad_usage
    release_arg="${positional[1]}"
    ;;
  fetch-ids)
    ((${#positional[@]} >= 2)) || bad_usage
    ids=("${positional[@]:1}")
    for id in "${ids[@]}"; do [[ $id =~ ^[0-9]+$ ]] || bad_usage; done
    ;;
esac
# Whether the run writes to the data directory.
writes=true
if [[ $cmd == batch || $cmd == replay ]] && [[ $play == false ]]; then writes=false; fi
if [[ $push == true && -n $data_dir ]]; then
  die 2 "--push works without --data only"
fi
if [[ $push == true && $writes == false ]]; then
  die 2 "--push needs --play: without it nothing is written"
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

release=""
if [[ -n $release_arg ]]; then
  release_path="$release_arg"
  [[ $release_path == /* ]] || release_path="$orig_pwd/$release_path"
  [[ -f $release_path ]] || die 2 "no such release file: $release_arg"
  [[ -d games/$game/releases ]] || die 2 "no games/$game/releases"
  release_path="$(realpath "$release_path")"
  [[ $release_path == "$(realpath "games/$game/releases")"/*.rs ]] ||
    die 2 "$release_arg is not a .rs file of games/$game/releases/"
  release="$(basename "$release_path" .rs)"
fi

token_file="${CODINGAME_TOKEN_FILE:-$HOME/.config/codingame/token}"
[[ -s $token_file ]] || die 2 "no token at $token_file; the device flow that makes one is in docs/CODINGAME.md, section API"

api="${CODINGAME_API:-https://www.codingame.com/api/v1}"
pace="${CODINGAME_PACE:-0.6}"

scratch="$(mktemp -d)"
# Leaves nothing behind: the scratch directory, and the worktree of --push
# (also when the run fails or the push is rejected).
remove_worktree=false
worktree=""
published=false
publish_what=""
cleanup() {
  # A play cut short by a failure still commits what it stored, so the
  # worktree is not removed with the games in it.
  if [[ $push == true && $published == false && -n $publish_what ]]; then
    publish "$publish_what" || true
  fi
  rm -rf "$scratch"
  if [[ $remove_worktree == true && -n $worktree ]]; then
    git worktree remove --force "$worktree" 2>/dev/null || true
  fi
}
trap cleanup EXIT
(
  umask 077
  token="$(tr -d '[:space:]' <"$token_file")"
  printf 'Authorization: Bearer %s\n' "$token" >"$scratch/auth"
)

# The status of the last request is left in $status, its body in the file $2.
status=""
request() { # request <path with query> <body file> [JSON file to POST]
  local url="$api$1" attempt wait post=()
  if [[ -n ${3:-} ]]; then
    post=(-X POST -H "Content-Type: application/json" --data-binary "@$3")
  fi
  for attempt in 1 2 3 4 5 6; do
    sleep "$pace"
    status="$("$curl_bin" -sS -D "$scratch/headers" -o "$2" -w '%{http_code}' \
      -H "@$scratch/auth" ${post[@]+"${post[@]}"} "$url" 2>"$scratch/curl-err")" || status=000
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

# What went wrong with the last request: curl's message and the API's hint.
detail() { # detail <body file>
  local text=""
  if [[ -s $scratch/curl-err ]]; then
    text="curl: $(head -n 1 "$scratch/curl-err")"
  fi
  if [[ -f $1 ]]; then
    hint="$(jq -r '.hint // .message // .error // empty' "$1" 2>/dev/null | head -n 1 || true)"
    [[ -z $hint ]] || text+="${text:+; }$hint"
  fi
  [[ -z $text ]] || printf ' (%s)' "$text"
}

# A request that must answer 200 with JSON that satisfies a jq test.
must_get() { # must_get <path> <body file> <jq test>
  request "$1" "$2"
  [[ $status == 200 ]] || die 1 "$api$1 answered $status$(detail "$2")"
  jq -e "$3" "$2" >/dev/null 2>&1 || die 1 "$api$1 answered something unexpected (not JSON, or without the expected members)"
}

# --- batch, replay and fetch-ids -------------------------------------------

exit_status=0
retry_wait="${CODINGAME_RETRY_WAIT:-60}"
quota_remaining="" quota_reset=0

now_utc() { date -u +%Y-%m-%dT%H:%M:%SZ; }
nap() { "${CODINGAME_SLEEP:-sleep}" "$1"; }

# Commits the data files of the worktree and pushes them, without force.
publish() { # publish <what>
  local noun_what="$1" message commit
  published=true
  if [[ -z $noun_what ]]; then
    echo "nothing to commit"
    return 0
  fi
  # Only the data files, never a temporary one.
  git -C "$worktree" add -- "$game/index.jsonl" "$game/leaderboard"
  if compgen -G "$worktree/$game/logs/*.json" >/dev/null; then
    git -C "$worktree" add -- "$game/logs/*.json"
  fi
  if compgen -G "$worktree/$game/batches/*.json" >/dev/null; then
    git -C "$worktree" add -- "$game/batches/*.json"
  fi
  if [[ $orphan == false ]] && git -C "$worktree" diff --cached --quiet; then
    echo "nothing to commit"
  else
    message="chore(repo): codingame games: $game, $noun_what"
    [[ -z ${CODINGAME_COMMIT_TRAILERS:-} ]] || message+=$'\n\n'"$CODINGAME_COMMIT_TRAILERS"
    if [[ $orphan == true ]]; then
      # A first commit without parent, made without a branch.
      commit="$(git -C "$worktree" commit-tree "$(git -C "$worktree" write-tree)" -m "$message")"
    else
      git -C "$worktree" commit -q -m "$message"
      commit="$(git -C "$worktree" rev-parse HEAD)"
    fi
    if ! git -C "$worktree" push origin "$commit:refs/heads/claude/codingame-games"; then
      # Keep the games: the commit stays reachable from the worktree's HEAD.
      git -C "$worktree" update-ref --no-deref HEAD "$commit" || true
      remove_worktree=false
      exit_status=1
      echo "the push failed; the games are kept, committed as $commit in $worktree" >&2
      echo "push them by hand: git -C $worktree push origin $commit:refs/heads/claude/codingame-games" >&2
      echo "(then remove the worktree: git worktree remove --force $worktree)" >&2
    fi
  fi
}

finish() { # finish <what>
  if [[ -n $worktree ]]; then
    if [[ $push == true ]]; then
      publish "$1"
    elif [[ $writes == true ]]; then
      echo "nothing was pushed. Inspect it with: git -C $worktree status"
      echo "--push publishes it to claude/codingame-games (commit and push, no force)."
    fi
  fi
}

open_session() {
  must_get "/puzzles/$puzzle" "$scratch/puzzle.json" 'type == "object"'
}

# The index line of a game, from its log; $kind, $puzzle, $ended ("" for
# unknown) and $now are arguments, $extra holds the keys added at the end.
# shellcheck disable=SC2016
index_jq='
  (.agents | map(select(.me == true))) as $me
  | (if ($me | length) > 1 then error("several agents with me")
     elif ($me | length) == 1 then $me[0].index else null end) as $seat
  | {game_id: .game_id, kind: $kind, puzzle: $puzzle, submission_id: null,
     ended_at: (if $ended == "" then null else $ended end), fetched_at: $now,
     our_seat: (if $seat == null then $dflt else $seat end),
     players: [.agents[] | {seat: .index, name: .name, agent_id: .agent_id}
               + (if .me == true then {me: true} else {} end)],
     ranking: .ranking, game_options: (.game_options // null)} + $extra'

# Stores a log (without "limits") and its index line, each by temp file and
# rename. On failure rec_err says why and nothing is stored.
rec_err="" rec_id=""
record_game() { # record_game <log file> <kind> <ended_at or ""> <extra JSON object> <seat if none is ours>
  local id dflt="$5" line="$scratch/line.json"
  id="$(jq -r '.game_id | if type == "number" then floor else empty end' "$1" 2>/dev/null || true)"
  if [[ ! $id =~ ^[0-9]+$ ]]; then
    rec_err="the log has no game_id"
    return 1
  fi
  if ! jq -c "$index_jq" --arg kind "$2" --arg puzzle "$puzzle" --arg ended "$3" \
    --arg now "$(now_utc)" --argjson extra "$4" --argjson dflt "$dflt" \
    "$1" >"$line" 2>"$scratch/jq-err"; then
    rec_err="log not usable ($(head -n 1 "$scratch/jq-err"))"
    return 1
  fi
  # Called in conditions, where set -e is off: every step is checked, the log
  # first, so an index line never lacks its log.
  if ! jq -c 'del(.limits)' "$1" >"$out/logs/.$id.json.tmp" ||
    ! mv "$out/logs/.$id.json.tmp" "$out/logs/$id.json"; then
    rec_err="cannot write the log of game $id"
    return 1
  fi
  if ! cat "$line" >>"$index" ||
    ! jq -cs 'unique_by(.game_id)[]' "$index" >"$out/.index.tmp" ||
    ! mv "$out/.index.tmp" "$index"; then
    rec_err="cannot write the index"
    return 1
  fi
  rec_id="$id"
}

# won, lost or draw for us, from a log.
result_of() { # result_of <log file>
  jq -r '(.agents | map(select(.me == true))) as $me
    | (if ($me | length) == 1 then $me[0].index else 0 end) as $s
    | if ((.scores // []) | length) == 2 and .scores[0] == .scores[1] then "draw"
      elif .ranking[0] == $s then "won" else "lost" end' "$1"
}

# The tightest test game window of a response: the next start waits when it
# has no request left.
note_limits() { # note_limits <body file>
  local r
  r="$(jq -r '.limits.test_games // empty | "\(.remaining) \(.resets_in_seconds)"' "$1" 2>/dev/null || true)"
  if [[ $r =~ ^[0-9]+\ [0-9]+$ ]]; then
    quota_remaining="${r% *}"
    quota_reset="${r#* }"
  fi
}
wait_quota() {
  if [[ $quota_remaining == 0 ]]; then
    echo "test game quota used up, waiting ${quota_reset}s"
    nap "$quota_reset"
    quota_remaining=""
  fi
}

# The slug of an RFC 9457 problem: the end of its type (or the error member).
problem_slug() { # problem_slug <body file>
  jq -r '((.type // "") | split("/") | last // "") as $t | if $t != "" then $t else (.error // "") end' "$1" 2>/dev/null || true
}

# Waits for a test game that runs (ours, or the one a 409 names) to end.
await_test_game() { # await_test_game <test game id>
  local polls st
  for polls in $(seq 1 30); do
    request "/test-games/$1?wait=30&format=json" "$scratch/await.json"
    [[ $status == 200 ]] || return 1
    st="$(jq -r '.status // empty' "$scratch/await.json" 2>/dev/null || true)"
    [[ $st == running ]] || return 0
  done
  return 1
}

# One start of a test game from the JSON body in a file. Sets tg_outcome to
# done (the log is in $scratch/tg-log.json), rejected (the API refused the
# request: no retry) or retry, with tg_message, tg_test_id and tg_seconds.
tg_outcome="" tg_message="" tg_test_id=null tg_seconds=0
play_once() { # play_once <body file>
  local busy=0 id st polls=0 t0 slug
  tg_outcome=retry tg_message="" tg_test_id=null tg_seconds=0
  while :; do
    wait_quota
    t0=$SECONDS
    request "/puzzles/$puzzle/test-games?wait=30&format=json" "$scratch/tg.json" "$1"
    note_limits "$scratch/tg.json"
    case "$status" in
      200 | 202) break ;;
      409)
        # Only a running test game of ours is waited for (one at a time); any
        # other 409 (challenge-paused...) is a refusal that names its slug.
        slug="$(problem_slug "$scratch/tg.json")"
        id="$(jq -r '.test_game_id // empty' "$scratch/tg.json" 2>/dev/null || true)"
        if [[ $slug != test-game-running || ! $id =~ ^[0-9]+$ ]]; then
          tg_outcome=refused
          tg_message="409 ${slug:-conflict}"
          return 0
        fi
        busy=$((busy + 1))
        if ((busy > 20)); then
          tg_message="another test game keeps running (409)"
          return 0
        fi
        await_test_game "$id" || true
        ;;
      400)
        tg_outcome=rejected
        tg_message="$(jq -r '.code // .error // "request-rejected"' "$scratch/tg.json" 2>/dev/null || true)"
        [[ -n $tg_message ]] || tg_message="request-rejected"
        return 0
        ;;
      *)
        tg_message="the test game request answered $status$(detail "$scratch/tg.json")"
        return 0
        ;;
    esac
  done
  st="$(jq -r '.status // empty' "$scratch/tg.json" 2>/dev/null || true)"
  id="$(jq -r '.test_game_id // empty' "$scratch/tg.json" 2>/dev/null || true)"
  [[ $id =~ ^[0-9]+$ ]] && tg_test_id="$id"
  while [[ $st == running ]]; do
    polls=$((polls + 1))
    if ((polls > 30)); then
      tg_message="the test game still runs after 30 polls"
      return 0
    fi
    if [[ ! $id =~ ^[0-9]+$ ]]; then
      tg_message="a running test game without test_game_id"
      return 0
    fi
    request "/test-games/$id?wait=30&format=json" "$scratch/tg.json"
    if [[ $status != 200 ]]; then
      tg_message="polling test game $id answered $status$(detail "$scratch/tg.json")"
      return 0
    fi
    st="$(jq -r '.status // empty' "$scratch/tg.json" 2>/dev/null || true)"
  done
  case "$st" in
    done)
      if ! jq -e '(.log.agents | type == "array") and (.log.game_id | type == "number")' "$scratch/tg.json" >/dev/null 2>&1; then
        tg_message="the finished test game has no usable log"
        return 0
      fi
      jq -c '.log' "$scratch/tg.json" >"$scratch/tg-log.json"
      tg_outcome="done"
      tg_seconds=$((SECONDS - t0))
      ;;
    failed)
      tg_message="$(jq -r '.failure // "failed"' "$scratch/tg.json" 2>/dev/null || true)"
      ;;
    *) tg_message="unexpected test game status '$st'" ;;
  esac
}

# A game played until it is done or has failed 3 times; a refused request is
# not tried again. Sets pg_outcome (done or failed), pg_attempts, pg_message.
pg_outcome="" pg_attempts=0 pg_message="" pg_refusal=""
play_game() { # play_game <body file> <attempts so far> <label>
  pg_attempts="$2"
  pg_message=""
  pg_refusal=""
  while :; do
    pg_attempts=$((pg_attempts + 1))
    play_once "$1"
    case "$tg_outcome" in
      done)
        pg_outcome="done"
        return 0
        ;;
      rejected | refused)
        pg_outcome=failed
        pg_message="$tg_message"
        pg_refusal="$tg_outcome"
        return 0
        ;;
    esac
    pg_message="$tg_message"
    if ((pg_attempts >= 3)); then
      pg_outcome=failed
      return 0
    fi
    echo "$3: attempt $pg_attempts failed ($tg_message), trying again after ${retry_wait}s" >&2
    nap "$retry_wait"
  done
}

plural() { # plural <n> <noun>
  if (($1 == 1)); then printf '%s %s' "$1" "$2"; else printf '%s %ss' "$1" "$2"; fi
}

# --- batch ---

batch_file="" batch_name_used=""
save_batch() {
  mkdir -p "$out/batches"
  jq . "$scratch/batch.json" >"$out/batches/.$batch_name_used.json.tmp"
  mv "$out/batches/.$batch_name_used.json.tmp" "$batch_file"
}
batch_set() { # batch_set <jq filter on .games[$i]> <jq args...>
  local filter="$1"
  shift
  jq -c "$@" "$filter" "$scratch/batch.json" >"$scratch/batch.new"
  mv "$scratch/batch.new" "$scratch/batch.json"
}

batch_totals() {
  local opp st gid r name
  declare -A t_played t_won t_lost t_drawn t_failed t_todo
  while IFS=$'\t' read -r opp st gid; do
    case "$st" in
      done)
        t_played["$opp"]=$((${t_played["$opp"]:-0} + 1))
        r="$(result_of "$out/logs/$gid.json" 2>/dev/null || echo lost)"
        case "$r" in
          won) t_won["$opp"]=$((${t_won["$opp"]:-0} + 1)) ;;
          draw) t_drawn["$opp"]=$((${t_drawn["$opp"]:-0} + 1)) ;;
          *) t_lost["$opp"]=$((${t_lost["$opp"]:-0} + 1)) ;;
        esac
        ;;
      failed) t_failed["$opp"]=$((${t_failed["$opp"]:-0} + 1)) ;;
      *) t_todo["$opp"]=$((${t_todo["$opp"]:-0} + 1)) ;;
    esac
  done < <(jq -r '.games[] | [.opponent, .status, (.game_id // "-")] | @tsv' "$scratch/batch.json")
  echo "totals of batch $batch_name_used:"
  while IFS= read -r name; do
    echo "  $name: played ${t_played["$name"]:-0}, won ${t_won["$name"]:-0}, lost ${t_lost["$name"]:-0}, drawn ${t_drawn["$name"]:-0}, failed ${t_failed["$name"]:-0}, not played ${t_todo["$name"]:-0}"
  done < <(jq -r '.opponents[].name' "$scratch/batch.json")
}

# The batch of the leaderboard's top players (or the named ones) on the fixed
# maps, in $scratch/batch.json; the snapshot in $scratch/snapshot.json.
new_batch() { # new_batch <name> <sha256>
  local seed_lines=() cycled=() i n now opps="[]" row nm names=()
  if [[ ! -f $seeds_file ]]; then die 2 "no seeds file $seeds_file"; fi
  mapfile -t seed_lines < <(sed -e 's/\r$//' -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' "$seeds_file" | grep -v '^#' | grep -v '^$' || true)
  n=${#seed_lines[@]}
  ((n > 0)) || die 2 "no map in $seeds_file"
  for ((i = 0; i < per_opp; i++)); do cycled+=("${seed_lines[i % n]}"); done

  open_session
  now="$(now_utc)"
  must_get "/puzzles/$puzzle/leaderboard?limit=100" "$scratch/leaderboard.json" '.rows | type == "array"'
  jq -c --arg puzzle "$puzzle" --arg now "$now" \
    '{puzzle: $puzzle, fetched_at: $now, rows: .rows}' "$scratch/leaderboard.json" >"$scratch/snapshot.json"
  snapshot_name="${now//:/}.json"

  if [[ -n $opp_names ]]; then
    IFS=, read -ra names <<<"$opp_names"
    for nm in "${names[@]}"; do
      [[ -n $nm ]] || continue
      row="$(jq -c --arg n "$nm" '[.rows[] | select(.nickname == $n)] | first // empty' "$scratch/leaderboard.json")"
      [[ -n $row ]] || die 2 "$nm is not in the leaderboard's top 100"
      [[ $(jq -r '.me // false' <<<"$row") != true ]] || die 2 "$nm is us"
      jq -e '.agent_id | type == "number"' <<<"$row" >/dev/null || die 2 "$nm has no agent_id"
      opps="$(jq -c --argjson row "$row" '. + [{name: $row.nickname, agent_id: $row.agent_id, rank: $row.rank}]' <<<"$opps")"
    done
  else
    opps="$(jq -c --argjson n "$top" '[.rows[] | select(.me != true and (.agent_id | type == "number"))]
      | .[:$n] | map({name: .nickname, agent_id: .agent_id, rank: .rank})' "$scratch/leaderboard.json")"
  fi
  [[ $(jq 'length' <<<"$opps") -gt 0 ]] || die 2 "no opponent to play"

  jq -n --arg name "$1" --arg game "$game" --arg puzzle "$puzzle" --arg release "$release" \
    --arg sha "$2" --arg now "$now" --arg snap "$snapshot_name" --argjson opps "$opps" \
    --argjson seeds "$(jq -cn '$ARGS.positional' --args "${cycled[@]}")" '
    {name: $name, game: $game, puzzle: $puzzle, release: $release, release_sha256: $sha,
     created_at: $now, leaderboard: $snap, opponents: $opps, seeds: $seeds,
     games: [$seeds[] as $s | $opps[]
             | {opponent: .name, agent_id: .agent_id, game_options: $s, status: "planned",
                attempts: 0, test_game_id: null, game_id: null, seconds: null, failure: null}]}' \
    >"$scratch/batch.json"
}

# A game to play: planned, or failed for a reason other than a refusal of the
# request (the failure of a refusal is an upper-case code such as
# INVALID_OPPONENT, which a retry would meet again).
playable_jq='(.status == "planned" or (.status == "failed" and ((.failure // "") | test("^[A-Z][A-Z0-9_]*$") | not)))'

# The name of the newest unfinished batch of this release (file) and maps
# with the same opponents (the named ones, or as many as --top), if any.
find_resumable() { # find_resumable <release sha256>
  local f names_json="[]" names_arr=()
  if [[ -n $opp_names ]]; then
    IFS=, read -ra names_arr <<<"$opp_names"
    names_json="$(jq -cn '$ARGS.positional' --args "${names_arr[@]}")"
  fi
  for f in "$out"/batches/*.json; do
    [[ -f $f ]] || continue
    jq -r --arg rel "$release" --arg sha "$1" --argjson k "$per_opp" --argjson top "$top" --argjson names "$names_json" "
      select(.release == \$rel and .release_sha256 == \$sha and (.seeds | length) == \$k
             and any(.games[]; $playable_jq)
             and (if (\$names | length) > 0 then ([.opponents[].name] | sort) == (\$names | sort)
                  else (.opponents | length) == \$top end))
      | \"\(.created_at) \(.name)\"" "$f" 2>/dev/null || true
  done | sort | tail -n 1 | cut -d' ' -f2-
}

run_batch() {
  local name sha state total i planned stored=0 opp aid opts att result failed_games streak=0 stopped=false resumable
  local streak_games=()
  [[ -n $seeds_file ]] || seeds_file="games/$game/codingame-seeds.txt"
  [[ $seeds_file == /* || ! -f $orig_pwd/$seeds_file ]] || seeds_file="$orig_pwd/$seeds_file"
  sha="$(sha256sum "$release_path" | cut -d' ' -f1)"
  name="$batch_name"
  if [[ -z $name ]]; then
    # Without --name an unfinished batch of this release, maps and opponents
    # is resumed (the newest), whatever day it is; else a new name.
    resumable="$(find_resumable "$sha")"
    if [[ -n $resumable ]]; then
      name="$resumable"
      echo "resuming the unfinished batch $name"
    elif [[ -n $opp_names ]]; then
      name="$(date -u +%F)-$release-${opp_names//,/+}"
      name="${name//[^A-Za-z0-9._+-]/_}"
    else
      name="$(date -u +%F)-$release-top$top"
    fi
  fi
  batch_name_used="$name"
  batch_file="$out/batches/$name.json"

  if [[ -f $batch_file ]]; then
    jq -e 'type == "object" and (.games | type == "array") and (.opponents | type == "array")' "$batch_file" >/dev/null 2>&1 ||
      die 2 "batch file $batch_file is damaged"
    cp "$batch_file" "$scratch/batch.json"
    [[ $(jq -r '.release_sha256' "$scratch/batch.json") == "$sha" && $(jq -r '.release' "$scratch/batch.json") == "$release" ]] ||
      die 2 "batch $name was made with another file than $release_path"
    state="resumed"
  else
    state="new"
    new_batch "$name" "$sha"
  fi
  total="$(jq '.games | length' "$scratch/batch.json")"
  planned="$(jq "[.games[] | select($playable_jq)] | length" "$scratch/batch.json")"

  echo "batch $name ($state): release $release, puzzle $puzzle"
  echo "opponents:"
  jq -r '.opponents[] | "  rank \(.rank)  \(.name)"' "$scratch/batch.json"
  echo "games per opponent: $(jq '.seeds | length' "$scratch/batch.json"), maps: $(jq -r '.seeds | join(" ")' "$scratch/batch.json")"
  echo "games: $planned to play of $total"
  echo "quota: $planned test games (limits: 10 a minute, 20 per 5 minutes, 30 per 15 minutes, 80 an hour, 600 a day)"
  echo "time: about $(((planned * 25 + 59) / 60)) minutes at about 25 s a game"
  if [[ $play == false ]]; then
    echo "dry run: no game was played and nothing was written; add --play to play the games"
    return 0
  fi

  prepare_out
  mkdir -p "$out/batches"
  if [[ $state == new ]]; then
    cp "$scratch/snapshot.json" "$out/leaderboard/.snapshot.tmp"
    mv "$out/leaderboard/.snapshot.tmp" "$out/leaderboard/$(jq -r '.leaderboard' "$scratch/batch.json")"
  fi
  save_batch
  publish_what="batch $name, 0 games"
  open_session
  for ((i = 0; i < total; i++)); do
    [[ $(jq -r --argjson i "$i" ".games[\$i] | if $playable_jq then \"yes\" else \"no\" end" "$scratch/batch.json") == yes ]] || continue
    opp="$(jq -r --argjson i "$i" '.games[$i].opponent' "$scratch/batch.json")"
    aid="$(jq -r --argjson i "$i" '.games[$i].agent_id' "$scratch/batch.json")"
    opts="$(jq -r --argjson i "$i" '.games[$i].game_options' "$scratch/batch.json")"
    att="$(jq -r --argjson i "$i" 'if .games[$i].status == "planned" then .games[$i].attempts else 0 end' "$scratch/batch.json")"
    jq -n --rawfile code "$release_path" --argjson opp "$aid" --arg o "$opts" \
      '{language: "Rust", code: $code, opponents: [$opp], game_options: $o, show_in_ide: false,
        overwrite_ide_code: false, overwrite_ide_game_parameters: false}' >"$scratch/post.json"
    play_game "$scratch/post.json" "$att" "game $((i + 1))/$total ($opp, $opts)"
    if [[ $pg_outcome == "done" ]] &&
      record_game "$scratch/tg-log.json" test "$(now_utc)" \
        "$(jq -nc --arg b "$name" --arg r "$release" '{batch: $b, release: $r}')" 0; then
      # shellcheck disable=SC2016
      batch_set '.games[$i].status = "done" | .games[$i].attempts = $att | .games[$i].test_game_id = $tid
        | .games[$i].game_id = $gid | .games[$i].seconds = $sec | .games[$i].failure = null' \
        --argjson i "$i" --argjson att "$pg_attempts" --argjson tid "$tg_test_id" \
        --argjson gid "$rec_id" --argjson sec "$tg_seconds"
      save_batch
      stored=$((stored + 1))
      streak=0
      streak_games=()
      publish_what="batch $name, $(plural "$stored" game)"
      result="$(result_of "$out/logs/$rec_id.json")"
      echo "game $((i + 1))/$total: vs $opp, $opts, $result, ${tg_seconds} s, quota left ${quota_remaining:-?}"
    else
      if [[ $pg_outcome == "done" ]]; then
        pg_message="$rec_err"
        pg_refusal=""
      fi
      # shellcheck disable=SC2016
      batch_set '.games[$i].status = "failed" | .games[$i].attempts = $att | .games[$i].failure = $msg' \
        --argjson i "$i" --argjson att "$pg_attempts" --arg msg "$pg_message"
      save_batch
      echo "game $((i + 1))/$total: vs $opp, $opts, FAILED after $pg_attempts attempts: $pg_message, quota left ${quota_remaining:-?}"
      # A game refused for itself (a 400 code) says nothing about the service.
      if [[ $pg_refusal != rejected ]]; then
        streak=$((streak + 1))
        streak_games+=("$i")
        if ((streak >= 3)); then
          stopped=true
          break
        fi
      fi
    fi
  done
  if [[ $stopped == true ]]; then
    # Those games stay planned, so that a resume plays them again.
    for i in "${streak_games[@]}"; do
      # shellcheck disable=SC2016
      batch_set '.games[$i].status = "planned" | .games[$i].attempts = 0' --argjson i "$i"
    done
    save_batch
    echo "stopping: 3 games failed in a row (the service looks down); they stay planned. Run the same command later to play them." >&2
    exit_status=1
  fi
  batch_totals
  failed_games="$(jq '[.games[] | select(.status == "failed")] | length' "$scratch/batch.json")"
  ((failed_games == 0)) || exit_status=1
}

# --- replay ---

run_replay() {
  local names_json="[]" names_arr=() count i gid opp result stored=0 failed_games=0 streak=0
  if [[ -n $opp_names ]]; then
    IFS=, read -ra names_arr <<<"$opp_names"
    names_json="$(jq -cn '$ARGS.positional' --args "${names_arr[@]}")"
  fi
  if [[ -s $index ]]; then
    jq -sc --arg rel "$release" --argjson names "$names_json" --argjson all "$all" --argjson limit "$limit" '
      ([.[] | select(.replay_of != null and .release == $rel) | .replay_of]) as $done
      | [.[] | select(.kind == "ranking" and .our_seat != null)
         | select($all or .ranking[0] != .our_seat)
         | select(([.players[] | select(.me != true) | .name]) as $opp
                  | ($names | length) == 0 or any($opp[]; IN($names[])))
         | select(.game_id as $g | ($done | index($g)) == null)]
      | sort_by([(.ended_at // ""), .game_id]) | reverse | .[:$limit]' "$index" >"$scratch/candidates.json"
  else
    echo '[]' >"$scratch/candidates.json"
  fi
  count="$(jq 'length' "$scratch/candidates.json")"
  echo "replay with $release: $(plural "$count" game) to play"
  jq -r '.[] | "  \(.game_id)  \(.ended_at // "?")  vs \([.players[] | select(.me != true) | .name] | join(", "))  (\(if .ranking[0] == .our_seat then "won" else "lost" end))"' "$scratch/candidates.json"
  echo "quota: $count test games; time: about $(((count * 25 + 59) / 60)) minutes at about 25 s a game"
  if [[ $play == false ]]; then
    echo "dry run: no game was played and nothing was written; add --play to play the games"
    return 0
  fi
  ((count > 0)) || return 0
  open_session
  for ((i = 0; i < count; i++)); do
    gid="$(jq -r --argjson i "$i" '.[$i].game_id' "$scratch/candidates.json")"
    opp="$(jq -r --argjson i "$i" '[.[$i].players[] | select(.me != true) | .name] | join(", ")' "$scratch/candidates.json")"
    jq -n --rawfile code "$release_path" --argjson id "$gid" \
      '{language: "Rust", code: $code, replay_game_id: $id, show_in_ide: false,
        overwrite_ide_code: false, overwrite_ide_game_parameters: false}' >"$scratch/post.json"
    play_game "$scratch/post.json" 0 "replay of $gid ($opp)"
    if [[ $pg_outcome == "done" ]] &&
      record_game "$scratch/tg-log.json" replay "$(now_utc)" \
        "$(jq -nc --argjson g "$gid" --arg r "$release" '{replay_of: $g, release: $r}')" null; then
      stored=$((stored + 1))
      streak=0
      publish_what="replay $release, $(plural "$stored" game)"
      result="$(result_of "$out/logs/$rec_id.json")"
      echo "replay of $gid: vs $opp, $result, ${tg_seconds} s, quota left ${quota_remaining:-?}"
    else
      [[ $pg_outcome == "done" ]] && pg_message="$rec_err"
      failed_games=$((failed_games + 1))
      echo "replay of $gid: vs $opp, FAILED after $pg_attempts attempts: $pg_message, quota left ${quota_remaining:-?}"
      streak=$((streak + 1))
      if ((streak >= 3)); then
        echo "stopping: 3 games failed in a row (the service looks down). Run the same command later." >&2
        break
      fi
    fi
  done
  echo "replayed: $stored, failed: $failed_games"
  ((failed_games == 0)) || exit_status=1
}

# --- fetch-ids ---

run_fetch_ids() {
  local id log stored=0 known=0 failed_games=0
  open_session
  for id in "${ids[@]}"; do
    if [[ -s $index ]] && jq -se --argjson id "$id" 'any(.[]; .game_id == $id)' "$index" >/dev/null 2>&1; then
      known=$((known + 1))
      continue
    fi
    log="$out/logs/$id.json"
    if [[ -f $log ]] && jq -e '.agents | type == "array"' "$log" >/dev/null 2>&1; then
      cp "$log" "$scratch/log.json"
    else
      request "/games/$id?format=json" "$scratch/log.json"
      if [[ $status != 200 ]]; then
        echo "failed: game $id answered $status$(detail "$scratch/log.json")" >&2
        failed_games=$((failed_games + 1))
        continue
      fi
      if ! jq -e --argjson id "$id" '(.agents | type == "array") and .game_id == $id' "$scratch/log.json" >/dev/null 2>&1; then
        echo "failed: game $id: $api/games/$id?format=json answered something unexpected" >&2
        failed_games=$((failed_games + 1))
        continue
      fi
    fi
    if record_game "$scratch/log.json" "$kind" "" '{}' null; then
      stored=$((stored + 1))
      publish_what="$(plural "$stored" game) by id"
    else
      echo "failed: game $id: $rec_err" >&2
      failed_games=$((failed_games + 1))
    fi
  done
  echo "stored: $stored"
  echo "already stored: $known"
  echo "failed: $failed_games"
  ((failed_games == 0)) || exit_status=1
}

# The data directory.
orphan=false
if [[ -n $data_dir ]]; then
  if [[ $writes == true ]]; then mkdir -p "$data_dir"; fi
else
  # Exit status 2 of ls-remote: the branch does not exist; another failure
  # (no network, no origin) stops the run.
  lsr=0
  git ls-remote --exit-code origin refs/heads/claude/codingame-games >/dev/null 2>"$scratch/lsr-err" || lsr=$?
  if ((lsr != 0 && lsr != 2)); then
    die 1 "cannot reach origin: $(head -n 1 "$scratch/lsr-err")"
  fi
  data_dir="$(mktemp -d)/codingame-games"
  if ((lsr == 0)); then
    git fetch -q origin +refs/heads/claude/codingame-games:refs/remotes/origin/claude/codingame-games
    git worktree add -q --detach "$data_dir" refs/remotes/origin/claude/codingame-games
  else
    # No branch yet: a detached worktree emptied of files. The first commit
    # is made without parent below, so no local branch is ever created.
    orphan=true
    git worktree add -q --detach "$data_dir" HEAD
    git -C "$data_dir" rm -rq --cached . >/dev/null
    git -C "$data_dir" clean -fdxq
  fi
  worktree="$data_dir"
  remove_worktree="$push"
  # A run that writes nothing leaves no worktree.
  if [[ $writes == false ]]; then remove_worktree=true; fi
fi
echo "data directory: $data_dir"
out="$data_dir/$game"
index="$out/index.jsonl"
# A batch makes its files only once its plan is sound (see run_batch).
prepare_out() {
  mkdir -p "$out/logs" "$out/leaderboard"
  rm -f "$out"/logs/.*.tmp "$out"/.*.tmp "$out"/batches/.*.tmp
  touch "$index"
}
if [[ $writes == true && $cmd != batch ]]; then prepare_out; fi

if [[ $cmd != fetch ]]; then
  "run_${cmd//-/_}"
  finish "$publish_what"
  exit "$exit_status"
fi

# 1. Open the puzzle's session.
must_get "/puzzles/$puzzle" "$scratch/puzzle.json" 'type == "object"'

# 2. Snapshot the leaderboard.
now="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
snapshot="$out/leaderboard/${now//:/}.json"
must_get "/puzzles/$puzzle/leaderboard?limit=100" "$scratch/leaderboard.json" '.rows | type == "array"'
jq -c --arg puzzle "$puzzle" --arg now "$now" \
  '{puzzle: $puzzle, fetched_at: $now, rows: .rows}' "$scratch/leaderboard.json" >"$out/.snapshot.tmp"
mv "$out/.snapshot.tmp" "$snapshot"

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
  if [[ -f $log ]] && ! jq -e '.agents | type == "array"' "$log" >/dev/null 2>&1; then
    echo "stored log of game $id is damaged, fetching it again" >&2
    rm -f "$log"
  fi
  if [[ -f $log ]]; then
    origin="stored"
  else
    origin="fetched"
    request "/games/$id?format=json" "$scratch/log.json"
    if [[ $status != 200 ]]; then
      echo "failed: game $id answered $status$(detail "$scratch/log.json")" >&2
      failed=$((failed + 1))
      continue
    fi
    if ! jq -e '.agents | type == "array"' "$scratch/log.json" >/dev/null 2>&1; then
      echo "failed: game $id: $api/games/$id?format=json answered something unexpected" >&2
      failed=$((failed + 1))
      continue
    fi
    # Written next to its place, then renamed: a run cut short leaves no half log.
    jq -c 'del(.limits)' "$scratch/log.json" >"$out/logs/.$id.json.tmp"
    mv "$out/logs/.$id.json.tmp" "$log"
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
jq -cs 'unique_by(.game_id)[]' "$index" >"$out/.index.tmp"
mv "$out/.index.tmp" "$index"

echo "games listed: $listed"
echo "already stored: $stored"
echo "fetched: $fetched"
echo "indexed from stored logs: $recovered"
echo "failed: $failed"
echo "not finished: $unfinished"
echo "leaderboard snapshot: $snapshot"

new=$((fetched + recovered))
noun="games"
((new != 1)) || noun="game"
finish "$new new $noun"
((failed == 0 && exit_status == 0)) || exit 1
