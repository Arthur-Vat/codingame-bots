#!/usr/bin/env bash
# Keeps one archive of arena games per game, as the workflow artifact
# arena-games-<game> (ADR 0031): the records of the league and SPRT samples
# (ADR 0027), each checked by studio-archive, with an index. The workflow
# .github/workflows/archive.yml runs the modes below.
#
# Usage:
#   scripts/arena-archive.sh plan
#   scripts/arena-archive.sh gather GAME WORK_DIR
#   scripts/arena-archive.sh prune GAME KEEP_ID
#
# plan: decides the games to archive and writes `games=<JSON array>` and
# `adding=<true|false>` to $GITHUB_OUTPUT (and stdout). For EVENT_NAME
# workflow_run (a League run of a push to main), the games whose release
# files the commit HEAD adds, with adding=true; none, and a notice, when no
# release was added. Otherwise (schedule, by hand) every game, adding=false.
# Run in a checkout of two commits.
#
# gather: builds the next archive of GAME in WORK_DIR/archive from the inputs
# it downloads to WORK_DIR/inputs/<kind>-<id>, each unzipped into a folder of
# its own:
#   0-archive     the current archive, the newest unexpired arena-games-GAME;
#                 failing to download it fails the step
#   seed-<id>     without a current archive, each unexpired league sample
#                 league-game-records-GAME of a run on main of this repository
#   league-<id>   when adding (ADDING=true): the league sample of the run
#                 LEAGUE_RUN_ID
#   sprt-<id>     when adding: the latest unexpired SPRT sample
#                 game-records-GAME of the merged pull request of HEAD_SHA
# then runs ARCHIVE_TOOL (studio-archive) over them. It writes upload, added
# and total to $GITHUB_OUTPUT and a line to $GITHUB_STEP_SUMMARY. upload is
# true when the archive has more games than the current one, or when the
# current one expires in less than 30 days; else false. Nothing is deleted.
#
# prune: deletes every arena-games-GAME artifact but KEEP_ID, the one just
# uploaded. Refuses an empty or non-numeric KEEP_ID, and a KEEP_ID that is
# not listed. With DRY_RUN=true (the default) it only prints what it would
# delete.
#
# Environment: REPO (owner/name) and GH_TOKEN for the gh CLI; DRY_RUN
# (default true, prune only); ARCHIVE_TOOL (gather); ADDING, LEAGUE_RUN_ID and
# HEAD_SHA (gather, when adding); EVENT_NAME (plan); NOW (unix seconds,
# default: now; for tests); GITHUB_OUTPUT and GITHUB_STEP_SUMMARY, written
# only when set. Needs gh, jq and unzip.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
cd "$here/.."

# A refresh is due when the current archive expires in less than this.
refresh_seconds=$((30 * 24 * 3600))

usage() {
  sed -n '/^# Usage:/,/^# plan:/p' "$0" | sed '$d;s/^# \{0,1\}//' >&2
  exit 2
}

die() {
  echo "::error title=Arena archive::$*"
  exit 1
}

notice() { echo "::notice title=Arena archive::$*"; }
warning() { echo "::warning title=Arena archive::$*"; }

# Appends a line to the file named by the variable, when it is set.
emit() { # emit <variable name> <line>
  local file="${!1:-}"
  if [[ -n $file ]]; then
    printf '%s\n' "$2" >>"$file"
  fi
}

valid_game() { [[ ${1-} =~ ^[A-Za-z0-9_-]+$ ]]; }

# Prints the artifacts called <name> of the repository as a JSON array. The
# listing is paginated, and holds expired artifacts too (expired: true).
list_artifacts() { # list_artifacts <name>
  gh api --paginate "repos/$REPO/actions/artifacts?name=$1&per_page=100" | jq -s '[.[].artifacts[]]'
}

# Downloads the artifact <id> to <work>/zips and unzips it into the new
# folder <dir>. Info-ZIP refuses paths that leave the folder.
download() { # download <work> <artifact id> <dir>
  local zip="$1/zips/$2.zip"
  mkdir -p "$1/zips" "$3"
  if ! gh api "repos/$REPO/actions/artifacts/$2/zip" >"$zip"; then
    die "cannot download artifact $2"
  fi
  if ! unzip -q -d "$3" "$zip"; then
    die "cannot unzip artifact $2"
  fi
}

# ---- plan --------------------------------------------------------------

