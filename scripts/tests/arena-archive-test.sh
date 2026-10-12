#!/usr/bin/env bash
# Tests scripts/arena-archive.sh with a stub `gh` (canned answers per API path,
# every call logged) and a stub studio-archive (counts the distinct record
# files of its inputs): the plan of the games, the inputs gathered, the
# decision to upload, and the deletion of older archives. Needs jq, zip, unzip
# and git.
# Usage: scripts/tests/arena-archive-test.sh
set -euo pipefail

cd "$(dirname "$0")/../.."
repo_root="$PWD"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
unset GITHUB_ACTIONS GITHUB_OUTPUT GITHUB_STEP_SUMMARY RUNNER_TEMP EVENT_NAME ADDING LEAGUE_RUN_ID HEAD_SHA DRY_RUN

checks=0
failures=0
fail() {
  echo "FAIL $*" >&2
  failures=$((failures + 1))
}

# ---- the stubs ----

stub="$tmp_dir/stub"
mkdir -p "$stub/bin"

# gh api [--paginate] [-X METHOD] PATH: prints the canned answer for PATH
# (file api/<PATH with / ? & = replaced by _>), or the zip zip/<id>.zip for
# artifacts/<id>/zip; fails for a path without answer.
cat >"$stub/bin/gh" <<'EOF'
#!/usr/bin/env bash
echo "gh $*" >>"$STUB/gh.log"
[[ $1 == api ]] || { echo "stub gh: only api" >&2; exit 1; }
shift
method=GET
path=""
while (($#)); do
  case $1 in
    --paginate) ;;
    -X) method=$2; shift ;;
    *) path=$1 ;;
  esac
  shift
done
if [[ $method == DELETE ]]; then exit 0; fi
if [[ $path =~ /artifacts/([0-9]+)/zip$ ]]; then
  zip="$STUB/zip/${BASH_REMATCH[1]}.zip"
  [[ -f $zip ]] || { echo "stub gh: no zip $zip" >&2; exit 1; }
  cat "$zip"
  exit 0
fi
key="$(printf '%s' "$path" | tr '/?&=' '____')"
file="$STUB/api/$key.json"
[[ -f $file ]] || { echo "stub gh: no answer for $path" >&2; exit 1; }
cat "$file"
EOF

# studio-archive --game G --out DIR INPUT...: one row and one record file per
# distinct record file name under the inputs; fails when STUB_TOOL_FAIL is set.
cat >"$stub/bin/studio-archive" <<'EOF'
#!/usr/bin/env bash
echo "studio-archive $*" >>"$STUB/tool.log"
[[ -z ${STUB_TOOL_FAIL:-} ]] || exit 1
shift 3 # --game, its value, --out
out=$1
shift
mkdir -p "$out/records"
for input in "$@"; do
  while IFS= read -r file; do
    cp "$file" "$out/records/$(basename "$file")"
  done < <(find "$input" -name '*.json' ! -name index.json)
done
ids=$(find "$out/records" -name '*.json' -printf '%f\n' | sed 's/\.json$//' | sort)
jq -n --arg ids "$ids" '{format: 1, rows: ($ids | split("\n") | map(select(. != "") | {id: .}))}' >"$out/index.json"
EOF
chmod +x "$stub/bin/gh" "$stub/bin/studio-archive"
export STUB="$stub"
export PATH="$stub/bin:$PATH"

# Starts a case: empty canned answers, no calls logged, new output files.
begin() {
  rm -rf "$stub/api" "$stub/zip" "$stub/gh.log" "$stub/tool.log" "$tmp_dir/work" "$tmp_dir/out" "$tmp_dir/summary"
  mkdir -p "$stub/api" "$stub/zip"
  touch "$stub/gh.log" "$tmp_dir/out" "$tmp_dir/summary"
  unset STUB_TOOL_FAIL
}

# canned <path> <json>
canned() {
  printf '%s\n' "$2" >"$stub/api/$(printf '%s' "$1" | tr '/?&=' '____').json"
}

# zip_of <artifact id> <file>...: a zip holding the files, each with its path
# as content; index.json holds the file names under records/ as rows.
zip_of() {
  local id="$1" dir file
  shift
  dir="$(mktemp -d "$tmp_dir/zip.XXXXXX")"
  for file in "$@"; do
    mkdir -p "$dir/$(dirname "$file")"
    printf '{"file":"%s"}\n' "$file" >"$dir/$file"
  done
  (cd "$dir" && zip -qr "$stub/zip/$id.zip" .)
}

