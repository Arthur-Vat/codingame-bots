#!/usr/bin/env bash
# Tests scripts/pr-hygiene.sh without the network: the title rules, the labels
# of each path, and the pr mode against a stand-in for `gh` that answers from
# JSON files and logs what would change. Needs jq (the stand-in applies the
# script's --jq filters). Usage: scripts/tests/pr-hygiene-test.sh
# shellcheck disable=SC2016 # the backquotes of the reasons are literal
set -euo pipefail

cd "$(dirname "$0")/../.."
script=scripts/pr-hygiene.sh
unset GITHUB_ACTIONS GAMES_DIR
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# The title passes.
title_ok() { # title_ok <title>
  local output
  checks=$((checks + 1))
  if ! output="$("$script" title "$1" 2>&1)"; then fail "should pass: $1 ($output)"; fi
}

# The title fails with exit status 1 and a reason that contains the text.
title_bad() { # title_bad <title> <text of the reason>
  local output status=0
  checks=$((checks + 1))
  output="$("$script" title "$1" 2>&1)" || status=$?
  if ((status != 1)); then
    fail "should fail with status 1, got $status: $1"
  elif [[ $output != *"$2"* ]]; then
    fail "the reason for '$1' lacks '$2': $output"
  fi
}

# The labels printed for the arguments, space separated, are the expected ones.
labels_are() { # labels_are <expected labels> <arguments of the labels mode>...
  local expected="$1" actual
  shift
  checks=$((checks + 1))
  actual="$("$script" labels "$@" | tr '\n' ' ')"
  actual="${actual% }"
  if [[ $actual != "$expected" ]]; then fail "labels of '$*': expected '$expected', got '$actual'"; fi
}

same() { # same <what> <expected> <actual>
  checks=$((checks + 1))
  if [[ $2 != "$3" ]]; then fail "$1: expected '$2', got '$3'"; fi
}

# Titles that pass: the examples of the convention, every type, every scope.
title_ok 'feat(uttt): pattern move policy (E015, uttt-v010)'
title_ok 'docs(adr): 0020, in Legend the SPRT plays at full time'
title_ok 'ci(ci,uttt): label pull requests by path'
for type in feat fix docs test ci refactor perf chore build revert; do
  title_ok "$type(uttt): a summary"
done
for scope in uttt core search arena bundler ci adr docs agents repo; do
  title_ok "fix($scope): a summary"
done
title_ok 'chore(core,search,arena,bundler,ci,adr,docs,agents,repo,uttt): every scope'
title_ok "fix(uttt): $(printf 'a%.0s' {1..89})" # 100 characters

# Titles that fail, and why.
title_bad 'Update README' 'expected `type(scope): summary`'
title_bad 'feat: no scope' 'the scope is required'
title_bad 'feat(foo): unknown scope' 'unknown scope `foo`'
title_bad 'feat(uttt,foo,bar): two unknown scopes' 'unknown scope `foo` `bar`'
title_bad 'feat(uttt): ends with a period.' 'must not end with a period'
title_bad 'feat(uttt): ...' 'must not end with a period'
title_bad '' 'the title is empty'
title_bad 'wip(uttt): a summary' 'unknown type `wip`'
title_bad 'Feat(uttt): a summary' 'unknown type `Feat`'
title_bad 'feat(): a summary' 'the scope is empty'
title_bad 'feat(uttt, core): a summary' 'without spaces'
title_bad 'feat(uttt,): a summary' 'empty scope'
title_bad 'feat(,uttt): a summary' 'empty scope'
title_bad 'feat(uttt,,core): a summary' 'empty scope'
title_bad 'feat(uttt):a summary' 'expected `type(scope): summary`'
title_bad 'feat(uttt)!: a summary' 'expected `type(scope): summary`'
title_bad 'feat(uttt): ' 'the summary is empty'
title_bad 'feat(uttt):  two spaces' 'start or end with a space'
title_bad "fix(uttt): $(printf 'a%.0s' {1..90})" 'the title has 101 characters, at most 100'
title_bad 'Merge pull request #40 from owner/claude/branch' 'expected `type(scope): summary`'
# Every problem is reported, not only the first.
title_bad 'Wip(foo): x.' 'unknown type `Wip`'
title_bad 'Wip(foo): x.' 'unknown scope `foo`'
title_bad 'Wip(foo): x.' 'must not end with a period'

# A scope is any directory of the games directory.
mkdir -p "$tmp_dir/games/alpha"
checks=$((checks + 2))
if ! GAMES_DIR="$tmp_dir/games" "$script" title 'feat(alpha): a summary' >/dev/null 2>&1; then
  fail "a game of the games directory should be a scope"
fi
if GAMES_DIR="$tmp_dir/games" "$script" title 'feat(uttt): a summary' >/dev/null 2>&1; then
  fail "a game outside the games directory should not be a scope"
fi

