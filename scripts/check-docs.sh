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
#    points to something that exists.
# f. Every decision record has exactly one `- Scope:` line in its header (the
#    lines before its first `## ` heading), with an allowed value: framework
#    or the name of a folder under games/. Its row in docs/adr/README.md (a
#    table row whose first link is the record) sits under the matching
#    heading: `## Framework`, or a `## ` heading that ends with (`<game>`).
#
# A link to a record or entry that does not exist is a broken link: check e
# reports it, in the index like anywhere else. Check e skips:
#
# - fenced code blocks and inline code (per line);
# - targets with a URL scheme (http://, https://, mailto:, ...), targets that
#   are only an #anchor, targets with spaces and targets that look like code
#   (with { or <);
# - the #anchor of a target: only the path is checked. A target that starts
#   with / is taken from the repository root, as GitHub does.
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

# Prints "line number<TAB>target" for each Markdown link [text](target) of
# file $1 outside fenced code blocks and inline code.
extract_links() {
  awk '
    # The leading run of one fence character (` or ~) of s, or "".
    function fence_run(s,    c, n) {
      c = substr(s, 1, 1)
      if (c != "`" && c != "~") return ""
      n = 0
      while (substr(s, n + 1, 1) == c) n++
      return substr(s, 1, n)
    }
    # s without its inline code: a run of n backquotes up to the next run of
    # exactly n. An unmatched run is kept as it is.
    function strip_code(s,    out, len, i, j, run, k, found) {
      out = ""
      len = length(s)
      i = 1
      while (i <= len) {
        if (substr(s, i, 1) != "`") {
          out = out substr(s, i, 1)
          i++
          continue
        }
        run = 0
        while (substr(s, i + run, 1) == "`") run++
        j = i + run
        found = 0
        while (j <= len) {
          if (substr(s, j, 1) != "`") {
            j++
            continue
          }
          k = 0
          while (substr(s, j + k, 1) == "`") k++
          if (k == run) {
            found = 1
            break
          }
          j += k
        }
        if (found) {
          out = out " "
          i = j + run
        } else {
          out = out substr(s, i, run)
          i += run
        }
      }
      return out
    }
    {
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
      # ```code``` at the start of a line is inline code.
      if (length(run) >= 3 && (substr(run, 1, 1) == "~" || index(rest, "`") == 0)) {
        fence = run
        next
      }
      code = strip_code($0)
      while (match(code, /\]\([^)]*\)/)) {
        printf "%d\t%s\n", NR, substr(code, RSTART + 2, RLENGTH - 3)
        code = substr(code, RSTART + RLENGTH)
      }
    }
  ' "$1"
}

# Prints $1 as a path from the repository root: "." and empty parts dropped,
# ".." resolved. A path that goes above the root keeps its leading "..".
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
    echo .
  else
    (
      IFS=/
      echo "${out[*]}"
    )
  fi
}

# Prints "line number<TAB>repository path<TAB>target as written" for each
# link of Markdown file $1 that check e covers.
links_of() {
  local file="$1" dir line target path
  dir="$(dirname "$file")"
  while IFS=$'\t' read -r line target; do
    case $target in
      '' | '#'* | *' '* | *'{'* | *'<'*) continue ;;
    esac
    [[ $target =~ ^[A-Za-z][A-Za-z0-9+.-]*: ]] && continue
    path="${target%%#*}"
    if [[ $path == /* ]]; then
      path="$(normalize "$path")"
    else
      path="$(normalize "$dir/$path")"
    fi
    printf '%s\t%s\t%s\n' "$line" "$path" "$target"
  done < <(extract_links "$file")
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
  local index=docs/adr/README.md record line value valid heading in_fence=0 path headings expected dir
  local row_link='\]\(([^)]*)\)'
  local -a allowed=(framework) scope_lines
  local -A scope_of=() rows=() linked=()
  for dir in games/*/; do
    allowed+=("$(basename "$dir")")
  done

  # The scope line of each record's header.
  for record in docs/adr/[0-9][0-9][0-9][0-9]-*.md; do
    scope_lines=()
    while IFS= read -r line; do
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

  # The heading each record's row sits under. Check a reports a missing index.
  [[ -f $index ]] || return 0
  heading=""
  while IFS= read -r line; do
    if [[ $line =~ ^[[:space:]]*(\`\`\`|~~~) ]]; then
      in_fence=$((1 - in_fence))
    elif ((in_fence)); then
      continue
    elif [[ $line == '## '* ]]; then
      heading="${line:3}"
      heading="${heading%%+([[:space:]])}"
    elif [[ $line == '|'* && $line =~ $row_link ]]; then
      path="$(normalize "${index%/*}/${BASH_REMATCH[1]%%#*}")"
      rows["$path"]+="$heading"$'\n'
    fi
  done <"$index"
  while IFS=$'\t' read -r _ path _; do
    linked["$path"]=1
  done < <(links_of "$index")

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

# e. Relative links.
files=0
links=0
while IFS= read -r file; do
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
done < <(git ls-files '*.md')

# f. Decision records' scopes and index groups.
check_adr_scopes

if ((problems > 0)); then
  echo "$problems problem(s) in the documentation" >&2
  exit 1
fi
echo "ok   documentation: $links relative links in $files Markdown files; indexes, scopes and status lines match"
