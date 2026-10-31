#!/usr/bin/env bash
# The steps of a game's nightly league (ADR 0049, 0050), run by the workflow
# .github/workflows/league-night.yml with the game's evaluation settings
# (games/GAME/evaluation.env: LEAGUE_* and LEAGUE_NIGHTLY_*).
#
# Usage:
#   scripts/league-night.sh plan GAME WAVE RESULTS OUT
#   scripts/league-night.sh play GAME PLAN SHARD DEADLINE_MINUTES
#   scripts/league-night.sh merge GAME RESULTS OUT FILE...
#   scripts/league-night.sh publish GAME RESULTS LEADERBOARD_OUT
#
# plan: chooses the night's pairings from the results file RESULTS with the
# arena's `plan`, and writes them to OUT. The pool is the baseline bots of
# LEAGUE_BOTS and every release of the game (vNNN, in release order); the
# anchor is LEAGUE_ANCHOR; the fixed opponents are those of LEAGUE_FIXED that
# are in the pool. WAVE 1 plans the night with the budget LEAGUE_NIGHTLY_PAIRS.
# WAVE 2 is planned on the results merged after wave 1, with a budget of 0: the
# `new` lines only, that is the nearest opponents of the releases that got
# their first games in wave 1. The lines are shared between
# LEAGUE_NIGHTLY_RUNNERS shards. The night is today's UTC date, or LEAGUE_NIGHT
# (YYYY-MM-DD) when set; wave 2 adds `-2`, so that its seeds differ. Prints the
# planner's summary, then `lines=N` (the lines of the plan) and `new=N` (those
# of kind new); both are also appended to $GITHUB_OUTPUT when it is set.
#
# play: plays the lines of shard SHARD of the plan PLAN, in the order of the
# plan, on this machine: builds the tools and the bots the lines name
# (releases are compiled the way CodinGame compiles them, baselines are
# bundled), calibrates (TIME_SCALE=auto, ADR 0048), and runs the arena's
# `play`, which starts no line after DEADLINE_MINUTES minutes. Writes the
# finished lines to target/league/GAME-night-SHARD.jsonl. When the shard has
# `new` lines, those are played first and apart, and a sample of their games
# is kept as records (ADR 0027) in target/league-records/GAME-league
# (RECORDS_SAMPLE games per pair of bots, 10 by default); their lines go to
# target/league/GAME-night-SHARD-new.jsonl, and the rest of the shard to
# GAME-night-SHARD.jsonl as before. A shard with no line in the plan exits
# with 0 without building or playing, and writes an empty file.
#
# merge: adds the lines of the FILEs to RESULTS and writes the result to OUT
# (which may be RESULTS), with the arena's `merge`. Prints its report, and each
# line `warning: drift` also as a warning annotation. With no FILE, OUT is a
# copy of RESULTS.
#
# publish: writes the leaderboard of RESULTS to LEADERBOARD_OUT, rated from
# LEAGUE_ANCHOR, with the game's journal.
#
# Environment: LEAGUE_COMMIT, the commit written to each line (HEAD by
# default); LEAGUE_NIGHT; RECORDS_SAMPLE.
#
# Exit status: 0 on success; 2 on errors.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

usage() {
  sed -n '/^# Usage:/,/^# plan:/p' "$0" | sed '$d;s/^# \{0,1\}//' >&2
  exit 2
}

die() {
  echo "error: $*" >&2
  exit 2
}

valid_game() { [[ ${1-} =~ ^[A-Za-z0-9_-]+$ ]]; }

# Checks that the settings $@ are positive whole numbers.
need_numbers() {
  local key
  for key in "$@"; do
    [[ ${!key:-} =~ ^[1-9][0-9]*$ ]] || die "$key must be a positive number (now '${!key:-}') in games/$game/evaluation.env"
  done
}

# Prints the arguments joined by commas.
join() {
  local IFS=,
  echo "$*"
}

# Prints the lines of plan $1 that belong to shard $2.
shard_lines() {
  grep -E "\"shard\":$2\}\$" "$1" || true
}

# ---- plan --------------------------------------------------------------

