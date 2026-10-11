#!/usr/bin/env bash
# Reports the failure of a CI step as a GitHub error annotation that carries
# an excerpt of the step's output, so the failure can be read from the check
# run without the job's log.
#
# Usage:
#   scripts/annotate.sh run TITLE COMMAND [ARGS...]
#   scripts/annotate.sh file TITLE PATH
#
# run: runs the command, passes its stdout to stdout and its stderr to stderr
# as they come, and exits with its status. When the status is not 0, a last line
# `::error title=TITLE::MESSAGE` follows the output. The message is an excerpt
# of both streams, in the order they arrived, and the line `exit status N`.
# file: prints the annotation for the content of the file, always, and exits
# with 0; the message is `no output` for a missing or empty file.
#
# The excerpt is taken from the output without colour codes and carriage
# returns, each line cut to 300 characters. From the first line that looks like
# a failure (`error:`, `error[E0000]:`, `failures:`, `---- name ----`, `thread
# 'name' panicked at`) it holds 40 lines, then `[…]` and the last 5 lines of
# the output if there are more. Without such a line it holds the last 40 lines,
# after `[…]` if there are more. The markers are those of cargo, rustc and
# Rust's tests; other tools (npm, TypeScript, Vitest) get the last lines.
set -euo pipefail

max_line=300
excerpt_lines=40
tail_lines=5
marker_re="^(error(\\[[A-Za-z0-9]+\\])?:|failures:\$|---- .+ ----\$|thread '.+' panicked at )"
esc=$'\033'

usage() {
  echo "usage: $0 run TITLE COMMAND [ARGS...]" >&2
  echo "       $0 file TITLE PATH" >&2
  exit 2
}

# Escapes the text for a workflow command; with a second argument, as the
# value of a property (the title).
escape() { # escape <text> [property]
  local text="$1"
  text="${text//'%'/%25}"
  text="${text//$'\r'/%0D}"
  text="${text//$'\n'/%0A}"
  if [[ -n ${2-} ]]; then
    text="${text//:/%3A}"
    text="${text//,/%2C}"
  fi
  printf '%s' "$text"
}

# Prints the excerpt of the file's content, lines joined by newlines.
excerpt() { # excerpt <path>
  local -a lines=() out=()
  local i marker=-1 n start
  if [[ -r $1 ]]; then
    mapfile -t lines < <(sed -e "s/${esc}\\[[0-9;]*[A-Za-z]//g" "$1" | tr -d '\r')
  fi
  n=${#lines[@]}
  if ((n == 0)); then
    printf 'no output'
    return
  fi
  for ((i = 0; i < n; i++)); do
    if [[ ${lines[i]} =~ $marker_re ]]; then
      marker=$i
      break
    fi
  done
  for ((i = 0; i < n; i++)); do
    if ((${#lines[i]} > max_line)); then lines[i]="${lines[i]:0:max_line}…"; fi
  done
  if ((marker >= 0)); then
    out=("${lines[@]:marker:excerpt_lines}")
    start=$((marker + excerpt_lines))
    if ((n > start)); then
      # A gap is marked; lines that follow the excerpt directly are not.
      if ((n - tail_lines > start)); then
        out+=("[…]")
        start=$((n - tail_lines))
      fi
      out+=("${lines[@]:start}")
    fi
  elif ((n > excerpt_lines)); then
    out=("[…]" "${lines[@]:n-excerpt_lines}")
  else
    out=("${lines[@]}")
  fi
  local IFS=$'\n'
  printf '%s' "${out[*]}"
}

# Prints the annotation of the file's content, then an optional last line.
annotation() { # annotation <title> <path> [last line]
  local message
  message="$(excerpt "$2")"
  if [[ -n ${3-} ]]; then message+=$'\n'"$3"; fi
  printf '::error title=%s::%s\n' "$(escape "$1" property)" "$(escape "$message")"
}

(($# >= 3)) || usage
mode="$1" title="$2"
shift 2

case "$mode" in
run)
  tmp="$(mktemp)"
  out_tmp="$(mktemp)"
  trap 'rm -f "$tmp" "$out_tmp"' EXIT
  # stderr goes to a tee of its own, so that it stays on stderr while both
  # streams are recorded in $tmp; $out_tmp holds stdout only.
  exec {err_fd}> >(tee -a "$tmp" >&2)
  err_pid=$!
  set +e
  "$@" 2>&"$err_fd" | tee -a "$tmp" "$out_tmp"
  status=${PIPESTATUS[0]}
  set -e
  # The copy of stderr must be complete before the excerpt is read.
  exec {err_fd}>&-
  wait "$err_pid" || true
  if ((status != 0)); then
    # A workflow command must start a line.
    if [[ -s $out_tmp && -n $(tail -c 1 "$out_tmp") ]]; then echo; fi
    annotation "$title" "$tmp" "exit status $status"
  fi
  exit "$status"
  ;;
file)
  (($# == 1)) || usage
  annotation "$title" "$1"
  ;;
*) usage ;;
esac
