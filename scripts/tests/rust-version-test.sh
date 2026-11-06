#!/usr/bin/env bash
# Checks that every place naming CodinGame's Rust names the same version
# (ADR 0055, decision 3): rust-toolchain.toml's channel, CG_RUST in each
# workflow, the toolchain each workflow installs, Cargo.toml's rust-version
# (the same major and minor), the bundler's VERSION, the header of its std
# names list and docs/CODINGAME.md. Also that every use of
# dtolnay/rust-toolchain is pinned to one commit, not a branch.
# Usage: scripts/tests/rust-version-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."

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

version="$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' rust-toolchain.toml)"
checks=$((checks + 1))
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  fail "rust-toolchain.toml: expected a channel like 1.90.0, got '$version'"
fi

same "Cargo.toml rust-version" "${version%.*}" \
  "$(sed -n 's/^rust-version = "\([^"]*\)"$/\1/p' Cargo.toml)"
same "cg-bundler VERSION" "$version" \
  "$(sed -n 's/^pub const VERSION: &str = "\([^"]*\)";$/\1/p' crates/cg-bundler/src/prune/rustc.rs)"
same "std names list header" "$version" \
  "$(sed -n '1s/^# The names that the standard library of Rust \([0-9.]*\) .*/\1/p' crates/cg-bundler/src/rename/std-names.txt)"
same "docs/CODINGAME.md" "$version" \
  "$(sed -n 's/^| Rust | rustc \([0-9.]*\)\. .*/\1/p' docs/CODINGAME.md)"

workflows=0
for file in .github/workflows/*.yml; do
  if grep -q 'dtolnay/rust-toolchain@' "$file"; then
    workflows=$((workflows + 1))
    same "$file: CG_RUST" "$version" "$(sed -n 's/^  CG_RUST: "\([^"]*\)"$/\1/p' "$file")"
    # The toolchain input of each use of the action. The ${{ }} is the
    # workflow's text, not a shell expansion.
    # shellcheck disable=SC2016
    same "$file: toolchains other than CG_RUST" "" \
      "$(grep -A3 'dtolnay/rust-toolchain@' "$file" | grep 'toolchain:' | grep -v 'toolchain: ${{ env.CG_RUST }}$' || true)"
  else
    same "$file: CG_RUST without the action" "" "$(grep '^  CG_RUST:' "$file" || true)"
  fi
done
checks=$((checks + 1))
if ((workflows == 0)); then fail "no workflow installs Rust with dtolnay/rust-toolchain"; fi

pins="$(grep -ho 'dtolnay/rust-toolchain@[^ ]*' .github/workflows/*.yml | sort -u)"
same "one pin of dtolnay/rust-toolchain" 1 "$(wc -l <<<"$pins")"
checks=$((checks + 1))
if [[ ! $pins =~ ^dtolnay/rust-toolchain@[0-9a-f]{40}$ ]]; then
  fail "dtolnay/rust-toolchain: expected a commit SHA, got '$pins'"
fi

if ((failures > 0)); then
  echo "rust-version-test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   rust-version-test: $checks checks, Rust $version in $workflows workflows"