# In a workflow the reason is an error annotation, where a % is escaped.
checks=$((checks + 1))
output="$(GITHUB_ACTIONS=true "$script" title 'feat(10%): a summary' 2>&1)" || true
if [[ $output != '::error title=Pull request title::unknown scope `10%25`, use one of: '* ]]; then
  fail "the annotation of a title: $output"
fi

# Labels, one rule at a time.
labels_are ''
labels_are 'game:uttt' games/uttt/bots/mcts/src/main.rs
labels_are 'framework' crates/cg-core/src/lib.rs
labels_are 'framework' ./crates/cg-core/src/lib.rs
labels_are 'ci' .github/workflows/ci.yml scripts/league.sh
labels_are 'ci' scripts/tests/pr-hygiene-test.sh
labels_are 'adr docs-only' docs/adr/0022-pull-requests.md
labels_are 'agents' .claude/settings.json
labels_are 'agents docs-only' .claude/agents/reviewer.md
labels_are 'game:uttt' games/uttt/releases/uttt-v011.rs
labels_are 'game:uttt release' games/uttt/releases/uttt-v011.rs --added games/uttt/releases/uttt-v011.rs
labels_are 'game:uttt release' --added games/uttt/releases/uttt-v011.rs
labels_are 'game:uttt framework' games/uttt/releases/uttt-v010.rs crates/cg-core/src/lib.rs --added
labels_are 'game:uttt experiment docs-only' games/uttt/journal/E017-new-idea.md
labels_are 'game:uttt docs-only' games/uttt/journal/README.md games/uttt/journal/template.md
labels_are 'game:uttt' games/uttt/journal/E017-new-idea.rs
labels_are 'game:uttt' games/uttt/journal/Ex-new-idea.md games/uttt/other/E017-x.rs
labels_are 'game:uttt training' games/uttt/training/runs/run-1/weights.bin
labels_are 'game:uttt training' games/uttt/trainer/src/main.rs
labels_are 'game:uttt training docs-only' games/uttt/training/README.md
labels_are 'ci training' .github/workflows/train.yml
labels_are 'ci' .github/workflows/sprt.yml
labels_are 'docs-only' README.md docs/ROADMAP.md
labels_are 'docs-only' docs/adr-notes.md
labels_are 'docs-only' games/README.md
labels_are '' README.md Cargo.toml
labels_are '' Cargo.toml deny.toml LICENSE-MIT
labels_are 'game:c4 game:uttt' games/uttt/engine/src/lib.rs games/c4/RULES.md
labels_are 'game:uttt framework ci adr' crates/cg-search/src/lib.rs games/uttt/bots/a.rs \
  scripts/b.sh docs/adr/0001-license.md

# The pr mode, against a stand-in for gh. The pull request is number 7 of o/r.
stub_bin="$tmp_dir/bin"
mkdir -p "$stub_bin"
cat >"$stub_bin/gh" <<'STUB'
#!/usr/bin/env bash
# Answers `gh api` calls from the JSON files of $STUB_DIR; logs the changes.
set -euo pipefail
[[ ${1:-} == api ]] || { echo "stub gh: unsupported: $*" >&2; exit 2; }
shift
method=GET
url=""
filter="."
fields=()
while (($#)); do
  case "$1" in
    --paginate) ;;
    -X) method="$2"; shift ;;
    --jq) filter="$2"; shift ;;
    -f) fields+=("$2"); shift ;;
    *) url="$1" ;;
  esac
  shift
done
path="${url%%\?*}"
log="$STUB_DIR/calls.log"
case "$method $path" in
  "GET repos/o/r/pulls/7") file=pull.json ;;
  "GET repos/o/r/pulls/7/files") file=files.json ;;
  "GET repos/o/r/labels") file=repo-labels.json ;;
  "GET repos/o/r/issues/7/labels") file=pr-labels.json ;;
  "POST repos/o/r/labels")
    echo "create ${fields[*]}" >>"$log"
    exit 0
    ;;
  "POST repos/o/r/issues/7/labels")
    if [[ -n ${STUB_FAIL_ADD:-} ]]; then echo "gh: HTTP 403" >&2; exit 1; fi
    echo "add ${fields[*]}" >>"$log"
    echo '[]'
    exit 0
    ;;
  "DELETE repos/o/r/issues/7/labels/"*)
    echo "remove ${path##*/}" >>"$log"
    echo '[]'
    exit 0
    ;;
  *)
    echo "stub gh: unexpected call: $method $path" >&2
    exit 2
    ;;
esac
jq -r "$filter" "$STUB_DIR/$file"
STUB
chmod +x "$stub_bin/gh"

