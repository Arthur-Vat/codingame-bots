#!/usr/bin/env bash
# Deletes the repository's finished claude/ branches (ADR 0021):
#
# - claude/train/<run>, once games/<game>/training/runs/<run>/report.md is
#   in this checkout of main;
# - any other claude/ branch, once every pull request from it is closed and
#   it still points at the last one's head: GitHub keeps a closed pull
#   request's commits (refs/pull/<number>/head), merged or not.
#
# Every other branch stays: main, branches outside claude/, branches with an
# open pull request or none yet, and branches with commits pushed after
# their last pull request closed. Merged branches are normally gone already:
# the repository deletes them on merge.
#
# Usage: REPO=owner/name [DRY_RUN=false] scripts/prune-branches.sh
# Needs the gh CLI, logged in or with GH_TOKEN. DRY_RUN defaults to true:
# only list what would be deleted.
set -euo pipefail

cd "$(dirname "$0")/.."
: "${REPO:?set REPO to owner/name}"
dry_run="${DRY_RUN:-true}"
owner="${REPO%%/*}"

deleted=()
kept=()
while IFS=$'\t' read -r branch sha; do
  [[ $branch == claude/* ]] || continue
  if [[ $branch == claude/train/* ]]; then
    run="${branch#claude/train/}"
    if ! compgen -G "games/*/training/runs/$run/report.md" >/dev/null; then
      kept+=("$branch: its report is not on main yet")
      continue
    fi
    reason="its report is on main"
  else
    # Count, open ones, and the last one's number and head.
    read -r count open number head < <(gh api "repos/$REPO/pulls?state=all&head=$owner:$branch&per_page=100" \
      --jq 'sort_by(.number) | [length, (map(select(.state == "open")) | length), (last.number // 0), (last.head.sha // "-")] | @tsv')
    if ((count == 0)); then
      kept+=("$branch: no pull request yet")
      continue
    fi
    if ((open > 0)); then
      kept+=("$branch: open pull request")
      continue
    fi
    if [[ $head != "$sha" ]]; then
      kept+=("$branch: commits after #$number closed")
      continue
    fi
    reason="#$number is closed; its commits stay in the pull request"
  fi
  if [[ $dry_run == true ]]; then
    deleted+=("$branch (would be deleted: $reason)")
  else
    gh api -X DELETE "repos/$REPO/git/refs/heads/$branch" >/dev/null
    deleted+=("$branch ($reason)")
  fi
done < <(gh api --paginate "repos/$REPO/branches?per_page=100" --jq '.[] | [.name, .commit.sha] | @tsv')

report() {
  echo "## Branches"
  echo
  if [[ $dry_run == true ]]; then
    echo "To delete (dry run): ${#deleted[@]}"
  else
    echo "Deleted: ${#deleted[@]}"
  fi
  for line in "${deleted[@]}"; do echo "- $line"; done
  echo
  echo "Kept: ${#kept[@]}"
  for line in "${kept[@]}"; do echo "- $line"; done
}
report
if [[ -n ${GITHUB_STEP_SUMMARY:-} ]]; then
  report >>"$GITHUB_STEP_SUMMARY"
  # Job logs are not readable everywhere; an annotation is.
  summary="$(report | grep '^- ' | tr '\n' ' ')"
  echo "::notice title=Branches::${summary:-nothing to do}"
fi
