#!/usr/bin/env bash
# Tests scripts/ci-scopes.sh: the area of each path (against a stand-in folder
# of games), the games of the sprt and league modes, and the verdict of the
# aggregate job. Needs jq.
# Usage: scripts/tests/ci-scopes-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
script="$PWD/scripts/ci-scopes.sh"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_ACTIONS NEEDS
mkdir -p "$tmp_dir/games/uttt" "$tmp_dir/games/other" "$tmp_dir/games/bad name"
touch "$tmp_dir/games/README.txt"
export GAMES_DIR="$tmp_dir/games"

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

all_true='rust=true framework=true games=["other","uttt"] studio=true web=true licenses=true'
none='rust=false framework=false games=[] studio=false web=false licenses=false'
framework='rust=true framework=true games=["other","uttt"] studio=true web=true licenses=false'

# The six lines printed for the arguments, space separated, are the expected ones.
scopes_are() { # scopes_are <expected> <arguments of the ci mode>...
  local expected="$1" actual
  shift
  checks=$((checks + 1))
  if ! actual="$("$script" ci "$@" 2>/dev/null | tr '\n' ' ')"; then
    fail "ci $*: the script failed"
    return
  fi
  actual="${actual% }"
  if [[ $actual != "$expected" ]]; then fail "ci $*: expected '$expected', got '$actual'"; fi
}

# The stderr summary names the areas.
summary_is() { # summary_is <expected> <arguments of the ci mode>...
  local expected="$1" actual
  shift
  checks=$((checks + 1))
  actual="$("$script" ci "$@" 2>&1 >/dev/null)"
  if [[ $actual != "$expected" ]]; then fail "summary of $*: expected '$expected', got '$actual'"; fi
}

# The results mode with NEEDS and the arguments: exit status, and the text
# printed (stdout and stderr) contains each further text.
results_are() { # results_are <status> <NEEDS> <expected text|-> <arguments>...
  local want_status="$1" needs="$2" text="$3" output status=0
  shift 3
  checks=$((checks + 1))
  output="$(NEEDS="$needs" "$script" results "$@" 2>&1)" || status=$?
  if ((status != want_status)); then
    fail "results $*: expected status $want_status, got $status: $output"
  elif [[ $text != - && $output != *"$text"* ]]; then
    fail "results $*: the output lacks '$text': $output"
  fi
}

# ---- ci ----

# Docs and the like: nothing runs.
scopes_are "$none"
scopes_are "$none" docs/ROADMAP.md games/uttt/journal/E012-x.md .claude/settings.json
scopes_are "$none" studio/web/README.md
scopes_are "$none" README.md games/uttt/README.md crates/cg-core/README.md
scopes_are "$none" docs/adr/0029-x.md docs/some/image.png

# A game.
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' \
  games/uttt/bots/mcts/src/main.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=true' \
  games/uttt/bots/mcts/Cargo.toml
scopes_are 'rust=true framework=false games=["other","uttt"] studio=true web=false licenses=false' \
  games/other/engine/src/lib.rs games/uttt/evaluation.env
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' \
  games/uttt/engine/src/lib.rs games/uttt/journal/E001-x.md
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' \
  games/uttt/studio/src/lib.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' \
  games/uttt/ci.sh

# The studio.
scopes_are 'rust=false framework=false games=[] studio=false web=true licenses=false' studio/web/src/App.tsx
scopes_are 'rust=true framework=false games=[] studio=true web=false licenses=false' studio/server/src/main.rs
scopes_are 'rust=true framework=false games=[] studio=true web=false licenses=false' studio/game/src/lib.rs
scopes_are 'rust=true framework=false games=[] studio=true web=false licenses=true' studio/server/Cargo.toml
scopes_are 'rust=false framework=false games=[] studio=false web=true licenses=false' \
  studio/web/package.json studio/web/package-lock.json

