#!/usr/bin/env bash
# Plays the league of each new release of a game (ADR 0038): the arena's
# incremental league, with the game's evaluation settings
# (games/<game>/evaluation.env).
#
# Usage: scripts/league.sh GAME NEW...
#
# Each NEW is a bot name as the arena knows it: a release (v019, from
# games/GAME/releases/GAME-v019.rs) or a baseline bot of LEAGUE_BOTS (greedy,
# from games/GAME/bots/greedy). They are played in order, each as a new
# release would be: against the opponents the arena chooses among the bots of
# the results file and of its pool, at the SPRT's time limits (SPRT_TIME_SCALE
# and SPRT_TIME_TOLERANCE_MS when set; CodinGame's exact limits, ADR 0042, or
# calibrated on this machine when they are `auto`, ADR 0048, with the
# machine's speed written to each new line). The pool of a NEW is the
# baseline bots of LEAGUE_BOTS (only those up to and including NEW when NEW is
# a baseline) and, when NEW is a release, every release up to and including
# NEW: a release after NEW is never passed to the arena, so that a replay of
# history sees only the bots that existed. The previous release is the highest
# release below NEW, if any. Of the fixed opponents LEAGUE_FIXED, only those in
# NEW's pool other than NEW itself are passed (greedy does not play itself,
# and v001 does not exist yet in a replay of the releases before it).
#
# The results file is games/GAME/league.jsonl, or an empty file under
# target/league/ when it does not exist: the script never creates it in
# games/. The new pairings of NEW go to target/league/GAME-NEW.jsonl and are
# printed between the lines `---- new results of NEW ----` and `---- end ----`,
# so that a log reader can copy them exactly. A pull request carries one
# release; with several, each is played against the stored results only, and
# the choice of the second ignores the pairings of the first.
#
# The canary LEAGUE_CANARY (A,B), when not empty, both bots are in the pool
# and the results file holds their pairing, is replayed after the pairings; a
# line `warning: canary ...` of the arena becomes a warning annotation. While
# the pairing is not stored yet (a replay of the history from an empty
# results file, before it plays A or B), it is not replayed, with a warning.
#
# Exit status: 0 on success; 1 when NEW faults (--expect-no-faults-from NEW,
# up to MAX_TIMEOUT_RATE); 2 on errors. It stops at the first NEW that fails.
#
# Environment:
# - LEAGUE_APPEND=true (the bootstrap, ADR 0038 decision 8): after each NEW,
#   appends its new lines to the results file in the working copy, so that
#   the next NEW sees them. False by default.
# - LEAGUE_STOP_AFTER_MINUTES=N (the bootstrap): starts no NEW once N minutes
#   have passed since the script started; prints `Stopped before NEW: ... Next:
#   NEW REST...` and exits with 0.
# - LEAGUE_COMMIT: the commit written to each new line; HEAD by default. A pull
#   request's workflow gives the pull request's last commit, since its HEAD is
#   a temporary merge commit.
# - RECORDS_DIR: when set and not empty, keeps a sample of the games as game
#   records (ADR 0027) in RECORDS_DIR/GAME-league. A relative RECORDS_DIR is
#   relative to the repository root. RECORDS_SAMPLE is the number of games kept
#   per pair of bots, 10 by default; every game that ended in a fault is kept
#   too. The arena never overwrites a record: give a new or empty RECORDS_DIR
#   (and no canary when several NEW are given, since each would replay it).
#   Without RECORDS_DIR, nothing changes.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -lt 2 ]]; then
  echo "usage: $0 GAME NEW..." >&2
  exit 2
fi
game="$1"
shift
news=("$@")
if [[ ! $game =~ ^[A-Za-z0-9_-]+$ ]]; then
  echo "error: '$game' is not a game name" >&2
  exit 2
fi
load_settings "$game"
TIME_SCALE="${SPRT_TIME_SCALE:-$TIME_SCALE}"
TIME_TOLERANCE_MS="${SPRT_TIME_TOLERANCE_MS:-$TIME_TOLERANCE_MS}"
append="${LEAGUE_APPEND:-false}"
stop_after="${LEAGUE_STOP_AFTER_MINUTES:-}"
if [[ $append != true && $append != false ]]; then
  echo "error: LEAGUE_APPEND must be true or false, not '$append'" >&2
  exit 2
