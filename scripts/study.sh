#!/usr/bin/env bash
# The steps of the study workflow (.github/workflows/study.yml): a research
# computation on the repository's own code, too long for a session, split over
# runners, its results committed to a branch claude/study/<run id> since
# sessions cannot download artifacts. Only the studies listed below run, each
# with arguments checked by its own rules. It writes nothing to GitHub itself:
# the workflow's last step commits what `collect` wrote. It also runs locally.
#
# Usage:
#   scripts/study.sh plan
#   scripts/study.sh run STUDY SHA SHARD SHARDS ARGS
#   scripts/study.sh collect STUDY DIR OUT
#
# plan: checks the dispatch inputs, which are untrusted text, given in the
# environment: STUDY (a study below), REF (a commit SHA of 7 to 40 hexadecimal
# digits, a branch or a tag of this repository, whose tree holds the study's
# code), ARGS (the study's arguments, separated by spaces) and SHARDS (1 to
# 20); ARGS may hold only letters, digits, spaces and , / . _ = -. A bad input is an `::error` annotation and exit status 1. Prints, also
# to $GITHUB_OUTPUT: matrix (a JSON array of the shard numbers), study, sha
# (REF resolved), ref, args (as checked) and shards.
#
# run: shard SHARD of SHARDS. Makes a git worktree of SHA in
# target/study/src, builds the study there (its target directory is this
# checkout's target/, so that the workflow's cache serves it) and runs it with
# ARGS on its share, for at most STUDY_MINUTES minutes (default 300, within
# the workflow's 330), writing target/study/out/STUDY-SHARD.csv (its rows),
# STUDY-SHARD.md (its report), STUDY-SHARD.log (its progress) and, once the
# study has finished, STUDY-SHARD.done.
#
# collect: DIR holds the shards' files (as `run` names them). Writes to OUT:
# STUDY.csv (every shard's rows under one header), STUDY.plans.tsv (likewise,
# when the shards wrote plans), STUDY-SHARD.md for each shard, and README.md
# (the study, its inputs from the environment: REF, SHA,
# ARGS, SHARDS, RUN_ID, and the shards whole, partial and missing). A
# `::notice` gives the rows; a `::warning` names each partial shard (rows
# without the .done mark: it failed or ran out of time; its rows are kept) and
# each missing one (no rows), either of which is also exit status 1; the files
# found are still written. Shards whose columns differ are an error.
#
# Studies:
#   oracle  The exact early game of one side alone (E072,
#           games/ice-and-fire/opening/examples/oracle.rs). ARGS: --player NAME
#           (letters, digits, _ and -), --per-seat N (1 to 500), --turns
#           T[,T...] (each 1 to 12), --cap N and --limit N (each 1 to
#           10000000), --suite PATH (a .tsv of games/ice-and-fire/openings/ at
#           SHA; default suite-v1.tsv) and --from PATH (a .tsv under
#           games/ice-and-fire/ at SHA: plans of an earlier run to start from).
#           Each shard runs on all the runner's processors and also writes
#           the best plans it found, STUDY-SHARD.plans.tsv.
#
# Exit status: 0 on success; 1 for a bad input of `plan`, a failed `run` or a
# missing shard in `collect`; 2 for a usage error.
set -euo pipefail

cd "$(dirname "$0")/.."

readonly STUDIES="oracle"
readonly MAX_SHARDS=20
readonly WORD_RE='^[A-Za-z0-9][A-Za-z0-9_-]{0,39}$'
# The most minutes a shard's study runs: the workflow's job allows 330, and
# the build and the upload need the rest. The environment may lower it.
STUDY_MINUTES="${STUDY_MINUTES:-300}"
[[ $STUDY_MINUTES =~ ^[1-9][0-9]{0,2}$ ]] || STUDY_MINUTES=300

usage() {
  echo "usage: $0 plan | run STUDY SHA SHARD SHARDS ARGS | collect STUDY DIR OUT" >&2
  exit 2
}

