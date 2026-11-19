#!/usr/bin/env bash
# Tests scripts/study.sh without a build: the validation of the dispatch
# inputs by `plan` (study, ref, shards, each study's arguments) and its
# outputs, in a stand-in repository with a branch and a tag; and `collect`
# (rows under one header, the README, a missing shard). Needs git and jq.
# Usage: scripts/tests/study-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_OUTPUT STUDY REF ARGS SHARDS SHA RUN_ID

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}
expect() { # expect <description> <actual> <expected>
  checks=$((checks + 1))
  if [[ $2 != "$3" ]]; then fail "$1: expected '$3', got '$2'"; fi
}
contains() { # contains <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 != *"$3"* ]]; then fail "$1: '$3' not found in: $2"; fi
}

repo="$tmp_dir/repo"
mkdir -p "$repo/scripts" "$repo/games/ice-and-fire/opening/examples" "$repo/games/ice-and-fire/openings"
cp "$repo_root/scripts/study.sh" "$repo/scripts/"
touch "$repo/games/ice-and-fire/openings/suite-v1.tsv" "$repo/games/ice-and-fire/openings/suite-v2.tsv"
git_in() { git -C "$repo" -c user.name=t -c user.email=t@example.com "$@"; }
git_in init -q
git_in add -A
git_in commit -q -m "no oracle"
bare="$(git_in rev-parse HEAD)"
touch "$repo/games/ice-and-fire/opening/examples/oracle.rs"
git_in add -A
git_in commit -q -m oracle
first="$(git_in rev-parse HEAD)"
git_in checkout -q -b claude/work
git_in commit -q --allow-empty -m work
work="$(git_in rev-parse HEAD)"
git_in tag v1 "$first"
git_in checkout -q -b main-line "$first"

run() { # run <args> [KEY=value]...: plan with defaults; sets output, status
  local args="$1"
  shift
  status=0
  output="$(
    cd "$repo"
    env STUDY=oracle REF=claude/work ARGS="$args" SHARDS=4 "$@" scripts/study.sh plan 2>&1
  )" || status=$?
}
bad() { # bad <description> <needle> <args> [KEY=value]...
  local description="$1" needle="$2"
  shift 2
  run "$@"
  expect "$description: status" "$status" 1
  contains "$description" "$output" "::error title=Study::"
  contains "$description" "$output" "$needle"
}

# ---- good inputs ----
run "--player zasmu --per-seat 50 --turns 6,7,8 --cap 1000000"
expect "good: status" "$status" 0
expect "good: matrix" "$(sed -n 's/^matrix=//p' <<<"$output")" "[0,1,2,3]"
contains "good" "$output" "sha=$work"
contains "good" "$output" "study=oracle"
contains "good" "$output" "shards=4"
contains "good: the suite by default" "$output" "args=--suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 50 --turns 6,7,8 --cap 1000000"
run "--suite games/ice-and-fire/openings/suite-v2.tsv" REF="${first:0:7}" SHARDS=1
expect "short sha, another suite: status" "$status" 0
contains "short sha" "$output" "sha=$first"
contains "another suite" "$output" "args=--suite games/ice-and-fire/openings/suite-v2.tsv"
run "" REF=v1 SHARDS=20
expect "tag, no arguments, 20 shards: status" "$status" 0
contains "20 shards" "$output" "matrix=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]"
out_file="$tmp_dir/output"
(
  cd "$repo"
  GITHUB_OUTPUT="$out_file" STUDY=oracle REF=claude/work ARGS="--turns 6" SHARDS=2 scripts/study.sh plan >/dev/null
)
contains "GITHUB_OUTPUT" "$(cat "$out_file")" "matrix=[0,1]"

