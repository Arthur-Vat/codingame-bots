#!/usr/bin/env bash
# The steps of the screening workflow (.github/workflows/screen.yml; ADR 0049,
# decision 2): variants of one bot, each a git ref, played against a release on
# runners of their own, calibrated as the SPRT is (ADR 0048), with the results
# gathered in the run's step summary, log and annotations. It writes nothing to
# GitHub except what the workflow's own steps do (artifacts, annotations, the
# step summary), so it has no dry-run mode. It also runs locally.
#
# Usage:
#   scripts/screen.sh plan
#   scripts/screen.sh play GAME BOT NAME REV BASELINE.rs PAIRS SHARD SHARDS SEED
#   scripts/screen.sh summary GAME DIR
#
# plan: checks the dispatch inputs, which are untrusted text, given in the
# environment: GAME, BOT, VARIANTS (space separated NAME=REF, 1 to 8: NAME is
# letters, digits, `_` and `-` up to 40 characters, unique and not the
# baseline's name; REF is a commit SHA of 7 to 40 hexadecimal digits, a branch
# or a tag), BASELINE (a release name such as ice-and-fire-v014, empty for the
# newest release), PAIRS (2 to 8000 in all, a multiple of SHARDS, at most 1,000 a shard), SHARDS (1 to
# 8; variants x shards at most 20) and SEED (a decimal u64, empty for the
# game's SEED in evaluation.env). A bad input is an `::error` annotation and
# exit status 1. It resolves each REF to a full SHA, checks that the bot exists
# there, and prints, also to $GITHUB_OUTPUT: matrix (a JSON array of
# {"variant":NAME,"sha":SHA,"shard":K}), game, bot, baseline (the file),
# baseline_name, pairs, shards and seed.
#
# play: one variant's shard K of SHARDS. Makes a git worktree of REV in
# target/screen/src-NAME, builds that tree's own bundler there and bundles the
# bot with it, minified, into target/screen/NAME.rs; compiles it, BASELINE.rs
# (a file of the current tree) with scripts/cg-check.sh (the variant must fit
# CodinGame's size limit); builds the arena, calibrates as sprt.sh does (the
# game's evaluation.env, SPRT_TIME_SCALE and SPRT_TIME_TOLERANCE_MS included),
# and plays PAIRS/SHARDS seat-swapped pairs of NAME against the baseline with
# SEED (with `--shard K` when SHARDS is above 1, so that the shards play other
# games; with one shard the games are those of a plain `arena match`), writing
# the games to target/screen/NAME-K.jsonl, the summary to NAME-K.txt and the
# settings in one line to NAME-K.settings. A variant that loses is a result,
# not an error; an arena error is. For example, a look at the working tree's
# last commit against the 13th release of A Code of Ice and Fire:
#   scripts/screen.sh play ice-and-fire plan local HEAD \
#     games/ice-and-fire/releases/ice-and-fire-v013.rs 20 0 1 1
#
# summary: DIR holds the artifacts of the run's shards (NAME-K.jsonl, .txt and
# .settings). Adds each variant's shards up with the arena's `sum` and writes a
# table to $GITHUB_STEP_SUMMARY (and stdout): per variant the Elo with its 95%
# interval, the score, the faults, the slowest answers and the time scales of
# its shards; then, per shard, the answers after the first. One
# `::notice title=Screening NAME::` line per variant carries the result, and a
# `::warning` names each shard whose files are missing (its job failed), which
# is also exit status 1. MATRIX (the plan's matrix) names the shards expected;
# without it, the files found are the shards.
#
# Exit status: 0 on success; 1 for a bad input of `plan` or a missing shard in
# `summary`, and for a failed step of `play`; 2 for a usage error.
set -euo pipefail

cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib/evaluation.sh
source scripts/lib/evaluation.sh

readonly NAME_RE='^[A-Za-z0-9][A-Za-z0-9_-]{0,39}$'
readonly WORD_RE='^[A-Za-z0-9][A-Za-z0-9_-]{0,59}$'
readonly MAX_VARIANTS=8
readonly MAX_SHARDS=8
readonly MAX_RUNNERS=20
readonly MAX_PAIRS=8000
readonly MAX_SHARD_PAIRS=1000

