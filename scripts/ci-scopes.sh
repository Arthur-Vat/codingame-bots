#!/usr/bin/env bash
# Decides which CI checks run (ADR 0029): the one place that maps changed paths
# to areas. The workflows read its output; nothing else decides what runs.
#
# Usage:
#   scripts/ci-scopes.sh ci [--all] [FILE...]
#   scripts/ci-scopes.sh sprt [FILE...]
#   scripts/ci-scopes.sh league [--all] [FILE...]
#   scripts/ci-scopes.sh results NAME=EXPECTED...
#
# ci: prints six lines to stdout, ready for $GITHUB_OUTPUT, and a one-line
# summary to stderr:
#   rust=true|false       the Rust lint job runs
#   framework=true|false  the framework's tests run
#   games=["id",...]      the games whose checks run (compact JSON, sorted)
#   studio=true|false     the studio's Rust checks run
#   web=true|false        the studio's front end checks run
#   licenses=true|false   the dependency policy runs
# Each FILE is a path from the repository root, as `git diff --name-only`
# prints it (a leading ./ is dropped); a file is classified by the first rule
# that matches:
#   1  *.md anywhere, .github/ included (the pull
#      request template)                               nothing (docs)
#   2  docs/, .claude/                                 nothing (docs)
#   3  .github/                                        everything
#   4  root Cargo.toml, Cargo.lock, deny.toml, rust-toolchain(.toml),
#      .cargo/                                         everything
#   5  Cargo.toml elsewhere                            licenses, and rules 6 to 9
#   6  crates/, scripts/                               framework
#   7  games/<g>/ for a directory games/<g>            game <g>; also web for
#      games/<g>/studio/, games/<g>/referee/ and
#      games/<g>/releases/
#   8  studio/web/                                     web
#   9  studio/core/, studio/rules/                     studio and web
#      studio/game/, studio/server/                    studio
#   10 anything else                                   everything
# framework means rust, framework, studio, web and every game; a game means
# rust, studio and that game; studio means rust and studio; web means the
# front end's checks only. The front end's browser engine (ADR 0030) runs the
# studio's rules, the games' referees and adapters, and every release as
# WebAssembly, and its browser tests use them: that is why rules 7 and 9 add
# web for those paths, and why crates/ (cg-arena) and scripts/
# (build-studio-wasm.sh), the framework, include it. Everything means
# every output true and every game. --all gives everything; no file and no
# --all, nothing. GAMES_DIR (default games) is where the game ids come from
# (directories directly under it), for the tests.
# The framework's tests also run when any game is selected (the bundler's tests
# bundle the games' bots, ADR 0029): the workflow decides that from `framework`
# and `games`; the outputs above do not say it.
#
# sprt: the games the SPRT workflow tests. The FILEs are the files the pull
# request adds (git diff --diff-filter=A). Prints one line to stdout and a
# one-line summary to stderr:
#   games=["id",...]      the games with a new release (compact JSON, sorted)
# A game is listed when a FILE is games/<g>/releases/<name>.rs, directly in
# releases/; files that are not .rs files under a releases/ folder are
# ignored. A FILE that git's pathspec games/*/releases/*.rs matches (the * crosses /)
# but that is not exactly games/<g>/releases/<name>.rs for a known game <g>
# (a nested file, an unknown game folder) fails with status 1 and a message on
# stderr: the SPRT cannot place it, and the league's find would rate it.
# --all is a usage error: the SPRT tests new releases only.
#
# league: the games the league workflow rates. The FILEs are the changed
# files. Prints the same line as sprt. --all, or a FILE among the league's
# own files (.github/workflows/league.yml, scripts/league.sh and
# scripts/lib/evaluation.sh), gives every game. Otherwise a
# game is listed when a FILE is under games/<g>/releases/ (a deleted release
# too) or is games/<g>/evaluation.env; other files are ignored.
#
# In sprt and league, a game counts only when it is a directory directly
# under GAMES_DIR with a name that matches [A-Za-z0-9_-]+, as in ci, and a
# FILE is read as in ci (a leading ./ is dropped, -- ends the options).
#
# results: the verdict of an aggregate job (`CI passed`, `SPRT passed`). The
# environment variable NEEDS holds the JSON of `${{ toJSON(needs) }}`. Each
# argument is JOB=true (the job had to run: its result must be success) or JOB=false (it
# could be skipped: success or skipped). Anything else fails: another result,
# a job named in an argument but missing from NEEDS, a job in NEEDS that no
# argument names, an expected value that is not true or false (empty when the
# job that decides it failed). Prints one line per job and, on failure, an
# error annotation, titled with the environment variable RESULTS_TITLE (default
# `CI passed`); exits 1 on failure.
#
# Needs jq for the results mode. Exit status 2: wrong usage.
set -euo pipefail

