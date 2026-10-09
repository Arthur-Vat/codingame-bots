#!/usr/bin/env bash
# Checks that the documentation's indexes, status lines and links match the
# repository. Prints one line per problem, `FAIL <file>: <problem>`, and exits
# 1 if there is any. Needs no network and no Rust toolchain.
#
# a. docs/adr/README.md links every record docs/adr/NNNN-*.md (not README.md
#    or template.md).
# b. For each game with a journal/: journal/README.md links every entry
#    journal/E<digits>-*.md.
# c. For each game with training/runs/: training/README.md links every
#    runs/<id>/report.md.
# d. For each game with releases/: let LATEST be the highest-numbered
#    releases/<game>-vNNN.rs. games/<game>/README.md must have a line with
#    **Current release:** [`LATEST`], and the root README.md must mention
#    `LATEST` in backquotes.
# e. Every relative link [text](target) of every tracked Markdown file
#    points to something that exists. It is a failure if git lists no
#    Markdown file (outside a git checkout, say).
# f. Every decision record has exactly one `- Scope:` line in its header (the
#    lines before its first `## ` heading), with an allowed value: framework
#    or the name of a folder under games/. Its row in docs/adr/README.md (a
#    table row whose first link is the record) sits under the matching
#    heading: `## Framework`, or a `## ` heading that ends with (`<game>`).
# g. Every agent of Claude's sessions, .claude/agents/*.md, has a header
#    (between `---` lines) with non-empty name:, description:, model: and
#    tools: lines: without tools:, an agent gets every tool of the session,
#    whose descriptions cost tokens on each of its steps.
# h. Every workflow, .github/workflows/NAME.yml, is named (as NAME.yml) in
#    docs/ARCHITECTURE.md and in README.md, and every script,
#    scripts/NAME.sh and scripts/lib/NAME.sh, in docs/ARCHITECTURE.md. A name
#    counts when it stands alone: `ci.yml` is not found in `pci.yml`.
#
# A link to a record or entry that does not exist is a broken link: check e
# reports it, in the index like anywhere else.
#
# Check e reads links like Markdown does:
#
# - [text](target), [text](target "Title"), [text](target 'Title') and
#   [text](target (Title)): the title is ignored, the target is checked;
#   parentheses in a target are allowed when balanced;
# - the target is percent-decoded (sp%20ace is "sp ace") and loses its
#   #anchor: only the path is checked. A target that starts with / is taken
#   from the repository root, as GitHub does;
# - CRLF line endings are accepted.
#
# It skips, line by line:
#
# - fenced code blocks (``` or ~~~, at any indentation: a nested list item
#   indented by 4 spaces is not code) and inline code;
# - HTML comments (<!-- ... -->, also over several lines);
# - targets with a URL scheme (http://, https://, mailto:, ...), targets that
#   are only an #anchor, targets with spaces and targets that look like code
#   (with { or <).
#
# Usage: scripts/check-docs.sh
set -euo pipefail

cd "$(dirname "$0")/.."
export LC_ALL=C
shopt -s extglob nullglob

problems=0
fail() {
  echo "FAIL $1: $2"
  problems=$((problems + 1))
}

