#!/usr/bin/env bash
# Tests scripts/lib/rustc-cache.sh with a stand-in rustc that counts its
# calls: a second run copies instead of compiling; the source's bytes and
# name, rustc's arguments and rustc's version each give a new entry; a failed
# compile stores nothing; pruning keeps only this run's entries; an empty
# folder turns the cache off. Needs sha256sum.
# Usage: scripts/tests/rustc-cache-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
# shellcheck source=scripts/lib/rustc-cache.sh
source scripts/lib/rustc-cache.sh

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

# The stand-in writes its arguments and the source into -o's file, fails on
# a source holding "error", and answers -vV with $FAKE_RUSTC_VERSION.
bin="$tmp_dir/bin"
mkdir -p "$bin"
cat >"$bin/rustc" <<'EOF'
#!/usr/bin/env bash
if [[ $1 == -vV ]]; then
  echo "rustc $FAKE_RUSTC_VERSION"
  exit 0
fi
echo x >>"$FAKE_RUSTC_CALLS"
out= source= args=()
while (($# > 0)); do
  case $1 in
    -o) out=$2 && shift ;;
    *.rs) source=$1 ;;
    *) args+=("$1") ;;
  esac
  shift
done
grep -q error "$source" && exit 1
{ echo "${args[*]}"; cat "$source"; } >"$out"
EOF
chmod +x "$bin/rustc"
PATH="$bin:$PATH"
export FAKE_RUSTC_CALLS="$tmp_dir/calls" FAKE_RUSTC_VERSION=1.0
calls() { if [[ -f $FAKE_RUSTC_CALLS ]]; then wc -l <"$FAKE_RUSTC_CALLS" | tr -d ' '; else echo 0; fi; }
entries() { find "$cache" -type f | wc -l | tr -d ' '; }

cache="$tmp_dir/cache"
src="$tmp_dir/src"
out="$tmp_dir/out"
mkdir -p "$src" "$out"
echo 'fn main() {}' >"$src/a.rs"
echo 'fn main() { 1; }' >"$src/b.rs"

rustc_cache_init "$cache"
rustc_cached "$out/a.wasm" "$src/a.rs" -C opt-level=3
rustc_cached "$out/b.wasm" "$src/b.rs" -C opt-level=3
rustc_cache_prune
expect "first run compiles both" "$(calls)" 2
expect "first run hits" "$rustc_cache_hits" 0
expect "first run entries" "$(entries)" 2
first_a="$(cat "$out/a.wasm")"

rm -f "$out"/*
rustc_cache_init "$cache"
rustc_cached "$out/a.wasm" "$src/a.rs" -C opt-level=3
rustc_cached "$out/b.wasm" "$src/b.rs" -C opt-level=3
expect "second run compiles nothing" "$(calls)" 2
expect "second run hits" "$rustc_cache_hits" 2
expect "a copy is the compiled file" "$(cat "$out/a.wasm")" "$first_a"

rustc_cache_init "$cache"
rustc_cached "$out/a.wasm" "$src/a.rs" -C opt-level=2
expect "other arguments compile" "$(calls)" 3
echo 'fn main() { 2; }' >"$src/a.rs"
rustc_cached "$out/a.wasm" "$src/a.rs" -C opt-level=3
expect "other bytes compile" "$(calls)" 4
cp "$src/b.rs" "$src/c.rs"
rustc_cached "$out/c.wasm" "$src/c.rs" -C opt-level=3
expect "another name compiles" "$(calls)" 5
FAKE_RUSTC_VERSION=2.0
rustc_cache_init "$cache"
rustc_cached "$out/b.wasm" "$src/b.rs" -C opt-level=3
expect "another rustc compiles" "$(calls)" 6
FAKE_RUSTC_VERSION=1.0

echo 'error' >"$src/bad.rs"
rustc_cache_init "$cache"
before="$(entries)"
status=0
rustc_cached "$out/bad.wasm" "$src/bad.rs" -C opt-level=3 || status=$?
expect "a failed compile fails" "$status" 1
expect "a failed compile stores nothing" "$(entries)" "$before"

rustc_cache_init "$cache"
rustc_cached "$out/b.wasm" "$src/b.rs" -C opt-level=3
touch "$cache/left.part"
rustc_cache_prune
expect "pruning keeps the used entry" "$(entries)" 1
expect "the kept entry is a hit" "$rustc_cache_hits" 1

rustc_cache_init ""
before="$(calls)"
rustc_cached "$out/b.wasm" "$src/b.rs" -C opt-level=3
rustc_cached "$out/b.wasm" "$src/b.rs" -C opt-level=3
expect "no cache compiles each time" "$(calls)" $((before + 2))
rustc_cache_prune
expect "no cache prunes nothing" "$(entries)" 1

if ((failures > 0)); then
  echo "rustc-cache-test: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   rustc-cache-test: $checks checks"
