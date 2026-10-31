# shellcheck shell=bash
# Shared by scripts/sprt.sh, scripts/league.sh, scripts/league-replay.sh,
# scripts/calibrate.sh, scripts/league-check.sh and scripts/new-release.sh.
# Source it from the repository root.

# Exports the settings of games/$1/evaluation.env, keeping any variable
# already set in the environment.
load_settings() {
  local file="games/$1/evaluation.env" key value
  if [[ ! -f $file ]]; then
    echo "error: no $file" >&2
    exit 2
  fi
  while IFS='=' read -r key value; do
    [[ -z $key || $key == \#* ]] && continue
    if [[ -z ${!key:-} ]]; then
      export "$key=$value"
    fi
  done <"$file"
}

# Builds the arena of game $1 and the bundler.
build_tools() {
  cargo build --quiet --release --locked -p "$1-arena" -p cg-bundler
}

# Bundles bot $2 of game $1 (a folder of games/$1/bots) into directory $3,
# as $3/$1-$2.rs, minified; further arguments go to the bundler (for
# example --keep-comments for the readable bundle).
bundle_bot() {
  local manifest="games/$1/bots/$2/Cargo.toml" package
  package="$(sed -n 's/^name = "\(.*\)"$/\1/p' "$manifest" | head -n 1)"
  target/release/cg-bundler "$package" --output "$3/$1-$2.rs" "${@:4}" >&2
}

# Release files of game $1, oldest first.
releases() {
  if [[ -d games/$1/releases ]]; then
    find "games/$1/releases" -name "$1-v*.rs" | sort -V
  fi
}

# When TIME_SCALE and TIME_TOLERANCE_MS are `auto` (ADR 0048), measures
# this machine with the game's CALIBRATION_BOT and sets them to give
# CodinGame's speed and real limit: TIME_SCALE = CG_SPEED / the machine's
# speed, TIME_TOLERANCE_MS = CG_EXTRA_MS x TIME_SCALE, rounded. Sets
# MACHINE_SPEED (units a millisecond of the bot's workload), and
# CALIBRATION_JOBS, to the empty string for fixed settings. $1 is the game,
# $2 a working directory. The bot runs in CALIBRATION_JOBS copies at once
# (the number of CPUs, the arena's default number of games at a time unless
# already set), each for 10 stretches of CALIBRATION_STRETCH_MS (1,000 by
# default): about 10 s.
calibrate_settings() {
  MACHINE_SPEED=""
  if [[ $TIME_SCALE != auto && $TIME_TOLERANCE_MS != auto ]]; then
    CALIBRATION_JOBS=""
    return 0
  fi
  if [[ $TIME_SCALE != auto || $TIME_TOLERANCE_MS != auto ]]; then
    echo "error: TIME_SCALE and TIME_TOLERANCE_MS must both be auto or both be numbers (now $TIME_SCALE and $TIME_TOLERANCE_MS)" >&2
    exit 2
  fi
  local key
  for key in CALIBRATION_BOT CG_SPEED CG_EXTRA_MS; do
    if [[ -z ${!key:-} ]]; then
      echo "error: TIME_SCALE=auto needs $key in games/$1/evaluation.env" >&2
      exit 2
    fi
  done
  local dir="$2" stretch="${CALIBRATION_STRETCH_MS:-1000}" bin pids=() i
  CALIBRATION_JOBS="${CALIBRATION_JOBS:-$(nproc)}"
  if [[ ! $CALIBRATION_JOBS =~ ^[1-9][0-9]*$ || ! $stretch =~ ^[1-9][0-9]*$ ]]; then
    echo "error: CALIBRATION_JOBS and CALIBRATION_STRETCH_MS must be positive numbers" >&2
    exit 2
  fi
  mkdir -p "$dir/bin"
  bundle_bot "$1" "$CALIBRATION_BOT" "$dir"
  CG_BIN_DIR="$dir/bin" scripts/cg-check.sh --any-size "$dir/$1-$CALIBRATION_BOT.rs" >&2
  bin="$dir/bin/$1-$CALIBRATION_BOT"
  # Files of an earlier run cut short would join the median.
  rm -f "$dir"/calibration-*.txt
  for ((i = 0; i < CALIBRATION_JOBS; i++)); do
    CG_WORKLOAD_MS="$stretch" "$bin" >"$dir/calibration-$i.txt" &
    pids+=("$!")
  done
  for i in "${pids[@]}"; do
    if ! wait "$i"; then
      echo "error: the calibration bot $bin failed" >&2
      exit 2
    fi
  done
  MACHINE_SPEED="$(cat "$dir"/calibration-*.txt | sort -g | awk '
    { speed[NR] = $1 }
    END {
      if (NR == 0) exit 1
      if (NR % 2) print speed[(NR + 1) / 2]
      else printf "%.2f\n", (speed[NR / 2] + speed[NR / 2 + 1]) / 2
    }')" || {
    echo "error: the calibration bot printed no speed" >&2
    exit 2
  }
  rm -f "$dir"/calibration-*.txt
  TIME_SCALE="$(awk -v cg="$CG_SPEED" -v here="$MACHINE_SPEED" 'BEGIN { printf "%.3f", cg / here }')"
  TIME_TOLERANCE_MS="$(awk -v extra="$CG_EXTRA_MS" -v scale="$TIME_SCALE" 'BEGIN { printf "%d", extra * scale + 0.5 }')"
  if [[ ! $TIME_SCALE =~ ^[0-9]+\.[0-9]+$ ]] || ! awk -v scale="$TIME_SCALE" 'BEGIN { exit !(scale > 0) }'; then
    echo "error: the calibration gave a time scale of '$TIME_SCALE'" >&2
    exit 2
  fi
  export TIME_SCALE TIME_TOLERANCE_MS MACHINE_SPEED CALIBRATION_JOBS
}

# Arena options shared by every evaluation.
arena_options() {
  echo --seed "$SEED" --time-scale "$TIME_SCALE" \
    --time-tolerance-ms "$TIME_TOLERANCE_MS" --opening-plies "$OPENING_PLIES" \
    --max-timeout-rate "$MAX_TIMEOUT_RATE"
}

# The arena settings in words.
describe_settings() {
  local percent
  percent="$(awk -v rate="$MAX_TIMEOUT_RATE" 'BEGIN { print rate * 100 }')"
  local calibration=""
  if [[ -n ${MACHINE_SPEED:-} ]]; then
    calibration=" (calibrated to CodinGame's speed: this machine runs the $CALIBRATION_BOT bot's workload at $MACHINE_SPEED units a millisecond, $CALIBRATION_JOBS at a time, CodinGame at $CG_SPEED; ADR 0048)"
  fi
  echo "time scale $TIME_SCALE, tolerance $TIME_TOLERANCE_MS ms$calibration, opening plies $OPENING_PLIES, seed $SEED, timeouts allowed in up to $percent% of games"
}
