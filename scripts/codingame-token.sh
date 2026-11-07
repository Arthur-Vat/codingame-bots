#!/usr/bin/env bash
# For the owner, on his own computer (ADR 0058): makes the CodinGame token of
# the collection workflow with the OAuth device flow and stores it as the
# secret CODINGAME_TOKEN of the GitHub environment `codingame`.
#
# Usage: scripts/codingame-token.sh
#
# It starts the flow with the scopes `challenge:read challenge:play` (which
# cannot submit a bot), prints the approval link and code, and waits while
# you approve on the page: choose a lifetime of ONE YEAR there. Then:
#   - with `gh` installed and logged in, and after you answer y to a question,
#     it sets the secret with `gh secret set` (the token goes through stdin);
#   - else it writes the token to the file codingame-token.txt (mode 600) in
#     the current directory, and tells you to paste it into GitHub (Settings,
#     Environments, codingame, Add secret, name CODINGAME_TOKEN) and delete
#     the file.
# The token is never printed. Nothing is written to GitHub without your y.
#
# Environment (for tests): CODINGAME_API (https://www.codingame.com/api/v1),
# CODINGAME_CURL, CODINGAME_GH (programs), CODINGAME_SLEEP, CODINGAME_REPO
# (Arthur-Vat/codingame-bots). Needs bash, curl and jq.
set -euo pipefail

api="${CODINGAME_API:-https://www.codingame.com/api/v1}"
curl_bin="${CODINGAME_CURL:-curl}"
gh_bin="${CODINGAME_GH:-gh}"
sleep_bin="${CODINGAME_SLEEP:-sleep}"
repo="${CODINGAME_REPO:-Arthur-Vat/codingame-bots}"
out_file="codingame-token.txt"

die() {
  echo "codingame-token.sh: $*" >&2
  exit 1
}
for tool in "$curl_bin" jq; do
  command -v "$tool" >/dev/null || die "$tool is required"
done

device="$("$curl_bin" -sS -X POST "$api/auth/device" \
  -d client_name="codingame-bots collection (GitHub)" \
  -d scope="challenge:read challenge:play")" || die "cannot reach CodinGame"
jq -e '.device_code and .user_code and .verification_uri_complete' <<<"$device" >/dev/null 2>&1 ||
  die "CodinGame did not start the device flow: $(jq -r '.hint // .message // .error // "unexpected answer"' <<<"$device" 2>/dev/null || echo "unexpected answer")"
device_code="$(jq -r '.device_code' <<<"$device")"
interval="$(jq -r '.interval // 5' <<<"$device")"
expires="$(jq -r '.expires_in // 900' <<<"$device")"

echo "Open this link and approve the request:"
echo "  $(jq -r '.verification_uri_complete' <<<"$device")"
echo "Code: $(jq -r '.user_code' <<<"$device")   (the link lasts $((expires / 60)) minutes)"
echo
echo "On that page choose a lifetime of ONE YEAR for the token."
echo "Waiting for your approval..."

token=""
waited=0
while ((waited <= expires)); do
  "$sleep_bin" "$interval"
  waited=$((waited + interval))
  reply="$("$curl_bin" -sS -X POST "$api/auth/token" \
    -d grant_type=urn:ietf:params:oauth:grant-type:device_code \
    -d device_code="$device_code")" || die "cannot reach CodinGame"
  token="$(jq -r '.access_token // empty' <<<"$reply" 2>/dev/null || true)"
  [[ -z $token ]] || break
  case "$(jq -r '.error // empty' <<<"$reply" 2>/dev/null || true)" in
    authorization_pending | "") ;;
    slow_down) interval=$((interval + 5)) ;;
    *) die "the flow ended: $(jq -r '.error' <<<"$reply")" ;;
  esac
done
[[ -n $token ]] || die "the link expired before you approved it; run the script again"
echo "Approved."

if command -v "$gh_bin" >/dev/null && "$gh_bin" auth status >/dev/null 2>&1; then
  read -r -p "Set the secret CODINGAME_TOKEN of the environment codingame in $repo with gh? [y/N] " answer || answer=""
  if [[ $answer == y || $answer == Y ]]; then
    printf '%s' "$token" | "$gh_bin" secret set CODINGAME_TOKEN --env codingame --repo "$repo" >/dev/null ||
      die "gh could not set the secret (does the environment codingame exist?)"
    echo "The secret CODINGAME_TOKEN is set. The token was not saved anywhere else."
    exit 0
  fi
  echo "Not set in GitHub."
fi
(
  umask 077
  printf '%s' "$token" >"$out_file"
)
chmod 600 "$out_file"
echo "The token is in $PWD/$out_file (mode 600)."
echo "Paste it into GitHub: Settings, Environments, codingame, Add secret, name CODINGAME_TOKEN."
echo "Then delete the file: rm $out_file"
