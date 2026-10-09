#!/usr/bin/env bash
# SessionStart hook of Claude's cloud sessions (.claude/settings.json).
# Installs the tools that CI uses and the session's image lacks, so that a
# session runs the same checks as CI before it pushes. Today: shellcheck, at
# the version of CI's ubuntu-latest runner (0.9.0), from PyPI's shellcheck-py
# into a virtual environment under ~/.cache, linked onto the session's PATH.
#
# Does nothing outside cloud sessions (CLAUDE_CODE_REMOTE is not "true").
# Never blocks a session: if the install fails, it says so in one line and
# exits 0, and CI still runs shellcheck.
#
# Usage: CLAUDE_CODE_REMOTE=true CLAUDE_ENV_FILE=<file> scripts/session-setup.sh
set -euo pipefail

[[ ${CLAUDE_CODE_REMOTE:-} == true ]] || exit 0

# Only a shims directory goes on PATH, so that the venv's python3 and pip do
# not shadow the session's.
tools="$HOME/.cache/cg-tools"
shims="$tools/shims"
if ! command -v shellcheck >/dev/null && [[ ! -x $shims/shellcheck ]]; then
  if ! { python3 -m venv "$tools/venv" && "$tools/venv/bin/pip" install -q shellcheck-py==0.9.0.6 &&
    mkdir -p "$shims" && ln -sf "$tools/venv/bin/shellcheck" "$shims/shellcheck"; } >/dev/null 2>&1; then
    echo "session-setup: shellcheck could not be installed; CI still runs it"
    exit 0
  fi
fi
# The hook also runs on resume, clear and compact: add the line once.
line="export PATH=\"$shims:\$PATH\""
if [[ -x $shims/shellcheck && -n ${CLAUDE_ENV_FILE:-} ]] && ! grep -qxF "$line" "$CLAUDE_ENV_FILE" 2>/dev/null; then
  echo "$line" >>"$CLAUDE_ENV_FILE"
fi
