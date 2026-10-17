#!/usr/bin/env bash
# The CI steps that only A Code of Ice and Fire needs (ADR 0029, decision 4:
# every game provides a games/<game>/ci.sh). The bot checks job of ci.yml runs
# it after bundling the bots and building them standalone:
#
#   scripts/bundle-bots.sh ice-and-fire
#   CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/ice-and-fire-*.rs
#   games/ice-and-fire/ci.sh
#
# It expects the compiled bundle in target/cg/bin/ice-and-fire-wait. The game
# has no arena yet, so for now it checks that and stops; stage 6 adds the
# arena's games here.
#
# Usage: games/ice-and-fire/ci.sh   (from any directory; no arguments)
set -euo pipefail

cd "$(dirname "$0")/../.."

if [[ ! -x target/cg/bin/ice-and-fire-wait ]]; then
  echo "games/ice-and-fire/ci.sh: target/cg/bin/ice-and-fire-wait is missing; first run scripts/bundle-bots.sh ice-and-fire and" >&2
  echo "  CG_BIN_DIR=target/cg/bin scripts/cg-check.sh target/cg/ice-and-fire-*.rs" >&2
  exit 2
fi

echo "== No arena yet: stage 6 adds the arena's games"
