#!/usr/bin/env bash
# Checks the collection workflow's safety rules (ADR 0058, decision 2) on
# .github/workflows/codingame-collect.yml, after checking the check itself on
# small workflows, and tests scripts/codingame-summary.sh on a stand-in data
# branch. The rules: the triggers are schedule and workflow_dispatch only;
# the job uses the environment codingame, the permission contents: write
# only, a time limit and a concurrency group that does not cancel; the secret
# is named once, in the env of the one step that writes it to a file; the only
# action is a checkout pinned to a commit; nothing prints the token.
# Needs jq and git. Usage: scripts/tests/codingame-collect-test.sh
# shellcheck disable=SC2016 # patterns and sed scripts hold literal $ and ${{ }}
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

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
contains() { # contains <what> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 != *"$3"* ]]; then fail "$1: '$3' not found in: $2"; fi
}
lacks() { # lacks <what> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 == *"$3"* ]]; then fail "$1: '$3' found in: $2"; fi
}

# Prints one line per broken rule of the workflow file.
problems() { # problems <workflow>
  local f="$1" triggers t secret_lines n name_line next_step step_name
  # The keys of the top-level `on:` block.
  triggers="$(awk '/^on:/ { on = 1; next } /^[^[:space:]#]/ { on = 0 } on && /^  [a-z_]+:/ { sub(/^  /, ""); sub(/:.*/, ""); print }' "$f" | sort | tr '\n' ' ')"
  [[ $triggers == "schedule workflow_dispatch " ]] || echo "triggers are '$triggers', not schedule and workflow_dispatch only"
  grep -qE '^on:[[:space:]]*[a-z\[]' "$f" && echo "the on: line holds triggers in one line"
  for t in pull_request pull_request_target workflow_run issue_comment push; do
    [[ " $triggers" != *" $t "* ]] || echo "trigger $t"
  done
  grep -qE '^    environment: codingame[[:space:]]*$' "$f" || echo "no environment: codingame"
  grep -qE '^    timeout-minutes: [0-9]+' "$f" || echo "no timeout-minutes"
  grep -qE '^      cancel-in-progress: false' "$f" || echo "the concurrency group may cancel a run"
  awk '/^permissions:/ { p = 1; next } /^[^[:space:]#]/ { p = 0 } p && NF' "$f" | sed 's/ *#.*//' | grep -vqx '  contents: write' && echo "permissions beyond contents: write"
  # Every action: a checkout pinned to a commit.
  grep -E '^[[:space:]]*-?[[:space:]]*uses:' "$f" | sed -E 's/^[[:space:]]*-?[[:space:]]*uses:[[:space:]]*//; s/[[:space:]]*#.*//' |
    grep -vE '^actions/checkout@[0-9a-f]{40}$' | sed 's/^/action other than a pinned checkout: /' || true
  # The secret: named once, in the step that writes the file.
  secret_lines="$(grep -n 'secrets\.' "$f" || true)"
  n="$(grep -c . <<<"$secret_lines" || true)"
  [[ -z $secret_lines ]] && n=0
  [[ $n -eq 1 ]] || echo "secrets. is used $n times, not once"
  if [[ $n -ge 1 ]]; then
    grep -q 'secrets\.CODINGAME_TOKEN' <<<"$secret_lines" || echo "a secret other than CODINGAME_TOKEN"
    name_line="$(awk -v at="${secret_lines%%:*}" 'NR <= at && /^      - (name|uses|run):/ { l = NR; s = $0 } END { print l ":" s }' "$f")"
    step_name="${name_line#*:}"
    [[ $step_name == *"Write the token to a file"* ]] || echo "the secret is in the step '$step_name'"
    # It must be in the step's env, not in a run line, and the step ends the secret's reach.
    sed -n "${secret_lines%%:*}p" "$f" | grep -qE '^          CODINGAME_TOKEN: \$\{\{ secrets\.CODINGAME_TOKEN \}\}[[:space:]]*$' || echo "the secret is not the step's env CODINGAME_TOKEN"
    next_step="$(awk -v at="${secret_lines%%:*}" 'NR > at && /^      - / { print NR; exit }' "$f")"
    : "${next_step:=999999}"
    awk -v a="${secret_lines%%:*}" -v b="$next_step" 'NR > a && NR < b' "$f" | grep -E 'CODINGAME_TOKEN\b' | grep -vE '^[[:space:]]*(if \[ -z "\$CODINGAME_TOKEN" \]; then|printf .%s. "\$CODINGAME_TOKEN" >"\$RUNNER_TEMP/codingame-token"|::error.*|echo "::error.*)' | grep -vE 'CODINGAME_TOKEN_FILE' | sed 's/^/the token is used by: /' || true
  fi
  # The token appears in no other line than its step's.
  grep -nE 'CODINGAME_TOKEN\b' "$f" | grep -vE 'CODINGAME_TOKEN_FILE|^[0-9]+:[[:space:]]*#' |
    grep -E 'echo|printf|cat |tee|GITHUB_STEP_SUMMARY|GITHUB_OUTPUT|GITHUB_ENV|set -x|::add-mask' |
    grep -vE 'printf .%s. "\$CODINGAME_TOKEN" >"\$RUNNER_TEMP/codingame-token"|echo "::error' | sed 's/^/token printed or exported: /' || true
  grep -nE 'set -[a-z]*x|xtrace|ACTIONS_STEP_DEBUG|ACTIONS_RUNNER_DEBUG' "$f" | grep -vE 'set \+x' | sed 's/^/tracing: /' || true
  # The token file: written under umask 077 and named only by that step.
  grep -q 'umask 077' "$f" || echo "the token file is not written under umask 077"
}