fi
if [[ -n $stop_after && ! $stop_after =~ ^[0-9]+$ ]]; then
  echo "error: LEAGUE_STOP_AFTER_MINUTES must be a number of minutes, not '$stop_after'" >&2
  exit 2
fi
build_tools "$game"

dir=target/league
mkdir -p "$dir/bin"
calibrate_settings "$game" "$dir/calibration"

IFS=',' read -r -a baselines <<<"$LEAGUE_BOTS"
# The number of a release name (v019 gives 19), or nothing.
release_number() { # release_number <name>
  if [[ $1 =~ ^v([0-9]+)$ ]]; then
    echo "$((10#${BASH_REMATCH[1]}))"
  fi
}
# Releases of the game, oldest first, as bot names (v001, ...).
release_names=()
while IFS= read -r file; do
  [[ -n $file ]] || continue
  name="$(basename "$file" .rs)"
  release_names+=("${name#"$game-"}")
done < <(releases "$game")

is_baseline() { # is_baseline <name>
  local bot
  for bot in "${baselines[@]}"; do
    [[ $bot == "$1" ]] && return 0
  done
  return 1
}
is_release() { # is_release <name>
  local release
  for release in "${release_names[@]}"; do
    [[ $release == "$1" ]] && return 0
  done
  return 1
}

# Checks the names, then bundles and compiles every bot that any NEW needs,
# once: the pool of the largest NEW holds the pools of the others.
needed=()
add_needed() { # add_needed <name>
  local known
  for known in ${needed[@]+"${needed[@]}"}; do
    [[ $known == "$1" ]] && return 0
  done
  needed+=("$1")
}
for new in "${news[@]}"; do
  if [[ ! $new =~ ^[A-Za-z0-9_-]+$ ]]; then
    echo "error: '$new' is not a bot name" >&2
    exit 2
  fi
  if ! is_baseline "$new" && ! is_release "$new"; then
    echo "error: '$new' is neither a baseline bot of LEAGUE_BOTS ($LEAGUE_BOTS) nor a release of games/$game/releases" >&2
    exit 2
  fi
  if ! is_baseline "$new" && [[ -z "$(release_number "$new")" ]]; then
    echo "error: the release '$new' is not named vNNN (digits only after the v)" >&2
    exit 2
  fi
done

# Prints the pool of $1, one bot name per line, baselines first.
pool_of() { # pool_of <new>
  local new="$1" bot release number limit
  for bot in "${baselines[@]}"; do
    echo "$bot"
    [[ $bot == "$new" ]] && return 0
  done
  limit="$(release_number "$new")"
  for release in ${release_names[@]+"${release_names[@]}"}; do
    number="$(release_number "$release")"
    if [[ -n $number && -n $limit ]] && ((number <= limit)); then
      echo "$release"
    fi
  done
}
# Prints the highest release below $1, or nothing.
previous_of() { # previous_of <new>
  local new="$1" release number limit previous=""
  limit="$(release_number "$new")"
  [[ -n $limit ]] || return 0
  for release in ${release_names[@]+"${release_names[@]}"}; do
    number="$(release_number "$release")"
    if [[ -n $number ]] && ((number < limit)); then
      previous="$release"
    fi
  done
  echo "$previous"
}

for new in "${news[@]}"; do
  while IFS= read -r bot; do
    add_needed "$bot"
  done < <(pool_of "$new")
done
files=()
for bot in "${needed[@]}"; do
  if is_baseline "$bot"; then
    bundle_bot "$game" "$bot" "$dir"
    files+=("$dir/$game-$bot.rs")
  else
    files+=("games/$game/releases/$game-$bot.rs")
  fi
done
CG_BIN_DIR="$dir/bin" scripts/cg-check.sh "${files[@]}" >&2

results="games/$game/league.jsonl"
if [[ ! -f $results ]]; then
  results="$dir/$game-results.jsonl"
  : >"$results"
fi

