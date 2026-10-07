#!/usr/bin/env bash
# Freezes a bot as the game's next release: bundles it into
# games/<game>/releases/<game>-vNNN.rs, the file pasted into CodinGame.
#
# Usage: scripts/new-release.sh GAME BOT
#
# BOT is a folder of games/<game>/bots. Release files never change once
# merged (scripts/check-releases.sh); see docs/WORKFLOW.md for the flow.
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
release="games/$game/releases/$(printf '%s-v%03d.rs' "$game" "$number")"

cargo build --quiet --release --locked -p cg-bundler
mkdir -p "games/$game/releases"
bundle_bot "$game" "$bot" "games/$game/releases"
mv "games/$game/releases/$game-$bot.rs" "$release"
scripts/cg-check.sh "$release"
echo "wrote $release; its journal entry needs the line 'release: $(basename "$release" .rs)'"