good="$repo_root/.github/workflows/codingame-collect.yml"
bad() { # bad <name> <sed script> <expected fragment>: a mutation of the real workflow must be caught
  sed -E "$2" "$good" >"$tmp_dir/$1.yml"
  contains "mutation $1 is caught" "$(problems "$tmp_dir/$1.yml")" "$3"
}

same "the workflow has no problem" "" "$(problems "$good")"

bad pr 's/^  workflow_dispatch:/  pull_request:\n  workflow_dispatch:/' "pull_request"
bad prt 's/^  workflow_dispatch:/  pull_request_target:\n  workflow_dispatch:/' "pull_request_target"
bad push 's/^  workflow_dispatch:/  push:\n  workflow_dispatch:/' "push"
bad run 's/^  workflow_dispatch:/  workflow_run:\n  workflow_dispatch:/' "workflow_run"
bad comment 's/^  workflow_dispatch:/  issue_comment:\n  workflow_dispatch:/' "issue_comment"
bad env 's/^    environment: codingame/    environment: other/' "no environment: codingame"
bad cancel 's/cancel-in-progress: false/cancel-in-progress: true/' "may cancel"
bad perm 's/^  contents: write.*/  contents: write\n  pull-requests: write/' "permissions beyond"
bad action 's|^(      - uses: actions/checkout)@[0-9a-f]{40}.*|\1@v7|' "action other than a pinned checkout"
bad other 's|^(      - name: Git identity)|      - uses: actions/cache@v6\n\1|' "action other than a pinned checkout"
bad echo 's|^(          umask 077)|          echo "$CODINGAME_TOKEN"\n\1|' "token printed"
bad trace 's|^(          set \+x)|          set -x|' "tracing"
bad second 's|^(          MAX_GAMES: .*)|\1\n          OTHER: ${{ secrets.CODINGAME_TOKEN }}|' "secrets. is used 2 times"
bad where '/^          CODINGAME_TOKEN: .*secrets/d; s|^(          PLAY: .*)|\1\n          CODINGAME_TOKEN: ${{ secrets.CODINGAME_TOKEN }}|' "the secret is in the step"

# Another file than the workflow: the shape of each rule on a minimal one.
cat >"$tmp_dir/one-line.yml" <<'EOF2'
on: push
jobs: {}
EOF2
contains "an on: with a trigger on its line" "$(problems "$tmp_dir/one-line.yml")" "the on: line holds triggers in one line"

