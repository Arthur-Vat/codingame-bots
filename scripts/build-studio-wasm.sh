#!/usr/bin/env bash
# Builds what the studio's browser engine needs (ADR 0030), for the target
# wasm32-wasip1:
#
# - OUT/rules.wasm: the rules program, the crate studio-rules;
# - OUT/<game>/<release>.wasm: every release, games/<game>/releases/<release>.rs,
#   compiled unchanged with rustc (edition 2021, opt-level 3, symbols
#   stripped);
# - OUT/<game>/analysis.wasm: the game's analysis program, the crate
#   <game>-analysis (ADR 0032), for each game with a games/<game>/analysis
#   folder, built with cargo like the rules program (symbols stripped);
# - OUT/manifest.json:
#   {"games":[{"id":"uttt","releases":["uttt-v010",...],"analysis":true}]},
#   the releases that compiled, newest first (the order of the server's
#   list of releases), and whether the game has an analysis program.
#
# A release that does not compile to WebAssembly is skipped with a warning on
# stderr and left out of the manifest: it is not offered in the browser. The
# release files are never changed. An analysis program that does not build is
# an error, not skipped.
#
# Usage: scripts/build-studio-wasm.sh [OUT] [--strict]
#   OUT       the output folder, relative to the current folder (default:
#             studio/web/public/wasm in the repository, not committed)
#   --strict  also fail when a release could not be built, so that nothing is
#             skipped without notice (used by CI)
set -euo pipefail

caller=$PWD
out=studio/web/public/wasm
strict=0
positional=0
for arg in "$@"; do
  case $arg in
    --strict) strict=1 ;;
    -*)
      echo "usage: $0 [OUT] [--strict]" >&2
      exit 2
      ;;
    *)
      positional=$((positional + 1))
      out=$arg
      ;;
  esac
done
if ((positional > 1)); then
  echo "usage: $0 [OUT] [--strict]" >&2
  exit 2
fi
if ((positional == 1)) && [[ $out != /* ]]; then
  out=$caller/$out
fi
cd "$(dirname "$0")/.."

target=wasm32-wasip1
installed=0
if command -v rustup >/dev/null 2>&1; then
  if rustup target list --installed 2>/dev/null | grep -qx "$target"; then
    installed=1
  fi
elif [[ -d "$(rustc --print target-libdir --target "$target" 2>/dev/null || true)" ]]; then
  installed=1
fi
if ((installed == 0)); then
  echo "build-studio-wasm: the target $target is not installed; run: rustup target add $target" >&2
  exit 1
fi

mkdir -p "$out"
# Without symbols the file is about a fifth smaller.
CARGO_PROFILE_RELEASE_STRIP=symbols cargo build --locked --release --target "$target" -p studio-rules
cp "${CARGO_TARGET_DIR:-target}/$target/release/studio-rules.wasm" "$out/rules.wasm"

shopt -s nullglob
total=0
built=0
analyses=0
manifest='{"games":['
separator=
for game_dir in games/*/; do
  game="$(basename "$game_dir")"
  [[ $game =~ ^[A-Za-z0-9_-]+$ ]] || continue
  # The releases, newest first: by number, then by name, both descending, as
  # studio/server/src/releases.rs sorts them.
  names=()
  while IFS= read -r name; do
    names+=("$name")
  done < <(
    for file in "games/$game/releases/$game"-v*.rs; do
      name="$(basename "$file" .rs)"
      number="${name#"$game"-v}"
      [[ $number =~ ^[0-9]+$ ]] && printf '%s %s\n' "$((10#$number))" "$name"
    done | sort -k1,1nr -k2,2r | cut -d' ' -f2
  )
  ((${#names[@]} > 0)) || continue
  mkdir -p "$out/$game"
  analysis=false
  if [[ -f games/$game/analysis/Cargo.toml ]]; then
    CARGO_PROFILE_RELEASE_STRIP=symbols cargo build --locked --release --target "$target" -p "$game-analysis"
    cp "${CARGO_TARGET_DIR:-target}/$target/release/$game-analysis.wasm" "$out/$game/analysis.wasm"
    analysis=true
    analyses=$((analyses + 1))
  fi
  compiled=()
  for name in "${names[@]}"; do
    total=$((total + 1))
    wasm="$out/$game/$name.wasm"
    rm -f "$wasm"
    if rustc --edition 2021 -C opt-level=3 -C strip=symbols --target "$target" \
      -o "$wasm" "games/$game/releases/$name.rs"; then
      compiled+=("$name")
      built=$((built + 1))
    else
      rm -f "$wasm"
      echo "build-studio-wasm: warning: $name does not compile to $target; it is left out of the manifest" >&2
    fi
  done
  list=
  for name in ${compiled[@]+"${compiled[@]}"}; do
    list+="${list:+,}\"$name\""
  done
  manifest+="$separator{\"id\":\"$game\",\"releases\":[$list],\"analysis\":$analysis}"
  separator=,
done
manifest+=']}'
printf '%s\n' "$manifest" >"$out/manifest.json"

if ((strict == 1 && built < total)); then
  echo "build-studio-wasm: only $built of $total releases are in the manifest (--strict)" >&2
  exit 1
fi
echo "ok   studio wasm: rules.wasm, analysis.wasm for $analyses game(s) and $built of $total releases in $out"
