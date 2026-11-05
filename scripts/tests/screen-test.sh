#!/usr/bin/env bash
# Tests scripts/screen.sh without a build: the validation of the dispatch
# inputs by `plan` (names, refs, baseline, pairs, shards, seed, limits) and its
# matrix, in a stand-in repository with branches and a tag; and the table of
# `summary` with a stand-in arena. Needs git and jq.
# Usage: scripts/tests/screen-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_OUTPUT GITHUB_STEP_SUMMARY MATRIX
unset GAME BOT VARIANTS BASELINE PAIRS SHARDS SEED

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
mkdir -p "$repo/scripts/lib" "$repo/games/toy/bots/plan" "$repo/games/toy/releases" "$repo/games/other/bots/x"
cp "$repo_root/scripts/screen.sh" "$repo/scripts/"
cp "$repo_root/scripts/lib/evaluation.sh" "$repo/scripts/lib/"
printf 'SEED=7\nTIME_SCALE=1\n' >"$repo/games/toy/evaluation.env"
touch "$repo/games/other/evaluation.env" "$repo/games/other/bots/x/Cargo.toml"
printf '[package]\nname = "toy-plan"\n' >"$repo/games/toy/bots/plan/Cargo.toml"
touch "$repo/games/toy/releases/toy-v001.rs" "$repo/games/toy/releases/toy-v002.rs"
git_in() { git -C "$repo" -c user.name=t -c user.email=t@example.com "$@"; }
git_in init -q
git_in add -A
git_in commit -q -m first
first="$(git_in rev-parse HEAD)"
# A branch without the bot, one with a slash in its name, and a tag.
git_in checkout -q -b nobot
git_in rm -q -r games/toy/bots
git_in commit -q -m "no bot"
nobot="$(git_in rev-parse HEAD)"
git_in checkout -q -b claude/work "$first"
git_in commit -q --allow-empty -m work
work="$(git_in rev-parse HEAD)"
git_in tag v1 "$first"
git_in update-ref refs/remotes/origin/remote-only "$work"
git_in checkout -q -b main-line "$first"

run() { # run <variants> [KEY=value]...: plan with defaults; sets output, status
  local variants="$1"
  shift
  status=0
  output="$(
    cd "$repo"
    env GAME=toy BOT=plan VARIANTS="$variants" BASELINE= PAIRS=400 SHARDS=1 SEED= "$@" \
      scripts/screen.sh plan 2>&1
  )" || status=$?
}
bad() { # bad <description> <needle> <variants> [KEY=value]...
  local description="$1" needle="$2"
  shift 2
  run "$@"
  expect "$description: status" "$status" 1
  contains "$description" "$output" "::error title=Screening::"
  contains "$description" "$output" "$needle"
}

# ---- a good input ----
run "a=$first b=claude/work c=v1 d=remote-only" SHARDS=2 PAIRS=800
expect "good: status" "$status" 0
matrix="$(sed -n 's/^matrix=//p' <<<"$output")"
expect "good: entries" "$(jq length <<<"$matrix")" 8
expect "good: first entry" "$(jq -c '.[0]' <<<"$matrix")" "{\"variant\":\"a\",\"sha\":\"$first\",\"shard\":0}"
expect "good: branch with a slash" "$(jq -r '.[2].sha' <<<"$matrix")" "$work"
expect "good: tag" "$(jq -r '[.[] | select(.variant == "c")] | map(.shard) | join(",")' <<<"$matrix")" "0,1"
expect "good: remote-only branch" "$(jq -r '.[6].sha' <<<"$matrix")" "$work"
contains "good" "$output" "baseline=games/toy/releases/toy-v002.rs"
contains "good" "$output" "baseline_name=toy-v002"
contains "good" "$output" "seed=7"
contains "good" "$output" "pairs=800"
contains "good" "$output" "shards=2"
run "short=${first:0:7}" BASELINE=toy-v001 SEED=00012
expect "short sha: status" "$status" 0
contains "short sha" "$output" "\"sha\":\"$first\""
contains "named baseline" "$output" "baseline=games/toy/releases/toy-v001.rs"
contains "seed without leading zeros" "$output" "seed=12"
run "$(printf 'a=%s\nb=%s' "$first" "$work")" SEED=18446744073709551615
expect "variants on two lines, the largest seed" "$status" 0
run "a=$first" PAIRS=2 SHARDS=2
expect "two pairs, two shards" "$status" 0
out_file="$tmp_dir/output"
(
  cd "$repo"
  GITHUB_OUTPUT="$out_file" GAME=toy BOT=plan VARIANTS="a=$first" BASELINE="" PAIRS=10 SHARDS=1 SEED="" scripts/screen.sh plan >/dev/null
)
contains "GITHUB_OUTPUT" "$(cat "$out_file")" "baseline_name=toy-v002"

