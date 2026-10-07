#!/usr/bin/env bash
# Bundles every bot into one paste-ready file: target/cg/<game>-<bot>.rs.
#
# Usage: scripts/bundle-bots.sh
set -euo pipefail

cd "$(dirname "$0")/.."
cargo build --quiet --release -p cg-bundler
mkdir -p target/cg

for manifest in games/*/bots/*/Cargo.toml; do
  dir="$(dirname "$manifest")"
  bot="$(basename "$dir")"
  game="$(basename "$(dirname "$(dirname "$dir")")")"
  package="$(sed -n 's/^name = "\(.*\)"$/\1/p' "$manifest" | head -n 1)"
  target/release/cg-bundler "$package" --output "target/cg/$game-$bot.rs"
done
