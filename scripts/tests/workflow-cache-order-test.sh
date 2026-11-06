#!/usr/bin/env bash
# Checks that no workflow job puts files in target/ before its
# Swatinem/rust-cache step: on a partial match the action cleans target/,
# files included, which lost the plan of the 2026-11-06 Ice and Fire night.
# A job whose cache step runs only when its restore found nothing
# (`cache-matched-key == ''` in the step's `if:`) is allowed. First checks
# the check itself on two small workflows, then checks .github/workflows/.
# Usage: scripts/tests/workflow-cache-order-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

# Prints "<file>:<line>: <job>" for each line that names target/ before the
# job's cache step. Comments do not count; `target-cpu` and the like do not
# match.
early_target() { # early_target <workflow>...
  awk '
    FNR == 1 { in_jobs = 0; job = ""; hits = "" }
    /^jobs:/ { in_jobs = 1; next }
    in_jobs && /^  [A-Za-z0-9_-]+:[[:space:]]*$/ {
      job = $1; sub(/:$/, "", job); seen_cache = 0; hits = ""; in_cache = 0
      next
    }
    job == "" || seen_cache { next }
    /^[[:space:]]*#/ { next }
    /^      - / {
      if (in_cache) { finish() }
      if ($0 ~ /uses: Swatinem\/rust-cache@/) { in_cache = 1; allowed = 0; next }
    }
    in_cache && /^        if:.*cache-matched-key == '\'''\''/ { allowed = 1 }
    !in_cache && /target\// { hits = hits FILENAME ":" FNR ": " job "\n" }
    function finish() {
      if (!allowed) { printf "%s", hits }
      seen_cache = 1; in_cache = 0
    }
    END { if (in_cache) { finish() } }
  ' "$@"
}

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}
same() { # same <what> <expected> <actual>
  checks=$((checks + 1))
  if [[ $2 != "$3" ]]; then fail "$1: expected '$2', got '$3'"; fi
}

cat >"$tmp_dir/bad.yml" <<'EOF'
jobs:
  early:
    steps:
      - uses: actions/checkout@v7
      # A comment about target/ does not count.
      - uses: actions/download-artifact@v8
        with:
          path: target/night
      - uses: Swatinem/rust-cache@v2
      - run: cat target/night/plan.jsonl
  late:
    env:
      RUSTFLAGS: -C target-cpu=native
    steps:
      - uses: Swatinem/rust-cache@v2
      - uses: actions/download-artifact@v8
        with:
          path: target/night
EOF
same "a download into target/ before the cache" "$tmp_dir/bad.yml:8: early" "$(early_target "$tmp_dir/bad.yml")"

cat >"$tmp_dir/allowed.yml" <<'EOF'
jobs:
  reuse:
    steps:
      - name: Reuse the latest result
        id: cache
        uses: actions/cache/restore@v6
        with:
          path: target/sprt-result
      - uses: Swatinem/rust-cache@v2
        if: steps.find.outputs.candidates != '' && steps.cache.outputs.cache-matched-key == ''
      - run: cat target/sprt-result/status
EOF
same "a cache step that runs only when the restore found nothing" "" "$(early_target "$tmp_dir/allowed.yml")"

found="$(early_target .github/workflows/*.yml)"
same "the workflows (put the cache step first)" "" "$found"

if ((failures > 0)); then
  echo "workflow-cache-order-test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "workflow-cache-order-test: $checks checks passed"