mode_plan() {
  local scopes list="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/arena-archive-added"
  local -a files=()
  case ${EVENT_NAME:-} in
    workflow_run)
      # HEAD is the commit pushed to main, HEAD^1 its parent. NUL-separated:
      # git quotes the odd file names otherwise. Written to a file first: a
      # process substitution would hide a failure of git, and no files would
      # then mean no release.
      if ! git diff -z --name-only --diff-filter=A --no-renames HEAD^1 HEAD -- 'games/*/releases/*.rs' >"$list"; then
        die "git diff failed; the added releases are unknown"
      fi
      mapfile -d '' -t files <"$list"
      rm -f "$list"
      scopes="$("$here/ci-scopes.sh" sprt -- "${files[@]}")" || die "scripts/ci-scopes.sh failed"
      if [[ $scopes == 'games=[]' ]]; then
        notice "no release added: nothing to archive"
      fi
      emit GITHUB_OUTPUT "$scopes"
      emit GITHUB_OUTPUT "adding=true"
      echo "$scopes"
      echo "adding=true"
      ;;
    schedule | workflow_dispatch)
      scopes="$("$here/ci-scopes.sh" league --all)" || die "scripts/ci-scopes.sh failed"
      emit GITHUB_OUTPUT "$scopes"
      emit GITHUB_OUTPUT "adding=false"
      echo "$scopes"
      echo "adding=false"
      ;;
    *)
      die "unexpected event '${EVENT_NAME:-}'"
      ;;
  esac
}

# ---- gather ------------------------------------------------------------

# Prints the id of the unexpired artifact <name> of the run <run id>, or
# nothing.
run_artifact() { # run_artifact <run id> <name>
  gh api "repos/$REPO/actions/runs/$1/artifacts?name=$2&per_page=100" |
    jq -r '[.artifacts[] | select(.expired | not)] | sort_by(.created_at) | last | .id // empty'
}

# Prints the head of the merged pull request of the commit $HEAD_SHA, from a
# branch of this repository, or nothing.
merged_pull_head() {
  gh api "repos/$REPO/commits/$HEAD_SHA/pulls?per_page=100" |
    jq -r --arg sha "$HEAD_SHA" --arg repo "$REPO" \
      '[.[] | select(.merge_commit_sha == $sha and ((.head.repo.full_name // "") == $repo))] | first | .head.sha // empty'
}

# The SPRT sample to add: prints the artifact id and the run id, space
# separated, or nothing and the reason on stderr.
sprt_sample() { # sprt_sample <game>
  local pr_head runs run id
  pr_head="$(merged_pull_head)"
  if [[ -z $pr_head ]]; then
    echo "no pull request of this repository was merged as $HEAD_SHA" >&2
    return 0
  fi
  runs="$(gh api "repos/$REPO/actions/workflows/sprt.yml/runs?head_sha=$pr_head&event=pull_request&per_page=20" |
    jq -r --arg repo "$REPO" \
      '[.workflow_runs[] | select((.head_repository.full_name // "") == $repo)] | sort_by(.created_at) | reverse | .[].id')"
  if [[ -z $runs ]]; then
    echo "the pull request's head $pr_head has no SPRT run (reused from the cache, or expired)" >&2
    return 0
  fi
  while read -r run; do
    [[ -n $run ]] || continue
    id="$(run_artifact "$run" "game-records-$1")"
    if [[ -n $id ]]; then
      echo "$id $run"
      return 0
    fi
  done <<<"$runs"
  echo "no SPRT run of the pull request's head $pr_head has an unexpired game-records-$1 sample" >&2
}