cd "$(dirname "$0")/.."

# Bytes and ASCII classes, whatever the locale.
export LC_ALL=C

games_dir="${GAMES_DIR:-games}"

usage() {
  sed -n '/^# Usage:/,/^# ci:/p' "$0" | sed '$d;s/^# \{0,1\}//' >&2
  exit 2
}

# ---- ci ----------------------------------------------------------------

# Flags set by the classification of the files.
f_rust=false
f_framework=false
f_studio=false
f_web=false
f_licenses=false
declare -A f_games=()
declare -a all_games=()

load_games() {
  local dir id
  all_games=()
  for dir in "$games_dir"/*/; do
    [[ -d $dir ]] || continue
    id="$(basename "$dir")"
    # A game id is a plain name; anything else is not trusted as a path part.
    if [[ $id =~ ^[A-Za-z0-9_-]+$ ]]; then
      all_games+=("$id")
    fi
  done
}

effect_everything() {
  local id
  f_rust=true
  f_framework=true
  f_studio=true
  f_web=true
  f_licenses=true
  for id in "${all_games[@]}"; do
    f_games["$id"]=1
  done
}

effect_framework() {
  local id
  f_rust=true
  f_framework=true
  f_studio=true
  f_web=true
  for id in "${all_games[@]}"; do
    f_games["$id"]=1
  done
}

# Rules 6 to 9 for a path; returns 1 when none matches.
area_effect() { # area_effect <path>
  local path="$1" id known
  case $path in
    crates/* | scripts/*)
      effect_framework
      return 0
      ;;
    games/*/*)
      id="${path#games/}"
      id="${id%%/*}"
      for known in "${all_games[@]}"; do
        if [[ $known == "$id" ]]; then
          f_rust=true
          f_studio=true
          f_games["$id"]=1
          case $path in
            games/*/studio/* | games/*/referee/* | games/*/releases/*) f_web=true ;;
          esac
          return 0
        fi
      done
      return 1
      ;;
    studio/web/*)
      f_web=true
      return 0
      ;;
    studio/core/* | studio/rules/*)
      f_rust=true
      f_studio=true
      f_web=true
      return 0
      ;;
    studio/game/* | studio/server/*)
      f_rust=true
      f_studio=true
      return 0
      ;;
  esac
  return 1
}

classify() { # classify <path>
  local path="$1"
  while [[ $path == ./* ]]; do
    path="${path#./}"
  done
  [[ -n $path ]] || return 0
  case $path in
    *.md | docs/* | .claude/*) return 0 ;;
    .github/*)
      effect_everything
      return 0
      ;;
    Cargo.toml | Cargo.lock | deny.toml | rust-toolchain | rust-toolchain.toml | .cargo/*)
      effect_everything
      return 0
      ;;
    */Cargo.toml)
      f_licenses=true
      ;;
  esac
  if ! area_effect "$path"; then
    effect_everything
  fi
}