# archive_zip <artifact id> <record id>...: the zip of an archive.
archive_zip() {
  local id="$1" dir rid
  shift
  dir="$(mktemp -d "$tmp_dir/zip.XXXXXX")"
  mkdir -p "$dir/records"
  for rid in "$@"; do printf '{"id":"%s"}\n' "$rid" >"$dir/records/$rid.json"; done
  jq -n --args '{format: 1, rows: ($ARGS.positional | map({id: .}))}' "$@" >"$dir/index.json"
  (cd "$dir" && zip -qr "$stub/zip/$id.zip" .)
}

# artifact <id> <expired> <created> <expires> <run id> <branch> <head repo id>
# <repo id>: one artifact of a listing.
artifact() {
  jq -nc --argjson id "$1" --argjson expired "$2" --arg created "$3" --arg expires "$4" \
    --argjson run "$5" --arg branch "$6" --argjson head "$7" --argjson repo "$8" \
    '{id: $id, expired: $expired, created_at: $created, expires_at: $expires,
      workflow_run: {id: $run, head_branch: $branch, head_repository_id: $head, repository_id: $repo}}'
}

listing() { # listing <artifact json>...: one page of a listing
  jq -nc --args '{total_count: ($ARGS.positional | length), artifacts: ($ARGS.positional | map(fromjson))}' "$@"
}

now="$(date -u -d 2026-10-12 +%s)"
export REPO=owner/repo GH_TOKEN=unused NOW="$now" ARCHIVE_TOOL="$stub/bin/studio-archive"
sha_a="$(printf 'a%.0s' {1..40})"
sha_b="$(printf 'b%.0s' {1..40})"
sha_c="$(printf 'c%.0s' {1..40})"
# Expiry dates relative to NOW (2026-10-12): in 50, 10 and 60 days.
in_50=2026-12-01T10:00:00Z
in_10=2026-10-22T10:00:00Z
in_60=2026-12-11T10:00:00Z

# gather <game> [VAR=value...]: runs the gather mode into $tmp_dir/work.
status=0
gather() {
  local game="$1"
  shift
  status=0
  : >"$tmp_dir/out"
  env "$@" GITHUB_OUTPUT="$tmp_dir/out" GITHUB_STEP_SUMMARY="$tmp_dir/summary" \
    scripts/arena-archive.sh gather "$game" "$tmp_dir/work" >"$tmp_dir/stdout" 2>"$tmp_dir/stderr" || status=$?
}

expect() { # expect <description> <condition result: 0 ok>
  checks=$((checks + 1))
  if (($2 != 0)); then fail "$1"; fi
}

has() { grep -qF -- "$2" "$1"; }
lacks() { ! grep -qF -- "$2" "$1"; }

output_is() { # output_is <description> <key> <value>
  checks=$((checks + 1))
  if ! grep -qxF "$2=$3" "$tmp_dir/out"; then
    fail "$1: expected $2=$3, got: $(tr '\n' ' ' <"$tmp_dir/out")"
  fi
}

status_is() { # status_is <description> <failure expected: true|false>
  checks=$((checks + 1))
  if [[ $2 == true && $status -eq 0 ]] || [[ $2 == false && $status -ne 0 ]]; then
    fail "$1: status $status; stdout: $(cat "$tmp_dir/stdout"); stderr: $(cat "$tmp_dir/stderr")"
  fi
}

# ---- plan ----

# A repository of its own: the script and the scripts it calls, two games.
plan_repo="$tmp_dir/plan"
mkdir -p "$plan_repo/scripts" "$plan_repo/games/uttt/releases" "$plan_repo/games/other"
cp "$repo_root/scripts/arena-archive.sh" "$repo_root/scripts/ci-scopes.sh" "$plan_repo/scripts/"
touch "$plan_repo/games/other/.gitkeep" "$plan_repo/games/uttt/releases/uttt-v001.rs"
g() { git -C "$plan_repo" -c user.name=t -c user.email=t@example.com "$@"; }
g init -q
g add -A
g commit -qm base

