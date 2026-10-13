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
unset GITHUB_ACTIONS NEEDS RESULTS_TITLE
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
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/bots/mcts/src/main.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=true' \
  games/uttt/bots/mcts/Cargo.toml
scopes_are 'rust=true framework=false games=["other","uttt"] studio=true web=true licenses=false' \
  games/other/engine/src/lib.rs games/uttt/evaluation.env
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/engine/src/lib.rs games/uttt/journal/E001-x.md
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/studio/src/lib.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/referee/src/lib.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/releases/uttt-v011.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=false licenses=false' \
  games/uttt/ci.sh
# The analysis program runs in the browser engine (ADR 0032), and is built
# from the game's engine and the bots' pattern weights.
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/analysis/src/lib.rs
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=true' \
  games/uttt/analysis/Cargo.toml
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' \
  games/uttt/bots/mcts/src/pattern_weights.rs

# The studio.
scopes_are 'rust=false framework=false games=[] studio=false web=true licenses=false' studio/web/src/App.tsx
scopes_are 'rust=true framework=false games=[] studio=true web=false licenses=false' studio/server/src/main.rs
scopes_are 'rust=true framework=false games=[] studio=true web=true licenses=false' studio/game/src/lib.rs
scopes_are 'rust=true framework=false games=[] studio=true web=false licenses=false' studio/server/src/lib.rs
scopes_are 'rust=true framework=false games=[] studio=true web=true licenses=false' studio/core/src/lib.rs
scopes_are 'rust=true framework=false games=[] studio=true web=true licenses=false' studio/rules/src/main.rs
scopes_are 'rust=true framework=false games=[] studio=true web=true licenses=true' studio/rules/Cargo.toml
scopes_are 'rust=true framework=false games=[] studio=true web=false licenses=true' studio/server/Cargo.toml
scopes_are 'rust=false framework=false games=[] studio=false web=true licenses=false' \
  studio/web/package.json studio/web/package-lock.json

# The framework.
scopes_are "$framework" crates/cg-core/src/lib.rs
scopes_are "$framework" scripts/sprt.sh
scopes_are "$framework" scripts/build-studio-wasm.sh
scopes_are "$framework" crates/cg-arena/src/live.rs
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
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' ./games/uttt/bots/mcts/src/main.rs
scopes_are "$none" ./docs/ROADMAP.md
scopes_are 'rust=true framework=false games=["uttt"] studio=true web=true licenses=false' 'games/uttt/bots/a b/src/main.rs'
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
games_are sprt '[]' games/uttt/releases/notes.md games/uttt/bots/x.rs games/uttt/releases/uttt-v011.rs.bak games/uttt/releases/.rs.md
games_are sprt '[]' games/nogame/releases/a.md games/uttt/releases/sub/notes.txt games/uttt/bots/mcts/src/main.rs
# A release file that the workflow's pathspec games/*/releases/*.rs matches but
# that is not games/<known game>/releases/<name>.rs fails: the SPRT cannot
# place it, and the league's find would rate it.
sprt_fails() { # sprt_fails <message part> <arguments>...
  local text="$1" output status=0
  shift
  checks=$((checks + 1))
  output="$("$script" sprt "$@" 2>&1)" || status=$?
  if ((status != 1)); then
    fail "sprt $*: expected status 1, got $status: $output"
  elif [[ $output != *"$text"* ]]; then
    fail "sprt $*: the output lacks '$text': $output"
  fi
}
sprt_fails 'cannot place games/uttt/releases/sub/uttt-v011.rs' games/uttt/releases/sub/uttt-v011.rs
sprt_fails 'cannot place games/nogame/releases/a.rs' games/nogame/releases/a.rs
sprt_fails 'cannot place games/bad name/releases/a.rs' 'games/bad name/releases/a.rs'
sprt_fails 'cannot place games/README.txt/releases/a.rs' games/README.txt/releases/a.rs
sprt_fails 'cannot place games/uttt/extra/releases/a.rs' games/uttt/extra/releases/a.rs
sprt_fails 'cannot place games/uttt/releases/releases/a.rs' games/uttt/releases/releases/a.rs
sprt_fails 'cannot place games/a/b/releases/c/d.rs' games/a/b/releases/c/d.rs
sprt_fails 'cannot place ./games/nogame/releases/a.rs' ./games/nogame/releases/a.rs
# One placed and one not: the whole run fails, naming the file.
sprt_fails 'cannot place games/other/releases/x/y.rs' games/uttt/releases/uttt-v011.rs games/other/releases/x/y.rs
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
# A change to the ci mode must not play the full league.
games_are league '[]' scripts/ci-scopes.sh scripts/tests/ci-scopes-test.sh .github/workflows/sprt.yml
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