# The framework.
scopes_are "$framework" crates/cg-core/src/lib.rs
scopes_are "$framework" scripts/sprt.sh
scopes_are "$framework" scripts/tests/pr-hygiene-test.sh
scopes_are "$framework" crates/cg-core/src/lib.rs docs/ROADMAP.md
scopes_are 'rust=true framework=true games=["other","uttt"] studio=true web=true licenses=true' crates/cg-arena/Cargo.toml

# Everything.
scopes_are "$all_true" Cargo.lock
scopes_are "$all_true" Cargo.toml
scopes_are "$all_true" deny.toml
scopes_are "$all_true" rust-toolchain
scopes_are "$all_true" rust-toolchain.toml
scopes_are "$all_true" .cargo/config.toml
scopes_are "$all_true" .github/workflows/ci.yml
scopes_are "$all_true" .github/pull_request_template.md.bak
scopes_are "$all_true" .gitignore
scopes_are "$all_true" LICENSE-MIT
scopes_are "$all_true" games/README.txt
scopes_are "$all_true" games/nogame/x.rs
scopes_are "$all_true" 'games/bad name/x.rs'
scopes_are "$all_true" games/uttt
scopes_are "$all_true" something/else/file.rs
scopes_are "$all_true" tools/Cargo.toml
scopes_are "$all_true" --all
scopes_are "$all_true" --all docs/ROADMAP.md
scopes_are "$all_true" docs/ROADMAP.md .gitignore
scopes_are "$all_true" games/uttt/bots/mcts/src/main.rs .github/workflows/ci.yml studio/web/src/App.tsx

# Path forms.
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' ./games/uttt/bots/mcts/src/main.rs
scopes_are "$none" ./docs/ROADMAP.md
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' 'games/uttt/bots/a b/src/main.rs'
scopes_are "$none" 'docs/a file with spaces.txt'
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' -- games/uttt/x.rs
scopes_are "$none" ''

summary_is 'ci-scopes: framework=false games=uttt studio=true web=false licenses=false (3 files)' \
  games/uttt/a.rs games/uttt/b.rs docs/c.txt
summary_is 'ci-scopes: framework=false games= studio=false web=false licenses=false (0 files)'

# A usage error.
checks=$((checks + 1))
status=0
"$script" ci --nonsense >/dev/null 2>&1 || status=$?
if ((status != 2)); then fail "ci --nonsense: expected status 2, got $status"; fi

# ---- sprt and league ----

# The games line printed for the arguments of the mode is the expected one.
games_are() { # games_are <mode> <expected> <arguments>...
  local mode="$1" expected="$2" actual
  shift 2
  checks=$((checks + 1))
  if ! actual="$("$script" "$mode" "$@" 2>/dev/null)"; then
    fail "$mode $*: the script failed"
    return
  fi
  if [[ $actual != "games=$expected" ]]; then fail "$mode $*: expected 'games=$expected', got '$actual'"; fi
}

# The stderr summary of a mode.
mode_summary_is() { # mode_summary_is <mode> <expected> <arguments>...
  local mode="$1" expected="$2" actual
  shift 2
  checks=$((checks + 1))
  actual="$("$script" "$mode" "$@" 2>&1 >/dev/null)"
  if [[ $actual != "$expected" ]]; then fail "summary of $mode $*: expected '$expected', got '$actual'"; fi
}

# The exit status of the script with the arguments.
status_is() { # status_is <status> <arguments>...
  local want="$1" status=0
  shift
  checks=$((checks + 1))
  "$script" "$@" >/dev/null 2>&1 || status=$?
  if ((status != want)); then fail "$*: expected status $want, got $status"; fi
}