# ---- bad inputs ----
bad "bad name" "variant name" "-a=$first"
bad "name with a slash" "variant name" "a/b=$first"
bad "name too long" "variant name" "$(printf 'n%.0s' {1..41})=$first"
bad "no equal sign" "not NAME=REF" "$first"
bad "duplicate names" "used twice" "a=$first a=$work"
bad "name of the baseline" "baseline's name" "toy-v002=$first"
bad "no variant" "1 to 8 variants" ""
bad "nine variants" "1 to 8 variants" "a=$first b=$first c=$first d=$first e=$first f=$first g=$first h=$first i=$first"
bad "too many runners" "variants x shards" "a=$first b=$first c=$first" PAIRS=700 SHARDS=7
bad "pairs not a multiple" "multiple of shards" "a=$first" PAIRS=401 SHARDS=2
bad "pairs too few" "pairs must be" "a=$first" PAIRS=1
bad "pairs too many" "pairs must be" "a=$first" PAIRS=8001
bad "pairs not a number" "pairs must be" "a=$first" PAIRS=4e2
bad "shards zero" "shards must be" "a=$first" SHARDS=0
bad "shards nine" "shards must be" "a=$first" SHARDS=9
bad "seed not a number" "seed must be" "a=$first" SEED=-1
bad "seed too large" "seed must be" "a=$first" SEED=18446744073709551616
bad "seed too long" "seed must be" "a=$first" SEED=123456789012345678901
bad "unknown baseline" "does not exist" "a=$first" BASELINE=toy-v009
bad "baseline with a path" "release name" "a=$first" BASELINE=../toy-v001
bad "unknown ref" "not a commit, a branch or a tag" "a=nosuchbranch"
bad "unknown hex ref" "not a commit, a branch or a tag" "a=deadbeefdeadbeef"
bad "ref with dots" "not a commit, a branch or a tag" "a=main..work"
bad "ref starting with a dash" "not a commit, a branch or a tag" "a=--upload-pack=x"
bad "ref without the bot" "does not exist at ref" "a=$nobot"
bad "unknown game" "evaluation.env" "a=$first" GAME=nope
bad "game with a slash" "game must be" "a=$first" GAME=../toy
bad "unknown bot" "does not exist" "a=$first" BOT=nope
bad "bot with a slash" "bot must be" "a=$first" BOT=../plan
# Text with a line break cannot start another workflow command.
run "a=$first
::set-output name=x::y" SHARDS=1
expect "line break: status" "$status" 1
checks=$((checks + 1))
if grep -q '^::set-output' <<<"$output"; then fail "line break: a workflow command got through"; fi
status=0
output="$(cd "$repo" && env GAME=toy BOT=plan VARIANTS="a=$first" BASELINE=$'toy-v001\n::warning::x' PAIRS=10 SHARDS=1 SEED= scripts/screen.sh plan 2>&1)" || status=$?
expect "baseline with a line break" "$status" 1
checks=$((checks + 1))
if grep -q '^::warning::x' <<<"$output"; then fail "baseline with a line break: a workflow command got through"; fi

# ---- usage ----
status=0
(cd "$repo" && scripts/screen.sh nothing >/dev/null 2>&1) || status=$?
expect "unknown mode" "$status" 2
status=0
(cd "$repo" && scripts/screen.sh play toy >/dev/null 2>&1) || status=$?
expect "play without arguments" "$status" 2

# ---- summary, with a stand-in arena ----
mkdir -p "$repo/target/release" "$tmp_dir/results"
cat >"$repo/target/release/toy-arena" <<'ARENA'
#!/usr/bin/env bash
# Stand-in: `sum FILE...` prints a summary naming the files it was given.
[[ $1 == sum ]] || exit 2
shift
echo "a vs toy-v002: $(($# * 20)) games, $(($# * 10)) complete pairs"
echo "  a: 12 wins, 0 draws, 8 losses (score 60.0%)"
echo "  Elo of a over toy-v002: +30.2 [-5.0, +65.4] (95%, from pairs)"
echo "  pairs by a's points (0, 1/2, 1, 3/2, 2): [1, 2, 3, 2, 2]"
echo "  a faults: 1 timeouts, 0 crashes, 0 invalid answers"
echo "  toy-v002 faults: 0 timeouts, 0 crashes, 0 invalid answers"
echo "  slowest answer: a 48.5 ms, toy-v002 40.0 ms"
ARENA
chmod +x "$repo/target/release/toy-arena"
for shard in 0 1; do
  : >"$tmp_dir/results/a-$shard.jsonl"
  echo "time scale 1.5$shard, tolerance 20 ms, opening plies 0, seed 7, timeouts allowed in up to 1% of games; machine speed 9.$shard" >"$tmp_dir/results/a-$shard.settings"
  echo "  answers after the first (median, 99%, 99.9%): a 1.0, 20.0, 30.0 ms; toy-v002 2.0, 21.0, 31.0 ms" >"$tmp_dir/results/a-$shard.txt"
done
sum_matrix="[{\"variant\":\"a\",\"sha\":\"$first\",\"shard\":0},{\"variant\":\"a\",\"sha\":\"$first\",\"shard\":1}]"
summary() { # summary [KEY=value]...: sets output, status
  status=0
  output="$(cd "$repo" && env GITHUB_STEP_SUMMARY="$tmp_dir/step-summary" "$@" scripts/screen.sh summary toy "$tmp_dir/results" 2>&1)" || status=$?
}
rm -f "$tmp_dir/step-summary"
summary MATRIX="$sum_matrix"
expect "summary: status" "$status" 0
contains "summary" "$output" "::notice title=Screening a::Elo +30.2 [-5.0, +65.4] over toy-v002, 20 pairs"
contains "summary" "$output" "| a | \`${first:0:7}\` | 20 | +30.2 [-5.0, +65.4] | 60.0% | 1/0/0 | 0/0/0 | 48.5 / 40.0 ms | 1.50, 1.51 |"
contains "summary" "$(cat "$tmp_dir/step-summary")" "| a | 1 | 1.51 | 9.1 | 1.0, 20.0, 30.0 ms | 2.0, 21.0, 31.0 ms |"
rm "$tmp_dir/results/a-1.jsonl"
summary MATRIX="$sum_matrix"
expect "missing shard: status" "$status" 1
contains "missing shard" "$output" "::warning title=Screening a::shard 1 of a has no result"
contains "missing shard" "$output" "::notice title=Screening a::"
summary
expect "no matrix, one shard found: status" "$status" 0

if ((failures > 0)); then
  echo "FAIL screen: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   screen: $checks checks"