usage() {
  echo "usage: $0 plan | play GAME BOT NAME REV BASELINE.rs PAIRS SHARD SHARDS SEED | summary GAME DIR" >&2
  exit 2
}

# Reports a bad input or a failed step. The message may hold text of whoever
# dispatched the run: line breaks are removed so that it cannot start another
# workflow command, and it is cut.
die() {
  local message="${*//[$'\r\n']/ }"
  message="${message:0:300}"
  echo "::error title=Screening::$message"
  echo "error: $message" >&2
  exit 1
}

emit() { # emit <name>=<value>: to stdout and to $GITHUB_OUTPUT
  echo "$1"
  if [[ -n ${GITHUB_OUTPUT:-} ]]; then
    echo "$1" >>"$GITHUB_OUTPUT"
  fi
}

# Prints the full SHA of the commit that REF names: a branch of origin, a local
# branch, a tag, or a commit given by 7 to 40 hexadecimal digits. Fails when
# there is none.
resolve_ref() {
  local ref="$1" candidate sha
  local candidates=("refs/remotes/origin/$ref" "refs/heads/$ref" "refs/tags/$ref")
  if [[ $ref =~ ^[0-9a-fA-F]{7,40}$ ]]; then
    candidates+=("$ref")
  fi
  for candidate in "${candidates[@]}"; do
    if sha="$(git rev-parse --verify --quiet "$candidate^{commit}")"; then
      echo "$sha"
      return 0
    fi
  done
  return 1
}

