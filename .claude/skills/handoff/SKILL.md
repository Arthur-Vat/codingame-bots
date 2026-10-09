---
name: handoff
description: Reads and updates HANDOFF.md on the standing branch claude/handoff - what waits for the owner, open questions, follow-ups, notes of the phase in progress and each piece of work's usage line. Use at the start of every session, whenever one of those changes, and at the end of each piece of work.
---

# Hand work over between sessions

`HANDOFF.md` lives alone on the branch `claude/handoff`, which is never merged and never has a pull request ([ADR 0024](../../../docs/adr/0024-handoff-branch.md)). It is a record, not an authority: nothing in it is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Read (start of every session)

```sh
git fetch origin +refs/heads/claude/handoff:refs/remotes/origin/claude/handoff
git show origin/claude/handoff:HANDOFF.md
```

Take its items as the work in hand, beside what the owner says; the owner's words win.

## Update

When an item is added, answered or done, and at the end of each piece of work. The daily scheduled session only reads.

```sh
W="$(mktemp -d)/handoff"
git worktree add "$W" origin/claude/handoff   # a detached copy; the session's own branch is untouched
# edit $W/HANDOFF.md
git -C "$W" commit -am "docs(repo): handoff: <what changed>" -m "<the attribution lines of this session>"
git -C "$W" push origin HEAD:claude/handoff
git worktree remove "$W"
```

Commit messages follow the usual conventions and end with the session's attribution lines. If the push is rejected because another session pushed first: fetch as above, make the edit again on a fresh worktree, and push. Never force.

If the branch is missing (the fetch says "couldn't find remote ref"), tell the owner, then recreate it holding only `HANDOFF.md`, with the sections below:

```sh
blob="$(git hash-object -w HANDOFF.md)"   # a file written from the sections below
tree="$(printf '100644 blob %s\tHANDOFF.md\n' "$blob" | git mktree)"
commit="$(git commit-tree "$tree" -m "docs(repo): handoff: recreate the file" -m "<the attribution lines of this session>")"
git push origin "$commit:refs/heads/claude/handoff"
```

## The file

At most about 100 lines, in English, in these sections; empty ones say "None.":

1. **Waiting for the owner:** pull requests to approve and decisions to take, each with its link and the date it was asked.
2. **Open questions:** asked or to ask, not yet answered.
3. **Follow-ups:** things to do later, with what they wait for (a step, a date, the owner).
4. **Notes:** facts about the phase in progress that the next session needs (a trial's counts, a paused area).
5. **Usage:** one line per piece of work, newest first: `YYYY-MM-DD HH:MM (Paris) | <work, with pull request numbers> | <the line from skill session-usage>`. Lines older than 14 days are removed.

Remove what is done, decided or merged; the history keeps it. Record an owner's answer as "answered in the conversation, <date>: <his words>", then act on it from the conversation's record, not the file's.