# run_pr <case name>: runs the pr mode on the files of $tmp_dir/<case name>;
# sets $output and $status, and $calls to the log of the changes.
run_pr() {
  local dir="$tmp_dir/$1"
  rm -f "$dir/calls.log"
  status=0
  output="$(PATH="$stub_bin:$PATH" STUB_DIR="$dir" REPO=o/r "$script" pr 7 2>&1)" || status=$?
  calls=""
  if [[ -f $dir/calls.log ]]; then
    # Without the descriptions, which the labels tests do not repeat.
    calls="$(sed 's/ description=.*//' "$dir/calls.log")"
  fi
  if [[ -n ${VERBOSE:-} ]]; then
    printf -- '--- %s (status %s)\n%s\n--- changes\n%s\n' "$1" "$status" "$output" "$calls"
  fi
}

# A valid title. The pull request adds a release, changes a journal entry,
# renames a crate file and deletes a script: game:uttt, framework, ci,
# release, experiment. ci and game:uttt are already on it; docs-only and
# agents no longer apply; bug is not ours.
mkdir -p "$tmp_dir/valid"
cat >"$tmp_dir/valid/pull.json" <<'JSON'
{"number": 7, "title": "feat(uttt): pattern move policy"}
JSON
cat >"$tmp_dir/valid/files.json" <<'JSON'
[
  {"filename": "games/uttt/releases/uttt-v011.rs", "status": "added"},
  {"filename": "games/uttt/journal/E017-foo.md", "status": "modified"},
  {"filename": "crates/cg-core/src/b.rs", "previous_filename": "crates/cg-core/src/a.rs", "status": "renamed"},
  {"filename": "scripts/old.sh", "status": "removed"}
]
JSON
echo '[{"name": "ci"}, {"name": "bug"}, {"name": "game:uttt"}, {"name": "docs-only"}, {"name": "agents"}]' \
  >"$tmp_dir/valid/repo-labels.json"
echo '[{"name": "ci"}, {"name": "docs-only"}, {"name": "bug"}, {"name": "game:uttt"}, {"name": "agents"}]' \
  >"$tmp_dir/valid/pr-labels.json"
run_pr valid
same "valid title: status" 0 "$status"
same "valid title: changes" "create name=framework color=ededed
create name=release color=ededed
create name=experiment color=ededed
add labels[]=framework labels[]=release labels[]=experiment
remove docs-only
remove agents" "$calls"
same "valid title: no annotation" "" "$(grep '^::error' <<<"$output" || true)"

# An invalid title: the labels are still applied, then the annotation and exit 1.
mkdir -p "$tmp_dir/invalid"
echo '{"number": 7, "title": "Update README"}' >"$tmp_dir/invalid/pull.json"
echo '[{"filename": "README.md", "status": "modified"}]' >"$tmp_dir/invalid/files.json"
echo '[]' >"$tmp_dir/invalid/repo-labels.json"
echo '[{"name": "bug"}, {"name": "game:uttt"}]' >"$tmp_dir/invalid/pr-labels.json"
run_pr invalid
same "invalid title: status" 1 "$status"
same "invalid title: changes" "create name=docs-only color=ededed
add labels[]=docs-only
remove game%3Auttt" "$calls"
same "invalid title: annotation" '::error title=Pull request title::expected `type(scope): summary`, for example `feat(uttt): pattern move policy`' \
  "$(grep '^::error' <<<"$output")"

# Nothing to change: no call that writes. A label that differs in case from
# the repository's is the same label.
mkdir -p "$tmp_dir/steady"
echo '{"number": 7, "title": "ci(ci): check titles"}' >"$tmp_dir/steady/pull.json"
echo '[{"filename": "scripts/pr-hygiene.sh", "status": "added"}]' >"$tmp_dir/steady/files.json"
echo '[{"name": "CI"}, {"name": "bug"}]' >"$tmp_dir/steady/repo-labels.json"
echo '[{"name": "CI"}, {"name": "bug"}]' >"$tmp_dir/steady/pr-labels.json"
run_pr steady
same "steady: status" 0 "$status"
same "steady: changes" "" "$calls"

# A label that exists in another case is added, not created.
echo '[{"name": "Ci"}]' >"$tmp_dir/steady/repo-labels.json"
echo '[{"name": "bug"}]' >"$tmp_dir/steady/pr-labels.json"
run_pr steady
same "case: changes" "add labels[]=ci" "$calls"

# The labelling fails: the title is checked all the same, and the status is 1.
mkdir -p "$tmp_dir/refused"
cp "$tmp_dir/invalid/"*.json "$tmp_dir/refused/"
export STUB_FAIL_ADD=1
run_pr refused
unset STUB_FAIL_ADD
same "refused: status" 1 "$status"
same "refused: label annotation" 1 "$(grep -c '^::error title=Pull request labels::' <<<"$output" || true)"
same "refused: title annotation" 1 "$(grep -c '^::error title=Pull request title::' <<<"$output" || true)"

# A pr number that is not a number is refused.
checks=$((checks + 1))
if REPO=o/r "$script" pr seven >/dev/null 2>&1; then fail "pr needs a number"; fi

echo "pr-hygiene tests: $checks checks, $failures failed"
if ((failures > 0)); then exit 1; fi
