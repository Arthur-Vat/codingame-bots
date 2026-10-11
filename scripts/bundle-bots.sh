#!/usr/bin/env bash
# Bundles every bot into one paste-ready file: target/cg/<game>-<bot>.rs.
# With GAME, bundles only the bots of games/GAME/bots/; an unknown GAME exits 2.
#
# Usage: scripts/bundle-bots.sh [GAME]
set -euo pipefail

cd "$(dirname "$0")/.."

if (($# > 1)); then
  echo "usage: $0 [GAME]" >&2
  exit 2
fi
game_dirs=(games/*)
if (($# == 1)); then
  if [[ ! $1 =~ ^[A-Za-z0-9_-]+$ || ! -d games/$1/bots ]]; then
    echo "bundle-bots: unknown game '$1' (no games/$1/bots/)" >&2
    exit 2
  fi
  game_dirs=("games/$1")
fi

cargo build --quiet --release -p cg-bundler
mkdir -p target/cg

shopt -s nullglob
for game_dir in "${game_dirs[@]}"; do
  for manifest in "$game_dir"/bots/*/Cargo.toml; do
    dir="$(dirname "$manifest")"
    bot="$(basename "$dir")"
    game="$(basename "$game_dir")"
    package="$(sed -n 's/^name = "\(.*\)"$/\1/p' "$manifest" | head -n 1)"
    target/release/cg-bundler "$package" --output "target/cg/$game-$bot.rs"
  done
done
