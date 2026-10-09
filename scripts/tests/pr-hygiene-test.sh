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
title_ok 'ci(workflows,uttt): label pull requests by path'
for type in feat fix docs test ci refactor perf chore build revert; do
  title_ok "$type(uttt): a summary"
done
for scope in uttt core search arena bundler workflows scripts adr docs agents repo; do
  title_ok "fix($scope): a summary"
done
title_ok 'chore(core,search,arena,bundler,workflows,scripts,adr,docs,agents,repo,uttt): every scope'
title_ok "fix(uttt): $(printf 'a%.0s' {1..89})" # 100 characters

# Titles that GitHub's revert button writes are exempt, however they look.
title_ok 'Revert "feat(uttt): pattern move policy"'
title_ok 'Revert "Update README"'
title_ok 'Revert "Revert "feat(uttt): pattern move policy""'
title_ok "Revert \"$(printf 'a%.0s' {1..120})\"" # no length limit either

# Titles that fail, and why.
title_bad 'Update README' 'expected `type(scope): summary`'
title_bad 'feat: no scope' 'the scope is required'
title_bad 'feat(foo): unknown scope' 'unknown scope `foo`'
# The scope ci is gone (the type stays), and so is a scope that names no folder.
title_bad 'ci(ci): a summary' 'unknown scope `ci`'
title_bad 'ci(ci,uttt): a summary' 'unknown scope `ci`'
title_bad 'ci(train): a summary' 'unknown scope `train`'
title_bad 'ci: a summary' 'the scope is required'
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
title_bad 'feat(uttt)!: a summary' 'no `!` marker: say so in the description'
title_bad 'feat!: a summary' 'no `!` marker'
title_bad 'feat!: a summary' 'the scope is required'
# Scopes are distinct.
title_bad 'feat(uttt,uttt): a summary' 'scopes must be distinct, but `uttt` appears twice'
title_bad 'feat(core,uttt,core): a summary' 'scopes must be distinct, but `core` appears twice'
title_bad 'feat(uttt,foo,foo): a summary' 'unknown scope `foo` `foo`'
# Only a revert that GitHub or git writes is exempt: Revert "...".
title_bad 'Revert ""' 'expected `type(scope): summary`'
title_bad 'Revert the change' 'expected `type(scope): summary`'
title_bad 'Revert "feat(uttt): x" again' 'expected `type(scope): summary`'
title_bad 'revert "feat(uttt): x"' 'expected `type(scope): summary`'
title_bad 'feat(uttt): ' 'the summary is empty'
title_bad 'feat(uttt):  two spaces' 'start or end with a space'
title_bad "fix(uttt): $(printf 'a%.0s' {1..90})" 'the title has 101 characters, at most 100'
title_bad 'Merge pull request #40 from owner/claude/branch' 'expected `type(scope): summary`'
# Every problem is reported, not only the first.
title_bad 'Wip(foo): x.' 'unknown type `Wip`'
title_bad 'Wip(foo): x.' 'unknown scope `foo`'
title_bad 'Wip(foo): x.' 'must not end with a period'

# The limit counts characters, not bytes, in any locale. The titles below hold
# 2-, 3- and 4-byte characters: e acute, the euro sign and a smiley.
e_acute='\xc3\xa9'
euro='\xe2\x82\xac'
smiley='\xf0\x9f\x98\x80'
# shellcheck disable=SC2059 # the byte escapes are in the variables
accented_89="$(printf "${e_acute}%.0s" {1..40})$(printf "${euro}%.0s" {1..30})$(printf "${smiley}%.0s" {1..19})"
for locale in C POSIX C.UTF-8; do
  export LC_ALL="$locale"
  title_ok "fix(uttt): $accented_89" # 100 characters, 257 bytes
  title_bad "fix(uttt): ${accented_89}a" 'the title has 101 characters, at most 100'
done
unset LC_ALL

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
labels_are 'framework ci' .github/workflows/ci.yml scripts/league.sh
labels_are 'framework ci' scripts/tests/pr-hygiene-test.sh
labels_are 'framework adr docs-only' docs/adr/0022-scopes-and-names.md
labels_are 'framework agents' .claude/settings.json
labels_are 'framework agents docs-only' .claude/agents/pr-reviewer.md
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
labels_are 'framework ci training' .github/workflows/train.yml
labels_are 'framework ci' .github/workflows/sprt.yml
# Anything outside a game's folder is the framework's.
labels_are 'framework docs-only' README.md docs/ROADMAP.md
labels_are 'framework docs-only' docs/ARCHITECTURE.md
labels_are 'framework docs-only' docs/adr-notes.md
labels_are 'framework docs-only' games/README.md
labels_are 'framework docs-only' CLAUDE.md
labels_are 'framework' README.md Cargo.toml
labels_are 'framework' Cargo.toml Cargo.lock
labels_are 'framework' Cargo.toml deny.toml LICENSE-MIT
labels_are 'game:c4 game:uttt' games/uttt/engine/src/lib.rs games/c4/RULES.md
labels_are 'game:uttt framework docs-only' games/uttt/README.md docs/CODINGAME.md
# The pull request's own examples.
labels_are 'game:uttt experiment docs-only' games/uttt/journal/E017-x.md games/uttt/journal/README.md
labels_are 'game:uttt release' --added games/uttt/releases/uttt-v011.rs
# An incomplete list of files never gives docs-only.
labels_are 'framework' docs/ARCHITECTURE.md --partial
labels_are 'game:uttt experiment' --partial games/uttt/journal/E017-x.md
labels_are 'game:uttt' games/uttt/bots/a.rs --partial
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
{"number": 7, "title": "feat(uttt): pattern move policy", "changed_files": 4}
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
same "valid title: no annotation" "" "$(grep '^::' <<<"$output" || true)"

