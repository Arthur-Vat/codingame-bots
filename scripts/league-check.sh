#!/usr/bin/env bash
# The merge gate of the incremental league (ADR 0038): checks that a game's
# results file holds the pairings of the releases a pull request adds, and that
# its LEADERBOARD.md is the one generated from the results.
#
# Usage: scripts/league-check.sh GAME [RELEASE_FILE...]
#
# Each RELEASE_FILE is a release the pull request adds,
# games/GAME/releases/GAME-vNNN.rs. Builds nothing: it expects the arena,
# target/release/GAME-arena. When games/GAME/league.jsonl does not exist, the
# game has no league yet: prints a notice and exits with 0. Otherwise:
# 1. the results file must hold a pairing with each release;
# 2. `GAME-arena leaderboard --check` must find games/GAME/LEADERBOARD.md equal
#    to the leaderboard generated from the results file (the ratings come from
#    Linux's math library, so generate it on Linux: docs/WORKFLOW.md).
# A failure prints an error annotation and exits with 1; a usage error exits
# with 2.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -lt 1 ]]; then
  echo "usage: $0 GAME [RELEASE_FILE...]" >&2
  exit 2
fi
game="$1"
shift
if [[ ! $game =~ ^[A-Za-z0-9_-]+$ ]]; then
  echo "error: '$game' is not a game name" >&2
  exit 2
fi
load_settings "$game"

results="games/$game/league.jsonl"
if [[ ! -f $results ]]; then
  echo "::notice title=League results::$game has no league results yet: nothing to check"
  exit 0
fi

for file in "$@"; do
  name="$(basename "$file" .rs)"
  bot="${name#"$game-"}"
  if [[ $file != "games/$game/releases/$game-"*.rs || ! $bot =~ ^v[0-9]+$ ]]; then
    echo "error: $file is not a release of $game (games/$game/releases/$game-vNNN.rs)" >&2
    exit 2
  fi
  # The arena's strict reader checks the lines; this only looks for the bot.
  if ! grep -Eq "\"(a|b)\" *: *\"$bot\"" "$results"; then
    echo "::error title=League results::$bot has no pairing in $results: copy the lines of the league comment"
    exit 1
  fi
  echo "ok   $bot has a pairing in $results"
done

arena="target/release/$game-arena"
if [[ ! -x $arena ]]; then
  echo "error: $arena is missing; build it first: cargo build --release -p $game-arena" >&2
  exit 2
fi
status=0
"$arena" leaderboard --results "$results" --anchor "$LEAGUE_ANCHOR" \
  --journal "games/$game/journal/README.md" --check "games/$game/LEADERBOARD.md" || status=$?
if ((status != 0)); then
  echo "::error title=League results::games/$game/LEADERBOARD.md does not match $results (arena status $status); regenerate it on Linux with: cargo run --release -p $game-arena -- leaderboard --results $results --anchor $LEAGUE_ANCHOR --journal games/$game/journal/README.md --out games/$game/LEADERBOARD.md"
  exit 1
fi
echo "ok   games/$game/LEADERBOARD.md matches $results"
