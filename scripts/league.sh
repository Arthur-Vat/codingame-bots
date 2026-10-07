#!/usr/bin/env bash
# Rates every release of a game, plus the game's baseline bots, with the
# arena's league and the game's evaluation settings
# (games/<game>/evaluation.env, ADR 0012).
#
# Usage: scripts/league.sh GAME
#
# Prints each matchup and the Elo table; games go to target/league/.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -ne 1 ]]; then
  echo "usage: $0 GAME" >&2
  exit 2
fi
game="$1"
load_settings "$game"
build_tools "$game"

dir=target/league
mkdir -p "$dir/bin"
files=()
IFS=',' read -r -a bots <<<"$LEAGUE_BOTS"
for bot in "${bots[@]}"; do
  bundle_bot "$game" "$bot" "$dir"
  files+=("$dir/$game-$bot.rs")
done
while IFS= read -r release; do
  files+=("$release")
done < <(releases "$game")
CG_BIN_DIR="$dir/bin" scripts/cg-check.sh "${files[@]}" >&2

read -r -a options <<<"$(arena_options)"
args=()
for file in "${files[@]}"; do
  name="$(basename "$file" .rs)"
  # Names drop the game prefix: random, greedy, v001, ...
  args+=(--bot "${name#"$game"-}=$dir/bin/$name")
done
echo "Settings: $(describe_settings), $LEAGUE_PAIRS pairs per matchup."
"target/release/$game-arena" league "${options[@]}" "${args[@]}" \
  --pairs "$LEAGUE_PAIRS" --anchor "$LEAGUE_ANCHOR" --out "$dir/$game.jsonl"
