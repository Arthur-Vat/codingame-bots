#!/usr/bin/env bash
# Measures this machine's speed and prints the time scale and tolerance at
# which a game's evaluations play at CodinGame's speed and real limit
# (ADR 0048), for a local screening with the arena.
#
# Usage: scripts/calibrate.sh GAME [JOBS]
#
# The game's evaluation.env must set TIME_SCALE=auto and
# TIME_TOLERANCE_MS=auto, its CALIBRATION_BOT, CG_SPEED and CG_EXTRA_MS. The
# calibration bot runs its workload in JOBS copies at once, as many as the
# games the screening will play at a time (the number of CPUs by default, the
# arena's default --jobs): a machine whose CPUs share physical cores is
# slower with more games. It takes about 10 seconds. Prints, on stdout:
#
#   TIME_SCALE=0.821
#   TIME_TOLERANCE_MS=11
#   MACHINE_SPEED=13.40
#
# and a sentence on stderr; pass the first two to the arena as
# --time-scale and --time-tolerance-ms. The SPRT and league scripts
# calibrate by themselves.
#
# Exit status: 0 on success, 2 on errors.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "usage: $0 GAME [JOBS]" >&2
  exit 2
fi
game="$1"
if [[ ! $game =~ ^[A-Za-z0-9_-]+$ ]]; then
  echo "error: '$game' is not a game name" >&2
  exit 2
fi
if [[ $# -eq 2 ]]; then
  export CALIBRATION_JOBS="$2"
fi
load_settings "$game"
if [[ $TIME_SCALE != auto ]]; then
  echo "error: games/$game/evaluation.env sets TIME_SCALE=$TIME_SCALE, not auto: this game is not calibrated" >&2
  exit 2
fi
build_tools "$game"
calibrate_settings "$game" target/calibration
echo "Settings: $(describe_settings)." >&2
echo "TIME_SCALE=$TIME_SCALE"
echo "TIME_TOLERANCE_MS=$TIME_TOLERANCE_MS"
echo "MACHINE_SPEED=$MACHINE_SPEED"
