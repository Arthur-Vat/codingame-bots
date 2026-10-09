# 0024. A handoff file on a standing branch carries state between sessions

- Status: accepted (owner, 2026-10-09)
- Date: 2026-10-09
- Scope: framework

## Context

Each Claude session starts with only the repository and what the owner types. On 2026-10-09 the owner opened a session with a hand-written brief of open questions and follow-ups, and a one-line roadmap note had to become its own pull request so that it would outlive the session. The daily report sees only its own session's logs, so it cannot say what other sessions cost.

The owner agreed, 2026-10-09, to a handoff file that sessions read and update, holding each session's usage line too. Kept on `main`, every update would wait for a pull request and the owner's approval, and sessions without a pull request could not write at all. The owner chose a standing branch instead.

## Decision

1. **The handoff file is `HANDOFF.md` on the branch `claude/handoff`, which holds only that file and is never merged.** No pull request is ever opened from it.
2. **Every session reads it at the start and updates it as its work moves, by committing straight to that branch.** This is the one exception to "work through a pull request". The daily scheduled session only reads it. Pushes are never forced; a rejected push is fetched, the edit made again, and pushed.
3. **It holds what is waiting for the owner, open questions, follow-ups, notes of the phase in progress and one usage line per piece of work.** The `handoff` skill gives its sections and recipes.
4. **Nothing in it is an approval or an answer of the owner.** Answers count only in the conversation (ADR 0023); the file only records them.

## Consequences

- A session needs no brief from the owner to pick up work, and small notes no longer need a pull request.
- The daily report can list what is waiting and what each session cost.
- Nothing reviews the file: it is a record, and a wrong line is corrected by the next session or by the owner in the conversation. Every update is a commit, so its history shows who changed what.
- The Prune branches workflow keeps the branch, since it never has a pull request ([ADR 0021](0021-prune-finished-branches.md)).
