#!/usr/bin/env bash
# Checks that every workflow job that runs on a runner sets timeout-minutes,
# so that a hung job stops long before GitHub's 360 minutes. A job that calls
# a reusable workflow (`uses:`) cannot set it: its called jobs do.
# First checks the check itself on small workflows, then .github/workflows/.
# Usage: scripts/tests/workflow-timeouts-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

# Prints "<file>: <job>" for each job with neither timeout-minutes nor uses,
# and "<file>: runs-on outside a job it reads" when a runs-on line is not at
# the level of a job it found (another indentation), so that the check cannot
# pass by reading nothing.
no_timeout() { # no_timeout <workflow>...
  awk '
    function finish() {
      if (job != "" && !limited) { print file ": " job }
      job = ""
    }
    function end_file() {
      finish()
      if (file != "" && runs_on != job_runs_on) { print file ": runs-on outside a job it reads" }
      runs_on = 0; job_runs_on = 0
    }
    FNR == 1 { end_file(); in_jobs = 0; file = FILENAME }
    /^[[:space:]]+runs-on:/ { runs_on++ }
    /^jobs:/ { in_jobs = 1; next }
    !in_jobs { next }
    /^[^[:space:]#]/ { finish(); in_jobs = 0; next }
    /^  [A-Za-z0-9_-]+:[[:space:]]*(#.*)?$/ {
      finish()
      job = $1; sub(/:$/, "", job); limited = 0
      next
    }
    /^    runs-on:/ && job != "" { job_runs_on++ }
    /^    (timeout-minutes|uses):/ { limited = 1 }
    END { end_file() }
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

cat >"$tmp_dir/forms.yml" <<'EOF2'
on: push
jobs:
  limited:
    runs-on: ubuntu-latest
    timeout-minutes: 5
    steps:
      - run: true
  open:
    runs-on: ubuntu-latest
    steps:
      - run: true
        timeout-minutes: 5
  called:
    uses: ./.github/workflows/other.yml
  # A comment between jobs.
  last:
    runs-on: ubuntu-latest
    steps:
      - run: true
EOF2
same "a step's timeout does not count; a call needs none" "$tmp_dir/forms.yml: open
$tmp_dir/forms.yml: last" "$(no_timeout "$tmp_dir/forms.yml")"

cat >"$tmp_dir/after.yml" <<'EOF2'
jobs:
  only:
    runs-on: ubuntu-latest
    steps:
      - run: true
defaults:
  run:
    shell: bash
EOF2
cat >"$tmp_dir/odd.yml" <<'EOF2'
jobs:
  first:
    runs-on: ubuntu-latest
    timeout-minutes: 5
  second: # a comment on the job's line
    runs-on: ubuntu-latest
EOF2
same "a job line with a comment" "$tmp_dir/odd.yml: second" "$(no_timeout "$tmp_dir/odd.yml")"

cat >"$tmp_dir/deep.yml" <<'EOF2'
jobs:
    only:
        runs-on: ubuntu-latest
EOF2
same "jobs indented otherwise are not read silently" "$tmp_dir/deep.yml: runs-on outside a job it reads" "$(no_timeout "$tmp_dir/deep.yml")"

same "a key after jobs: ends the last job" "$tmp_dir/after.yml: only" "$(no_timeout "$tmp_dir/after.yml")"

same "several files: each job under its own file" "$tmp_dir/after.yml: only
$tmp_dir/forms.yml: open
$tmp_dir/forms.yml: last" "$(no_timeout "$tmp_dir/after.yml" "$tmp_dir/forms.yml")"

found="$(no_timeout .github/workflows/*.yml)"
same "the workflows (add timeout-minutes)" "" "$found"

if ((failures > 0)); then
  echo "workflow-timeouts-test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   workflow-timeouts-test: $checks checks"