# sprt: the added files decide.
games_are sprt '[]'
games_are sprt '[]' ''
games_are sprt '["uttt"]' games/uttt/releases/uttt-v011.rs
games_are sprt '["other","uttt"]' games/uttt/releases/uttt-v011.rs games/other/releases/other-v001.rs
games_are sprt '["other","uttt"]' games/other/releases/other-v001.rs games/uttt/releases/uttt-v011.rs games/uttt/releases/uttt-v012.rs
games_are sprt '[]' games/uttt/releases/notes.md games/uttt/bots/x.rs games/nogame/releases/a.rs
games_are sprt '[]' games/uttt/releases/sub/uttt-v011.rs games/uttt/releases/uttt-v011.rs.bak games/uttt/releases/.rs.md
games_are sprt '[]' 'games/bad name/releases/a.rs' games/README.txt/releases/a.rs
games_are sprt '[]' games/uttt/evaluation.env docs/ROADMAP.md scripts/league.sh .github/workflows/sprt.yml
games_are sprt '["uttt"]' ./games/uttt/releases/uttt-v011.rs
games_are sprt '["uttt"]' -- games/uttt/releases/uttt-v011.rs
games_are sprt '["uttt"]' -- games/uttt/releases/uttt-v011.rs 'games/uttt/releases/a b.rs'
mode_summary_is sprt 'ci-scopes: sprt: games=uttt (2 files)' games/uttt/releases/uttt-v011.rs docs/x.md
mode_summary_is sprt 'ci-scopes: sprt: games= (0 files)'
status_is 2 sprt --all
status_is 2 sprt --all games/uttt/releases/uttt-v011.rs
status_is 2 sprt --nonsense

# league: the changed files decide; the league's own files give every game.
games_are league '[]'
games_are league '[]' ''
games_are league '["uttt"]' games/uttt/evaluation.env
games_are league '["uttt"]' games/uttt/releases/uttt-v011.rs
games_are league '["uttt"]' games/uttt/releases/uttt-v010.rs games/uttt/evaluation.env
games_are league '["other","uttt"]' games/uttt/releases/uttt-v011.rs games/other/evaluation.env
games_are league '["uttt"]' games/uttt/releases/notes.md
games_are league '["uttt"]' games/uttt/releases/sub/x.rs
games_are league '[]' games/uttt/bots/x.rs games/uttt/README.md games/uttt/evaluation.env.bak games/uttt/releases
games_are league '[]' crates/cg-arena/src/lib.rs docs/ROADMAP.md .github/workflows/ci.yml
games_are league '[]' games/nogame/releases/a.rs games/nogame/evaluation.env 'games/bad name/evaluation.env'
games_are league '["other","uttt"]' scripts/league.sh
games_are league '["other","uttt"]' scripts/lib/evaluation.sh
games_are league '["other","uttt"]' scripts/ci-scopes.sh
games_are league '["other","uttt"]' .github/workflows/league.yml
games_are league '["other","uttt"]' crates/cg-arena/src/lib.rs scripts/league.sh
games_are league '["other","uttt"]' --all
games_are league '["other","uttt"]' --all docs/ROADMAP.md
games_are league '["other","uttt"]' --all games/uttt/evaluation.env
games_are league '["uttt"]' ./games/uttt/evaluation.env
games_are league '["other","uttt"]' ./scripts/league.sh
games_are league '["uttt"]' -- games/uttt/releases/uttt-v011.rs
# A deleted release still lists its game: the league rates what is left.
games_are league '["uttt"]' games/uttt/releases/uttt-v001.rs
mode_summary_is league 'ci-scopes: league: games=other uttt (1 files)' scripts/league.sh
mode_summary_is league 'ci-scopes: league: games= (0 files)'
status_is 2 league --nonsense

# ---- results ----

ok_needs='{
  "changes": {"result": "success", "outputs": {"rust": "true"}},
  "lint": {"result": "success", "outputs": {}},
  "game-bots": {"result": "success", "outputs": {}},
  "docs": {"result": "success", "outputs": {}}
}'
args=(changes=true lint=true game-bots=true docs=true)

results_are 0 "$ok_needs" 'ok   lint: success' "${args[@]}"
results_are 0 "$ok_needs" - changes=true lint=false game-bots=false docs=true

