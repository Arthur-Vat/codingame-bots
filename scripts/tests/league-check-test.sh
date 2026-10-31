#!/usr/bin/env bash
# Tests scripts/league-check.sh, the merge gate of the incremental league
# (ADR 0038), in a repository of its own with a stand-in arena: no results
# file, a release without a pairing, a leaderboard that does not match, and
# a release with its pairing and its leaderboard.
# Usage: scripts/tests/league-check-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_ACTIONS LEAGUE_ANCHOR LEAGUE_NIGHTLY

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# A repository of its own: the script, the settings library, a game "toy".
repo="$tmp_dir/repo"
mkdir -p "$repo/scripts/lib" "$repo/games/toy/journal" "$repo/games/toy/releases" "$repo/target/release"
cp "$repo_root/scripts/league-check.sh" "$repo/scripts/"
cp "$repo_root/scripts/lib/evaluation.sh" "$repo/scripts/lib/"
printf 'LEAGUE_ANCHOR=random\n' >"$repo/games/toy/evaluation.env"
touch "$repo/games/toy/journal/README.md" "$repo/games/toy/releases/toy-v001.rs" "$repo/games/toy/releases/toy-v002.rs"

# The stand-in arena: records its arguments, and fails the leaderboard check
# unless LEADERBOARD.md holds the line "ok" (what the real one generates is
# the arena's own tests' business).
cat >"$repo/target/release/toy-arena" <<'STUB'
#!/usr/bin/env bash
echo "$*" >>"$STUB_LOG"
[[ $1 == leaderboard ]] || exit 2
while (($#)); do
  if [[ $1 == --check ]]; then file=$2; fi
  shift
done
[[ "$(cat "$file" 2>/dev/null)" == ok ]]
STUB
chmod +x "$repo/target/release/toy-arena"
export STUB_LOG="$tmp_dir/arena.log"

pairing() { # pairing <a> <b>: a line of the results file
  printf '{"a":"%s","b":"%s","pairs":1,"wins":1,"draws":0,"losses":1,"pair_points":[0,0,1,0,0],"faults":[0,0],"time_scale":1.0,"time_tolerance_ms":0,"opening_plies":4,"seed":18446744073709551615,"date":"2026-10-16","commit":"unknown","cpu":"unknown"}\n' "$1" "$2"
}

# check <expected status> <text the output must contain|-> <arguments>...
check() {
  local want="$1" text="$2" output status=0
  shift 2
  checks=$((checks + 1))
  : >"$STUB_LOG"
  output="$(cd "$repo" && scripts/league-check.sh "$@" 2>&1)" || status=$?
  if ((status != want)); then
    fail "$*: expected status $want, got $status: $output"
  elif [[ $text != - && $output != *"$text"* ]]; then
    fail "$*: the output lacks '$text': $output"
  fi
}

expect_unused() { # expect_unused <description>
  checks=$((checks + 1))
  if [[ -s $STUB_LOG ]]; then fail "$1: the arena was called: $(cat "$STUB_LOG")"; fi
}

# No results file: a notice, success, the arena not called.
check 0 'toy has no league results yet: nothing to check' toy games/toy/releases/toy-v002.rs
expect_unused "no results file"

# LEAGUE_BASE: a results file that the base has and HEAD lacks was deleted.
g() { git -C "$repo" -c user.name=t -c user.email=t@example.com "$@"; }
g init -q
mkdir -p "$repo/games/toy"
pairing v001 random >"$repo/games/toy/league.jsonl"
g add -A
g commit -qm base
g rm -q games/toy/league.jsonl
g commit -qm "delete the results"
LEAGUE_BASE=HEAD^1 check 1 '::error title=League results::games/toy/league.jsonl was deleted' toy games/toy/releases/toy-v002.rs
# Never in the base: no league yet. Without LEAGUE_BASE: no league yet too.
LEAGUE_BASE=HEAD check 0 'no league results yet' toy
check 0 'no league results yet' toy
LEAGUE_BASE='bad base!' check 2 'is not a revision' toy

# A release without a pairing.
{
  pairing v001 random
  pairing v001 greedy
} >"$repo/games/toy/league.jsonl"
check 1 '::error title=League results::v002 has no pairing in games/toy/league.jsonl: copy the lines of the league comment' \
  toy games/toy/releases/toy-v002.rs
expect_unused "no pairing"
# A bot whose name only starts like the release's does not count.
pairing v0020 random >>"$repo/games/toy/league.jsonl"
check 1 'v002 has no pairing' toy games/toy/releases/toy-v002.rs

# A game with a nightly league: a release needs no pairing, the leaderboard
# is still checked.
echo stale >"$repo/games/toy/LEADERBOARD.md"
LEAGUE_NIGHTLY=true check 1 'does not match' toy games/toy/releases/toy-v002.rs
LEAGUE_NIGHTLY=true check 1 'v002 needs no pairing in games/toy/league.jsonl yet' toy games/toy/releases/toy-v002.rs
echo ok >"$repo/games/toy/LEADERBOARD.md"
LEAGUE_NIGHTLY=true check 0 'matches games/toy/league.jsonl' toy games/toy/releases/toy-v002.rs
LEAGUE_NIGHTLY=false check 1 'v002 has no pairing' toy games/toy/releases/toy-v002.rs

# The pairing is there (as the second bot too), the leaderboard is stale.
pairing greedy v002 >>"$repo/games/toy/league.jsonl"
echo stale >"$repo/games/toy/LEADERBOARD.md"
check 1 'games/toy/LEADERBOARD.md does not match games/toy/league.jsonl' toy games/toy/releases/toy-v001.rs games/toy/releases/toy-v002.rs
checks=$((checks + 1))
if ! grep -qx 'leaderboard --results games/toy/league.jsonl --anchor random --journal games/toy/journal/README.md --check games/toy/LEADERBOARD.md' "$STUB_LOG"; then
  fail "the arena's arguments: $(cat "$STUB_LOG")"
fi
check 1 'regenerate it on Linux with: cargo run --release -p toy-arena -- leaderboard' toy games/toy/releases/toy-v002.rs

# Everything matches.
echo ok >"$repo/games/toy/LEADERBOARD.md"
check 0 'ok   games/toy/LEADERBOARD.md matches games/toy/league.jsonl' toy games/toy/releases/toy-v001.rs games/toy/releases/toy-v002.rs
# No release file: the leaderboard is still checked.
check 0 - toy
echo stale >"$repo/games/toy/LEADERBOARD.md"
check 1 'does not match' toy

# Usage errors.
check 2 'usage'
check 2 'not a game name' '../toy'
check 2 'is not a release of toy' toy games/toy/releases/other.rs
check 2 'is not a release of toy' toy games/toy/releases/toy-final.rs
rm "$repo/target/release/toy-arena"
echo ok >"$repo/games/toy/LEADERBOARD.md"
check 2 'toy-arena is missing' toy

if ((failures > 0)); then
  echo "FAIL league-check: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   league-check: $checks checks"
