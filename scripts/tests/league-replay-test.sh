#!/usr/bin/env bash
# Tests the calibration of scripts/lib/evaluation.sh (ADR 0048) and the
# merge of scripts/league-replay.sh, in a repository of its own with stand-in
# tools: a calibration bot that prints known speeds gives a known scale and
# tolerance; shards merged in any order give the stored order back; a
# missing, repeated or unknown pairing is an error.
# Usage: scripts/tests/league-replay-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}
expect_equal() { # expect_equal <what> <expected> <got>
  checks=$((checks + 1))
  if [[ $2 != "$3" ]]; then
    fail "$1: expected '$2', got '$3'"
  fi
}

# A repository of its own: the scripts, the settings library, a game "toy".
repo="$tmp_dir/repo"
mkdir -p "$repo/scripts/lib" "$repo/games/toy/bots/probe" "$repo/target/release"
cp "$repo_root/scripts/league-replay.sh" "$repo/scripts/"
cp "$repo_root/scripts/lib/evaluation.sh" "$repo/scripts/lib/"
printf '[package]\nname = "toy-bot-probe"\n' >"$repo/games/toy/bots/probe/Cargo.toml"

# The stand-in bundler writes a "bundle" naming the package; the stand-in
# cg-check copies the stand-in bot in its place. The bot prints speeds 10 to
# 19 when CG_WORKLOAD_MS is set, plus the copy's number from PROBE_OFFSET.
cat >"$repo/target/release/cg-bundler" <<'STUB'
#!/usr/bin/env bash
while (($#)); do
  if [[ $1 == --output ]]; then echo "// $0" >"$2"; fi
  shift
done
STUB
cat >"$repo/scripts/cg-check.sh" <<'STUB'
#!/usr/bin/env bash
[[ $1 == --any-size ]] && shift
for file in "$@"; do
  cp "$PROBE_BOT" "$CG_BIN_DIR/$(basename "$file" .rs)"
done
STUB
cat >"$tmp_dir/probe" <<'STUB'
#!/usr/bin/env bash
[[ -n ${CG_WORKLOAD_MS:-} ]] || exit 3
for speed in 10 11 12 13 14 15 16 17 18 19; do echo "$speed"; done
STUB
chmod +x "$repo/target/release/cg-bundler" "$repo/scripts/cg-check.sh" "$tmp_dir/probe"

# Runs calibrate_settings in the repository with the settings given and
# prints scale, tolerance, speed and jobs.
calibrate() { # calibrate <TIME_SCALE> <TIME_TOLERANCE_MS> <CALIBRATION_JOBS>
  (
    cd "$repo"
    # shellcheck source=/dev/null
    source scripts/lib/evaluation.sh
    export PROBE_BOT="$tmp_dir/probe" CALIBRATION_BOT=probe CG_SPEED=11 CG_EXTRA_MS=13
    export CALIBRATION_STRETCH_MS=1
    TIME_SCALE="$1" TIME_TOLERANCE_MS="$2" CALIBRATION_JOBS="$3"
    calibrate_settings toy target/calibration
    echo "$TIME_SCALE $TIME_TOLERANCE_MS $MACHINE_SPEED $CALIBRATION_JOBS"
  )
}

# 2 copies print 10 to 19 each: 20 speeds, median (14 + 15) / 2 = 14.5.
# 11 / 14.5 = 0.759, and 13 x 0.759 = 9.87, rounded to 10.
expect_equal "calibrated settings" "0.759 10 14.50 2" "$(calibrate auto auto 2 2>/dev/null)"
# One copy gives the same 20 / 2 speeds' median.
expect_equal "one copy" "0.759 10 14.50 1" "$(calibrate auto auto 1 2>/dev/null)"
# Fixed settings are left alone, with no speed.
expect_equal "fixed settings" "1 0  " "$(calibrate 1 0 4 2>/dev/null)"
checks=$((checks + 1))
if calibrate auto 0 2 >/dev/null 2>&1; then
  fail "auto with a fixed tolerance should be an error"
fi
checks=$((checks + 1))
if calibrate auto auto 0 >/dev/null 2>&1; then
  fail "0 calibration jobs should be an error"
fi

# The merge.
line() { # line <a> <b> <wins>
  printf '{"a":"%s","b":"%s","pairs":2,"wins":%s,"rest":"..."}\n' "$1" "$2" "$3"
}
{
  line bronze random 4
  line v001 bronze 3
  line v002 v001 2
  line v002 bronze 1
} >"$repo/games/toy/league.jsonl"
{
  line v002 v001 0
  line bronze random 0
} >"$tmp_dir/shard-1.jsonl"
{
  line v002 bronze 0
  line v001 bronze 0
} >"$tmp_dir/shard-0.jsonl"
merge() { # merge <file>...
  (cd "$repo" && scripts/league-replay.sh --merge toy "$@")
}
expected="$(sed 's/"wins":[0-9]*/"wins":0/' "$repo/games/toy/league.jsonl")"
expect_equal "merged in the stored order" "$expected" \
  "$(merge "$tmp_dir/shard-1.jsonl" "$tmp_dir/shard-0.jsonl" 2>&1)"

checks=$((checks + 1))
if merge "$tmp_dir/shard-1.jsonl" >/dev/null 2>&1; then
  fail "a missing pairing should be an error"
fi
checks=$((checks + 1))
if merge "$tmp_dir/shard-1.jsonl" "$tmp_dir/shard-0.jsonl" "$tmp_dir/shard-0.jsonl" >/dev/null 2>&1; then
  fail "a repeated pairing should be an error"
fi
line v003 v002 0 >"$tmp_dir/extra.jsonl"
checks=$((checks + 1))
if merge "$tmp_dir/shard-1.jsonl" "$tmp_dir/shard-0.jsonl" "$tmp_dir/extra.jsonl" >/dev/null 2>&1; then
  fail "a line of no stored pairing should be an error"
fi
echo '{"b":"x","a":"y"}' >"$tmp_dir/odd.jsonl"
checks=$((checks + 1))
if merge "$tmp_dir/shard-1.jsonl" "$tmp_dir/shard-0.jsonl" "$tmp_dir/odd.jsonl" >/dev/null 2>&1; then
  fail "a line that does not start like a pairing should be an error"
fi

if ((failures > 0)); then
  echo "$failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   league replay: $checks checks"