# An optional job skipped passes; a needed one skipped fails.
skipped_lint="$(jq -c '.lint.result = "skipped"' <<<"$ok_needs")"
results_are 0 "$skipped_lint" 'ok   lint: skipped' changes=true lint=false game-bots=true docs=true
results_are 1 "$skipped_lint" 'FAIL lint: skipped (had to run)' "${args[@]}"
results_are 1 "$skipped_lint" '::error title=CI passed::lint: skipped (had to run)' "${args[@]}"

# A failure, a cancellation.
failed="$(jq -c '."game-bots".result = "failure"' <<<"$ok_needs")"
results_are 1 "$failed" 'FAIL game-bots: failure (had to run)' "${args[@]}"
results_are 1 "$failed" 'FAIL game-bots: failure' changes=true lint=true game-bots=false docs=true
results_are 1 "$failed" '::error title=CI passed::game-bots: failure' "${args[@]}"
cancelled="$(jq -c '.docs.result = "cancelled"' <<<"$ok_needs")"
results_are 1 "$cancelled" 'FAIL docs: cancelled' "${args[@]}"
results_are 1 "$cancelled" 'FAIL docs: cancelled' changes=true lint=true game-bots=true docs=false
both="$(jq -c '.docs.result = "cancelled" | .lint.result = "failure"' <<<"$ok_needs")"
results_are 1 "$both" '::error title=CI passed::lint: failure (had to run), docs: cancelled (had to run)' "${args[@]}"

# A job named but missing from NEEDS; a job in NEEDS that no argument names.
missing="$(jq -c 'del(.lint)' <<<"$ok_needs")"
results_are 1 "$missing" 'FAIL lint: missing' "${args[@]}"
results_are 1 "$missing" 'FAIL lint: missing' changes=true lint=false game-bots=true docs=true
extra="$(jq -c '.studio = {"result": "success", "outputs": {}}' <<<"$ok_needs")"
results_are 1 "$extra" 'FAIL studio: success (no argument names this job)' "${args[@]}"
results_are 1 "$extra" '::error title=CI passed::studio: not named' "${args[@]}"

# An empty or invalid expected value (the changes job failed: no outputs).
results_are 1 "$ok_needs" "FAIL lint: expected value '' is neither true nor false" changes=true lint= game-bots=true docs=true
results_are 1 "$ok_needs" "FAIL lint: expected value 'maybe' is neither true nor false" changes=true lint=maybe game-bots=true docs=true
results_are 1 "$ok_needs" "FAIL lint: expected value 'True' is neither true nor false" changes=true lint=True game-bots=true docs=true
changes_failed="$(jq -c '.changes.result = "failure" | .lint.result = "skipped" | ."game-bots".result = "skipped"' <<<"$ok_needs")"
results_are 1 "$changes_failed" 'FAIL changes: failure (had to run)' changes=true lint= game-bots= docs=true

# NEEDS that is not a JSON object fails too.
results_are 1 'not json' 'NEEDS is not a JSON object' "${args[@]}"

# Usage errors.
results_are 2 "$ok_needs" -
results_are 2 "$ok_needs" - lint
results_are 2 "$ok_needs" - =true
results_are 2 "$ok_needs" - lint=true lint=false
checks=$((checks + 1))
status=0
"$script" results "${args[@]}" >/dev/null 2>&1 || status=$?
if ((status != 2)); then fail "results without NEEDS: expected status 2, got $status"; fi
checks=$((checks + 1))
status=0
"$script" >/dev/null 2>&1 || status=$?
if ((status != 2)); then fail "no mode: expected status 2, got $status"; fi
checks=$((checks + 1))
status=0
"$script" frobnicate >/dev/null 2>&1 || status=$?
if ((status != 2)); then fail "unknown mode: expected status 2, got $status"; fi

if ((failures > 0)); then
  echo "FAIL ci-scopes: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   ci-scopes: $checks checks"
