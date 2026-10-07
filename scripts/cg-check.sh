#!/usr/bin/env bash
# Checks that paste-ready bot files will be accepted by CodinGame.
#
# Usage: scripts/cg-check.sh FILE.rs...
#
# For each file: the size must fit CodinGame's limit, and the file must
# compile on its own with rustc and the 2021 edition, without Cargo or any
# crate, the way CodinGame compiles it.
#
# Set CG_RUST to require an exact compiler version. CI sets CG_RUST=1.90.0,
# CodinGame's version (see docs/CODINGAME.md).
set -euo pipefail

# Players report a strict 100 kB limit on submitted code.
readonly MAX_BYTES=100000

if [[ $# -eq 0 ]]; then
  echo "usage: $0 FILE.rs..." >&2
  exit 2
fi

version="$(rustc --version)"
echo "compiler: $version"
if [[ -n "${CG_RUST:-}" && "$version" != "rustc ${CG_RUST} "* ]]; then
  echo "error: expected rustc ${CG_RUST} (CodinGame's version), got: $version" >&2
  exit 1
fi

out_dir="$(mktemp -d)"
trap 'rm -rf "$out_dir"' EXIT

status=0
for file in "$@"; do
  bytes=$(wc -c <"$file")
  if ((bytes > MAX_BYTES)); then
    echo "FAIL $file: $bytes bytes, the limit is $MAX_BYTES" >&2
    status=1
    continue
  fi
  if rustc --edition 2021 -C opt-level=3 --crate-type bin -o "$out_dir/bot" "$file"; then
    echo "ok   $file ($bytes bytes)"
  else
    echo "FAIL $file: does not compile on its own" >&2
    status=1
  fi
done
exit "$status"
