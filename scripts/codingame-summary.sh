#!/usr/bin/env bash
# Writes, as Markdown on stdout, the summary of one run of the collection
# workflow (.github/workflows/codingame-collect.yml, ADR 0058): the games
# fetched and played, the totals of the day's batch by opponent, the test
# games of the 24 hours that are left, and the top-10 players whose agent id
# changed since the previous UTC day's leaderboard snapshot on the data branch.
#
# Usage: scripts/codingame-summary.sh GAME FETCH_LOG BATCH_LOG
#   FETCH_LOG, BATCH_LOG   the output of `scripts/codingame.sh fetch` and
#                          `scripts/codingame.sh batch` (a missing or empty
#                          file is reported, not an error)
# Environment: CODINGAME_DATA_REF, the data branch as a ref to read the
# snapshots from (refs/remotes/origin/claude/codingame-games).
# Needs bash, git and jq. Exits 0 when its arguments are right.
set -euo pipefail

[[ $# -eq 3 && $1 =~ ^[a-z0-9][a-z0-9-]*$ ]] || {
  echo "usage: $0 GAME FETCH_LOG BATCH_LOG" >&2
  exit 2
}
game="$1" fetch_log="$2" batch_log="$3"
ref="${CODINGAME_DATA_REF:-refs/remotes/origin/claude/codingame-games}"

echo "## CodinGame collection: $game"
echo

fetched="$(sed -n 's/^fetched: //p' "$fetch_log" 2>/dev/null | tail -n 1 || true)"
echo "- Games fetched (our new ranking games): ${fetched:-unknown (no fetch output)}"
if [[ -s $batch_log ]]; then
  played="$(grep -cE '^game [0-9]+/[0-9]+: vs .*, (won|lost|draw), ' "$batch_log" || true)"
  echo "- Test games played in this run: $played"
  quota="$(sed -n 's/.*quota left \([0-9][0-9]*\).*/\1/p' "$batch_log" | tail -n 1)"
  echo "- Test games left in the quota window as of the last game: ${quota:-unknown}"
  stop="$(grep -E '^(stopped|stopping)' "$batch_log" | tail -n 1 || true)"
  [[ -z $stop ]] || echo "- Run ended: ${stop%%;*}"
  echo
  echo "### Won and lost today, by opponent (the day's batch, all its runs)"
  echo
  echo '```'
  sed -n '/^totals of batch/,$p' "$batch_log"
  echo '```'
else
  echo "- No batch output: the batch did not run, or nothing was played"
fi
echo

echo "### Top 10 agent changes since the previous UTC day"
echo
files="$(git ls-tree --name-only "$ref" "$game/leaderboard/" 2>/dev/null | sed 's|.*/||' | sort || true)"
newest="$(tail -n 1 <<<"$files")"
previous=""
if [[ -n $newest ]]; then
  previous="$(awk -v day="${newest:0:10}" 'substr($0, 1, 10) < day' <<<"$files" | tail -n 1)"
fi
if [[ -z $newest || -z $previous ]]; then
  echo "No snapshot of an earlier day to compare with yet."
else
  git show "$ref:$game/leaderboard/$previous" >"${TMPDIR:-/tmp}/cg-summary-old.$$"
  git show "$ref:$game/leaderboard/$newest" >"${TMPDIR:-/tmp}/cg-summary-new.$$"
  changes="$(jq -rn --slurpfile a "${TMPDIR:-/tmp}/cg-summary-old.$$" --slurpfile b "${TMPDIR:-/tmp}/cg-summary-new.$$" '
    def top: [.rows[] | select(.me != true)][:10] | map({key: .nickname, value: .agent_id}) | from_entries;
    ($a[0] | top) as $o | ($b[0] | top) as $n
    | ($n | to_entries[] | select($o[.key] != null and $o[.key] != .value)
       | "- \(.key): agent \($o[.key]) -> \(.value)"),
      ($n | keys[] | select($o[.] == null) | "- \(.) entered the top 10"),
      ($o | keys[] | select($n[.] == null) | "- \(.) left the top 10")')"
  rm -f "${TMPDIR:-/tmp}/cg-summary-old.$$" "${TMPDIR:-/tmp}/cg-summary-new.$$"
  echo "Between \`$previous\` and \`$newest\`:"
  echo
  echo "${changes:-No change.}"
fi
