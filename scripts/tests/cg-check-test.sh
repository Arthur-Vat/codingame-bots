#!/usr/bin/env bash
# Tests scripts/cg-check.sh's size check: CodinGame counts characters, as
# UTF-16 units (ADR 0037), so the limit is 100,000 units, whatever the bytes.
# Needs rustc and iconv.
# Usage: scripts/tests/cg-check-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
script=scripts/cg-check.sh
# This tests the size check, not the compiler's version: CI sets CG_RUST for
# the bots' jobs, and any rustc compiles these files.
unset CG_RUST CG_BIN_DIR
dir="$(mktemp -d)"
trap 'rm -rf "$dir"' EXIT

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# bot <file> <units> <character> <width>: a program that compiles, padded
# with a comment of <character> to exactly <units> UTF-16 units. The character
# is given as printf escapes, and <width> is its length in UTF-16 units.
bot() {
  local file="$1" units="$2" char="$3" width="$4" head='fn main() {}'$'\n''// '
  local count=$(((units - ${#head} - 1) / width))
  {
    printf '%s' "$head"
    for ((i = 0; i < count; i++)); do printf '%b' "$char"; done
    printf '%*s\n' $((units - ${#head} - 1 - count * width)) ''
  } >"$file"
}

# check <expected status> <text the output must contain> <file> [option]
check() {
  local want="$1" text="$2" file="$3" option="${4:-}" output status=0
  checks=$((checks + 1))
  if [[ -n $option ]]; then
    output="$("$script" "$option" "$file" 2>&1)" || status=$?
  else
    output="$("$script" "$file" 2>&1)" || status=$?
  fi
  if ((status != want)); then
    fail "status $status, wanted $want, for $(basename "$file"): $output"
  elif [[ $output != *"$text"* ]]; then
    fail "the output for $(basename "$file") lacks '$text': $output"
  fi
}

bot "$dir/ascii-limit.rs" 100000 'x' 1
check 0 "100000 characters, 100000 bytes" "$dir/ascii-limit.rs"

bot "$dir/ascii-over.rs" 100001 'x' 1
check 1 "100001 characters (UTF-16 units), the limit is 100000" "$dir/ascii-over.rs"

# U+4E00, 3 bytes in UTF-8 and one UTF-16 unit: about 300,000 bytes pass.
bot "$dir/bmp-limit.rs" 100000 '\xe4\xb8\x80' 1
check 0 "100000 characters, 299966 bytes" "$dir/bmp-limit.rs"

bot "$dir/bmp-over.rs" 100001 '\xe4\xb8\x80' 1
check 1 "100001 characters" "$dir/bmp-over.rs"

# U+1F600, above U+FFFF: two UTF-16 units each, though one character.
bot "$dir/astral-limit.rs" 100000 '\xf0\x9f\x98\x80' 2
check 0 "100000 characters" "$dir/astral-limit.rs"

bot "$dir/astral-over.rs" 100002 '\xf0\x9f\x98\x80' 2
check 1 "100002 characters" "$dir/astral-over.rs"

printf 'fn main() {}\n// \xff\n' >"$dir/invalid.rs"
check 1 "not valid UTF-8" "$dir/invalid.rs"

printf 'fn main() {\n' >"$dir/broken.rs"
check 1 "does not compile on its own" "$dir/broken.rs"

# --any-size reports the size of a readable copy but does not enforce it.
check 0 "100001 characters, over the limit of 100000; not enforced" "$dir/ascii-over.rs" --any-size
check 0 "100002 characters, over the limit" "$dir/astral-over.rs" --any-size
check 0 "100000 characters, 100000 bytes" "$dir/ascii-limit.rs" --any-size
check 1 "does not compile on its own" "$dir/broken.rs" --any-size
check 1 "not valid UTF-8" "$dir/invalid.rs" --any-size

if ((failures > 0)); then
  echo "cg-check test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   cg-check test: $checks checks"
