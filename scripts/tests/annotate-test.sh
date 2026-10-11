#!/usr/bin/env bash
# Tests scripts/annotate.sh: the annotation of a failing command and of a
# file, the choice of the excerpt, the escaping, the usage errors.
# Usage: scripts/tests/annotate-test.sh
# shellcheck disable=SC2016 # the scripts of the commands are single-quoted on purpose
set -euo pipefail

cd "$(dirname "$0")/../.."
script=scripts/annotate.sh
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

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

# Runs the script, setting output (stdout and stderr) and status.
output="" status=0
run() { # run <arguments>...
  status=0
  output="$("$script" "$@" 2>&1)" || status=$?
}

# The annotation line of the output.
annotation_of() { # annotation_of <output>
  grep '^::error' <<<"$1" || true
}

# Lines "line N" for N from A to B, joined by the escaped newline.
lines() { # lines <a> <b>
  local i out=""
  for ((i = $1; i <= $2; i++)); do out+="line $i%0A"; done
  printf '%s' "$out"
}

# The annotation of a command that prints the lines (a script for bash -c) and
# exits with 1.
annotation_for() { # annotation_for <script of the command>
  run run Demo bash -c "$1; exit 1"
  annotation_of "$output"
}

# A failing command: its output is passed through, then the annotation.
run run Demo bash -c 'echo a; echo b; exit 3'
same "failing: status" 3 "$status"
same "failing: output" $'a\nb\n::error title=Demo::a%0Ab%0Aexit status 3' "$output"

# A command that succeeds prints its output and no annotation.
run run Demo bash -c 'echo fine'
same "passing: status" 0 "$status"
same "passing: output" "fine" "$output"
run run Demo true
same "silent: status" 0 "$status"
same "silent: output" "" "$output"

# The title and the message are escaped; the marker starts the excerpt.
run run 'Lint: clippy, all' bash -c 'echo warning: x; echo "error: unused 50%"; echo " --> a.rs:1:1"; exit 101'
same "escaping: status" 101 "$status"
same "escaping: annotation" \
  '::error title=Lint%3A clippy%2C all::error: unused 50%25%0A --> a.rs:1:1%0Aexit status 101' \
  "$(annotation_of "$output")"

# A command that fails without output.
run run Demo bash -c 'exit 4'
same "no output: status" 4 "$status"
same "no output: annotation" '::error title=Demo::no output%0Aexit status 4' "$output"

# stderr is merged with stdout, in order.
run run Demo bash -c 'echo out; echo err >&2; exit 1'
same "merged: output" $'out\nerr\n::error title=Demo::out%0Aerr%0Aexit status 1' "$output"

# A command that cannot be found: status 127 and an annotation.
run run Demo no-such-command-here
same "not found: status" 127 "$status"
same "not found: annotation" 1 "$(grep -c '^::error title=Demo::.*not found.*%0Aexit status 127$' <<<"$output" || true)"

# A cargo test like output of 100 lines, failures: at line 10: lines 10 to 49,
# a gap, the last 5 lines.
body='for i in $(seq 100); do if [ $i = 10 ]; then echo failures:; else echo "line $i"; fi; done'
expected="failures:%0A$(lines 11 49)[…]%0A$(lines 96 100)exit status 1"
same "marker: annotation" "::error title=Demo::$expected" "$(annotation_for "$body")"
run run Demo bash -c "$body; exit 1"
same "marker: all the output is passed through" 101 "$(wc -l <<<"$output" | tr -d ' ')"

# The excerpt reaches the end of the output: no gap, no repetition.
body='for i in $(seq 60); do if [ $i = 20 ]; then echo "error[E0308]: x"; else echo "line $i"; fi; done'
expected="error[E0308]: x%0A$(lines 21 60)exit status 1"
same "marker near the end: annotation" "::error title=Demo::$expected" "$(annotation_for "$body")"