plan() { # plan <event>: runs the plan mode; sets status, fills $tmp_dir/out
  status=0
  : >"$tmp_dir/out"
  (cd "$plan_repo" && EVENT_NAME="$1" GITHUB_OUTPUT="$tmp_dir/out" scripts/arena-archive.sh plan >"$tmp_dir/stdout" 2>"$tmp_dir/stderr") || status=$?
}

# A release is added.
touch "$plan_repo/games/uttt/releases/uttt-v002.rs"
g add -A
g commit -qm "add a release"
plan workflow_run
status_is "plan: a release added" false
output_is "plan: a release added" games '["uttt"]'
output_is "plan: a release added" adding true

# Only a change to a release, and a new README: no release added.
echo changed >"$plan_repo/games/uttt/releases/uttt-v002.rs"
touch "$plan_repo/games/other/README.md"
g add -A
g commit -qm "change a release, add a file"
plan workflow_run
status_is "plan: no release added" false
output_is "plan: no release added" games '[]'
expect "plan: no release added: the notice" "$(has "$tmp_dir/stdout" '::notice title=Arena archive::no release added: nothing to archive' && echo 0 || echo 1)"

# A release in a folder below releases/ cannot be placed: fail closed.
mkdir -p "$plan_repo/games/uttt/releases/old"
touch "$plan_repo/games/uttt/releases/old/x.rs"
g add -A
g commit -qm "nested"
plan workflow_run
status_is "plan: a release that cannot be placed" true

# A schedule and a run by hand: every game, nothing added.
for event in schedule workflow_dispatch; do
  plan "$event"
  status_is "plan: $event" false
  output_is "plan: $event" games '["other","uttt"]'
  output_is "plan: $event" adding false
done

# Another event does nothing.
plan push
status_is "plan: an unexpected event" true

# ---- gather ----

# A current archive (r1, r2) among older and expired ones, new samples: the
# league run's (r2, r3) and the SPRT's (r4, r5). The listing has two pages.
begin
page1="$(listing "$(artifact 90 true 2026-01-01T00:00:00Z '' 1 main 7 7)" "$(artifact 95 false 2026-05-01T00:00:00Z 2026-09-01T00:00:00Z 1 main 7 7)")"
page2="$(listing "$(artifact 100 false 2026-08-01T00:00:00Z "$in_50" 1 main 7 7)")"
canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' "$page1
$page2"
archive_zip 100 r1 r2
canned "repos/owner/repo/actions/runs/500/artifacts?name=league-game-records-uttt&per_page=100" \
  "$(listing "$(artifact 600 false 2026-10-11T00:00:00Z 2026-11-11T00:00:00Z 500 main 7 7)")"
zip_of 600 records/r2.json records/r3.json
# Pull requests of the commit: a fork's with the same merge commit, a merged
# one of the repository, and another commit's.
canned "repos/owner/repo/commits/$sha_a/pulls?per_page=100" "$(jq -nc --arg a "$sha_a" --arg b "$sha_b" --arg c "$sha_c" '[
  {number: 1, merge_commit_sha: $a, head: {sha: $c, ref: "forked", repo: {full_name: "fork/repo"}}},
  {number: 2, merge_commit_sha: $a, head: {sha: $b, ref: "claude/feature", repo: {full_name: "owner/repo"}}},
  {number: 3, merge_commit_sha: $b, head: {sha: $b, ref: "claude/other", repo: {full_name: "owner/repo"}}},
  {number: 4, merge_commit_sha: $a, head: {sha: $b, ref: "gone", repo: null}}]')"
# The runs of the branch, on two pages: the newest (73) is a fork's on a branch
# of the same name, the head run (72) reused the cached result and kept no
# sample, 74 has only an expired one, and 71 has the sample.
sprt_runs="repos/owner/repo/actions/workflows/sprt.yml/runs?branch=claude/feature&event=pull_request&per_page=100"
canned "$sprt_runs" "$(jq -nc '{workflow_runs: [
  {id: 73, created_at: "2026-10-03T00:00:00Z", head_repository: {full_name: "fork/repo"}},
  {id: 72, created_at: "2026-10-02T00:00:00Z", head_repository: {full_name: "owner/repo"}}]}')