plan() {
  local game="${GAME:-}" bot="${BOT:-}" variants="${VARIANTS:-}" baseline="${BASELINE:-}"
  local pairs="${PAIRS:-}" shards="${SHARDS:-}" seed="${SEED:-}"

  [[ $game =~ $WORD_RE ]] || die "game must be a folder of games/ (letters, digits, - and _)"
  [[ -f games/$game/evaluation.env ]] || die "games/$game/evaluation.env does not exist"
  [[ $bot =~ $WORD_RE ]] || die "bot must be a folder of games/$game/bots (letters, digits, - and _)"
  [[ -f games/$game/bots/$bot/Cargo.toml ]] || die "games/$game/bots/$bot/Cargo.toml does not exist"

  if [[ -z $baseline ]]; then
    baseline="$(releases "$game" | tail -n 1)"
    [[ -n $baseline ]] || die "games/$game has no release to use as the baseline"
    baseline="$(basename "$baseline" .rs)"
  fi
  [[ $baseline =~ $NAME_RE ]] || die "baseline must be a release name such as $game-v001"
  [[ -f games/$game/releases/$baseline.rs ]] || die "games/$game/releases/$baseline.rs does not exist"

  [[ $pairs =~ ^[1-9][0-9]{0,3}$ ]] || die "pairs must be a number from 2 to $MAX_PAIRS"
  ((pairs >= 2 && pairs <= MAX_PAIRS)) || die "pairs must be a number from 2 to $MAX_PAIRS"
  [[ $shards =~ ^[1-9]$ ]] || die "shards must be a number from 1 to $MAX_SHARDS"
  ((shards <= MAX_SHARDS)) || die "shards must be a number from 1 to $MAX_SHARDS"
  ((pairs % shards == 0)) || die "pairs ($pairs) must be a multiple of shards ($shards)"
  ((pairs / shards <= MAX_SHARD_PAIRS)) || die "at most $MAX_SHARD_PAIRS pairs a shard: raise shards (now $shards for $pairs pairs)"

  if [[ -z $seed ]]; then
    seed="$(sed -n 's/^SEED=//p' "games/$game/evaluation.env" | head -n 1)"
  fi
  if [[ ! $seed =~ ^[0-9]{1,20}$ ]]; then
    die "seed must be a decimal number below 2^64"
  fi
  while ((${#seed} > 1)) && [[ $seed == 0* ]]; do
    seed="${seed#0}"
  done
  # Same length: the digits compare as text.
  # shellcheck disable=SC2071
  if ((${#seed} == 20)) && [[ $seed > 18446744073709551615 ]]; then
    die "seed must be a decimal number below 2^64"
  fi

  local items=() item name ref sha
  local names=" " rows=()
  read -r -d '' -a items <<<"$variants" || true
  ((${#items[@]} >= 1 && ${#items[@]} <= MAX_VARIANTS)) || die "give 1 to $MAX_VARIANTS variants as NAME=REF, separated by spaces"
  ((${#items[@]} * shards <= MAX_RUNNERS)) || die "variants x shards must be at most $MAX_RUNNERS (now ${#items[@]} x $shards)"
  for item in "${items[@]}"; do
    [[ $item == *=* ]] || die "variant '$item' is not NAME=REF"
    name="${item%%=*}"
    ref="${item#*=}"
    [[ $name =~ $NAME_RE ]] || die "variant name '$name' must be letters, digits, _ - (1 to 40, starting with a letter or digit)"
    [[ $names != *" $name "* ]] || die "variant name '$name' is used twice"
    [[ $name != "$baseline" ]] || die "variant name '$name' is the baseline's name"
    names+="$name "
    [[ $ref =~ ^[A-Za-z0-9][A-Za-z0-9_./-]{0,199}$ && $ref != *..* ]] || die "ref '$ref' of $name is not a commit, a branch or a tag"
    sha="$(resolve_ref "$ref")" || die "ref '$ref' of $name is not a commit, a branch or a tag of this repository"
    git cat-file -e "$sha:games/$game/bots/$bot/Cargo.toml" 2>/dev/null ||
      die "games/$game/bots/$bot/Cargo.toml does not exist at ref '$ref' of $name (${sha:0:7})"
    rows+=("$name $sha")
  done

  local matrix
  matrix="$(printf '%s\n' "${rows[@]}" | jq -Rnc --argjson shards "$shards" \
    '[inputs | split(" ") | . as $f | range(0; $shards) as $k | {variant: $f[0], sha: $f[1], shard: $k}]')"
  emit "matrix=$matrix"
  emit "game=$game"
  emit "bot=$bot"
  emit "baseline=games/$game/releases/$baseline.rs"
  emit "baseline_name=$baseline"
  emit "pairs=$pairs"
  emit "shards=$shards"
  emit "seed=$seed"
}

play() {
  [[ $# -eq 9 ]] || usage
  local game="$1" bot="$2" name="$3" rev="$4" baseline="$5" pairs="$6" shard="$7" shards="$8" seed="$9"
  [[ $game =~ $WORD_RE ]] || die "bad game"
  [[ $bot =~ $WORD_RE ]] || die "bad bot"
  [[ $name =~ $NAME_RE ]] || die "bad variant name"
  [[ $rev =~ ^[A-Za-z0-9][A-Za-z0-9_./^~{}@-]*$ ]] || die "bad revision"
  [[ $baseline =~ ^games/$game/releases/[A-Za-z0-9][A-Za-z0-9_-]*\.rs$ && -f $baseline ]] || die "bad baseline $baseline"
  [[ $pairs =~ ^[1-9][0-9]{0,3}$ && $shards =~ ^[1-9]$ && $shard =~ ^[0-9]$ ]] || die "bad pairs, shard or shards"
  ((pairs % shards == 0 && shard < shards)) || die "pairs must be a multiple of shards, and shard below shards"
  [[ $seed =~ ^[0-9]{1,20}$ ]] || die "bad seed"
  local sha
  sha="$(git rev-parse --verify --quiet "$rev^{commit}")" || die "no commit $rev"
  [[ -f games/$game/bots/$bot/Cargo.toml ]] || die "games/$game/bots/$bot/Cargo.toml does not exist"

  local work="$PWD/target/screen" src
  src="$work/src-$name"
  mkdir -p "$work/bin"
  rm -rf "$src"
  git worktree prune
  git worktree add --detach "$src" "$sha" >&2
  git cat-file -e "$sha:games/$game/bots/$bot/Cargo.toml" 2>/dev/null ||
    die "games/$game/bots/$bot/Cargo.toml does not exist at $rev (${sha:0:7})"

  # The variant is bundled by the bundler of its own tree, in that tree's own
  # target directory.
  echo "Variant $name: ${sha:0:7}, bot $bot; shard $shard of $shards, $((pairs / shards)) pairs of $pairs."
  local package
  package="$(sed -n 's/^name = "\(.*\)"$/\1/p' "$src/games/$game/bots/$bot/Cargo.toml" | head -n 1)"
  (
    cd "$src"
    unset CARGO_TARGET_DIR
    cargo build --quiet --release --locked -p cg-bundler >&2
    target/release/cg-bundler "$package" --output "$work/$name.rs" >&2
  ) || die "could not bundle $bot at $rev (${sha:0:7})"

  local base
  base="$(basename "$baseline" .rs)"
  CG_BIN_DIR="$work/bin" scripts/cg-check.sh "$work/$name.rs" "$baseline" >&2 ||
    die "$name (${sha:0:7}) or the baseline $base does not compile or is over CodinGame's size limit"

  export SEED="$seed"
  load_settings "$game"
  TIME_SCALE="${SPRT_TIME_SCALE:-$TIME_SCALE}"
  TIME_TOLERANCE_MS="${SPRT_TIME_TOLERANCE_MS:-$TIME_TOLERANCE_MS}"
  build_tools "$game"
  calibrate_settings "$game" "$work/calibration"

  local arena="target/release/$game-arena" options shard_option=() status=0
  read -r -a options <<<"$(arena_options)"
  if ((shards > 1)); then
    shard_option=(--shard "$shard")
  fi
  local settings
  settings="$(describe_settings); machine speed ${MACHINE_SPEED:-none}"
  echo "Settings: $settings."
  echo "$settings" >"$work/$name-$shard.settings"
  # No --expect-* option: a variant that loses is a result. A status of 2 or
  # more is an arena error.
  "$arena" match "${options[@]}" ${shard_option[@]+"${shard_option[@]}"} \
    --bot "$name=$work/bin/$name" --bot "$base=$work/bin/$base" \
    --pairs "$((pairs / shards))" --out "$work/$name-$shard.jsonl.part" |
    tee "$work/$name-$shard.txt" || status=$?
  if ((status >= 2)); then
    # The games of a shard cut short are not kept: summary reports it missing.
    rm -f "$work/$name-$shard.jsonl.part"
    die "the arena could not play $name (status $status)"
  fi
  mv "$work/$name-$shard.jsonl.part" "$work/$name-$shard.jsonl"
}

# Prints "$1 $2 ..." joined by $1: join <separator> <items>...
join() {
  local separator="$1" out="" item
  shift
  for item in "$@"; do
    out+="${out:+$separator}$item"
  done
  echo "$out"
}

summary() {
  [[ $# -eq 2 ]] || usage
  local game="$1" dir="$2" arena="target/release/$1-arena"
  [[ $game =~ $WORD_RE ]] || die "bad game"
  [[ -d $dir ]] || die "$dir is not a folder"
  if [[ ! -x $arena ]]; then
    cargo build --quiet --release --locked -p "$game-arena" >&2
  fi

  # The shards expected: "NAME SHARD SHA" lines.
  local expected
  if [[ -n ${MATRIX:-} ]]; then
    expected="$(jq -r '.[] | "\(.variant) \(.shard) \(.sha)"' <<<"$MATRIX")"
  else
    expected="$(
      shopt -s nullglob
      for file in "$dir"/*.jsonl; do
        file="$(basename "$file" .jsonl)"
        echo "${file%-*} ${file##*-} -"
      done | sort -k1,1 -k2,2n
    )"
  fi
  [[ -n $expected ]] || die "no shard to report on"

  local variants=() missing=0 failed=0 shard_rows=""
  local md="| Variant | Ref | Pairs | Elo over the baseline [95%] | Score | Faults of the variant | Faults of the baseline | Slowest answer, variant / baseline | Time scales |
|---|---|---|---|---|---|---|---|---|
"
  mapfile -t variants < <(awk '!seen[$1]++ { print $1 }' <<<"$expected")
  local variant files scales shard sha ref out settings scale speed answers vcell bcell
  local count total_shards baseline_name pairs_played elo score vfaults bfaults slow faults
  for variant in "${variants[@]}"; do
    files=()
    scales=()
    ref="-"
    total_shards="$(awk -v v="$variant" '$1 == v' <<<"$expected" | wc -l)"
    while read -r _ shard sha; do
      [[ $sha == - ]] || ref="${sha:0:7}"
      if [[ ! -f $dir/$variant-$shard.jsonl ]]; then
        echo "::warning title=Screening $variant::shard $shard of $variant has no result (its job failed or was cancelled)"
        missing=$((missing + 1))
        continue
      fi
      files+=("$dir/$variant-$shard.jsonl")
      settings=""
      scale="?"
      speed="?"
      if [[ -f $dir/$variant-$shard.settings ]]; then
        settings="$(cat "$dir/$variant-$shard.settings")"
        scale="$(sed -n 's/^time scale \([0-9.]*\),.*/\1/p' <<<"$settings")"
        speed="$(sed -n 's/.*machine speed \(.*\)$/\1/p' <<<"$settings")"
        scale="${scale:-?}"
        speed="${speed:-?}"
      fi
      scales+=("$scale")
      # The answers after the first of this shard, from the arena's report
      # (the games' files do not hold them).
      vcell="n/a"
      bcell="n/a"
      if [[ -f $dir/$variant-$shard.txt ]]; then
        answers="$(sed -n 's/^  answers after the first (median, 99%, 99.9%): //p' "$dir/$variant-$shard.txt")"
        if [[ -n $answers ]]; then
          vcell="${answers%%; *}"
          bcell="${answers#*; }"
          vcell="${vcell#* }"
          bcell="${bcell#* }"
        fi
      fi
      shard_rows+="| $variant | $shard | $scale | $speed | $vcell | $bcell |"$'\n'
    done < <(awk -v v="$variant" '$1 == v' <<<"$expected")

    if ((${#files[@]} == 0)); then
      md+="| $variant | \`$ref\` | 0 | no result | | | | | |"$'\n'
      failed=$((failed + 1))
      continue
    fi
    if ! out="$("$arena" sum "${files[@]}" 2>&1)"; then
      echo "::error title=Screening $variant::the arena could not add up the shards: ${out//$'\n'/ }"
      md+="| $variant | \`$ref\` | | sum failed | | | | | |"$'\n'
      failed=$((failed + 1))
      continue
    fi
    echo "== $variant =="
    echo "$out"

    baseline_name="$(sed -n '1s/^[^ ]* vs \([^:]*\):.*/\1/p' <<<"$out")"
    pairs_played="$(sed -n '1s/.*, \([0-9]*\) complete pairs$/\1/p' <<<"$out")"
    elo="$(sed -n 's/^  Elo of .* over .*: \(.*\) (95%, from pairs)$/\1/p' <<<"$out")"
    elo="${elo:-n/a}"
    score="$(sed -n '2s/.*(score \(.*\))$/\1/p' <<<"$out")"
    mapfile -t faults < <(sed -n 's/^  .* faults: \([0-9]*\) timeouts, \([0-9]*\) crashes, \([0-9]*\) invalid answers$/\1\/\2\/\3/p' <<<"$out")
    vfaults="${faults[0]:-?}"
    bfaults="${faults[1]:-?}"
    slow="$(sed -n 's/^  slowest answer: .* \([0-9.]*\) ms, .* \([0-9.]*\) ms$/\1 \/ \2 ms/p' <<<"$out")"
    count=""
    if ((${#files[@]} < total_shards)); then
      count=" (${#files[@]} of $total_shards shards)"
    fi
    md+="| $variant$count | \`$ref\` | $pairs_played | $elo | ${score:-n/a} | $vfaults | $bfaults | ${slow:-n/a} | $(join ', ' "${scales[@]}") |"$'\n'
    echo "::notice title=Screening $variant::Elo $elo over $baseline_name, $pairs_played pairs, faults (timeouts/crashes/invalid) $vfaults against the baseline's $bfaults, ${#files[@]} shard(s)"
  done

  local text="## Screening of $game
"
  text+="$md"
  if [[ -n $shard_rows ]]; then
    text+="
Faults are timeouts/crashes/invalid answers. Per shard (answers after the first, in ms: median, 99%, 99.9%):

| Variant | Shard | Time scale | Machine speed | Variant's answers | Baseline's answers |
|---|---|---|---|---|---|
$shard_rows"
  fi
  if ((missing > 0)); then
    text+="
**$missing shard(s) have no result.**
"
  fi
  printf '%s\n' "$text"
  if [[ -n ${GITHUB_STEP_SUMMARY:-} ]]; then
    printf '%s\n' "$text" >>"$GITHUB_STEP_SUMMARY"
  fi
  if ((missing > 0 || failed > 0)); then
    exit 1
  fi
}

[[ $# -ge 1 ]] || usage
mode="$1"
shift
case "$mode" in
  plan) plan ;;
  play) play "$@" ;;
  summary) summary "$@" ;;
  *) usage ;;
esac