mode_gather() {
  local game="${1-}" work="${2-}"
  valid_game "$game" || usage
  [[ -n $work ]] || usage
  : "${REPO:?set REPO to owner/name}"
  : "${ARCHIVE_TOOL:?set ARCHIVE_TOOL to the studio-archive binary}"
  local adding="${ADDING:-false}"
  if [[ $adding == true ]]; then
    : "${LEAGUE_RUN_ID:?set LEAGUE_RUN_ID to the id of the League run}"
    : "${HEAD_SHA:?set HEAD_SHA to the commit of the League run}"
    [[ $LEAGUE_RUN_ID =~ ^[0-9]+$ ]] || die "LEAGUE_RUN_ID is not a number"
    [[ $HEAD_SHA =~ ^[0-9a-f]{40}$ ]] || die "HEAD_SHA is not a commit"
  fi
  local now="${NOW:-$(date +%s)}"
  if [[ -e $work/inputs || -e $work/archive ]]; then
    die "$work is not fresh"
  fi
  mkdir -p "$work/inputs"

  # 1. The current archive: the newest unexpired one.
  local listing current cur_id="" cur_expires="" cur_expires_epoch=0
  listing="$(list_artifacts "arena-games-$game")"
  current="$(jq -c '[.[] | select(.expired | not)] | sort_by(.created_at) | last // empty' <<<"$listing")"
  if [[ -n $current ]]; then
    cur_id="$(jq -r '.id' <<<"$current")"
    cur_expires="$(jq -r '.expires_at // ""' <<<"$current")"
    if [[ -n $cur_expires ]]; then
      cur_expires_epoch="$(jq -r 'sub("\\.[0-9]+Z$"; "Z") | fromdateiso8601' <<<"\"$cur_expires\"")"
    fi
    # 2. Never build a new archive without the current one.
    download "$work" "$cur_id" "$work/inputs/0-archive"
    [[ -f $work/inputs/0-archive/index.json ]] || die "the current archive $cur_id has no index.json"
  else
    local seeds id
    seeds="$(list_artifacts "league-game-records-$game" |
      jq -r --arg skip "${LEAGUE_RUN_ID:-0}" --arg adding "$adding" \
        '[.[] | select((.expired | not)
            and .workflow_run.head_branch == "main"
            and .workflow_run.head_repository_id != null
            and .workflow_run.head_repository_id == .workflow_run.repository_id
            and ($adding != "true" or (.workflow_run.id | tostring) != $skip))]
          | sort_by(.id) | .[].id')"
    if [[ -z $seeds && $adding != true ]]; then
      notice "$game: no archive, and no league sample of main to start from"
    fi
    while read -r id; do
      [[ -n $id ]] || continue
      download "$work" "$id" "$work/inputs/seed-$id"
    done <<<"$seeds"
  fi

  # 3. The samples of the release.
  if [[ $adding == true ]]; then
    local league_id sprt run
    league_id="$(run_artifact "$LEAGUE_RUN_ID" "league-game-records-$game")"
    if [[ -n $league_id ]]; then
      download "$work" "$league_id" "$work/inputs/league-$LEAGUE_RUN_ID"
    else
      warning "$game: league run $LEAGUE_RUN_ID has no unexpired league-game-records-$game sample"
    fi
    sprt="$(sprt_sample "$game" 2>"$work/sprt-why")" || die "$game: the SPRT sample lookup failed: $(cat "$work/sprt-why")"
    if [[ -n $sprt ]]; then
      read -r id run <<<"$sprt"
      download "$work" "$id" "$work/inputs/sprt-$run"
    else
      notice "$game: no SPRT sample added: $(cat "$work/sprt-why")"
    fi
  fi

  # 4. The tool checks every record and writes the archive, or nothing.
  local -a inputs=()
  mapfile -t inputs < <(find "$work/inputs" -mindepth 1 -maxdepth 1 -type d | LC_ALL=C sort)
  if ((${#inputs[@]} == 0)); then
    notice "$game: nothing to archive: no current archive and no samples"
    emit GITHUB_OUTPUT "upload=false"
    emit GITHUB_OUTPUT "added=0"
    emit GITHUB_OUTPUT "total=0"
    emit GITHUB_STEP_SUMMARY "$game: no games to archive; upload: false"
    return 0
  fi
  "$ARCHIVE_TOOL" --game "$game" --out "$work/archive" "${inputs[@]}" || die "$game: studio-archive failed; nothing is uploaded or deleted"
  [[ -f $work/archive/index.json ]] || die "$game: studio-archive wrote no index.json"

  # 5. Upload when there is something new or the current archive is near its end.
  local total before=0 added upload=false line
  total="$(jq '.rows | length' "$work/archive/index.json")"
  if [[ -n $cur_id ]]; then
    before="$(jq '.rows | length' "$work/inputs/0-archive/index.json")"
  fi
  added=$((total - before))
  if ((total > before)); then
    upload=true
  elif [[ -n $cur_id ]] && ((cur_expires_epoch - now < refresh_seconds)); then
    upload=true
  fi
  if ((total == 0)); then
    upload=false
  fi
  line="$game: $total games ($added added); "
  if [[ -n $cur_id ]]; then
    line+="current archive expires ${cur_expires%%T*}"
  else
    line+="no current archive"
  fi
  line+="; upload: $upload"
  echo "$line"
  emit GITHUB_STEP_SUMMARY "$line"
  emit GITHUB_OUTPUT "upload=$upload"
  emit GITHUB_OUTPUT "added=$added"
  emit GITHUB_OUTPUT "total=$total"
}

# ---- prune -------------------------------------------------------------

mode_prune() {
  local game="${1-}" keep="${2-}" dry_run="${DRY_RUN:-true}"
  valid_game "$game" || usage
  : "${REPO:?set REPO to owner/name}"
  [[ $keep =~ ^[0-9]+$ ]] || die "prune needs the numeric id of the artifact to keep, not '$keep'"
  local ids id deleted=0
  ids="$(list_artifacts "arena-games-$game" | jq -r '.[] | [.id, (.expired | tostring)] | @tsv')"
  # The artifact just uploaded must be listed, or something is wrong.
  if ! awk -v keep="$keep" '$1 == keep { found = 1 } END { exit !found }' <<<"$ids"; then
    die "$game: artifact $keep to keep is not among the arena-games-$game artifacts; nothing is deleted"
  fi
  while IFS=$'\t' read -r id expired; do
    [[ -n $id && $id != "$keep" ]] || continue
    # An expired artifact is gone already.
    if [[ $expired == true ]]; then
      continue
    fi
    if [[ $dry_run == true ]]; then
      echo "$game: would delete artifact $id (dry run)"
    else
      gh api -X DELETE "repos/$REPO/actions/artifacts/$id" >/dev/null
      echo "$game: deleted artifact $id"
    fi
    deleted=$((deleted + 1))
  done <<<"$ids"
  echo "$game: kept artifact $keep; $deleted other(s) $([[ $dry_run == true ]] && echo 'would be deleted' || echo deleted)"
}

# ---- main --------------------------------------------------------------

(($# >= 1)) || usage
mode="$1"
shift
case $mode in
  plan) mode_plan ;;
  gather) mode_gather "$@" ;;
  prune) mode_prune "$@" ;;
  *) usage ;;
esac