mode_ci() {
  local all=false parse=true file id json="" count=0
  local -a files=()
  for file in "$@"; do
    if [[ $parse == true && $file == -- ]]; then
      parse=false
    elif [[ $parse == true && $file == --all ]]; then
      all=true
    elif [[ $parse == true && $file == -* ]]; then
      echo "ci-scopes: unknown option $file" >&2
      usage
    else
      files+=("$file")
    fi
  done

  load_games
  if [[ $all == true ]]; then
    effect_everything
  fi
  for file in "${files[@]}"; do
    classify "$file"
    count=$((count + 1))
  done

  local -a selected=()
  for id in "${all_games[@]}"; do
    if [[ -n ${f_games[$id]:-} ]]; then
      selected+=("$id")
    fi
  done
  if ((${#selected[@]} > 0)); then
    mapfile -t selected < <(printf '%s\n' "${selected[@]}" | sort)
    json="$(printf '"%s",' "${selected[@]}")"
    json="[${json%,}]"
  else
    json="[]"
  fi

  printf 'rust=%s\nframework=%s\ngames=%s\nstudio=%s\nweb=%s\nlicenses=%s\n' \
    "$f_rust" "$f_framework" "$json" "$f_studio" "$f_web" "$f_licenses"
  echo "ci-scopes: framework=$f_framework games=${selected[*]:-} studio=$f_studio web=$f_web licenses=$f_licenses ($count files)" >&2
}

# ---- sprt and league ---------------------------------------------------

# Splits the arguments into `files`, and sets `all` when --all is given.
parse_files() { # parse_files <allow --all: true|false> <mode> <argument>...
  local allow="$1" mode="$2" parse=true file
  shift 2
  all=false
  files=()
  for file in "$@"; do
    if [[ $parse == true && $file == -- ]]; then
      parse=false
    elif [[ $parse == true && $file == --all && $allow == true ]]; then
      all=true
    elif [[ $parse == true && $file == -* ]]; then
      echo "ci-scopes: $mode: unknown option $file" >&2
      usage
    else
      files+=("$file")
    fi
  done
}

# Flags the game <id> when it is a known game.
flag_game() { # flag_game <id>
  local known
  for known in "${all_games[@]}"; do
    if [[ $known == "$1" ]]; then
      f_games["$1"]=1
      return 0
    fi
  done
  return 0
}

# Prints games=<JSON array> for the games flagged, and the summary on stderr.
print_games() { # print_games <mode> <file count>
  local id json
  local -a selected=()
  for id in "${all_games[@]}"; do
    if [[ -n ${f_games[$id]:-} ]]; then
      selected+=("$id")
    fi
  done
  if ((${#selected[@]} > 0)); then
    mapfile -t selected < <(printf '%s\n' "${selected[@]}" | sort)
    json="$(printf '"%s",' "${selected[@]}")"
    json="[${json%,}]"
  else
    json="[]"
  fi
  printf 'games=%s\n' "$json"
  echo "ci-scopes: $1: games=${selected[*]:-} ($2 files)" >&2
}

mode_sprt() {
  local file path id rest known placed count=0
  local all files
  local -a unplaced=()
  parse_files false sprt "$@"
  load_games
  for file in "${files[@]}"; do
    count=$((count + 1))
    path="$file"
    while [[ $path == ./* ]]; do
      path="${path#./}"
    done
    case $path in
      games/*/releases/*.rs)
        # The same pattern as git's pathspec in the workflow (* crosses /).
        id="${path#games/}"
        id="${id%%/*}"
        placed=false
        for known in "${all_games[@]}"; do
          if [[ $known == "$id" && $path == "games/$id/releases/"* ]]; then
            rest="${path#"games/$id/releases/"}"
            # Directly in releases/: no further folder.
            if [[ $rest != */* ]]; then
              placed=true
            fi
          fi
        done
        if [[ $placed == true ]]; then
          flag_game "$id"
        else
          unplaced+=("$file")
        fi
        ;;
    esac
  done
  if ((${#unplaced[@]} > 0)); then
    for file in "${unplaced[@]}"; do
      echo "ci-scopes: sprt: cannot place $file: a release is games/<game>/releases/<name>.rs for a game of this repository" >&2
    done
    exit 1
  fi
  print_games sprt "$count"
}

mode_league() {
  local file path id count=0 every=false
  local all files
  parse_files true league "$@"
  load_games
  if [[ $all == true ]]; then
    every=true
  fi
  for file in "${files[@]}"; do
    count=$((count + 1))
    path="$file"
    while [[ $path == ./* ]]; do
      path="${path#./}"
    done
    case $path in
      .github/workflows/league.yml | scripts/league.sh | scripts/lib/evaluation.sh)
        every=true
        ;;
      games/*/releases/* | games/*/evaluation.env)
        id="${path#games/}"
        id="${id%%/*}"
        # The id is the first folder; the path must be under its releases/
        # or be its evaluation.env (a folder of another name does not count).
        if [[ $path == "games/$id/releases/"* || $path == "games/$id/evaluation.env" ]]; then
          flag_game "$id"
        fi
        ;;
    esac
  done
  if [[ $every == true ]]; then
    for id in "${all_games[@]}"; do
      f_games["$id"]=1
    done
  fi
  print_games league "$count"
}

# ---- results -----------------------------------------------------------

# Escapes the text for a workflow command message.
escape() { # escape <text>
  local text="$1"
  text="${text//'%'/%25}"
  text="${text//$'\r'/%0D}"
  text="${text//$'\n'/%0A}"
  printf '%s' "$text"
}

mode_results() {
  local arg name expected result line status=0 title="${RESULTS_TITLE:-CI passed}"
  local -a order=() failing=()
  local -A want=() got=() seen=()

  if (($# == 0)); then
    echo "ci-scopes: results needs at least one NAME=EXPECTED" >&2
    usage
  fi
  for arg in "$@"; do
    if [[ $arg != *=* || ${arg%%=*} == "" ]]; then
      echo "ci-scopes: not NAME=EXPECTED: $arg" >&2
      usage
    fi
    name="${arg%%=*}"
    if [[ -n ${seen[$name]:-} ]]; then
      echo "ci-scopes: $name is given twice" >&2
      usage
    fi
    seen["$name"]=1
    want["$name"]="${arg#*=}"
    order+=("$name")
  done
  if [[ -z ${NEEDS:-} ]]; then
    echo "ci-scopes: the environment variable NEEDS is not set (the JSON of toJSON(needs))" >&2
    usage
  fi

  local listing
  if ! listing="$(jq -r 'to_entries[] | [.key, (.value.result // "")] | @tsv' <<<"$NEEDS" 2>&1)"; then
    echo "FAIL NEEDS is not a JSON object of jobs: $listing"
    echo "::error title=$title::NEEDS is not a JSON object of jobs"
    exit 1
  fi
  while IFS=$'\t' read -r name result; do
    [[ -n $name ]] || continue
    got["$name"]="$result"
  done <<<"$listing"

  for name in "${order[@]}"; do
    expected="${want[$name]}"
    if [[ -z ${got[$name]+x} ]]; then
      echo "FAIL $name: missing from the jobs gathered"
      failing+=("$name: missing")
      status=1
      continue
    fi
    result="${got[$name]}"
    case $expected in
      true)
        if [[ $result == success ]]; then
          echo "ok   $name: $result"
        else
          echo "FAIL $name: $result (had to run)"
          failing+=("$name: $result (had to run)")
          status=1
        fi
        ;;
      false)
        if [[ $result == success || $result == skipped ]]; then
          echo "ok   $name: $result"
        else
          echo "FAIL $name: $result"
          failing+=("$name: $result")
          status=1
        fi
        ;;
      *)
        echo "FAIL $name: expected value '$expected' is neither true nor false (result: $result)"
        failing+=("$name: expected '$expected', result $result")
        status=1
        ;;
    esac
  done
  # Jobs that no argument names: the aggregate does not cover them.
  while IFS= read -r name; do
    [[ -n $name ]] || continue
    if [[ -z ${seen[$name]:-} ]]; then
      echo "FAIL $name: ${got[$name]} (no argument names this job)"
      failing+=("$name: not named in the arguments")
      status=1
    fi
  done < <(printf '%s\n' "${!got[@]}" | sort)

  if ((status != 0)); then
    line="$(printf '%s, ' "${failing[@]}")"
    echo "::error title=$title::$(escape "${line%, }")"
  fi
  exit "$status"
}

# ---- main --------------------------------------------------------------

(($# >= 1)) || usage
mode="$1"
shift
case $mode in
  ci) mode_ci "$@" ;;
  sprt) mode_sprt "$@" ;;
  league) mode_league "$@" ;;
  results) mode_results "$@" ;;
  *)
    echo "ci-scopes: unknown mode $mode" >&2
    usage
    ;;
esac
