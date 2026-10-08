# shellcheck shell=bash
# Shared by scripts/sprt.sh, scripts/league.sh and scripts/new-release.sh. Source it from the
# repository root.

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
# as $3/$1-$2.rs.
bundle_bot() {
  local manifest="games/$1/bots/$2/Cargo.toml" package
  package="$(sed -n 's/^name = "\(.*\)"$/\1/p' "$manifest" | head -n 1)"
  target/release/cg-bundler "$package" --output "$3/$1-$2.rs" >&2
}

# Release files of game $1, oldest first.
releases() {
  if [[ -d games/$1/releases ]]; then
    find "games/$1/releases" -name "$1-v*.rs" | sort -V
  fi
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
  echo "time scale $TIME_SCALE, tolerance $TIME_TOLERANCE_MS ms, opening plies $OPENING_PLIES, seed $SEED, timeouts allowed in up to $percent% of games"
}
