#!/usr/bin/env bash
# The SPRT of a release played in blocks over several calibrated runners
# (ADR 0049, decision 1), run by the workflow
# .github/workflows/sprt-parallel.yml with the game's evaluation settings
# (games/GAME/evaluation.env: SPRT_*, TIME_SCALE=auto). scripts/sprt.sh stays
# the SPRT on one machine, for local runs and for games with one runner.
#
# Usage:
#   scripts/sprt-blocks.sh classify GAMES_JSON [FILE...]
#   scripts/sprt-blocks.sh plan GAME CANDIDATE.rs [BASELINE.rs]
#   scripts/sprt-blocks.sh smoke GAME CANDIDATE.rs BASELINE
#   scripts/sprt-blocks.sh shard-start GAME CANDIDATE.rs BASELINE SHARD
#   scripts/sprt-blocks.sh shard-wait GAME SHARD
#   scripts/sprt-blocks.sh shard-finish
#   scripts/sprt-blocks.sh coordinate GAME CANDIDATE.rs BASELINE
#
# How it works. Each of SPRT_RUNNERS shards builds, calibrates itself on its
# own runner at its own parallelism (ADR 0048, decision 2) and plays, with the
# arena's `match`, one long tournament of its own seed (--shard), cut into
# blocks of SPRT_BLOCK_PAIRS seat-swapped pairs. Block n of shard k is a file
# the workflow uploads as the artifact sprt-block-GAME-ATTEMPT-K-N as soon as
# it is whole. The coordinator downloads the blocks, takes the complete rounds
# (round n is block n of every shard), and runs the arena's `sprt-check` over
# them in a fixed order; a verdict is uploaded as the artifact
# sprt-verdict-GAME-ATTEMPT, which stops the shards. A shard plays at most
# ceil(SPRT_MAX_PAIRS / (SPRT_RUNNERS x SPRT_BLOCK_PAIRS)) blocks. A game has
# at most MAX_BLOCKS blocks a shard, the number of upload steps of the
# workflow.
#
# classify: for the games of the JSON array GAMES_JSON and the added release
# FILEs of the pull request, prints (and appends to $GITHUB_OUTPUT) three lines:
#   serial_games=["id",...]     the games with SPRT_RUNNERS 1 (or none)
#   parallel_games=["id",...]   the games with more
#   parallel_matrix={"include":[{"game":"id","candidate":"games/id/releases/x.rs"},...]}
# A game with more than one runner needs TIME_SCALE=auto (calibrated; the
# runners differ in speed otherwise), SPRT_BLOCK_PAIRS, and exactly one added
# release: one experiment per pull request. Exit status 1 otherwise.
#
# plan: checks the game and the files (a game folder name, releases of
# games/GAME/releases), finds the baseline (BASELINE.rs, else the newest
# release older than the candidate, else `bot:` and the game's BASELINE_BOT),
# and prints, also to $GITHUB_OUTPUT: candidate, baseline, baseline_name,
# runners, shards (a JSON array), block_pairs, blocks (a shard's), key (of the
# kept result: the framework's and the game's code and these two bots).
#
# smoke: builds, calibrates this machine, and plays the smoke test of
# scripts/sprt.sh. Writes target/sprt-parallel/head.txt (the report's
# beginning, with the smoke test) and smoke-status (0 passed, 1 failed). The
# smoke test's games are kept as records in target/records when RECORDS_SAMPLE
# is set or by default 10 a step. Exit status 2 on errors only.
#
# shard-start: builds, calibrates, compiles the two bots, writes the
# calibration to target/sprt-parallel/calibration-K.txt, and starts the
# tournament of shard K in the background (session of its own, log in
# target/sprt-parallel/player.log). Shard 0 keeps a sample of its games as
# records in target/records.
#
# shard-wait: waits for the next block of the shard (a counter in the work
# folder says which), and appends to $GITHUB_ENV: SPRT_UPLOAD=true with
# SPRT_BLOCK (the artifact's name) and SPRT_BLOCK_FILE for the upload step;
# or SPRT_UPLOAD=false and SPRT_DONE=true when the shard is over: the verdict
# artifact exists, or the tournament ended. It stops the tournament then. A
# tournament that died before its last block is an error (status 1).
#
# shard-finish: stops the tournament and prints the end of its log.
#
# coordinate: polls the run's artifacts, adds up the complete rounds with the
# arena's `sprt-check`, and writes the result to target/sprt-result:
# report.txt, status (0 accepted; 1 rejected, inconclusive at the cap, or
# a candidate fault or failed smoke test; 2 error) and origin. Exit status 0
# whenever a result was written, 2 on errors before.
#
# Environment: for shard-wait and coordinate, GH_TOKEN, REPO (owner/name),
# RUN_ID and RUN_ATTEMPT; SPRT_WORK (target/sprt-parallel), SPRT_RESULT
# (target/sprt-result), POLL_SECONDS (30, the coordinator's polling),
# VERDICT_POLL_SECONDS (120, the shards'), RECORDS_SAMPLE (10).
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