read -r -a options <<<"$(arena_options)"
arena="target/release/$game-arena"
commit="${LEAGUE_COMMIT:-$(git rev-parse HEAD)}"
records=()
if [[ -n ${RECORDS_DIR:-} ]]; then
  records=(--records "$RECORDS_DIR/$game-league" --records-sample "${RECORDS_SAMPLE:-10}")
fi
canary="${LEAGUE_CANARY:-}"
speed=()
if [[ -n $MACHINE_SPEED ]]; then
  speed=(--speed "$MACHINE_SPEED")
fi

echo "Settings: $(describe_settings), $LEAGUE_PAIRS pairs per matchup."
echo "Results file: $results"
index=0
for new in "${news[@]}"; do
  if [[ -n $stop_after ]] && ((SECONDS >= stop_after * 60)); then
    echo "Stopped before $new: time budget of $stop_after minutes reached. Next: ${news[*]:index}"
    exit 0
  fi
  index=$((index + 1))

  pool=()
  while IFS= read -r bot; do
    pool+=("$bot")
  done < <(pool_of "$new")
  previous="$(previous_of "$new")"
  args=()
  for bot in "${pool[@]}"; do
    args+=(--bot "$bot=$dir/bin/$game-$bot")
  done
  extra=()
  if [[ -n $previous ]]; then
    extra+=(--previous "$previous")
  fi
  # The fixed opponents that exist in this pool and are not NEW itself: the
  # first releases of a replay have no v001 yet, and greedy does not play itself.
  fixed=""
  IFS=',' read -r -a wanted <<<"$LEAGUE_FIXED"
  for bot in ${wanted[@]+"${wanted[@]}"}; do
    [[ $bot == "$new" ]] && continue
    for member in "${pool[@]}"; do
      if [[ $member == "$bot" ]]; then
        fixed="${fixed:+$fixed,}$bot"
      fi
    done
  done
  if [[ -n $canary ]]; then
    # Both bots of the canary must be in the pool, else it is not replayed.
    canary_a="${canary%%,*}"
    canary_b="${canary#*,}"
    in_pool=0
    for bot in "${pool[@]}"; do
      if [[ $bot == "$canary_a" || $bot == "$canary_b" ]]; then
        in_pool=$((in_pool + 1))
      fi
    done
    # A value that is not A,B (both halves equal) goes straight to the arena.
    # Otherwise the canary is replayed only once the results file stores its
    # pairing, which a replay of the history has not done before both bots play.
    if [[ $canary_a == "$canary_b" ]]; then
      extra+=(--canary "$canary")
    elif [[ $in_pool -eq 2 ]]; then
      if grep -Fq "\"a\":\"$canary_a\",\"b\":\"$canary_b\"" "$results" 2>/dev/null ||
        grep -Fq "\"a\":\"$canary_b\",\"b\":\"$canary_a\"" "$results" 2>/dev/null; then
        extra+=(--canary "$canary")
      else
        echo "::warning title=League canary::$canary is not in $results yet: not replayed"
      fi
    fi
  fi
  new_results="$dir/$game-$new.jsonl"
  log="$dir/$game-$new.log"

  echo
  echo "League of $new (previous release: ${previous:-none}; pool: ${pool[*]})"
  status=0
  "$arena" league --results "$results" --new "$new" ${extra[@]+"${extra[@]}"} \
    --fixed-opponents "$fixed" --nearest "$LEAGUE_NEAREST" \
    --pairs "$LEAGUE_PAIRS" --anchor "$LEAGUE_ANCHOR" \
    --commit "$commit" --new-results "$new_results" --expect-no-faults-from "$new" \
    ${speed[@]+"${speed[@]}"} \
    "${options[@]}" "${args[@]}" ${records[@]+"${records[@]}"} | tee "$log" || status=$?

  while IFS= read -r line; do
    echo "::warning title=League canary::${line#warning: canary }"
  done < <(grep '^warning: canary' "$log" || true)
  if ((status != 0)); then
    echo "error: the league of $new failed (arena status $status)" >&2
    exit "$status"
  fi
  echo "---- new results of $new ----"
  cat "$new_results"
  echo "---- end ----"
  if [[ $append == true ]]; then
    cat "$new_results" >>"$results"
  fi
done