$(jq -nc '{workflow_runs: [
  {id: 74, created_at: "2026-10-01T12:00:00Z", head_repository: {full_name: "owner/repo"}},
  {id: 71, created_at: "2026-10-01T00:00:00Z", head_repository: {full_name: "owner/repo"}}]}')"
canned "repos/owner/repo/actions/runs/72/artifacts?name=game-records-uttt&per_page=100" "$(listing)"
canned "repos/owner/repo/actions/runs/74/artifacts?name=game-records-uttt&per_page=100" \
  "$(listing "$(artifact 810 true 2026-10-01T12:00:00Z '' 74 main 7 7)")"
canned "repos/owner/repo/actions/runs/71/artifacts?name=game-records-uttt&per_page=100" \
  "$(listing "$(artifact 800 false 2026-10-01T00:00:00Z 2026-11-01T00:00:00Z 71 main 7 7)")"
zip_of 800 records/r4.json records/r5.json
gather uttt ADDING=true LEAGUE_RUN_ID=500 HEAD_SHA="$sha_a"
status_is "gather: add to the archive" false
output_is "gather: add to the archive" upload true
output_is "gather: add to the archive" added 3
output_is "gather: add to the archive" total 5
expect "gather: add: the current archive, the league and the newest SPRT sample are inputs" \
  "$(grep -q "inputs/0-archive .*inputs/league-500 .*inputs/sprt-71\$" "$stub/tool.log" && echo 0 || echo 1)"
expect "gather: add: the SPRT run of a fork is not used" "$(lacks "$stub/gh.log" '/runs/73/' && echo 0 || echo 1)"
expect "gather: add: no seeding with a current archive" "$(lacks "$stub/gh.log" 'actions/artifacts?name=league-game-records-uttt' && echo 0 || echo 1)"
expect "gather: add: the summary line" \
  "$(has "$tmp_dir/summary" "uttt: 5 games (3 added); current archive expires 2026-12-01; upload: true" && echo 0 || echo 1)"
expect "gather: add: the older archive is not downloaded" "$(lacks "$stub/gh.log" 'artifacts/95/zip' && echo 0 || echo 1)"

# The league sample and the pull request are missing: warn, go on; nothing new.
canned "repos/owner/repo/actions/runs/500/artifacts?name=league-game-records-uttt&per_page=100" "$(listing)"
canned "repos/owner/repo/commits/$sha_a/pulls?per_page=100" '[]'
rm -rf "$tmp_dir/work" "$stub/tool.log"
gather uttt ADDING=true LEAGUE_RUN_ID=500 HEAD_SHA="$sha_a"
status_is "gather: no samples" false
output_is "gather: no samples" upload false
output_is "gather: no samples" added 0
expect "gather: no samples: a warning" "$(has "$tmp_dir/stdout" '::warning title=Arena archive::uttt: league run 500 has no unexpired' && echo 0 || echo 1)"
expect "gather: no samples: a notice about the SPRT" "$(has "$tmp_dir/stdout" 'no SPRT sample added: no pull request of this repository was merged' && echo 0 || echo 1)"

# An API failure while looking for the SPRT sample fails the step: the pull
# requests of the commit cannot be listed, then the runs of its branch.
rm "$stub/api/$(printf '%s' "repos/owner/repo/commits/$sha_a/pulls?per_page=100" | tr '/?&=' '____').json"
rm -rf "$tmp_dir/work" "$stub/tool.log"
gather uttt ADDING=true LEAGUE_RUN_ID=500 HEAD_SHA="$sha_a"
status_is "gather: the pull requests cannot be listed" true
expect "gather: pull request failure: the tool did not run" "$([[ ! -e $stub/tool.log ]] && echo 0 || echo 1)"
canned "repos/owner/repo/commits/$sha_a/pulls?per_page=100" "$(jq -nc --arg a "$sha_a" --arg b "$sha_b" \
  '[{number: 2, merge_commit_sha: $a, head: {sha: $b, ref: "claude/feature", repo: {full_name: "owner/repo"}}}]')"
