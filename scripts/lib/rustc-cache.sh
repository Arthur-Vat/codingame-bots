# shellcheck shell=bash
# Shared by scripts/build-studio-wasm.sh. Compiles one Rust file with rustc
# and keeps the result in a cache folder, so that a later run reuses it when
# nothing that decides the output changed: the source's name and bytes,
# rustc's arguments and rustc's version (`rustc -vV`). Released files never change, so
# CI compiles each release once per Rust version.
#
# rustc_cache_init DIR      starts a run; an empty DIR turns the cache off
# rustc_cached OUT SOURCE [ARGS...]
#                           rustc ARGS -o OUT SOURCE, or a copy from the cache
# rustc_cache_prune         deletes the entries this run did not use, so the
#                           folder holds only what the current files need
#
# rustc_cache_hits counts the copies. Needs sha256sum when the cache is on.

rustc_cache_dir=
rustc_cache_version=
rustc_cache_used=()
rustc_cache_hits=0

rustc_cache_init() {
  rustc_cache_dir=$1
  rustc_cache_used=()
  rustc_cache_hits=0
  [[ -n $rustc_cache_dir ]] || return 0
  mkdir -p "$rustc_cache_dir"
  rustc_cache_version="$(rustc -vV)"
}

rustc_cached() {
  local out=$1 source=$2
  shift 2
  if [[ -z $rustc_cache_dir ]]; then
    rustc "$@" -o "$out" "$source"
    return
  fi
  local key entry
  # One argument a line, then a line no argument holds, then the source. The
  # file's name counts too: rustc takes the crate's name from it.
  key="$({
    printf '%s\n' "$rustc_cache_version" "$(basename "$source")" "$@"
    printf '\0\n'
    cat "$source"
  } | sha256sum)"
  key=${key%% *}
  entry="$rustc_cache_dir/$key"
  rustc_cache_used+=("$key")
  if [[ -f $entry ]]; then
    cp "$entry" "$out"
    rustc_cache_hits=$((rustc_cache_hits + 1))
    return
  fi
  rustc "$@" -o "$out" "$source" || return
  # Written under another name first: a run cut short leaves no half file
  # under the entry's name.
  cp "$out" "$entry.part"
  mv "$entry.part" "$entry"
}

rustc_cache_prune() {
  [[ -n $rustc_cache_dir ]] || return 0
  local file name used keep
  for file in "$rustc_cache_dir"/*; do
    [[ -e $file ]] || continue
    name="$(basename "$file")"
    keep=0
    for used in ${rustc_cache_used[@]+"${rustc_cache_used[@]}"}; do
      if [[ $used == "$name" ]]; then
        keep=1
        break
      fi
    done
    ((keep == 1)) || rm -f "$file"
  done
}