# ---- bad inputs ----
bad "unknown study" "study must be one of" "" STUDY=other
bad "empty study" "study must be one of" "" STUDY=
bad "ref with ..'" "is not a commit" "" REF="claude/../x"
bad "unknown ref" "of this repository" "" REF=nowhere
bad "study missing at the ref" "oracle.rs does not exist" "" REF="$bare"
bad "no shards" "shards must be" "" SHARDS=
bad "zero shards" "shards must be" "" SHARDS=0
bad "21 shards" "shards must be" "" SHARDS=21
bad "shards with a sign" "shards must be" "" SHARDS=+2
bad "unknown flag" "unknown argument '--jobs'" "--jobs 4"
bad "flag without a value" "--flag value pairs" "--player"
bad "flag twice" "is given twice" "--player a --player b"
bad "player with a space" "--flag value pairs" "--player a b"
bad "player with a semicolon" "args may hold only" "--player a;b"
bad "player with a quote" "args may hold only" "--player 'a'"
bad "per-seat 0" "--per-seat must be" "--per-seat 0"
bad "per-seat 501" "--per-seat must be" "--per-seat 501"
bad "turns 13" "--turns must be" "--turns 6,13"
bad "turns 0" "--turns must be" "--turns 0"
bad "turns with a space" "--turns must be" "--turns 6,"
bad "cap too large" "--cap must be" "--cap 10000001"
bad "suite outside the folder" "--suite must be" "--suite games/ice-and-fire/x.tsv"
bad "suite with .." "--suite must be" "--suite games/ice-and-fire/openings/..x.tsv"
bad "missing suite" "does not exist" "--suite games/ice-and-fire/openings/suite-v9.tsv"
bad "a line break in the arguments" "args may hold only" $'--player a\n::error::x'

# ---- run checks its own inputs ----
status=0
output="$(cd "$repo" && scripts/study.sh run oracle "$first" 2 2 "" 2>&1)" || status=$?
expect "run, shard not below shards: status" "$status" 1
contains "run, shard not below shards" "$output" "bad shard or shards"
status=0
output="$(cd "$repo" && scripts/study.sh run oracle "${first:0:7}" 0 1 "" 2>&1)" || status=$?
expect "run, short sha: status" "$status" 1
contains "run, short sha" "$output" "bad sha"

# ---- collect ----
results="$tmp_dir/results"
mkdir -p "$results"
printf 'seed,seat\n1,0\n2,1\n' >"$results/oracle-0.csv"
printf 'seed,seat\n3,0\n' >"$results/oracle-2.csv"
echo "# shard 0" >"$results/oracle-0.md"
status=0
output="$(cd "$repo" && SHARDS=3 REF=claude/work SHA="$work" ARGS="--turns 6" RUN_ID=42 scripts/study.sh collect oracle "$results" "$tmp_dir/out" 2>&1)" || status=$?
expect "collect, a shard missing: status" "$status" 1
contains "collect" "$output" "::notice title=Study oracle::3 rows from 2 of 3 shards"
contains "collect" "$output" "::warning title=Study oracle::shard 1 has no results"
expect "collect: one header" "$(cat "$tmp_dir/out/oracle.csv")" "$(printf 'seed,seat\n1,0\n2,1\n3,0')"
expect "collect: the shard's report" "$(cat "$tmp_dir/out/oracle-0.md")" "# shard 0"
readme="$(cat "$tmp_dir/out/README.md")"
contains "collect: README run" "$readme" "# Study oracle, run 42"
contains "collect: README code" "$readme" "- Code: claude/work at $work"
contains "collect: README shards" "$readme" "found: 0 2; missing: 1"
printf 'seed,seat\n4,1\n' >"$results/oracle-1.csv"
status=0
output="$(cd "$repo" && SHARDS=3 scripts/study.sh collect oracle "$results" "$tmp_dir/out2" 2>&1)" || status=$?
expect "collect, every shard: status" "$status" 0
expect "collect, every shard: rows" "$(($(wc -l <"$tmp_dir/out2/oracle.csv") - 1))" 4

echo "study-test: $checks checks, $failures failures"
((failures == 0))
