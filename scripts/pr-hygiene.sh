#!/usr/bin/env bash
# Title (ADR 0022): type(scope): summary, at most 100 characters, no final period, scopes joined by commas.
# Types and scopes: the lists "types" and "framework_scopes" below, and the game ids (directories of games/).
#
# Labels a pull request from the paths it changes and checks its title.
#
#   scripts/pr-hygiene.sh title "<title>"
#       Exit 0 if the title is valid, else say why and exit 1.
#   scripts/pr-hygiene.sh labels <file>... [--added <file>...]
#       Print the labels that apply, one per line. No network. The files
#       after --added are the ones the pull request adds; they count as
#       changed too.
#   REPO=owner/name scripts/pr-hygiene.sh pr <number>
#       Read the pull request's title and files, add the labels that apply
#       (creating the ones the repository lacks), remove the managed ones
#       that no longer apply, then check the title: an invalid one prints
#       an error annotation and exits 1, after the labelling.
#
# Labels, from the files changed (several can apply; other labels are never
# touched):
#   game:<id>   a file under games/<id>/
#   framework   a file under crates/
#   ci          a file under .github/ or scripts/
#   adr         a file under docs/adr/
#   agents      a file under .claude/
#   release     a file added under games/*/releases/
#   experiment  a file games/*/journal/E<digits>-*.md
#   training    a file under games/*/training/ or games/*/trainer/, or
#               .github/workflows/train.yml
#   docs-only   every file changed is Markdown (*.md)
#
# Needs the gh CLI, logged in or with GH_TOKEN, for the pr mode only. GAMES_DIR
# (default games) is where the game ids come from, for the tests.
set -euo pipefail

cd "$(dirname "$0")/.."

types=(feat fix docs test ci refactor perf chore build revert)
framework_scopes=(core search arena bundler ci adr docs agents repo)
max_length=100
label_color=ededed
games_dir="${GAMES_DIR:-games}"

usage() {
  sed -n '/^#   scripts\/pr-hygiene.sh title/,/^# Labels, from/p' "$0" | sed '$d;s/^# \{0,1\}//' >&2
  exit 2
}

in_list() { # in_list <item> <list item>...
  local item="$1" other
  shift
  for other in "$@"; do
    if [[ $other == "$item" ]]; then return 0; fi
  done
  return 1
}

# A piece of the title that is safe to print: no control characters, at most
# 40 characters.
show() {
  local text
  text="$(printf '%s' "$1" | tr -d '\000-\037\177')"
  # shellcheck disable=SC2016 # the backquotes are literal
  printf '`%s`' "${text:0:40}"
}

# An error annotation of the workflow log (the message is escaped for it).
annotate_error() { # annotate_error <title> <message>
  local message="$2"
  message="${message//'%'/%25}"
  message="${message//$'\r'/%0D}"
  message="${message//$'\n'/%0A}"
  echo "::error title=$1::$message"
}

