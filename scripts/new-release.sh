#!/usr/bin/env bash
# Freezes a bot as the game's next release (ADR 0045): two files, bundled from
# the bot's sources,
#
# - games/<game>/releases/<game>-vNNN.rs, the minified bundle, the file pasted
#   into CodinGame;
# - games/<game>/readable/<game>-vNNN.rs, the same bundle with its comments
#   and layout (the bundler's --keep-comments), which may be over the size
#   limit.
#
# It checks both with scripts/cg-check.sh and plays them against each other
# (scripts/check-minified.sh): the same answers in every game.
#
# Usage: scripts/new-release.sh GAME BOT
#
# BOT is a folder of games/<game>/bots. Release files and their readable
# copies never change once merged (scripts/check-releases.sh); see
# docs/WORKFLOW.md for the flow.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -ne 2 ]]; then
  echo "usage: $0 GAME BOT" >&2
  exit 2
fi
game="$1"
bot="$2"
if [[ ! -f games/$game/bots/$bot/Cargo.toml ]]; then
  echo "error: no bot games/$game/bots/$bot" >&2
  exit 2
fi

last="$(releases "$game" | tail -n 1)"
number=1
if [[ -n $last ]]; then
  last="$(basename "$last" .rs)"
  number=$((10#${last##*-v} + 1))
fi
file="$(printf '%s-v%03d.rs' "$game" "$number")"
release="games/$game/releases/$file"
readable="games/$game/readable/$file"

cargo build --quiet --release --locked -p cg-bundler
mkdir -p "games/$game/releases" "games/$game/readable"
bundle_bot "$game" "$bot" "games/$game/releases"
mv "games/$game/releases/$game-$bot.rs" "$release"
bundle_bot "$game" "$bot" "games/$game/readable" --keep-comments
mv "games/$game/readable/$game-$bot.rs" "$readable"
scripts/cg-check.sh "$release"
scripts/cg-check.sh --any-size "$readable"
scripts/check-minified.sh "$game" 500 "$release"
echo "wrote $release and $readable; its journal entry needs the line 'release: $(basename "$release" .rs)'"
