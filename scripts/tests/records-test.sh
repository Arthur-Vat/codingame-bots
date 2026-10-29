#!/usr/bin/env bash
# Tests the comparison of game records of scripts/lib/records.sh, used by
# scripts/check-determinism.sh and scripts/check-minified.sh: times are
# ignored; a different answer, winner or end, a missing or extra game and a
# record jq rejects are problems. Needs jq.
# Usage: scripts/tests/records-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
# shellcheck source=scripts/lib/records.sh
source scripts/lib/records.sh
dir="$(mktemp -d)"
trap 'rm -rf "$dir"' EXIT

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# record <file> <winner> <end kind> <line of turn 1> <line of turn 2> <ms>
record() {
  printf '{"format":1,"winner":%s,"end":{"kind":"%s"},"turns":[[{"lines":["%s"],"ms":%s}],[{"lines":["%s"],"ms":%s}]]}\n' \
    "$2" "$3" "$4" "$6" "$5" "$6" >"$1"
}

# expect <name> <expected status> <text the output must contain>: compares the
# folders run1 and run2.
expect() {
  local name="$1" want="$2" text="$3" output status=0
  checks=$((checks + 1))
  output="$(compare_runs "$dir/run1" "$dir/run2" "run 1" "run 2")" || status=$?
  if ((status != want)); then
    fail "$name: status $status, wanted $want: $output"
  elif [[ $output != *"$text"* ]]; then
    fail "$name: the output lacks '$text': $output"
  fi
}

reset() {
  rm -rf "$dir/run1" "$dir/run2"
  mkdir -p "$dir/run1" "$dir/run2"
  record "$dir/run1/g1.json" 0 win "1 1" "2 2" 5
  record "$dir/run2/g1.json" 0 win "1 1" "2 2" 5
}

reset
expect "identical records" 0 ""

record "$dir/run2/g1.json" 0 win "1 1" "2 2" 987
expect "other times" 0 ""

record "$dir/run2/g1.json" 0 win "1 1" "3 3" 5
expect "another answer" 1 "run 2 differs from run 1 in g1.json at turn 2"

record "$dir/run2/g1.json" 1 win "1 1" "2 2" 5
expect "another winner" 1 "at the winner"

record "$dir/run2/g1.json" 0 timeout "1 1" "2 2" 5
expect "another end" 1 "at the end"

reset
rm "$dir/run2/g1.json"
expect "a game missing from run 2" 1 "run 2 has no g1.json"

reset
record "$dir/run2/g2.json" 0 win "1 1" "2 2" 5
expect "a game only in run 2" 1 "run 2 has other games than run 1"

reset
echo '{"format":2}' >"$dir/run2/g1.json"
expect "an unknown format" 1 "cannot read"

reset
echo 'not json' >"$dir/run1/g1.json"
expect "a record jq rejects" 1 "cannot read"

reset
rm "$dir/run1/g1.json" "$dir/run2/g1.json"
expect "no games" 1 "has no"

if ((failures > 0)); then
  echo "records test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   records test: $checks checks"
