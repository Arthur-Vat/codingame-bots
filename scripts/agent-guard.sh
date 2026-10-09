#!/usr/bin/env bash
# PreToolUse hook of Claude's sessions (.claude/settings.json) for the Agent
# tool. Lets a session start only the project's agents (implementer,
# pr-reviewer, rules-reviewer) and the read-only built-in ones (Explore, Plan,
# claude-code-guide). Any other agent, including the default one that an
# omitted subagent_type selects, carries every tool of the session, and their
# descriptions cost about 40,000 tokens of context on each of its steps.
#
# Reads the hook's JSON from stdin. Exits 0 to allow; exits 2 with the reason
# on stderr to refuse, which the session sees. It refuses too when it cannot
# tell (no jq, unreadable input): a guard that fails open guards nothing.
#
# Usage: scripts/agent-guard.sh < hook-input.json
set -euo pipefail

allowed=(implementer pr-reviewer rules-reviewer Explore Plan claude-code-guide)

refuse() {
  echo "Agent refused: $1. Allowed agents: ${allowed[*]}. Do the work in the main session or pick one of these (docs/ROADMAP.md, phase 5)." >&2
  exit 2
}

command -v jq >/dev/null || refuse "jq is missing, so the agent type cannot be read"
input="$(cat)"
type="$(jq -r '.tool_input.subagent_type // ""' <<<"$input" 2>/dev/null)" || refuse "the hook input is not JSON"
[[ -n $type ]] || refuse "no subagent_type, which selects the general-purpose agent with every tool"
for name in "${allowed[@]}"; do
  [[ $type == "$name" ]] && exit 0
done
refuse "'$type' is not on the list"