# The upload steps of the workflow's shard job: blocks a shard can upload.
readonly MAX_BLOCKS=100

work="${SPRT_WORK:-target/sprt-parallel}"
result_dir="${SPRT_RESULT:-target/sprt-result}"

usage() {
  sed -n '/^# Usage:/,/^# How it works/p' "$0" | sed '$d;s/^# \{0,1\}//' >&2
  exit 2
}

die() {
  echo "error: $*" >&2
  exit 2
}

valid_game() { [[ ${1-} =~ ^[A-Za-z0-9_-]+$ ]]; }

# A release of game $1 given as path $2: games/GAME/releases/NAME.rs, a file.
check_release() { # check_release <game> <path> <what>
  local pattern="^games/$1/releases/[A-Za-z0-9._-]+\\.rs\$"
  [[ $2 =~ $pattern ]] || die "$3 '$2' is not a release of $1 (games/$1/releases/NAME.rs)"
  [[ -f $2 ]] || die "$3 $2 does not exist"
}

emit() { # emit <name>=<value>: to stdout and to $GITHUB_OUTPUT
  echo "$1"
  if [[ -n ${GITHUB_OUTPUT:-} ]]; then
    echo "$1" >>"$GITHUB_OUTPUT"
  fi
}

name_of() { basename "$1" .rs; }

# Sets the shared settings for game $1 and checks the ones a parallel SPRT needs.
parallel_settings() {
  load_settings "$1"
  TIME_SCALE="${SPRT_TIME_SCALE:-$TIME_SCALE}"
  TIME_TOLERANCE_MS="${SPRT_TIME_TOLERANCE_MS:-$TIME_TOLERANCE_MS}"
  local key
  for key in SPRT_RUNNERS SPRT_BLOCK_PAIRS SPRT_MAX_PAIRS; do
    [[ ${!key:-} =~ ^[1-9][0-9]*$ ]] || die "$key must be a positive number (now '${!key:-}') in games/$1/evaluation.env"
  done
  if ((SPRT_RUNNERS < 2)); then
    die "$1 has SPRT_RUNNERS=$SPRT_RUNNERS: its SPRT is not played over runners"
  fi
  if [[ $TIME_SCALE != auto || $TIME_TOLERANCE_MS != auto ]]; then
    die "$1 plays its SPRT over $SPRT_RUNNERS runners, which differ in speed: it needs TIME_SCALE=auto and TIME_TOLERANCE_MS=auto (calibrated, ADR 0048, ADR 0049 decision 8)"
  fi
  BLOCKS_PER_SHARD=$(((SPRT_MAX_PAIRS + SPRT_RUNNERS * SPRT_BLOCK_PAIRS - 1) / (SPRT_RUNNERS * SPRT_BLOCK_PAIRS)))
  if ((BLOCKS_PER_SHARD > MAX_BLOCKS)); then
    die "the cap of $SPRT_MAX_PAIRS pairs in blocks of $SPRT_BLOCK_PAIRS over $SPRT_RUNNERS runners is $BLOCKS_PER_SHARD blocks a runner, but the workflow uploads at most $MAX_BLOCKS: raise SPRT_BLOCK_PAIRS"
  fi
}