# Reports a bad input or a failed step. The message may hold text of whoever
# dispatched the run: line breaks are removed so that it cannot start another
# workflow command, and it is cut.
die() {
  local message="${*//[$'\r\n']/ }"
  message="${message:0:300}"
  echo "::error title=Study::$message"
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

# Checks the words of the oracle study's ARGS at SHA and sets CHECKED to
# them, the suite always given. It runs in the caller's shell (no
# subshell), so that `die` reports the error and exits.
CHECKED=()
oracle_args() {
  local sha="$1"
  shift
  local words=("$@") kept=() flag value suite="games/ice-and-fire/openings/suite-v1.tsv" seen=" " i t
  ((${#words[@]} % 2 == 0)) || die "oracle: arguments come as --flag value pairs"
  for ((i = 0; i < ${#words[@]}; i += 2)); do
    flag="${words[i]}"
    value="${words[i + 1]}"
    [[ $seen != *" $flag "* ]] || die "oracle: $flag is given twice"
    seen+="$flag "
    case "$flag" in
    --player)
      [[ $value =~ $WORD_RE ]] || die "oracle: --player must be letters, digits, _ and - (up to 40)"
      ;;
    --per-seat)
      if ! [[ $value =~ ^[1-9][0-9]{0,2}$ ]] || ((value > 500)); then
        die "oracle: --per-seat must be a number from 1 to 500"
      fi
      ;;
    --turns)
      [[ $value =~ ^[1-9][0-9]?(,[1-9][0-9]?){0,11}$ ]] || die "oracle: --turns must be game turns from 1 to 12, separated by commas"
      for t in ${value//,/ }; do
        ((t <= 12)) || die "oracle: --turns must be game turns from 1 to 12, separated by commas"
      done
      ;;
    --cap | --limit)
      if ! [[ $value =~ ^[1-9][0-9]{0,7}$ ]] || ((value > 10000000)); then
        die "oracle: $flag must be a number from 1 to 10000000"
      fi
      ;;
    --from)
      if ! [[ $value =~ ^games/ice-and-fire/[A-Za-z0-9][A-Za-z0-9_./-]{0,199}\.tsv$ ]] || [[ $value == *..* ]]; then
        die "oracle: --from must be a .tsv under games/ice-and-fire/"
      fi
      git cat-file -e "$sha:$value" 2>/dev/null || die "oracle: $value does not exist at ${sha:0:7}"
      ;;
    --suite)
      if ! [[ $value =~ ^games/ice-and-fire/openings/[A-Za-z0-9][A-Za-z0-9_.-]{0,59}\.tsv$ ]] || [[ $value == *..* ]]; then
        die "oracle: --suite must be a .tsv of games/ice-and-fire/openings/"
      fi
      suite="$value"
      continue
      ;;
    *)
      die "oracle: unknown argument '$flag' (allowed: --player, --per-seat, --turns, --cap, --limit, --suite, --from)"
      ;;
    esac
    kept+=("$flag" "$value")
  done
  git cat-file -e "$sha:$suite" 2>/dev/null || die "oracle: $suite does not exist at ${sha:0:7}"
  git cat-file -e "$sha:games/ice-and-fire/opening/examples/oracle.rs" 2>/dev/null ||
    die "oracle: games/ice-and-fire/opening/examples/oracle.rs does not exist at ${sha:0:7}"
  CHECKED=(--suite "$suite" "${kept[@]}")
}

plan() {
  local study="${STUDY:-}" ref="${REF:-}" args="${ARGS:-}" shards="${SHARDS:-}" sha words=()
  [[ " $STUDIES " == *" $study "* ]] || die "study must be one of: $STUDIES"
  if ! [[ $ref =~ ^[A-Za-z0-9][A-Za-z0-9_./-]{0,199}$ ]] || [[ $ref == *..* ]]; then
    die "ref '$ref' is not a commit, a branch or a tag"
  fi
  sha="$(resolve_ref "$ref")" || die "ref '$ref' is not a commit, a branch or a tag of this repository"
  if ! [[ $shards =~ ^[1-9][0-9]?$ ]] || ((shards > MAX_SHARDS)); then
    die "shards must be a number from 1 to $MAX_SHARDS"
  fi
  [[ $args =~ ^[A-Za-z0-9\ ,/._=-]*$ ]] || die "args may hold only letters, digits, spaces and , / . _ = -"
  read -r -a words <<<"$args" || true
  "${study}_args" "$sha" "${words[@]}"
  emit "matrix=$(seq 0 $((shards - 1)) | jq -sc .)"
  emit "study=$study"
  emit "ref=$ref"
  emit "sha=$sha"
  emit "args=${CHECKED[*]}"
  emit "shards=$shards"
}

run() {
  [[ $# -eq 5 ]] || usage
  local study="$1" sha="$2" shard="$3" shards="$4" args="$5" words=()
  [[ " $STUDIES " == *" $study "* ]] || die "bad study"
  [[ $sha =~ ^[0-9a-f]{40}$ ]] || die "bad sha"
  if ! [[ $shards =~ ^[1-9][0-9]?$ && $shard =~ ^[0-9][0-9]?$ ]] || ((shard >= shards || shards > MAX_SHARDS)); then
    die "bad shard or shards"
  fi
  # Checked again here: `run` may be called on its own.
  [[ $args =~ ^[A-Za-z0-9\ ,/._=-]*$ ]] || die "bad arguments"
  read -r -a words <<<"$args" || true
  "${study}_args" "$sha" "${words[@]}"

  local target_dir="$PWD/target" work="$PWD/target/study"
  local src="$work/src" base="$work/out/$study-$shard"
  mkdir -p "$work/out"
  if [[ -d $src ]]; then
    git worktree remove --force "$src" 2>/dev/null || rm -rf "$src"
  fi
  git worktree add --detach --quiet "$src" "$sha"
  case "$study" in
  oracle)
    (cd "$src" && CARGO_TARGET_DIR="$target_dir" cargo build --release --locked -p ice-and-fire-opening --example oracle)
    rm -f "$base.csv" "$base.plans.tsv" "$base.done"
    # The suite and the plans to start from are read from the worktree.
    local argv=() i
    for ((i = 0; i < ${#CHECKED[@]}; i += 2)); do
      if [[ ${CHECKED[i]} == --suite || ${CHECKED[i]} == --from ]]; then
        argv+=("${CHECKED[i]}" "$src/${CHECKED[i + 1]}")
      else
        argv+=("${CHECKED[i]}" "${CHECKED[i + 1]}")
      fi
    done
    # Stopped before the job's limit, so that the rows written so far
    # (the oracle writes each start's rows as it finishes) are still kept.
    if ! timeout "$STUDY_MINUTES"m "$target_dir/release/examples/oracle" "${argv[@]}" --jobs "$(nproc)" \
      --shard "$shard/$shards" --csv "$base.csv" --plans "$base.plans.tsv" >"$base.md" 2>"$base.log"; then
      tail -n 20 "$base.log" >&2
      die "the oracle failed or ran out of time on shard $shard; its rows so far are kept"
    fi
    ;;
  esac
  # The mark of a whole shard: `collect` reports a shard without it as
  # partial.
  touch "$base.done"
  echo "shard $shard of $shards: $(($(wc -l <"$base.csv") - 1)) rows in $base.csv"
}

collect() {
  [[ $# -eq 3 ]] || usage
  local study="$1" dir="$2" dest="$3"
  [[ " $STUDIES " == *" $study "* ]] || die "bad study"
  [[ -d $dir ]] || die "no directory $dir"
  local shards="${SHARDS:-0}" csv="$dest/$study.csv" header="" rows=0 found=() partial=() missing=() k file
  if ! [[ $shards =~ ^[1-9][0-9]?$ ]] || ((shards > MAX_SHARDS)); then
    die "SHARDS must be a number from 1 to $MAX_SHARDS"
  fi
  mkdir -p "$dest"
  : >"$csv"
  rm -f "$dest/$study.plans.tsv"
  for ((k = 0; k < shards; k++)); do
    file="$dir/$study-$k.csv"
    if [[ ! -s $file ]]; then
      missing+=("$k")
      continue
    fi
    # Rows of a shard that failed or ran out of time are kept, the shard
    # reported as partial.
    if [[ -f $dir/$study-$k.done ]]; then
      found+=("$k")
    else
      partial+=("$k")
    fi
    if [[ -z $header ]]; then
      header="$(head -n 1 "$file")"
      echo "$header" >"$csv"
    elif [[ "$(head -n 1 "$file")" != "$header" ]]; then
      die "shard $k's columns differ from the first shard's"
    fi
    tail -n +2 "$file" >>"$csv"
    rows=$((rows + $(tail -n +2 "$file" | wc -l)))
    # The shard's other table, its best plans, under one header too.
    local plans="$dir/$study-$k.plans.tsv"
    if [[ -s $plans ]]; then
      if [[ ! -s $dest/$study.plans.tsv ]]; then
        head -n 1 "$plans" >"$dest/$study.plans.tsv"
      elif [[ "$(head -n 1 "$plans")" != "$(head -n 1 "$dest/$study.plans.tsv")" ]]; then
        die "shard $k's plan columns differ from the first shard's"
      fi
      tail -n +2 "$plans" >>"$dest/$study.plans.tsv"
    fi
    if [[ -f $dir/$study-$k.md ]]; then
      cp "$dir/$study-$k.md" "$dest/$study-$k.md"
    fi
  done
  {
    echo "# Study $study, run ${RUN_ID:-local}"
    echo
    echo "Written by \`scripts/study.sh collect\` (.github/workflows/study.yml)."
    echo
    echo "- Code: ${REF:-?} at ${SHA:-?}"
    echo "- Arguments: ${ARGS:-?}"
    echo "- Shards: $shards; whole: ${found[*]:-none}; partial (failed or out of time, rows kept): ${partial[*]:-none}; missing: ${missing[*]:-none}"
    echo "- Rows: $rows in $study.csv; each shard's report in $study-K.md"
    if [[ -s $dest/$study.plans.tsv ]]; then
      echo "- Plans: $(($(wc -l <"$dest/$study.plans.tsv") - 1)) in $study.plans.tsv"
    fi
  } >"$dest/README.md"
  echo "::notice title=Study $study::$rows rows; ${#found[@]} of $shards shards whole"
  for k in "${partial[@]}"; do
    echo "::warning title=Study $study::shard $k is partial (it failed or ran out of time); its rows are kept"
  done
  for k in "${missing[@]}"; do
    echo "::warning title=Study $study::shard $k has no results (its job failed)"
  done
  ((${#partial[@]} == 0 && ${#missing[@]} == 0))
}

case "${1:-}" in
plan)
  [[ $# -eq 1 ]] || usage
  plan
  ;;
run)
  shift
  run "$@"
  ;;
collect)
  shift
  collect "$@"
  ;;
*) usage ;;
esac