# A file renamed into a releases folder is a new release, like an added one;
# the one it comes from is a game's file too.
mkdir -p "$tmp_dir/renamed"
echo '{"number": 7, "title": "feat(uttt): promote the candidate", "changed_files": 1}' >"$tmp_dir/renamed/pull.json"
cat >"$tmp_dir/renamed/files.json" <<'JSON'
[{"filename": "games/uttt/releases/uttt-v012.rs", "previous_filename": "games/uttt/bots/mcts/candidate.rs", "status": "renamed"}]
JSON
echo '[]' >"$tmp_dir/renamed/repo-labels.json"
echo '[]' >"$tmp_dir/renamed/pr-labels.json"
run_pr renamed
same "renamed release: status" 0 "$status"
same "renamed release: changes" "create name=game:uttt color=ededed
create name=release color=ededed
add labels[]=game:uttt labels[]=release" "$calls"

# GitHub lists 3000 files at most. This pull request has more than the files
# listed: a warning, and no docs-only (it could change a Rust file).
mkdir -p "$tmp_dir/large"
echo '{"number": 7, "title": "docs(docs): rewrite the guides", "changed_files": 3001}' >"$tmp_dir/large/pull.json"
echo '[{"filename": "docs/a.md", "status": "modified"}, {"filename": "docs/b.md", "status": "modified"}]' \
  >"$tmp_dir/large/files.json"
echo '[{"name": "framework"}, {"name": "docs-only"}]' >"$tmp_dir/large/repo-labels.json"
echo '[{"name": "docs-only"}]' >"$tmp_dir/large/pr-labels.json"
run_pr large
same "large: status" 0 "$status"
same "large: changes" "add labels[]=framework
remove docs-only" "$calls"
same "large: warning" '::warning title=Pull request files::read 2 of 3001 changed files (GitHub lists 3000 at most): docs-only is not given and other labels may be missing' \
  "$(grep '^::' <<<"$output")"

# The list is complete when it is as long as changed_files: docs-only is given.
echo '{"number": 7, "title": "docs(docs): rewrite the guides", "changed_files": 2}' >"$tmp_dir/large/pull.json"
echo '[{"name": "framework"}]' >"$tmp_dir/large/repo-labels.json"
echo '[{"name": "framework"}]' >"$tmp_dir/large/pr-labels.json"
run_pr large
same "complete: changes" "create name=docs-only color=ededed
add labels[]=docs-only" "$calls"
same "complete: no warning" "" "$(grep '^::' <<<"$output" || true)"

# An invalid title: the labels are still applied, then the annotation and exit 1.
mkdir -p "$tmp_dir/invalid"
echo '{"number": 7, "title": "Update README"}' >"$tmp_dir/invalid/pull.json"
echo '[{"filename": "README.md", "status": "modified"}]' >"$tmp_dir/invalid/files.json"
echo '[]' >"$tmp_dir/invalid/repo-labels.json"
echo '[{"name": "bug"}, {"name": "game:uttt"}]' >"$tmp_dir/invalid/pr-labels.json"
run_pr invalid
same "invalid title: status" 1 "$status"
same "invalid title: changes" "create name=framework color=ededed
create name=docs-only color=ededed
add labels[]=framework labels[]=docs-only
remove game%3Auttt" "$calls"
same "invalid title: annotation" '::error title=Pull request title::expected `type(scope): summary`, for example `feat(uttt): pattern move policy`' \
  "$(grep '^::error' <<<"$output")"

# Nothing to change: no call that writes. A label that differs in case from
# the repository's is the same label.
mkdir -p "$tmp_dir/steady"
echo '{"number": 7, "title": "ci(scripts): check titles", "changed_files": 1}' >"$tmp_dir/steady/pull.json"
echo '[{"filename": "scripts/pr-hygiene.sh", "status": "added"}]' >"$tmp_dir/steady/files.json"
echo '[{"name": "CI"}, {"name": "bug"}, {"name": "framework"}]' >"$tmp_dir/steady/repo-labels.json"
echo '[{"name": "CI"}, {"name": "bug"}, {"name": "Framework"}]' >"$tmp_dir/steady/pr-labels.json"
run_pr steady
same "steady: status" 0 "$status"
same "steady: changes" "" "$calls"

# A label that exists in another case is added, not created.
echo '[{"name": "Ci"}, {"name": "framework"}]' >"$tmp_dir/steady/repo-labels.json"
echo '[{"name": "bug"}, {"name": "framework"}]' >"$tmp_dir/steady/pr-labels.json"
run_pr steady
same "case: changes" "add labels[]=ci" "$calls"

# A managed label added by hand does not stick; another label does.
echo '[{"name": "agents"}, {"name": "bug"}, {"name": "framework"}, {"name": "ci"}]' >"$tmp_dir/steady/repo-labels.json"
echo '[{"name": "bug"}, {"name": "framework"}, {"name": "ci"}, {"name": "agents"}]' >"$tmp_dir/steady/pr-labels.json"
run_pr steady
same "by hand: changes" "remove agents" "$calls"

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