# A few lines remain after the excerpt: they follow it without a gap.
body='for i in $(seq 47); do if [ $i = 10 ]; then echo "---- t ----"; else echo "line $i"; fi; done'
expected="---- t ----%0A$(lines 11 47)exit status 1"
same "marker, short rest: annotation" "::error title=Demo::$expected" "$(annotation_for "$body")"

# The other markers.
body='echo before; echo "thread '"'main'"' panicked at src/lib.rs:1:1:"; echo after'
same "panic marker" "::error title=Demo::thread 'main' panicked at src/lib.rs:1:1:%0Aafter%0Aexit status 1" "$(annotation_for "$body")"
body='echo before; echo "---- tests::it stdout ----"; echo after'
same "stdout marker" "::error title=Demo::---- tests::it stdout ----%0Aafter%0Aexit status 1" "$(annotation_for "$body")"
body='echo before; echo "error: x"; echo "errors: not a marker"'
same "marker anchors" "::error title=Demo::error: x%0Aerrors: not a marker%0Aexit status 1" "$(annotation_for "$body")"
body='echo before; echo "  error: indented"; echo after'
same "indented is no marker" "::error title=Demo::before%0A  error: indented%0Aafter%0Aexit status 1" "$(annotation_for "$body")"

# Without a marker: the last 40 lines after a gap.
expected="[…]%0A$(lines 61 100)exit status 1"
same "no marker: annotation" "::error title=Demo::$expected" "$(annotation_for 'seq -f "line %g" 100')"
expected="$(lines 1 40)exit status 1"
same "40 lines: annotation" "::error title=Demo::$expected" "$(annotation_for 'seq -f "line %g" 40')"

# Colours and carriage returns are removed before the marker is looked for.
body="printf '\\033[1m\\033[31merror\\033[0m: x\\r\\n'; printf 'more\\n'"
same "colours" "::error title=Demo::error: x%0Amore%0Aexit status 1" "$(annotation_for "$body")"
run run Demo bash -c "printf '\\033[1merror\\033[0m: x\\n'; exit 1"
same "colours: the output keeps them" $'\033[1merror\033[0m: x' "$(head -n 1 <<<"$output")"

# Long lines are cut to 300 characters.
long="$(printf 'x%.0s' $(seq 400))"
cut="$(printf 'x%.0s' $(seq 300))…"
same "long line" "::error title=Demo::$cut%0Aexit status 1" "$(annotation_for "echo $long")"
same "300 characters stay" "::error title=Demo::${long:0:300}%0Aexit status 1" "$(annotation_for "echo ${long:0:300}")"
run run Demo bash -c "echo $long; exit 1"
same "long line: the output is whole" "$long" "$(head -n 1 <<<"$output")"

# The file mode: the content, a missing file and an empty file.
printf 'one\ntwo\n' >"$tmp_dir/log"
run file 'Server log' "$tmp_dir/log"
same "file: status" 0 "$status"
same "file: annotation" '::error title=Server log::one%0Atwo' "$output"
run file 'Server log' "$tmp_dir/missing"
same "missing file: status" 0 "$status"
same "missing file: annotation" '::error title=Server log::no output' "$output"
: >"$tmp_dir/empty"
run file 'Server log' "$tmp_dir/empty"
same "empty file: status" 0 "$status"
same "empty file: annotation" '::error title=Server log::no output' "$output"
seq -f 'error: %g' 100 >"$tmp_dir/long-log"
run file 'Server log' "$tmp_dir/long-log"
same "file: no status line" "1" "$(grep -c '^::error title=Server log::error: 1%0Aerror: 2%0A' <<<"$output" || true)"
same "file: ends with the last line" "1" "$(grep -c 'error: 100$' <<<"$output" || true)"

# Wrong usage: a message on stderr and status 2.
for args in "" "run" "run Demo" "file Demo" "file Demo a b" "other Demo true"; do
  # shellcheck disable=SC2086 # the words of the case
  run $args
  same "usage '$args': status" 2 "$status"
  same "usage '$args': message" 1 "$(grep -c '^usage: ' <<<"$output" || true)"
done

echo "annotate tests: $checks checks, $failures failed"
if ((failures > 0)); then exit 1; fi
