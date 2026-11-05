#!/usr/bin/env bash
# Tests scripts/check.sh without running any check: with --dry-run and the
# files given after --, the steps each kind of change selects (docs, a game,
# the framework, the studio's Rust crates, the front end, a workflow, --all)
# and the usage errors. Reads the real games/ folder, as scripts/ci-scopes.sh
# does. Needs git.
# Usage: scripts/tests/check-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}
contains() { # contains <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 != *"$3"* ]]; then fail "$1: '$3' not found in: $2"; fi
}
lacks() { # lacks <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 == *"$3"* ]]; then fail "$1: '$3' found in: $2"; fi
}
plan() { scripts/check.sh --dry-run "$@" 2>/dev/null; }

out="$(plan -- docs/WORKFLOW.md games/uttt/README.md)"
contains "docs: docs check" "$out" "step: docs check"
contains "docs: script tests" "$out" "step: script tests"
lacks "docs: no cargo" "$out" "cargo"
lacks "docs: no bundles" "$out" "bundles"

out="$(plan -- games/uttt/engine/src/lib.rs)"
contains "game: fmt" "$out" "step: cargo fmt"
contains "game: clippy" "$out" "step: cargo clippy"
contains "game: its tests, the shared crates', the studio's" "$out" "step: cargo test of games/uttt/ crates/ studio/ games/*/studio/"
contains "game: its bundles" "$out" "step: bundles of uttt"
lacks "game: not the other game's bundles" "$out" "bundles of ice-and-fire"
lacks "game: not the whole workspace" "$out" "cargo test --workspace"
contains "game engine: the front end" "$out" "note: the front end changed"

out="$(plan -- games/ice-and-fire/journal/notes.txt)"
contains "game, not for the browser: its tests" "$out" "step: cargo test of games/ice-and-fire/ crates/ studio/"
lacks "game, not for the browser: no front end" "$out" "front end"

out="$(plan -- crates/cg-core/src/lib.rs)"
contains "framework: workspace tests" "$out" "step: cargo test --workspace"
contains "framework: every bundle" "$out" "step: bundles of every game"
lacks "framework: no per-game bundles" "$out" "bundles of uttt"

out="$(plan -- studio/server/src/main.rs)"
contains "studio: its tests" "$out" "step: cargo test of studio/ games/*/studio/"
lacks "studio: no bundles" "$out" "bundles"
lacks "studio: no front end" "$out" "front end"

out="$(plan -- studio/web/src/main.ts)"
contains "web: the note" "$out" "note: the front end changed"
lacks "web: no cargo" "$out" "cargo"

out="$(plan -- .github/workflows/ci.yml)"
contains "workflow: everything" "$out" "step: cargo test --workspace"
contains "workflow: actionlint or its note" "$out" "actionlint"

out="$(plan --all --)"
contains "--all: everything" "$out" "step: cargo test --workspace"
contains "--all: licenses" "$out" "deny"
contains "--all: actionlint or its note" "$out" "actionlint"

out="$(plan --)"
contains "no file: docs check" "$out" "step: docs check"
lacks "no file: no cargo" "$out" "cargo"

for args in "--unknown" "--base" "--base --all" "extra"; do
  status=0
  # shellcheck disable=SC2086 # the arguments are split on purpose
  scripts/check.sh --dry-run $args >/dev/null 2>&1 || status=$?
  checks=$((checks + 1))
  [[ $status == 2 ]] || fail "usage '$args': expected status 2, got $status"
done
status=0
scripts/check.sh --dry-run --base no-such-ref >/dev/null 2>&1 || status=$?
checks=$((checks + 1))
[[ $status == 2 ]] || fail "unknown base: expected status 2, got $status"

if ((failures > 0)); then
  echo "check-test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   check-test: $checks checks"