# --- scripts/codingame-summary.sh ---------------------------------------------

summary="$repo_root/scripts/codingame-summary.sh"
repo="$tmp_dir/repo"
mkdir -p "$repo/toy/leaderboard"
git -C "$repo" init -q
git -C "$repo" config user.email t@example.com
git -C "$repo" config user.name t
snapshot() { # snapshot <file> <alpha agent> <beta agent> <third nickname>
  echo "{\"rows\":[{\"rank\":1,\"nickname\":\"Alpha\",\"agent_id\":$2},{\"rank\":2,\"nickname\":\"Mine\",\"agent_id\":99,\"me\":true},{\"rank\":3,\"nickname\":\"Beta\",\"agent_id\":$3},{\"rank\":4,\"nickname\":\"$4\",\"agent_id\":44}]}" \
    >"$repo/toy/leaderboard/$1"
}
snapshot 2026-01-01T000000Z.json 11 22 Gamma
snapshot 2026-01-02T060000Z.json 11 22 Gamma
snapshot 2026-01-02T120000Z.json 12 22 Delta
git -C "$repo" add -A
git -C "$repo" commit -q -m data
git -C "$repo" update-ref refs/heads/data HEAD

fetch_log="$tmp_dir/fetch.log"
batch_log="$tmp_dir/batch.log"
printf 'games listed: 3\nfetched: 2\nfailed: 0\n' >"$fetch_log"
cat >"$batch_log" <<'EOF2'
batch 2026-01-02-v1-daily (resumed): release v1, puzzle toy
game 1/4: vs Alpha, seed=1, won, 25 s, quota left 300
game 2/4: vs Alpha, seed=2, lost, 25 s, quota left 299
stopped: 100 test games of the day left to the owner (100 of the 600 in 24 hours remain); the games left stay planned.
totals of batch 2026-01-02-v1-daily:
  Alpha: played 2, won 1, lost 1, drawn 0, failed 0, not played 2
EOF2
out="$(cd "$repo" && CODINGAME_DATA_REF=refs/heads/data "$summary" toy "$fetch_log" "$batch_log")"
contains "summary: games fetched" "$out" "Games fetched (our new ranking games): 2"
contains "summary: games played" "$out" "played in this run: 2"
contains "summary: quota" "$out" "as of the last game: 299"
contains "summary: why it ended" "$out" "Run ended: stopped: 100 test games of the day left to the owner"
contains "summary: totals" "$out" "Alpha: played 2, won 1, lost 1"
contains "summary: the snapshots compared" "$out" "Between \`2026-01-01T000000Z.json\` and \`2026-01-02T120000Z.json\`"
contains "summary: an agent change" "$out" "- Alpha: agent 11 -> 12"
contains "summary: a player enters" "$out" "- Delta entered the top 10"
contains "summary: a player leaves" "$out" "- Gamma left the top 10"
lacks "summary: unchanged players are not listed" "$out" "- Beta"
lacks "summary: we are not a top player" "$out" "Mine"

# Nothing from the day before, and no logs.
out="$(cd "$repo" && git rm -q toy/leaderboard/2026-01-01T000000Z.json && git commit -q -m x && git update-ref refs/heads/data2 HEAD &&
  CODINGAME_DATA_REF=refs/heads/data2 "$summary" toy "$tmp_dir/none1" "$tmp_dir/none2")"
contains "summary: no earlier day" "$out" "No snapshot of an earlier day"
contains "summary: no fetch output" "$out" "unknown (no fetch output)"
contains "summary: no batch output" "$out" "No batch output"
out="$(cd "$repo" && CODINGAME_DATA_REF=refs/heads/missing "$summary" toy "$fetch_log" "$batch_log")"
contains "summary: a missing data branch" "$out" "No snapshot of an earlier day"
same "summary: usage" 2 "$("$summary" >/dev/null 2>&1 && echo 0 || echo $?)"

if ((failures > 0)); then
  echo "codingame-collect-test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   codingame-collect-test: $checks checks"