mode_plan() {
  [[ $# -eq 4 ]] || usage
  game="$1"
  local wave="$2" results="$3" out="$4"
  valid_game "$game" || usage
  [[ $wave == 1 || $wave == 2 ]] || die "WAVE must be 1 or 2, not '$wave'"
  [[ -f $results ]] || die "$results does not exist"
  load_settings "$game"
  need_numbers LEAGUE_NIGHTLY_RUNNERS LEAGUE_PAIRS LEAGUE_TOP LEAGUE_BLOCK_PAIRS
  [[ ${LEAGUE_NIGHTLY_PAIRS:-} =~ ^[0-9]+$ ]] || die "LEAGUE_NIGHTLY_PAIRS must be a number"
  [[ ${LEAGUE_TARGET_MARGIN:-} =~ ^[0-9]+(\.[0-9]+)?$ ]] || die "LEAGUE_TARGET_MARGIN must be a number of Elo"
  [[ ${LEAGUE_LOW_SHARE:-} =~ ^(0|1|0?\.[0-9]+)$ ]] || die "LEAGUE_LOW_SHARE must be a number from 0 to 1"
  [[ -n ${LEAGUE_ANCHOR:-} ]] || die "LEAGUE_ANCHOR is missing"
  [[ -n ${LEAGUE_NEAREST:-} ]] || die "LEAGUE_NEAREST is missing"
  local night="${LEAGUE_NIGHT:-$(date -u +%F)}" budget="$LEAGUE_NIGHTLY_PAIRS"
  [[ $night =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || die "LEAGUE_NIGHT must be a date, YYYY-MM-DD, not '$night'"
  if [[ $wave == 2 ]]; then
    night="$night-2"
    budget=0
  fi
  build_tools "$game"

  # The pool: the baselines, then the releases in order.
  local -a baselines=() names=() fixed=() wanted=()
  local file name bot member
  IFS=',' read -r -a baselines <<<"$LEAGUE_BOTS"
  names=("${baselines[@]}")
  local -a release_names=()
  while IFS= read -r file; do
    [[ -n $file ]] || continue
    name="$(basename "$file" .rs)"
    name="${name#"$game-"}"
    [[ $name =~ ^v[0-9]+$ ]] || die "the release $file is not named $game-vNNN.rs"
    release_names+=("$name")
  done < <(releases "$game")
  ((${#release_names[@]} > 0)) || die "games/$game/releases has no release"
  names+=("${release_names[@]}")
  IFS=',' read -r -a wanted <<<"${LEAGUE_FIXED:-}"
  for bot in ${wanted[@]+"${wanted[@]}"}; do
    for member in "${names[@]}"; do
      if [[ $member == "$bot" ]]; then
        fixed+=("$bot")
      fi
    done
  done

  mkdir -p "$(dirname "$out")"
  echo "League night $night, wave $wave, game $game: $LEAGUE_NIGHTLY_RUNNERS shards, budget $budget pairs."
  target/release/"$game-arena" plan --results "$results" \
    --pool "$(join "${names[@]}")" --releases "$(join "${release_names[@]}")" \
    --anchor "$LEAGUE_ANCHOR" --fixed-opponents "$(join ${fixed[@]+"${fixed[@]}"})" \
    --nearest "$LEAGUE_NEAREST" --new-pairs "$LEAGUE_PAIRS" --budget-pairs "$budget" \
    --block-pairs "$LEAGUE_BLOCK_PAIRS" --top "$LEAGUE_TOP" \
    --target-margin "$LEAGUE_TARGET_MARGIN" --low-share "$LEAGUE_LOW_SHARE" \
    --night "$night" --shards "$LEAGUE_NIGHTLY_RUNNERS" --out "$out"
  local lines new
  lines="$(grep -c . "$out" || true)"
  new="$(grep -c '"kind":"new"' "$out" || true)"
  echo "lines=$lines"
  echo "new=$new"
  if [[ -n ${GITHUB_OUTPUT:-} ]]; then
    {
      echo "lines=$lines"
      echo "new=$new"
    } >>"$GITHUB_OUTPUT"
  fi
}

# ---- play --------------------------------------------------------------

mode_play() {
  [[ $# -eq 4 ]] || usage
  game="$1"
  local plan="$2" shard="$3" deadline="$4"
  valid_game "$game" || usage
  [[ $shard =~ ^[0-9]+$ && $deadline =~ ^[0-9]+$ ]] || usage
  [[ -f $plan ]] || die "$plan does not exist"
  load_settings "$game"
  local dir=target/league
  mkdir -p "$dir/bin"
  local mine new_count
  mine="$(shard_lines "$plan" "$shard")"
  local output="$dir/$game-night-$shard.jsonl"
  if [[ -z $mine ]]; then
    : >"$output"
    echo "Shard $shard has no line in $plan: nothing to play."
    return 0
  fi
  new_count="$(grep -c '"kind":"new"' <<<"$mine" || true)"
  echo "Shard $shard: $(grep -c . <<<"$mine") lines, $new_count of them new; the deadline is $deadline minutes."

  TIME_SCALE="${SPRT_TIME_SCALE:-$TIME_SCALE}"
  TIME_TOLERANCE_MS="${SPRT_TIME_TOLERANCE_MS:-$TIME_TOLERANCE_MS}"
  local started=$SECONDS
  build_tools "$game"
  calibrate_settings "$game" "$dir/calibration"

  # The bots the lines name.
  local -a baselines=() files=() args=() needed=()
  IFS=',' read -r -a baselines <<<"$LEAGUE_BOTS"
  local a b bot known
  while IFS= read -r line; do
    a="$(sed -n 's/^{"a":"\([A-Za-z0-9_-]*\)","b":"\([A-Za-z0-9_-]*\)",.*/\1/p' <<<"$line")"
    b="$(sed -n 's/^{"a":"\([A-Za-z0-9_-]*\)","b":"\([A-Za-z0-9_-]*\)",.*/\2/p' <<<"$line")"
    [[ -n $a && -n $b ]] || die "cannot read the bots of the plan line: $line"
    for bot in "$a" "$b"; do
      known=false
      for member in ${needed[@]+"${needed[@]}"}; do
        [[ $member == "$bot" ]] && known=true
      done
      if [[ $known == false ]]; then
        needed+=("$bot")
      fi
    done
  done <<<"$mine"
  local is_baseline member
  for bot in "${needed[@]}"; do
    is_baseline=false
    for member in "${baselines[@]}"; do
      [[ $member == "$bot" ]] && is_baseline=true
    done
    if [[ $is_baseline == true ]]; then
      bundle_bot "$game" "$bot" "$dir"
      files+=("$dir/$game-$bot.rs")
    elif [[ -f games/$game/releases/$game-$bot.rs ]]; then
      files+=("games/$game/releases/$game-$bot.rs")
    else
      die "'$bot' is neither a baseline bot of LEAGUE_BOTS ($LEAGUE_BOTS) nor a release of games/$game/releases"
    fi
    args+=(--bot "$bot=$dir/bin/$game-$bot")
  done
  CG_BIN_DIR="$dir/bin" scripts/cg-check.sh "${files[@]}" >&2

  local -a speed=()
  if [[ -n $MACHINE_SPEED ]]; then
    speed=(--speed "$MACHINE_SPEED")
  fi
  local -a options
  read -r -a options <<<"$(arena_options)"
  local commit="${LEAGUE_COMMIT:-$(git rev-parse HEAD)}"
  echo "Settings: $(describe_settings)."

  # Plays the shard's lines of plan file $1 into file $2, within what is left
  # of the deadline, with the arena options $3...
  play_lines() {
    local from="$1" into="$2" left status=0
    shift 2
    left=$((deadline - (SECONDS - started) / 60))
    ((left >= 0)) || left=0
    "target/release/$game-arena" play --plan "$from" --shard "$shard" --new-results "$into" \
      --deadline-minutes "$left" --commit "$commit" \
      ${speed[@]+"${speed[@]}"} "${options[@]}" "${args[@]}" "$@" || status=$?
    if ((status != 0)); then
      die "the play of shard $shard failed (arena status $status)"
    fi
  }

  if ((new_count > 0)); then
    # The new lines first, with a sample of their games kept.
    local first="$dir/$game-night-$shard-plan-new.jsonl" rest="$dir/$game-night-$shard-plan-rest.jsonl"
    grep '"kind":"new"' <<<"$mine" >"$first"
    grep -v '"kind":"new"' <<<"$mine" >"$rest" || true
    play_lines "$first" "$dir/$game-night-$shard-new.jsonl" \
      --records "target/league-records/$game-league" --records-sample "${RECORDS_SAMPLE:-10}"
    if [[ -s $rest ]]; then
      play_lines "$rest" "$output"
    else
      : >"$output"
    fi
  else
    play_lines "$plan" "$output"
  fi
  echo "---- night results of shard $shard ----"
  cat "$dir/$game-night-$shard-new.jsonl" 2>/dev/null || true
  cat "$output"
  echo "---- end ----"
}

# ---- merge -------------------------------------------------------------

mode_merge() {
  [[ $# -ge 3 ]] || usage
  game="$1"
  local results="$2" out="$3"
  shift 3
  valid_game "$game" || usage
  [[ -f $results ]] || die "$results does not exist"
  if (($# == 0)); then
    if [[ $results != "$out" ]]; then
      cp "$results" "$out"
    fi
    echo "merged 0 new lines into $out"
    return 0
  fi
  local file
  local -a add=()
  for file in "$@"; do
    [[ -f $file ]] || die "$file does not exist"
    add+=(--add "$file")
  done
  load_settings "$game"
  build_tools "$game"
  report="$(mktemp)"
  trap 'rm -f "$report"' EXIT
  local status=0
  "target/release/$game-arena" merge --results "$results" "${add[@]}" --out "$out" >"$report" || status=$?
  cat "$report"
  if ((status != 0)); then
    die "the merge failed (arena status $status)"
  fi
  local line
  while IFS= read -r line; do
    echo "::warning title=League drift::${line#warning: }"
  done < <(grep '^warning: drift' "$report" || true)
}

# ---- publish -----------------------------------------------------------

mode_publish() {
  [[ $# -eq 3 ]] || usage
  game="$1"
  local results="$2" out="$3"
  valid_game "$game" || usage
  [[ -f $results ]] || die "$results does not exist"
  load_settings "$game"
  [[ -n ${LEAGUE_ANCHOR:-} ]] || die "LEAGUE_ANCHOR is missing"
  build_tools "$game"
  "target/release/$game-arena" leaderboard --results "$results" --anchor "$LEAGUE_ANCHOR" \
    --journal "games/$game/journal/README.md" --out "$out"
}

# ---- main --------------------------------------------------------------

(($# >= 1)) || usage
mode="$1"
shift
case $mode in
  plan) mode_plan "$@" ;;
  play) mode_play "$@" ;;
  merge) mode_merge "$@" ;;
  publish) mode_publish "$@" ;;
  *) usage ;;
esac