# ---- classify ----------------------------------------------------------

mode_classify() {
  (($# >= 1)) || usage
  local games_json="$1" game runners file rest count
  shift
  local -a serial=() parallel=() entries=() found=()
  while IFS= read -r game; do
    [[ -n $game ]] || continue
    valid_game "$game" || die "'$game' is not a game name"
    runners="$(sed -n 's/^SPRT_RUNNERS=//p' "games/$game/evaluation.env" | head -n 1)"
    if [[ -z $runners || $runners == 1 ]]; then
      serial+=("$game")
      continue
    fi
    [[ $runners =~ ^[1-9][0-9]*$ ]] || die "SPRT_RUNNERS of $game is '$runners', not a number"
    # The settings of a game over runners are checked in full here, before any
    # runner starts.
    (
      parallel_settings "$game"
    ) || exit 2
    found=()
    for file in "$@"; do
      file="${file#./}"
      rest="${file#"games/$game/releases/"}"
      if [[ $file == "games/$game/releases/"* && $rest != */* && $file == *.rs ]]; then
        found+=("$file")
      fi
    done
    count=${#found[@]}
    if ((count != 1)); then
      echo "::error title=$game SPRT::the pull request adds $count releases of $game, which plays its SPRT over $runners runners; one experiment per pull request, so add exactly one release" >&2
      exit 1
    fi
    [[ ${found[0]} =~ ^[A-Za-z0-9._/-]+$ ]] || die "unsupported characters in the name of ${found[0]}"
    parallel+=("$game")
    entries+=("$(jq -cn --arg game "$game" --arg candidate "${found[0]}" '{game: $game, candidate: $candidate}')")
  done < <(jq -r '.[]' <<<"$games_json")
  emit "serial_games=$(printf '%s\n' ${serial[@]+"${serial[@]}"} | jq -R . | jq -sc 'map(select(. != ""))')"
  emit "parallel_games=$(printf '%s\n' ${parallel[@]+"${parallel[@]}"} | jq -R . | jq -sc 'map(select(. != ""))')"
  emit "parallel_matrix=$(printf '%s\n' ${entries[@]+"${entries[@]}"} | jq -sc '{include: map(select(. != null))}')"
}

# ---- plan --------------------------------------------------------------

mode_plan() {
  (($# >= 2 && $# <= 3)) || usage
  game="$1"
  local candidate="$2" baseline="${3:-}"
  valid_game "$game" || die "'$game' is not a game name"
  [[ -f games/$game/evaluation.env ]] || die "'$game' is not a game with an evaluation.env"
  check_release "$game" "$candidate" "the candidate"
  parallel_settings "$game"
  if [[ -n $baseline ]]; then
    check_release "$game" "$baseline" "the baseline"
  else
    local candidate_name
    candidate_name="$(name_of "$candidate")"
    baseline="$(releases "$game" | awk -v c="$candidate_name" '
      { n = $0; sub(".*/", "", n); sub("\\.rs$", "", n) }
      n == c { exit }
      { last = $0 }
      END { print last }')"
    if [[ -z $baseline ]]; then
      [[ ${BASELINE_BOT:-} =~ ^[A-Za-z0-9_-]+$ ]] || die "BASELINE_BOT is missing in games/$game/evaluation.env"
      baseline="bot:$BASELINE_BOT"
    fi
  fi
  [[ $baseline != "$candidate" ]] || die "the candidate and the baseline are the same file"
  local baseline_name="${baseline#bot:}"
  baseline_name="$(name_of "$baseline_name")"
  # A result is kept for the exact code and settings it was obtained with, as
  # in sprt.yml, and for these two bots: a run may test any two releases.
  local hash
  hash="$(git ls-tree -r HEAD -- Cargo.toml Cargo.lock crates scripts "games/$game" | grep -v -e '\.md$' -e "[[:space:]]games/$game/league\.jsonl\$" | sha256sum | cut -c 1-20)"
  local shards
  shards="$(seq 0 $((SPRT_RUNNERS - 1)) | paste -sd, -)"
  emit "candidate=$candidate"
  emit "candidate_name=$(name_of "$candidate")"
  emit "baseline=$baseline"
  emit "baseline_name=$baseline_name"
  emit "runners=$SPRT_RUNNERS"
  emit "shards=[$shards]"
  emit "block_pairs=$SPRT_BLOCK_PAIRS"
  emit "blocks=$BLOCKS_PER_SHARD"
  emit "key=sprt-blocks-$game-$(name_of "$candidate")-$baseline_name-$hash"
}

# ---- shared by smoke and the shards ------------------------------------

# Builds the tools, calibrates this machine (CALIBRATION_JOBS copies at once,
# the number of CPUs: the games the arena plays at once, ADR 0048), compiles
# the two bots, and sets baseline_file, cand, base and options. $1 is the
# candidate, $2 the baseline as `plan` prints it.
prepare_bots() {
  local candidate="$1" baseline="$2"
  build_tools "$game"
  mkdir -p "$work/bin" "$work/calibration"
  calibrate_settings "$game" "$work/calibration"
  baseline_file="$baseline"
  if [[ $baseline == bot:* ]]; then
    bundle_bot "$game" "${baseline#bot:}" "$work"
    baseline_file="$work/$game-${baseline#bot:}.rs"
  fi
  CG_BIN_DIR="$work/bin" scripts/cg-check.sh "$candidate" "$baseline_file" >&2
  cand="$(name_of "$candidate")"
  base="$(name_of "$baseline_file")"
  read -r -a options <<<"$(arena_options)"
}

# One line on a runner's calibration.
calibration_line() { # calibration_line <label>
  echo "  $1: $CALIBRATION_BOT workload at $MACHINE_SPEED units a millisecond, $CALIBRATION_JOBS games at a time (CodinGame $CG_SPEED): time scale $TIME_SCALE, tolerance $TIME_TOLERANCE_MS ms"
}

# The settings in words, apart from each runner's calibration.
describe_parallel() {
  local percent
  percent="$(awk -v rate="$MAX_TIMEOUT_RATE" 'BEGIN { print rate * 100 }')"
  echo "opening plies $OPENING_PLIES, seed $SEED, timeouts allowed in up to $percent% of games; each runner calibrated to CodinGame's speed and real limit (ADR 0048); the SPRT played in blocks of $SPRT_BLOCK_PAIRS pairs over $SPRT_RUNNERS runners, checked after each round of $((SPRT_RUNNERS * SPRT_BLOCK_PAIRS)) pairs (ADR 0049)"
}

# ---- smoke -------------------------------------------------------------

mode_smoke() {
  (($# == 3)) || usage
  game="$1"
  local candidate="$2" baseline="$3"
  valid_game "$game" || die "'$game' is not a game name"
  check_release "$game" "$candidate" "the candidate"
  parallel_settings "$game"
  mkdir -p "$work"
  local baseline_file cand base
  local -a options
  prepare_bots "$candidate" "$baseline"
  bundle_bot "$game" random "$work"
  CG_BIN_DIR="$work/bin" scripts/cg-check.sh "$work/$game-random.rs" >&2
  local -a records=()
  if [[ -n ${RECORDS_DIR:-} ]]; then
    records=(--records "$RECORDS_DIR/$cand-smoke" --records-sample "${RECORDS_SAMPLE:-10}")
  fi
  local head="$work/head.txt" arena="target/release/$game-arena" status=0
  {
    echo "Candidate: $cand. Baseline: $base."
    echo "Settings: $(describe_parallel)."
    echo
  } >"$head"
  if ((SMOKE_PAIRS > 0)); then
    {
      echo "Smoke test against $game-random, on a runner calibrated as follows:"
      calibration_line "smoke test"
    } >>"$head"
    "$arena" match "${options[@]}" ${records[@]+"${records[@]}"} --expect-no-faults-from "$cand" \
      --bot "$cand=$work/bin/$cand" --bot "random=$work/bin/$game-random" \
      --pairs "$SMOKE_PAIRS" --min-score "$SMOKE_MIN_SCORE" \
      --out "$work/$cand-smoke.jsonl" >>"$head" || status=$?
    if ((status >= 2)); then
      die "the smoke test could not run (arena status $status)"
    fi
    if ((status != 0)); then
      {
        echo
        echo "Verdict: SMOKE TEST FAILED (needs a score of $SMOKE_MIN_SCORE, no crash or invalid answer, and timeouts within MAX_TIMEOUT_RATE=$MAX_TIMEOUT_RATE)"
      } >>"$head"
    else
      echo >>"$head"
    fi
  fi
  echo "$status" >"$work/smoke-status"
  cat "$head"
  emit "smoke_status=$status"
  emit "machine_speed=$MACHINE_SPEED"
}

# ---- shards ------------------------------------------------------------

# Runs `gh api` with a few tries; prints nothing and fails when it cannot.
gh_get() {
  local try
  for try in 1 2 3; do
    if gh api "$@" 2>/dev/null; then
      return 0
    fi
    sleep $((try * 3))
  done
  return 1
}

# Whether the verdict artifact of this run exists. Fails when the API does not
# answer, which is no answer: the caller goes on.
verdict_exists() { # verdict_exists <game>
  local count
  count="$(gh_get "repos/$REPO/actions/runs/$RUN_ID/artifacts?name=sprt-verdict-$1-$RUN_ATTEMPT&per_page=1" --jq '.total_count')" || return 2
  [[ $count != 0 ]]
}

stop_player() {
  local pid
  pid="$(cat "$work/player.pid" 2>/dev/null || true)"
  if [[ -n $pid ]]; then
    # The tournament leads a session of its own: its bots die with it.
    kill -- "-$pid" 2>/dev/null || kill "$pid" 2>/dev/null || true
  fi
}

mode_shard_start() {
  (($# == 4)) || usage
  game="$1"
  local candidate="$2" baseline="$3" shard="$4"
  valid_game "$game" || die "'$game' is not a game name"
  check_release "$game" "$candidate" "the candidate"
  [[ $shard =~ ^[0-9]+$ ]] || die "SHARD must be a number"
  parallel_settings "$game"
  ((shard < SPRT_RUNNERS)) || die "shard $shard is not one of the $SPRT_RUNNERS runners"
  mkdir -p "$work/blocks" "$work/upload"
  local baseline_file cand base
  local -a options
  prepare_bots "$candidate" "$baseline"
  {
    echo "shard $shard"
    calibration_line "shard $shard"
  } >"$work/calibration-$shard.txt"
  echo "$game: shard $shard of $SPRT_RUNNERS. $(calibration_line "this runner")"
  echo 0 >"$work/next"
  : >"$work/last-poll"
  local -a records=()
  if ((shard == 0)); then
    records=(--records "target/records/$cand-sprt" --records-sample "${RECORDS_SAMPLE:-10}")
  fi
  local pairs=$((BLOCKS_PER_SHARD * SPRT_BLOCK_PAIRS))
  echo "Settings: $(describe_parallel). This shard plays up to $BLOCKS_PER_SHARD blocks ($pairs pairs)."
  # No --expect-no-faults: the coordinator decides on faults.
  setsid nohup "target/release/$game-arena" match "${options[@]}" ${records[@]+"${records[@]}"} \
    --shard "$shard" --block-pairs "$SPRT_BLOCK_PAIRS" --block-dir "$work/blocks" --pairs "$pairs" \
    --bot "$cand=$work/bin/$cand" --bot "$base=$work/bin/$base" \
    >"$work/player.log" 2>&1 </dev/null &
  echo "$!" >"$work/player.pid"
  echo "Tournament started (process $!)."
}

# Sets the variables the upload step reads, in the file $GITHUB_ENV names.
set_env() { # set_env NAME=value...
  local line
  for line in "$@"; do
    echo "$line"
    if [[ -n ${GITHUB_ENV:-} ]]; then
      echo "$line" >>"$GITHUB_ENV"
    fi
  done
}

mode_shard_wait() {
  (($# == 2)) || usage
  game="$1"
  local shard="$2"
  valid_game "$game" || die "'$game' is not a game name"
  [[ $shard =~ ^[0-9]+$ ]] || die "SHARD must be a number"
  : "${REPO:?set REPO to owner/name}" "${RUN_ID:?set RUN_ID}" "${RUN_ATTEMPT:?set RUN_ATTEMPT}"
  parallel_settings "$game"
  local n file pid interval="${VERDICT_POLL_SECONDS:-120}" last now
  n="$(cat "$work/next")"
  echo $((n + 1)) >"$work/next"
  file="$work/blocks/block-$n.jsonl"
  pid="$(cat "$work/player.pid")"
  while true; do
    if [[ -f $file ]]; then
      cp "$file" "$work/upload/sprt-block-$game-$RUN_ATTEMPT-$shard-$n.jsonl"
      set_env SPRT_UPLOAD=true "SPRT_BLOCK=sprt-block-$game-$RUN_ATTEMPT-$shard-$n" \
        "SPRT_BLOCK_FILE=$work/upload/sprt-block-$game-$RUN_ATTEMPT-$shard-$n.jsonl"
      return 0
    fi
    if ((n >= BLOCKS_PER_SHARD)); then
      echo "Shard $shard played its $BLOCKS_PER_SHARD blocks."
      set_env SPRT_UPLOAD=false SPRT_DONE=true
      return 0
    fi
    if ! kill -0 "$pid" 2>/dev/null; then
      echo "error: the tournament of shard $shard ended before block $n; the end of its log:" >&2
      tail -n 20 "$work/player.log" >&2 || true
      set_env SPRT_UPLOAD=false SPRT_DONE=true
      return 1
    fi
    now="$(date +%s)"
    last="$(cat "$work/last-poll" 2>/dev/null || true)"
    if [[ -z $last ]] || ((now - last >= interval)); then
      echo "$now" >"$work/last-poll"
      if verdict_exists "$game"; then
        echo "The verdict is in: shard $shard stops."
        stop_player
        set_env SPRT_UPLOAD=false SPRT_DONE=true
        return 0
      fi
    fi
    sleep "${WAIT_SECONDS:-3}"
  done
}

mode_shard_finish() {
  stop_player
  if [[ -f $work/player.log ]]; then
    echo "---- the end of the tournament's log ----"
    tail -n 5 "$work/player.log"
  fi
}

# ---- coordinate --------------------------------------------------------

# Downloads the block artifacts not seen yet into $work/coordinator/blocks,
# as SHARD-N.jsonl. $1 is `full` to list every page, else the newest page
# and the next ones only while they hold nothing known.
fetch_blocks() {
  local mode="$1" page=1 place prefix="sprt-block-$game-$RUN_ATTEMPT-" listing id name rest unseen
  local dir="$work/coordinator/blocks"
  mkdir -p "$dir"
  while true; do
    listing="$(gh_get "repos/$REPO/actions/runs/$RUN_ID/artifacts?per_page=100&page=$page" \
      --jq '.artifacts[] | select(.expired | not) | [.id, .name] | @tsv')" || {
      echo "::warning title=SPRT::the artifacts could not be listed; trying again at the next poll"
      return 0
    }
    unseen=0
    while IFS=$'\t' read -r id name; do
      [[ -n $id ]] || continue
      [[ $name == "$prefix"* ]] || continue
      rest="${name#"$prefix"}"
      [[ $rest =~ ^([0-9]+)-([0-9]+)$ ]] || continue
      place="${BASH_REMATCH[1]}-${BASH_REMATCH[2]}"
      if [[ -n ${seen[$id]:-} ]]; then
        continue
      fi
      unseen=$((unseen + 1))
      if gh_get "repos/$REPO/actions/artifacts/$id/zip" >"$work/coordinator/artifact.zip" &&
        unzip -p "$work/coordinator/artifact.zip" >"$dir/$place.jsonl.part"; then
        mv "$dir/$place.jsonl.part" "$dir/$place.jsonl"
        seen[$id]=1
      else
        echo "::warning title=SPRT::artifact $name could not be downloaded; trying again at the next poll"
      fi
    done <<<"$listing"
    # A page of 100 that holds something new may be followed by older ones.
    if [[ $(grep -c . <<<"$listing" || true) -lt 100 ]]; then
      break
    fi
    if [[ $mode != full && $unseen -eq 0 ]]; then
      break
    fi
    page=$((page + 1))
  done
}

# The number of rounds whose blocks are all downloaded.
complete_rounds() {
  local round=0 shard
  while true; do
    for ((shard = 0; shard < SPRT_RUNNERS; shard++)); do
      [[ -s $work/coordinator/blocks/$shard-$round.jsonl ]] || {
        echo "$round"
        return 0
      }
    done
    round=$((round + 1))
  done
}

# Whether every shard job of this run has ended. Fails when unknown.
shards_ended() {
  local listing shard
  listing="$(gh_get "repos/$REPO/actions/runs/$RUN_ID/attempts/$RUN_ATTEMPT/jobs?per_page=100" \
    --jq '.jobs[] | [.name, .status] | @tsv')" || return 2
  for ((shard = 0; shard < SPRT_RUNNERS; shard++)); do
    grep -qE "Shard $shard \\($game\\)"$'\t'"completed\$" <<<"$listing" || return 1
  done
  return 0
}

# Runs sprt-check over the first $1 rounds; sets check_code and check_output.
run_check() {
  local rounds="$1" round shard
  local -a blocks=()
  for ((round = 0; round < rounds; round++)); do
    for ((shard = 0; shard < SPRT_RUNNERS; shard++)); do
      blocks+=(--block "$work/coordinator/blocks/$shard-$round.jsonl")
    done
  done
  check_code=0
  check_output="$("target/release/$game-arena" sprt-check "${blocks[@]}" --round-blocks "$SPRT_RUNNERS" \
    --candidate "$cand" --baseline "$base" \
    --elo0 "$SPRT_ELO0" --elo1 "$SPRT_ELO1" --alpha "$SPRT_ALPHA" --beta "$SPRT_BETA" \
    --max-pairs "$SPRT_MAX_PAIRS" --expect-no-faults-from "$cand" \
    --max-timeout-rate "$MAX_TIMEOUT_RATE" 2>&1)" || check_code=$?
}

# Gets the shards' calibration lines (artifacts sprt-calibration-...), sorted.
fetch_calibrations() {
  local shard id file="$work/coordinator/calibrations.txt"
  : >"$file"
  for ((shard = 0; shard < SPRT_RUNNERS; shard++)); do
    id="$(gh_get "repos/$REPO/actions/runs/$RUN_ID/artifacts?name=sprt-calibration-$game-$RUN_ATTEMPT-$shard&per_page=1" \
      --jq '.artifacts[0].id // empty')" || id=""
    if [[ -n $id ]] && gh_get "repos/$REPO/actions/artifacts/$id/zip" >"$work/coordinator/artifact.zip"; then
      unzip -p "$work/coordinator/artifact.zip" | sed -n '2p' >>"$file" || true
    else
      echo "  shard $shard: no calibration found" >>"$file"
    fi
  done
}

# Writes the result: $1 the status, $2 the text after the head and the
# calibrations (the SPRT's lines, or the reason).
write_result() {
  mkdir -p "$result_dir"
  {
    cat "$work/head.txt"
    if [[ ! -f $work/smoke-failed ]]; then
      echo "Calibration of the SPRT's runners:"
      cat "$work/coordinator/calibrations.txt"
      echo
      echo "SPRT against $base, in blocks of $SPRT_BLOCK_PAIRS pairs over $SPRT_RUNNERS runners:"
      echo "$2"
    fi
  } | sed '/^::error title=/d' >"$result_dir/report.txt"
  echo "$1" >"$result_dir/status"
  echo "$RUN_ID-$RUN_ATTEMPT" >"$result_dir/origin"
  echo "Result: status $1."
}

mode_coordinate() {
  (($# == 3)) || usage
  game="$1"
  local candidate="$2" baseline="$3"
  valid_game "$game" || die "'$game' is not a game name"
  check_release "$game" "$candidate" "the candidate"
  : "${REPO:?set REPO to owner/name}" "${RUN_ID:?set RUN_ID}" "${RUN_ATTEMPT:?set RUN_ATTEMPT}"
  parallel_settings "$game"
  [[ -f $work/head.txt && -f $work/smoke-status ]] || die "$work has no head.txt or smoke-status: the prepare job's files are missing"
  cand="$(name_of "$candidate")"
  if [[ $baseline == bot:* ]]; then
    base="$game-${baseline#bot:}"
  else
    base="$(name_of "$baseline")"
  fi
  mkdir -p "$work/coordinator"
  # The smoke test failed: that is the result, with nothing to wait for.
  if [[ $(cat "$work/smoke-status") != 0 ]]; then
    touch "$work/smoke-failed"
    write_result 1 ""
    return 0
  fi
  declare -gA seen=()
  local poll=0 checked=0 rounds
  local poll_seconds="${POLL_SECONDS:-30}"
  echo "Waiting for the blocks of $SPRT_RUNNERS shards: up to $BLOCKS_PER_SHARD rounds of $((SPRT_RUNNERS * SPRT_BLOCK_PAIRS)) pairs."
  while true; do
    poll=$((poll + 1))
    if ((poll % 10 == 0)); then
      fetch_blocks full
    else
      fetch_blocks newest
    fi
    rounds="$(complete_rounds)"
    if ((rounds > checked)); then
      run_check "$rounds"
      checked=$rounds
      echo "Round $rounds: sprt-check exit $check_code. $(grep -E '^SPRT ' <<<"$check_output" | tail -n 1)"
      if conclude; then
        return 0
      fi
    fi
    # The shard jobs are looked at every fourth poll, to spare the API.
    if ((poll % 4 == 0)) && shards_ended; then
      # Nothing new will come: a last look at what was uploaded just before.
      fetch_blocks full
      rounds="$(complete_rounds)"
      if ((rounds > checked)); then
        run_check "$rounds"
        checked=$rounds
        if conclude; then
          return 0
        fi
      fi
      fetch_calibrations
      local note="error: every shard ended before a verdict ($checked complete rounds counted)."
      echo "$note"
      write_result 2 "${check_output:-}
$note"
      return 0
    fi
    sleep "$poll_seconds"
  done
}

# Writes the result and returns 0 when sprt-check's exit status in $check_code
# is a verdict; returns 1 to go on; writes an error result for any other.
conclude() {
  case $check_code in
    0)
      fetch_calibrations
      write_result 0 "$check_output"
      return 0
      ;;
    1 | 3)
      fetch_calibrations
      write_result 1 "$check_output"
      return 0
      ;;
    4) return 1 ;;
    *)
      fetch_calibrations
      write_result 2 "$check_output
error: sprt-check failed (status $check_code)"
      return 0
      ;;
  esac
}

# ---- main --------------------------------------------------------------

(($# >= 1)) || usage
mode="$1"
shift
case $mode in
  classify) mode_classify "$@" ;;
  plan) mode_plan "$@" ;;
  smoke) mode_smoke "$@" ;;
  shard-start) mode_shard_start "$@" ;;
  shard-wait) mode_shard_wait "$@" ;;
  shard-finish) mode_shard_finish "$@" ;;
  coordinate) mode_coordinate "$@" ;;
  *)
    echo "sprt-blocks: unknown mode $mode" >&2
    usage
    ;;
esac
