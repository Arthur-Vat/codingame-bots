#!/usr/bin/env bash
# Tests scripts/codingame-token.sh offline: a stand-in curl plays the device
# flow (pending, then the token), a stand-in gh records what it is given.
# Needs jq. Usage: scripts/tests/codingame-token-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
script="$PWD/scripts/codingame-token.sh"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}
expect() { # expect <description> <actual> <expected>
  checks=$((checks + 1))
  if [[ $2 != "$3" ]]; then fail "$1: expected '$3', got '$2'"; fi
}
contains() { # contains <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 != *"$3"* ]]; then fail "$1: '$3' not found in: $2"; fi
}
lacks() { # lacks <description> <text> <needle>
  checks=$((checks + 1))
  if [[ $2 == *"$3"* ]]; then fail "$1: '$3' found in: $2"; fi
}

secret="tok-SECRET-1234567890"
stub_dir="$tmp_dir/stub"
mkdir -p "$stub_dir"
cat >"$tmp_dir/curl" <<'STUB'
#!/usr/bin/env bash
echo "$*" >>"$STUB_DIR/curl.log"
case "$*" in
  *auth/device*)
    echo '{"device_code":"dev-code","user_code":"BCDF-GHJK","verification_uri":"https://x/authorize","verification_uri_complete":"https://x/authorize?code=BCDF-GHJK","expires_in":900,"interval":5}' ;;
  *auth/token*)
    n=$(( $(cat "$STUB_DIR/polls" 2>/dev/null || echo 0) + 1 ))
    echo "$n" >"$STUB_DIR/polls"
    if ((n < 2)); then echo '{"error":"authorization_pending"}'; else echo "{\"access_token\":\"$STUB_SECRET\"}"; fi ;;
esac
STUB
cat >"$tmp_dir/gh" <<'STUB'
#!/usr/bin/env bash
echo "$*" >>"$STUB_DIR/gh.log"
case "$1" in
  auth) [[ ${STUB_GH_LOGGED_IN:-yes} == yes ]] ;;
  secret) cat >"$STUB_DIR/gh-stdin" ;;
esac
STUB
# shellcheck disable=SC2016
printf '#!/usr/bin/env bash\necho "$*" >>"$STUB_DIR/sleeps.log"\n' >"$tmp_dir/sleep"
chmod +x "$tmp_dir/curl" "$tmp_dir/gh" "$tmp_dir/sleep"
export STUB_DIR="$stub_dir" STUB_SECRET="$secret"
export CODINGAME_CURL="$tmp_dir/curl" CODINGAME_GH="$tmp_dir/gh" CODINGAME_SLEEP="$tmp_dir/sleep"
export CODINGAME_API="http://stub" CODINGAME_REPO="owner/repo"

reset() { rm -rf "${stub_dir:?}"/*; }
run() { # run <stdin text> <directory>: sets out and rc
  rc=0
  out="$(cd "$2" && printf '%s\n' "$1" | "$script" 2>&1)" || rc=$?
}

# The flow asks for the two scopes, a client name, and polls until approved.
work="$tmp_dir/w1"
mkdir -p "$work"
reset
run "y" "$work"
expect "gh path exit" "$rc" 0
contains "device request scopes" "$(cat "$stub_dir/curl.log")" "scope=challenge:read challenge:play"
contains "device request client name" "$(cat "$stub_dir/curl.log")" "client_name=codingame-bots collection (GitHub)"
contains "link shown" "$out" "https://x/authorize?code=BCDF-GHJK"
contains "code shown" "$out" "BCDF-GHJK"
contains "lifetime hint" "$out" "ONE YEAR"
expect "two polls" "$(cat "$stub_dir/polls")" 2
# With gh and a y: the secret is set from stdin, the token is nowhere else.
contains "gh arguments" "$(grep '^secret' "$stub_dir/gh.log")" "secret set CODINGAME_TOKEN --env codingame --repo owner/repo"
expect "gh stdin is the token" "$(cat "$stub_dir/gh-stdin")" "$secret"
lacks "gh arguments do not hold the token" "$(cat "$stub_dir/gh.log")" "$secret"
lacks "output does not hold the token" "$out" "$secret"
expect "no file left" "$(ls "$work")" ""

# With gh but answering n: a file, mode 600, and no secret set.
work="$tmp_dir/w2"
mkdir -p "$work"
reset
run "n" "$work"
expect "n exit" "$rc" 0
expect "n sets nothing" "$(grep -c '^secret' "$stub_dir/gh.log" || true)" 0
expect "n writes the file" "$(cat "$work/codingame-token.txt")" "$secret"
expect "file mode" "$(stat -c %a "$work/codingame-token.txt")" 600
lacks "n output does not hold the token" "$out" "$secret"
contains "n output tells the paste" "$out" "Settings, Environments, codingame"

# gh not logged in: a file, and gh is not asked to set anything.
work="$tmp_dir/w3"
mkdir -p "$work"
reset
STUB_GH_LOGGED_IN=no run "y" "$work"
expect "not logged in exit" "$rc" 0
expect "not logged in sets nothing" "$(grep -c '^secret' "$stub_dir/gh.log" || true)" 0
expect "not logged in writes the file" "$(stat -c %a "$work/codingame-token.txt")" 600

# No gh at all: a file.
work="$tmp_dir/w4"
mkdir -p "$work"
reset
CODINGAME_GH="$tmp_dir/no-such-gh" run "y" "$work"
expect "no gh exit" "$rc" 0
expect "no gh writes the file" "$(cat "$work/codingame-token.txt")" "$secret"

# A refusal ends the flow.
cat >"$tmp_dir/curl-denied" <<'STUB'
#!/usr/bin/env bash
case "$*" in
  *auth/device*) echo '{"device_code":"d","user_code":"U","verification_uri_complete":"https://x","expires_in":900,"interval":5}' ;;
  *) echo '{"error":"access_denied"}' ;;
esac
STUB
chmod +x "$tmp_dir/curl-denied"
work="$tmp_dir/w5"
mkdir -p "$work"
reset
CODINGAME_CURL="$tmp_dir/curl-denied" run "y" "$work"
expect "denied exit" "$rc" 1
contains "denied message" "$out" "access_denied"
expect "denied writes nothing" "$(ls "$work")" ""

if ((failures > 0)); then
  echo "$failures of $checks codingame-token checks failed" >&2
  exit 1
fi
echo "ok   codingame-token.sh: $checks checks"