# The annotation's title comes from RESULTS_TITLE, `CI passed` without it.
checks=$((checks + 1))
status=0
output="$(RESULTS_TITLE='SPRT passed' NEEDS="$skipped_lint" "$script" results "${args[@]}" 2>&1)" || status=$?
if ((status != 1)) || [[ $output != *'::error title=SPRT passed::lint: skipped (had to run)'* ]] || [[ $output == *'title=CI passed'* ]]; then
  fail "results with RESULTS_TITLE: status $status, output: $output"
fi
checks=$((checks + 1))
status=0
output="$(RESULTS_TITLE='SPRT passed' NEEDS='not json' "$script" results "${args[@]}" 2>&1)" || status=$?
if ((status != 1)) || [[ $output != *'::error title=SPRT passed::NEEDS is not a JSON object'* ]]; then
  fail "results with RESULTS_TITLE and bad NEEDS: status $status, output: $output"
fi
checks=$((checks + 1))
status=0
output="$(RESULTS_TITLE='' NEEDS="$skipped_lint" "$script" results "${args[@]}" 2>&1)" || status=$?
if ((status != 1)) || [[ $output != *'::error title=CI passed::'* ]]; then
  fail "results with an empty RESULTS_TITLE: status $status, output: $output"
fi

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

# ---- the workflow ----

# An aggregate job gathers every other job of its workflow, and its results
# call expects each of them: a job added to the workflow and forgotten there
# would never block a merge.
aggregate_covers_jobs() { # aggregate_covers_jobs <workflow> <aggregate job id>
  local workflow="$1" aggregate="$2" id
  local -a job_ids needed expected_ids
  mapfile -t job_ids < <(sed -n '/^jobs:/,$p' "$workflow" | sed -n 's/^  \([a-z][a-z0-9-]*\):[[:space:]]*$/\1/p')
  mapfile -t needed < <(sed -n "/^  $aggregate:/,\$p" "$workflow" | sed -n '/^    needs:/,/^    [a-z]/p' | sed -n 's/^      - //p')
  mapfile -t expected_ids < <(sed -n "/^  $aggregate:/,\$p" "$workflow" | sed -n '/scripts\/ci-scopes.sh results/,$p' | grep -o '[a-z][a-z0-9-]*=' | sed 's/=$//')
  checks=$((checks + 1))
  if ((${#job_ids[@]} < 2)); then fail "$workflow: found no jobs"; fi
  for id in "${job_ids[@]}"; do
    [[ $id == "$aggregate" ]] && continue
    checks=$((checks + 1))
    if ! printf '%s\n' "${needed[@]}" | grep -qx -- "$id"; then fail "$workflow: $aggregate does not list $id in needs"; fi
    if ! printf '%s\n' "${expected_ids[@]}" | grep -qx -- "$id"; then fail "$workflow: the results call of $aggregate lacks $id="; fi
  done
  for id in "${needed[@]}" "${expected_ids[@]}"; do
    checks=$((checks + 1))
    if ! printf '%s\n' "${job_ids[@]}" | grep -qx -- "$id"; then fail "$workflow: $aggregate names $id, which is not a job"; fi
  done
}

# `CI passed` for ci.yml, `SPRT passed` for sprt.yml.
aggregate_covers_jobs .github/workflows/ci.yml ci-passed
aggregate_covers_jobs .github/workflows/sprt.yml sprt-passed

# The SPRT's annotation carries the name of its check (the default title is
# `CI passed`).
checks=$((checks + 1))
if ! grep -qx "          RESULTS_TITLE: SPRT passed" .github/workflows/sprt.yml; then
  fail "sprt.yml: the results call lacks RESULTS_TITLE: SPRT passed"
fi

if ((failures > 0)); then
  echo "FAIL ci-scopes: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   ci-scopes: $checks checks"