# Reads Markdown file $1 and prints what the checks need, tab separated: a
# kind (H or L), a line number, a flag and a text.
#
# - H: a "## " heading outside code and comments; the flag is 0 and the text
#   is the heading, with its inline code and without trailing blanks.
# - L: a link [text](target) outside code and comments; the flag is 1 when the
#   line is a table row and the text is the target as written, without its
#   title. A link with an empty target is not printed.
scan_markdown() {
  awk '
    # The leading run of one fence character (` or ~) of s, or "".
    function fence_run(s,    c, n) {
      c = substr(s, 1, 1)
      if (c != "`" && c != "~") return ""
      n = 0
      while (substr(s, n + 1, 1) == c) n++
      return substr(s, 1, n)
    }
    # The length of the run of backquotes of s that starts at i.
    function run_at(s, i,    n) {
      n = 0
      while (substr(s, i + n, 1) == "`") n++
      return n
    }
    # For the run of n backquotes that opens at i, the index after the next
    # run of exactly n backquotes, or 0 if there is none: the end of a code
    # span.
    function span_end(s, i, n,    len, j, k) {
      len = length(s)
      j = i + n
      while (j <= len) {
        if (substr(s, j, 1) != "`") {
          j++
          continue
        }
        k = run_at(s, j)
        if (k == n) return j + k
        j += k
      }
      return 0
    }
    # s without its inline code (each span becomes a space). An unmatched run
    # is kept as it is.
    function strip_code(s,    out, len, i, n, e) {
      out = ""
      len = length(s)
      i = 1
      while (i <= len) {
        if (substr(s, i, 1) != "`") {
          out = out substr(s, i, 1)
          i++
          continue
        }
        n = run_at(s, i)
        e = span_end(s, i, n)
        if (e == 0) {
          out = out substr(s, i, n)
          i += n
        } else {
          out = out " "
          i = e
        }
      }
      return out
    }
    # s without its HTML comments, which may start on an earlier line
    # (in_comment) or go on after this one. Inline code is kept, and a comment
    # marker inside it starts nothing.
    function strip_comments(s,    out, len, i, n, e, stop) {
      out = ""
      len = length(s)
      i = 1
      while (i <= len) {
        if (in_comment) {
          stop = index(substr(s, i), "-->")
          if (stop == 0) return out
          i += stop + 2
          in_comment = 0
        } else if (substr(s, i, 1) == "`") {
          n = run_at(s, i)
          e = span_end(s, i, n)
          if (e == 0) e = i + n
          out = out substr(s, i, e - i)
          i = e
        } else if (substr(s, i, 4) == "<!--") {
          in_comment = 1
          i += 4
        } else {
          out = out substr(s, i, 1)
          i++
        }
      }
      return out
    }
    function skip_blanks(s, i) {
      while (substr(s, i, 1) == " " || substr(s, i, 1) == "\t") i++
      return i
    }
    # Reads the rest of a link, from index i just after "](": the target, an
    # optional title ("...", \047...\047 or (...)) and the closing ")".
    # Returns the target and sets link_end to the index after the ")", or
    # sets it to 0 if this is not a link. A <target> keeps its brackets.
    function parse_link(s, i,    len, start, depth, c, dest, j) {
      link_end = 0
      len = length(s)
      i = skip_blanks(s, i)
      if (substr(s, i, 1) == "<") {
        j = index(substr(s, i), ">")
        if (j == 0) return ""
        dest = substr(s, i, j)
        i += j
      } else {
        start = i
        depth = 0
        while (i <= len) {
          c = substr(s, i, 1)
          if (c == " " || c == "\t") break
          if (c == "(") {
            depth++
          } else if (c == ")") {
            if (depth == 0) break
            depth--
          }
          i++
        }
        dest = substr(s, start, i - start)
      }
      i = skip_blanks(s, i)
      c = substr(s, i, 1)
      if (c == "\"" || c == "\047") {
        j = index(substr(s, i + 1), c)
        if (j == 0) return ""
        i += j + 1
      } else if (c == "(") {
        j = index(substr(s, i + 1), ")")
        if (j == 0) return ""
        i += j + 1
      }
      i = skip_blanks(s, i)
      if (substr(s, i, 1) != ")") return ""
      link_end = i + 1
      return dest
    }
    {
      sub(/\r$/, "")
      s = $0
      sub(/^[ \t>]*/, "", s)
      run = fence_run(s)
      rest = substr(s, length(run) + 1)
      if (fence != "") {
        # Inside a fenced block: it ends at a run of the same character, at
        # least as long as the opening one, with nothing after it.
        if (length(run) >= length(fence) && substr(run, 1, 1) == substr(fence, 1, 1) && rest ~ /^[ \t]*$/) fence = ""
        next
      }
      # A backquote fence cannot have a backquote in its info string:
      # ```code``` at the start of a line is inline code. A line inside a
      # comment opens nothing.
      if (!in_comment && length(run) >= 3 && (substr(run, 1, 1) == "~" || index(rest, "`") == 0)) {
        fence = run
        next
      }
      kept = strip_comments($0)
      if (kept ~ /^## /) {
        heading = substr(kept, 4)
        sub(/[ \t]+$/, "", heading)
        printf "H\t%d\t0\t%s\n", NR, heading
      }
      row = (kept ~ /^[ \t]*\|/) ? 1 : 0
      code = strip_code(kept)
      pos = 1
      while ((p = index(substr(code, pos), "](")) > 0) {
        start = pos + p + 1
        dest = parse_link(code, start)
        if (link_end > 0) {
          if (dest != "") printf "L\t%d\t%d\t%s\n", NR, row, dest
          pos = link_end
        } else {
          pos = start
        }
      }
    }
  ' "$1"
}