# Prints why the title is invalid and returns 1, or prints nothing and returns 0.
check_title() {
  local title="$1" type scope_group scope summary scope_name
  local -a problems=() allowed=() game_ids=() unknown=() scopes=()
  local shape='^([A-Za-z]+)(\(([^()]*)\))?: (.*)$'

  if [[ -z $title ]]; then
    echo "the title is empty"
    return 1
  fi
  if [[ ! $title =~ $shape ]]; then
    echo "expected \`type(scope): summary\`, for example \`feat(uttt): pattern move policy\`"
    return 1
  fi
  type="${BASH_REMATCH[1]}"
  scope_group="${BASH_REMATCH[2]}"
  scope="${BASH_REMATCH[3]}"
  summary="${BASH_REMATCH[4]}"

  if ! in_list "$type" "${types[@]}"; then
    problems+=("unknown type $(show "$type"), use one of: ${types[*]}")
  fi

  if [[ -z $scope_group ]]; then
    problems+=("the scope is required: \`type(scope): summary\`")
  elif [[ -z $scope ]]; then
    problems+=("the scope is empty")
  elif [[ $scope == *" "* ]]; then
    problems+=("separate scopes with commas, without spaces")
  elif [[ $scope == ,* || $scope == *, || $scope == *,,* ]]; then
    problems+=("the scope list has an empty scope")
  else
    for scope_name in "$games_dir"/*/; do
      if [[ -d $scope_name ]]; then
        scope_name="${scope_name%/}"
        game_ids+=("${scope_name##*/}")
      fi
    done
    allowed=("${game_ids[@]}" "${framework_scopes[@]}")
    IFS=, read -ra scopes <<<"$scope"
    for scope_name in "${scopes[@]}"; do
      if ! in_list "$scope_name" "${allowed[@]}"; then unknown+=("$(show "$scope_name")"); fi
    done
    if ((${#unknown[@]} > 0)); then
      problems+=("unknown scope ${unknown[*]}, use one of: ${allowed[*]}")
    fi
  fi

  if [[ -z ${summary//[[:space:]]/} ]]; then
    problems+=("the summary is empty")
  else
    if [[ $summary == [[:space:]]* || $summary == *[[:space:]] ]]; then
      problems+=("the summary must not start or end with a space")
    fi
    if [[ $summary == *. ]]; then
      problems+=("the summary must not end with a period")
    fi
  fi

  if ((${#title} > max_length)); then
    problems+=("the title has ${#title} characters, at most $max_length")
  fi

  if ((${#problems[@]} == 0)); then return 0; fi
  local reason="" problem
  for problem in "${problems[@]}"; do reason+="${reason:+; }$problem"; done
  echo "$reason"
  return 1
}

title_mode() {
  local reason
  if reason="$(check_title "$1")"; then return 0; fi
  if [[ ${GITHUB_ACTIONS:-} == true ]]; then
    annotate_error "Pull request title" "$reason"
  else
    echo "invalid title: $reason" >&2
  fi
  return 1
}

# Prints the labels for the changed files, one per line, in a fixed order:
# the games', then the others'. Usage: labels_for <file>... [--added <file>...]
labels_for() {
  local -a files=() added=() sorted=()
  local arg part=files file label count=0 all_markdown=1
  local -A want=()
  for arg in "$@"; do
    if [[ $arg == --added ]]; then
      part=added
    elif [[ $part == files ]]; then
      files+=("${arg#./}")
    else
      added+=("${arg#./}")
    fi
  done
  files+=("${added[@]}")

  for file in "${files[@]}"; do
    count=$((count + 1))
    if [[ $file =~ ^games/([^/]+)/. ]]; then want["game:${BASH_REMATCH[1]}"]=1; fi
    if [[ $file =~ ^crates/. ]]; then want[framework]=1; fi
    if [[ $file =~ ^(\.github|scripts)/. ]]; then want[ci]=1; fi
    if [[ $file =~ ^docs/adr/. ]]; then want[adr]=1; fi
    if [[ $file =~ ^\.claude/. ]]; then want[agents]=1; fi
    if [[ $file =~ ^games/[^/]+/journal/E[0-9]+-[^/]*\.md$ ]]; then want[experiment]=1; fi
    if [[ $file =~ ^games/[^/]+/(training|trainer)/. || $file == .github/workflows/train.yml ]]; then
      want[training]=1
    fi
    if [[ $file != *.md ]]; then all_markdown=0; fi
  done
  for file in "${added[@]}"; do
    if [[ $file =~ ^games/[^/]+/releases/. ]]; then want[release]=1; fi
  done
  if ((count > 0 && all_markdown == 1)); then want[docs-only]=1; fi

  if ((${#want[@]} == 0)); then return 0; fi
  mapfile -t sorted < <(printf '%s\n' "${!want[@]}" | LC_ALL=C sort)
  for label in "${sorted[@]}"; do
    if [[ $label == game:* ]]; then echo "$label"; fi
  done
  for label in framework ci adr agents release experiment training docs-only; do
    if [[ -n ${want[$label]:-} ]]; then echo "$label"; fi
  done
}

# The labels this script manages: only these are ever removed.
is_managed() {
  case "${1,,}" in
    game:* | framework | ci | adr | agents | release | experiment | training | docs-only) return 0 ;;
  esac
  return 1
}

label_description() {
  case "$1" in
    game:*) echo "Changes under games/${1#game:}/" ;;
    framework) echo "Changes under crates/" ;;
    ci) echo "Changes to workflows and scripts" ;;
    adr) echo "Changes to decision records" ;;
    agents) echo "Changes to agents and skills (.claude/)" ;;
    release) echo "Adds a released bot" ;;
    experiment) echo "Adds or changes an experiment's journal entry" ;;
    training) echo "Changes to the training pipeline or its results" ;;
    docs-only) echo "Only Markdown files change" ;;
    *) echo "" ;;
  esac
}

urlencode() {
  local text="$1" encoded="" char i
  local LC_ALL=C
  for ((i = 0; i < ${#text}; i++)); do
    char="${text:i:1}"
    case "$char" in
      [a-zA-Z0-9.~_-]) encoded+="$char" ;;
      *) encoded+="$(printf '%%%02X' "'$char")" ;;
    esac
  done
  printf '%s' "$encoded"
}

# Splits the lines of a text into the array named by the first argument,
# without the empty ones.
lines_to_array() { # lines_to_array <array name> <text>
  local -n array_out="$1"
  local line
  array_out=()
  while IFS= read -r line; do
    if [[ -n $line ]]; then array_out+=("$line"); fi
  done <<<"$2"
}

create_label() { # create_label <name>
  local output
  if ! output="$(gh api -X POST "repos/$REPO/labels" -f name="$1" -f color="$label_color" \
    -f description="$(label_description "$1")" 2>&1)"; then
    # A run for another pull request may have created it just now.
    if grep -q 'already_exists' <<<"$output"; then return 0; fi
    echo "$output" >&2
    return 1
  fi
  echo "Created the label $1"
}

# Adds the wanted labels the pull request lacks, creating those the
# repository lacks, and removes the managed ones it should no longer have.
# Runs in a condition: every failure must return.
apply_labels() { # apply_labels <number> <wanted label>...
  local number="$1" label key
  shift
  local current_text repo_text
  local -a wanted=("$@") current=() known=() to_add=() to_remove=() args=()
  local -A is_wanted=() is_current=() is_known=()

  current_text="$(gh api --paginate "repos/$REPO/issues/$number/labels?per_page=100" --jq '.[].name')" || return 1
  repo_text="$(gh api --paginate "repos/$REPO/labels?per_page=100" --jq '.[].name')" || return 1
  lines_to_array current "$current_text"
  lines_to_array known "$repo_text"
  # Names are compared in lower case: GitHub does not tell labels apart by case.
  for label in "${wanted[@]}"; do is_wanted["${label,,}"]=1; done
  for label in "${current[@]}"; do is_current["${label,,}"]=1; done
  for label in "${known[@]}"; do is_known["${label,,}"]=1; done

  for label in "${wanted[@]}"; do
    key="${label,,}"
    if [[ -n ${is_current[$key]:-} ]]; then continue; fi
    if [[ -z ${is_known[$key]:-} ]]; then create_label "$label" || return 1; fi
    to_add+=("$label")
  done
  for label in "${current[@]}"; do
    if is_managed "$label" && [[ -z ${is_wanted[${label,,}]:-} ]]; then to_remove+=("$label"); fi
  done

  if ((${#to_add[@]} > 0)); then
    for label in "${to_add[@]}"; do args+=(-f "labels[]=$label"); done
    gh api -X POST "repos/$REPO/issues/$number/labels" "${args[@]}" >/dev/null || return 1
  fi
  for label in "${to_remove[@]}"; do
    gh api -X DELETE "repos/$REPO/issues/$number/labels/$(urlencode "$label")" >/dev/null || return 1
  done

  echo "Labels: ${wanted[*]:-none}"
  echo "Added: ${to_add[*]:-none}; removed: ${to_remove[*]:-none}"
}

pr_mode() { # pr_mode <number>
  local number="$1" title files_text status file reason failed=0 labels_text
  local -a changed=() added=() wanted=()
  : "${REPO:?set REPO to owner/name}"

  # Read everything before changing anything: a failed read must not look
  # like a pull request without files.
  title="$(gh api "repos/$REPO/pulls/$number" --jq '.title')"
  # shellcheck disable=SC2016 # $path is the filter's, not the shell's
  files_text="$(gh api --paginate "repos/$REPO/pulls/$number/files?per_page=100" \
    --jq '.[] | (.filename, (.previous_filename // empty)) as $path | [.status, $path] | @tsv')"
  while IFS=$'\t' read -r status file; do
    if [[ -z $file ]]; then continue; fi
    changed+=("$file")
    if [[ $status == added ]]; then added+=("$file"); fi
  done <<<"$files_text"

  labels_text="$(labels_for "${changed[@]}" --added "${added[@]}")"
  lines_to_array wanted "$labels_text"
  if ! apply_labels "$number" "${wanted[@]}"; then
    annotate_error "Pull request labels" "could not update the labels of the pull request, see the log"
    failed=1
  fi

  if ! reason="$(check_title "$title")"; then
    annotate_error "Pull request title" "$reason"
    failed=1
  fi
  return "$failed"
}

case "${1:-}" in
  title)
    if (($# != 2)); then usage; fi
    title_mode "$2"
    ;;
  labels)
    shift
    labels_for "$@"
    ;;
  pr)
    if (($# != 2)) || [[ ! $2 =~ ^[0-9]+$ ]]; then usage; fi
    pr_mode "$2"
    ;;
  *) usage ;;
esac