rm "$stub/api/$(printf '%s' "$sprt_runs" | tr '/?&=' '____').json"
rm -rf "$tmp_dir/work" "$stub/tool.log"
gather uttt ADDING=true LEAGUE_RUN_ID=500 HEAD_SHA="$sha_a"
status_is "gather: the SPRT runs cannot be listed" true
expect "gather: runs failure: the tool did not run" "$([[ ! -e $stub/tool.log ]] && echo 0 || echo 1)"
canned "$sprt_runs" "$(jq -nc '{workflow_runs: [{id: 75, created_at: "2026-10-01T00:00:00Z", head_repository: {full_name: "owner/repo"}}]}')"
rm -rf "$tmp_dir/work" "$stub/tool.log"
gather uttt ADDING=true LEAGUE_RUN_ID=500 HEAD_SHA="$sha_a"
status_is "gather: the SPRT run's artifacts cannot be listed" true

# The current archive expires in 10 days and nothing is new: refresh. In 60 days: not.
for case in "$in_10 true" "$in_60 false"; do
  read -r expires want <<<"$case"
  begin
  canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' \
    "$(listing "$(artifact 100 false 2026-08-01T00:00:00Z "$expires" 1 main 7 7)")"
  archive_zip 100 r1 r2
  gather uttt ADDING=false
  status_is "gather: refresh $expires" false
  output_is "gather: refresh $expires" upload "$want"
  output_is "gather: refresh $expires" added 0
  output_is "gather: refresh $expires" total 2
done

# No archive: start from the league samples of main in this repository.
begin
canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' "$(listing)"
canned 'repos/owner/repo/actions/artifacts?name=league-game-records-uttt&per_page=100' "$(listing \
  "$(artifact 11 false 2026-10-01T00:00:00Z 2026-11-01T00:00:00Z 21 main 7 7)" \
  "$(artifact 12 false 2026-10-01T00:00:00Z 2026-11-01T00:00:00Z 22 feature 7 7)" \
  "$(artifact 13 false 2026-10-01T00:00:00Z 2026-11-01T00:00:00Z 23 main 8 7)" \
  "$(artifact 14 true 2026-07-01T00:00:00Z '' 24 main 7 7)" \
  "$(artifact 15 false 2026-10-05T00:00:00Z 2026-11-05T00:00:00Z 25 main 7 7)")"
zip_of 11 records/s1.json
zip_of 15 records/s2.json records/s1.json
gather uttt ADDING=false
status_is "gather: seed" false
output_is "gather: seed" upload true
output_is "gather: seed" total 2
output_is "gather: seed" added 2
expect "gather: seed: only the samples of main in this repository" \
  "$(grep -q "inputs/seed-11 .*inputs/seed-15\$" "$stub/tool.log" && echo 0 || echo 1)"
for id in 12 13 14; do
  expect "gather: seed: artifact $id is not downloaded" "$(lacks "$stub/gh.log" "artifacts/$id/zip" && echo 0 || echo 1)"
done
expect "gather: seed: the summary line" "$(has "$tmp_dir/summary" 'uttt: 2 games (2 added); no current archive; upload: true' && echo 0 || echo 1)"

# Adding without an archive: the league run is added once, not also as a seed.
canned 'repos/owner/repo/actions/artifacts?name=league-game-records-uttt&per_page=100' "$(listing \
  "$(artifact 11 false 2026-10-01T00:00:00Z 2026-11-01T00:00:00Z 21 main 7 7)" \
  "$(artifact 15 false 2026-10-05T00:00:00Z 2026-11-05T00:00:00Z 25 main 7 7)")"
canned "repos/owner/repo/actions/runs/25/artifacts?name=league-game-records-uttt&per_page=100" \
  "$(listing "$(artifact 15 false 2026-10-05T00:00:00Z 2026-11-05T00:00:00Z 25 main 7 7)")"
canned "repos/owner/repo/commits/$sha_a/pulls?per_page=100" '[]'
rm -rf "$tmp_dir/work" "$stub/tool.log" "$stub/gh.log"
gather uttt ADDING=true LEAGUE_RUN_ID=25 HEAD_SHA="$sha_a"
status_is "gather: seed and add" false
expect "gather: seed and add: the triggering run is one input" \
  "$(grep -q "inputs/league-25 .*inputs/seed-11\$" "$stub/tool.log" && echo 0 || echo 1)"
expect "gather: seed and add: downloaded once" "$([[ "$(grep -c 'artifacts/15/zip' "$stub/gh.log")" == 1 ]] && echo 0 || echo 1)"