# Sets `normalized` to $1 as a path from the repository root: "." and empty
# parts dropped, ".." resolved. A path that goes above the root keeps its
# leading "..".
normalize() {
  local -a parts out=()
  local part
  IFS=/ read -ra parts <<<"$1"
  for part in "${parts[@]}"; do
    case $part in
      '' | .) ;;
      ..)
        if ((${#out[@]} > 0)) && [[ ${out[-1]} != .. ]]; then
          unset 'out[-1]'
        else
          out+=(..)
        fi
        ;;
      *) out+=("$part") ;;
    esac
  done
  if ((${#out[@]} == 0)); then
    normalized=.
  else
    local IFS=/
    normalized="${out[*]}"
  fi
}

# Sets `decoded` to $1 with its %XX escapes replaced by the bytes they stand
# for. A % that does not start an escape stays as it is.
percent_decode() {
  local rest="$1" plain="" byte
  while [[ $rest == *%* ]]; do
    plain+="${rest%%\%*}"
    rest="${rest#*%}"
    if [[ $rest =~ ^[0-9A-Fa-f]{2} ]]; then
      printf -v byte '%b' "\\x${rest:0:2}"
      plain+="$byte"
      rest="${rest:2}"
    else
      plain+="%"
    fi
  done
  decoded="$plain$rest"
}

# Sets `resolved` to the repository path that link target $2 of Markdown file
# $1 points to and returns 0, or returns 1 if check e skips the link.
resolve_link() {
  local file="$1" target="$2" path dir=.
  case $target in
    '' | '#'* | *' '* | *'{'* | *'<'*) return 1 ;;
  esac
  if [[ $target =~ ^[A-Za-z][A-Za-z0-9+.-]*: ]]; then
    return 1
  fi
  percent_decode "${target%%#*}"
  path="$decoded"
  if [[ $path != /* && $file == */* ]]; then
    dir="${file%/*}"
  fi
  normalize "$dir/$path"
  resolved="$normalized"
}

# Prints "line number<TAB>repository path<TAB>target as written" for each
# link of Markdown file $1 that check e covers.
links_of() {
  local kind line row target
  while IFS=$'\t' read -r kind line row target; do
    if [[ $kind == L ]] && resolve_link "$1" "$target"; then
      printf '%s\t%s\t%s\n' "$line" "$resolved" "$target"
    fi
  done < <(scan_markdown "$1")
}

# check_index INDEX KIND ENTRY...: INDEX links every ENTRY (a file of the
# repository, of the given KIND).
check_index() {
  local index="$1" kind="$2" entry path
  shift 2
  local -A linked=()
  if [[ ! -f $index ]]; then
    fail "$index" "missing, but it must link every $kind"
    return
  fi
  while IFS=$'\t' read -r _ path _; do
    linked["$path"]=1
  done < <(links_of "$index")
  for entry in "$@"; do
    if [[ -z ${linked[$entry]:-} ]]; then
      fail "$index" "does not link the $kind $entry"
    fi
  done
}

# Checks f: the scope of each decision record, and where the index lists it.
check_adr_scopes() {
  local index=docs/adr/README.md record line value valid heading path headings expected dir
  local kind row target
  local -a allowed=(framework) scope_lines
  local -A scope_of=() rows=() linked=() row_seen=()
  for dir in games/*/; do
    allowed+=("$(basename "$dir")")
  done

  # The scope line of each record's header.
  for record in docs/adr/[0-9][0-9][0-9][0-9]-*.md; do
    scope_lines=()
    while IFS= read -r line; do
      line="${line%$'\r'}"
      if [[ $line == '## '* ]]; then
        break
      fi
      if [[ $line == '- Scope:'* ]]; then
        value="${line#- Scope:}"
        value="${value##+([[:space:]])}"
        value="${value%%+([[:space:]])}"
        scope_lines+=("$value")
      fi
    done <"$record"
    if ((${#scope_lines[@]} == 0)); then
      fail "$record" "no '- Scope:' line in the header"
      continue
    elif ((${#scope_lines[@]} > 1)); then
      fail "$record" "${#scope_lines[@]} '- Scope:' lines in the header, expected one"
      continue
    fi
    valid=0
    for value in "${allowed[@]}"; do
      if [[ $value == "${scope_lines[0]}" ]]; then
        valid=1
      fi
    done
    if ((valid)); then
      scope_of["$record"]="${scope_lines[0]}"
    else
      fail "$record" "scope '${scope_lines[0]}' is not one of: ${allowed[*]}"
    fi
  done

  # The heading each record's row sits under: the first link of a table row.
  # Check a reports a missing index.
  [[ -f $index ]] || return 0
  heading=""
  while IFS=$'\t' read -r kind line row target; do
    if [[ $kind == H ]]; then
      heading="$target"
    elif resolve_link "$index" "$target"; then
      path="$resolved"
      linked["$path"]=1
      if [[ $row == 1 && -z ${row_seen[$line]:-} ]]; then
        row_seen["$line"]=1
        rows["$path"]+="$heading"$'\n'
      fi
    fi
  done < <(scan_markdown "$index")

  for record in docs/adr/[0-9][0-9][0-9][0-9]-*.md; do
    [[ -n ${scope_of[$record]:-} ]] || continue
    value="${scope_of[$record]}"
    headings="${rows[$record]:-}"
    if [[ $value == framework ]]; then
      expected="'## Framework'"
    else
      expected="a '## ' heading ending with (\`$value\`)"
    fi
    if [[ -z $headings ]]; then
      # A record not linked at all is check a's problem.
      if [[ -n ${linked[$record]:-} ]]; then
        fail "$index" "$record has no table row; with scope $value, it belongs under $expected"
      fi
      continue
    fi
    while IFS= read -r heading; do
      if [[ $value == framework && $heading == Framework ]] ||
        [[ $value != framework && $heading == *"(\`$value\`)" ]]; then
        continue
      fi
      fail "$index" "$record has scope $value, so its row belongs under $expected, not under '## ${heading:-(no heading)}'"
    done <<<"${headings%$'\n'}"
  done
}

# a. Architecture decision records.
check_index docs/adr/README.md "record" docs/adr/[0-9][0-9][0-9][0-9]-*.md

for dir in games/*/; do
  game="$(basename "$dir")"

  # b. Journal entries.
  if [[ -d games/$game/journal ]]; then
    check_index "games/$game/journal/README.md" "journal entry" \
      "games/$game/journal/"E+([0-9])-*.md
  fi

  # c. Training run reports.
  if [[ -d games/$game/training/runs ]]; then
    check_index "games/$game/training/README.md" "training report" \
      "games/$game/training/runs/"*/report.md
  fi

  # d. The latest release in the status lines.
  if [[ -d games/$game/releases ]]; then
    latest=""
    latest_number=-1
    for release in "games/$game/releases/$game-v"+([0-9]).rs; do
      number="${release##*-v}"
      number=$((10#${number%.rs}))
      if ((number > latest_number)); then
        latest_number=$number
        latest="$(basename "$release" .rs)"
      fi
    done
    if [[ -n $latest ]]; then
      if [[ ! -f games/$game/README.md ]]; then
        fail "games/$game/README.md" "missing, but it must state the current release $latest"
      elif ! grep -qF -- "**Current release:** [\`$latest\`]" "games/$game/README.md"; then
        fail "games/$game/README.md" "no line with **Current release:** [\`$latest\`]"
      fi
      if ! grep -qF -- "\`$latest\`" README.md; then
        fail README.md "does not mention the current release \`$latest\` of $game"
      fi
    fi
  fi
done

# e. Relative links. The names are NUL-separated: git quotes the odd ones
# otherwise.
files=0
links=0
mapfile -d '' -t markdown_files < <(git ls-files -z '*.md')
if ((${#markdown_files[@]} == 0)); then
  fail . "git lists no Markdown file; run this from a git checkout of the repository"
fi
for file in "${markdown_files[@]}"; do
  [[ -f $file ]] || continue
  files=$((files + 1))
  while IFS=$'\t' read -r line path target; do
    links=$((links + 1))
    if [[ $path == .. || $path == ../* ]]; then
      fail "$file" "line $line: broken link $target (outside the repository)"
    elif [[ ! -e $path ]]; then
      fail "$file" "line $line: broken link $target (no $path)"
    fi
  done < <(links_of "$file")
done

# f. Decision records' scopes and index groups.
check_adr_scopes

# g. Agents declare their model and their tools.
agents=0
for file in .claude/agents/*.md; do
  agents=$((agents + 1))
  # The header's lines, then CLOSED once its closing --- line is found.
  header="$(awk '{ sub(/\r$/, "") } NR == 1 { if ($0 != "---") exit; next } $0 == "---" { print "CLOSED"; exit } { print }' "$file")"
  if [[ $header != *CLOSED ]]; then
    fail "$file" "no header between two '---' lines"
    continue
  fi
  for key in name description model tools; do
    grep -q "^$key:[[:space:]]*[^[:space:]]" <<<"$header" || fail "$file" "no '$key:' line in the header"
  done
done

# h. Workflows and scripts are listed in the architecture (and workflows in
# the README).
listed_in() {
  local pattern
  pattern="(^|[^A-Za-z0-9_.-])${1//./\\.}($|[^A-Za-z0-9_-])"
  grep -qE -- "$pattern" "$2"
}
workflows=0
for file in .github/workflows/*.yml; do
  workflows=$((workflows + 1))
  for doc in docs/ARCHITECTURE.md README.md; do
    listed_in "$(basename "$file")" "$doc" || fail "$doc" "does not list the workflow $(basename "$file")"
  done
done
scripts=0
for file in scripts/*.sh scripts/lib/*.sh; do
  scripts=$((scripts + 1))
  listed_in "$(basename "$file")" docs/ARCHITECTURE.md || fail docs/ARCHITECTURE.md "does not list the script $file"
done

if ((problems > 0)); then
  echo "$problems problem(s) in the documentation" >&2
  exit 1
fi
echo "ok   documentation: $links relative links in $files Markdown files; indexes, scopes and status lines match; $agents agents declare model and tools; $workflows workflows and $scripts scripts are listed"
