#!/usr/bin/env bash
# Checks that paste-ready bot files will be accepted by CodinGame.
#
# Usage: scripts/cg-check.sh FILE.rs...
#
# For each file: it must be valid UTF-8, its size must fit CodinGame's limit,
# and it must compile on its own with rustc and the 2021 edition, without
# Cargo or any crate, the way CodinGame compiles it.
#
# CodinGame limits code size in characters, not bytes (docs/CODINGAME.md). The
# size is counted in UTF-16 units, as a browser counts a string's length: a
# character above U+FFFF counts as two (ADR 0037).
#
# Set CG_RUST to require an exact compiler version. CI sets CG_RUST=1.90.0,
# CodinGame's version (see docs/CODINGAME.md).
# Set CG_BIN_DIR to keep the compiled bots there, named after each file.
set -euo pipefail

readonly MAX_UNITS=100000

if [[ $# -eq 0 ]]; then
  echo "usage: $0 FILE.rs..." >&2
  exit 2
fi
if ! command -v iconv >/dev/null; then
  echo "error: iconv is needed to count characters" >&2
  exit 1
fi

version="$(rustc --version)"
echo "compiler: $version"
if [[ -n "${CG_RUST:-}" && "$version" != "rustc ${CG_RUST} "* ]]; then
  echo "error: expected rustc ${CG_RUST} (CodinGame's version), got: $version" >&2
  exit 1
fi

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
out_dir="${CG_BIN_DIR:-$tmp_dir}"
mkdir -p "$out_dir"

status=0
for file in "$@"; do
  bytes=$(wc -c <"$file")
  # iconv refuses invalid UTF-8 (pipefail passes its failure on); UTF-16LE
  # takes 2 bytes per unit and has no byte order mark.
  if ! utf16_bytes=$(iconv -f UTF-8 -t UTF-16LE "$file" 2>/dev/null | wc -c); then
    echo "FAIL $file: not valid UTF-8" >&2
    status=1
    continue
  fi
  units=$((utf16_bytes / 2))
  if ((units > MAX_UNITS)); then
    echo "FAIL $file: $units characters (UTF-16 units), the limit is $MAX_UNITS" >&2
    status=1
    continue
  fi
  if rustc --edition 2021 -C opt-level=3 --crate-type bin \
    -o "$out_dir/$(basename "$file" .rs)" "$file"; then
    echo "ok   $file ($units characters, $bytes bytes)"
  else
    echo "FAIL $file: does not compile on its own" >&2
    status=1
  fi
done
exit "$status"