# Nothing at all: no archive, no upload, no error.
begin
canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' "$(listing)"
canned 'repos/owner/repo/actions/artifacts?name=league-game-records-uttt&per_page=100' "$(listing)"
gather uttt ADDING=false
status_is "gather: nothing" false
output_is "gather: nothing" upload false
expect "gather: nothing: the tool did not run" "$([[ ! -e $stub/tool.log ]] && echo 0 || echo 1)"

# The current archive cannot be downloaded: the step fails, no tool run.
begin
canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' \
  "$(listing "$(artifact 100 false 2026-08-01T00:00:00Z "$in_10" 1 main 7 7)")"
gather uttt ADDING=false
status_is "gather: the archive cannot be downloaded" true
expect "gather: download failure: the tool did not run" "$([[ ! -e $stub/tool.log ]] && echo 0 || echo 1)"
expect "gather: download failure: an error annotation" "$(has "$tmp_dir/stdout" '::error title=Arena archive::cannot download artifact 100' && echo 0 || echo 1)"
expect "gather: download failure: no upload output" "$(lacks "$tmp_dir/out" 'upload=' && echo 0 || echo 1)"

# The tool fails: the step fails.
begin
canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' \
  "$(listing "$(artifact 100 false 2026-08-01T00:00:00Z "$in_10" 1 main 7 7)")"
archive_zip 100 r1
export STUB_TOOL_FAIL=1
gather uttt ADDING=false
unset STUB_TOOL_FAIL
status_is "gather: the tool fails" true
expect "gather: the tool fails: no upload output" "$(lacks "$tmp_dir/out" 'upload=' && echo 0 || echo 1)"

# A game name that is not a plain name is refused.
begin
gather '../x' ADDING=false
status_is "gather: bad game" true

# ---- prune ----

prune() { # prune <game> <keep id> [VAR=value...]
  local game="$1" keep="$2"
  shift 2
  status=0
  env "$@" scripts/arena-archive.sh prune "$game" "$keep" >"$tmp_dir/stdout" 2>"$tmp_dir/stderr" || status=$?
}

begin
canned 'repos/owner/repo/actions/artifacts?name=arena-games-uttt&per_page=100' "$(listing \
  "$(artifact 1 false 2026-06-01T00:00:00Z 2026-09-01T00:00:00Z 1 main 7 7)" \
  "$(artifact 2 false 2026-10-01T00:00:00Z 2027-01-01T00:00:00Z 1 main 7 7)" \
  "$(artifact 3 false 2026-08-01T00:00:00Z 2026-11-01T00:00:00Z 1 main 7 7)" \
  "$(artifact 4 true 2026-01-01T00:00:00Z '' 1 main 7 7)")"

prune uttt 2 DRY_RUN=false
status_is "prune: keep one" false
expect "prune: the others are deleted" \
  "$(has "$stub/gh.log" 'api -X DELETE repos/owner/repo/actions/artifacts/1' && has "$stub/gh.log" 'api -X DELETE repos/owner/repo/actions/artifacts/3' && echo 0 || echo 1)"
expect "prune: the kept and the expired ones are not deleted" \
  "$(lacks "$stub/gh.log" 'DELETE repos/owner/repo/actions/artifacts/2' && lacks "$stub/gh.log" 'DELETE repos/owner/repo/actions/artifacts/4' && echo 0 || echo 1)"

: >"$stub/gh.log"
prune uttt 2
status_is "prune: dry run by default" false
expect "prune: dry run deletes nothing" "$(lacks "$stub/gh.log" DELETE && echo 0 || echo 1)"
expect "prune: dry run says what it would delete" "$(has "$tmp_dir/stdout" 'would delete artifact 1' && echo 0 || echo 1)"
prune uttt 2 DRY_RUN=true
expect "prune: DRY_RUN=true deletes nothing" "$(lacks "$stub/gh.log" DELETE && echo 0 || echo 1)"

for keep in '' abc 12x 9; do
  prune uttt "$keep" DRY_RUN=false
  status_is "prune: keep id '$keep'" true
  expect "prune: keep id '$keep': nothing deleted" "$(lacks "$stub/gh.log" DELETE && echo 0 || echo 1)"
done

if ((failures > 0)); then
  echo "FAIL arena-archive: $failures of $checks checks failed" >&2
  exit 1
fi
echo "ok   arena-archive: $checks checks"
