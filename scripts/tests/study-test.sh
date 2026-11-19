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
mkdir -p "$repo/games/ice-and-fire/research/plans"
touch "$repo/games/ice-and-fire/research/plans/run-1.tsv"
mkdir -p "$repo/games/ice-and-fire/research/folder.tsv"
touch "$repo/games/ice-and-fire/research/folder.tsv/x"
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
contains "good" "$output" "ref=claude/work"
contains "good" "$output" "study=oracle"
contains "good" "$output" "shards=4"
contains "good: the suite by default" "$output" "args=--suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 50 --turns 6,7,8 --cap 1000000"
run "--limit 20000 --from games/ice-and-fire/research/plans/run-1.tsv"
expect "limit and plans to start from: status" "$status" 0
contains "limit and plans to start from" "$output" "args=--suite games/ice-and-fire/openings/suite-v1.tsv --limit 20000 --from games/ice-and-fire/research/plans/run-1.tsv"
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
bad "limit 0" "--limit must be" "--limit 0"
bad "plans outside the game" "--from must be" "--from games/uttt/x.tsv"
bad "plans with .." "--from must be" "--from games/ice-and-fire/a/../x.tsv"
bad "plans that are a folder" "does not exist" "--from games/ice-and-fire/research/folder.tsv"
bad "missing plans" "does not exist" "--from games/ice-and-fire/research/plans/run-2.tsv"
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
collect() { # collect <dir> <out> [KEY=value]...: sets output, status
  local dir="$1" dest="$2"
  shift 2
  status=0
  output="$(cd "$repo" && env SHARDS=3 "$@" scripts/study.sh collect oracle "$dir" "$dest" 2>&1)" || status=$?
}
results="$tmp_dir/results"
mkdir -p "$results"
printf 'seed,seat\n1,0\n2,1\n' >"$results/oracle-0.csv"
printf 'seed,seat\n3,0\n' >"$results/oracle-2.csv"
touch "$results/oracle-0.done" "$results/oracle-2.done"
echo "# shard 0" >"$results/oracle-0.md"
printf 'seed\tplan\n1\tWAIT\n' >"$results/oracle-0.plans.tsv"
printf 'seed\tplan\n3\tTRAIN 1 1 0\n4\tWAIT\n' >"$results/oracle-2.plans.tsv"
collect "$results" "$tmp_dir/out" REF=claude/work SHA="$work" ARGS="--turns 6" RUN_ID=42
expect "collect, a shard missing: status" "$status" 1
contains "collect" "$output" "::notice title=Study oracle::3 rows; 2 of 3 shards whole"
contains "collect" "$output" "::warning title=Study oracle::shard 1 has no results"
expect "collect: one header" "$(cat "$tmp_dir/out/oracle.csv")" "$(printf 'seed,seat\n1,0\n2,1\n3,0')"
expect "collect: the shard's report" "$(cat "$tmp_dir/out/oracle-0.md")" "# shard 0"
readme="$(cat "$tmp_dir/out/README.md")"
contains "collect: README run" "$readme" "# Study oracle, run 42"
contains "collect: README code" "$readme" "- Code: claude/work at $work"
contains "collect: README shards" "$readme" "whole: 0 2; partial (failed or out of time, rows kept): none; missing: 1"
expect "collect: the plans under one header" "$(cat "$tmp_dir/out/oracle.plans.tsv")" "$(printf 'seed\tplan\n1\tWAIT\n3\tTRAIN 1 1 0\n4\tWAIT')"
contains "collect: README plans" "$readme" "- Plans: 3 in oracle.plans.tsv"
# Shard 1's rows without its mark: it failed or ran out of time.
printf 'seed,seat\n4,1\n' >"$results/oracle-1.csv"
collect "$results" "$tmp_dir/out2"
expect "collect, a partial shard: status" "$status" 1
contains "collect, a partial shard" "$output" "::warning title=Study oracle::shard 1 is partial"
expect "collect, a partial shard: its rows are kept" "$(($(wc -l <"$tmp_dir/out2/oracle.csv") - 1))" 4
contains "collect, a partial shard: README" "$(cat "$tmp_dir/out2/README.md")" "partial (failed or out of time, rows kept): 1; missing: none"
touch "$results/oracle-1.done"
collect "$results" "$tmp_dir/out3"
expect "collect, every shard whole: status" "$status" 0
expect "collect, every shard whole: rows" "$(($(wc -l <"$tmp_dir/out3/oracle.csv") - 1))" 4
printf 'seed,other\n5,0\n' >"$results/oracle-1.csv"
collect "$results" "$tmp_dir/out4"
expect "collect, other columns: status" "$status" 1
contains "collect, other columns" "$output" "shard 1's columns differ"
collect "$results" "$tmp_dir/out5" SHARDS=21
expect "collect, 21 shards: status" "$status" 1
contains "collect, 21 shards" "$output" "SHARDS must be a number from 1 to 20"
printf 'seed,seat\n4,1\n' >"$results/oracle-1.csv"
printf 'other\tplan\n4\tWAIT\n' >"$results/oracle-1.plans.tsv"
collect "$results" "$tmp_dir/out6"
expect "collect, other plan columns: status" "$status" 1
contains "collect, other plan columns" "$output" "shard 1's plan columns differ"
# Shards that wrote no plans: no plans file, no line in the README.
mkdir -p "$tmp_dir/noplans"
printf 'seed,seat\n1,0\n' >"$tmp_dir/noplans/oracle-0.csv"
touch "$tmp_dir/noplans/oracle-0.done"
collect "$tmp_dir/noplans" "$tmp_dir/out7" SHARDS=1
expect "collect, no plans: status" "$status" 0
expect "collect, no plans: no file" "$([[ -e $tmp_dir/out7/oracle.plans.tsv ]] && echo yes || echo no)" no
checks=$((checks + 1))
if grep -q 'Plans:' "$tmp_dir/out7/README.md"; then fail "collect, no plans: a Plans line in the README"; fi

echo "study-test: $checks checks, $failures failures"
((failures == 0))
