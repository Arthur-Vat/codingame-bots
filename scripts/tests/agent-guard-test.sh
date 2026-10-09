#!/usr/bin/env bash
# Tests scripts/agent-guard.sh: the allowed agents pass, everything else is
# refused with status 2 and a reason, and bad input is refused too. Needs jq.
# Usage: scripts/tests/agent-guard-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
script=scripts/agent-guard.sh

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# check <expected status> <text the output must contain, or ""> <stdin>
check() {
  local want="$1" text="$2" input="$3" output status=0
  checks=$((checks + 1))
  output="$("$script" <<<"$input" 2>&1)" || status=$?
  if ((status != want)); then
    fail "status $status, wanted $want: $input"
  elif [[ $output != *"$text"* ]]; then
    fail "the output for $input lacks '$text': $output"
  fi
}

for name in implementer pr-reviewer rules-reviewer Explore Plan claude-code-guide; do
  check 0 "" "{\"tool_name\":\"Agent\",\"tool_input\":{\"subagent_type\":\"$name\",\"prompt\":\"x\"}}"
done
check 2 "not on the list" '{"tool_input":{"subagent_type":"general-purpose"}}'
check 2 "not on the list" '{"tool_input":{"subagent_type":"claude"}}'
check 2 "not on the list" '{"tool_input":{"subagent_type":"statusline-setup"}}'
check 2 "not on the list" '{"tool_input":{"subagent_type":"explore"}}'
check 2 "no subagent_type" '{"tool_input":{"prompt":"x"}}'
check 2 "no subagent_type" '{"tool_input":{"subagent_type":""}}'
check 2 "no subagent_type" '{}'
check 2 "unreadable" 'not json'
check 2 "unreadable" '{"tool_input":"implementer"}'
check 2 "not on the list" '{"tool_input":{"subagent_type":"implementer\n"}}'
check 2 "not on the list" '{"tool_input":{"subagent_type":" implementer"}}'
check 2 "not on the list" '{"tool_input":{"subagent_type":["implementer"]}}'

if ((failures > 0)); then
  echo "$failures of $checks checks failed" >&2
  exit 1
fi
echo "agent-guard: $checks checks passed"
