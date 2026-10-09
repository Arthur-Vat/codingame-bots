---
name: pr-train
description: Plans and lands a piece of work as one or more pull requests - settle conventions first, write specs for implementer agents, plan the stack and merge order, review at the right depth, and merge on the owner's approval. Use for any work that needs more than one pull request or any implementer agent.
---

# Plan, dispatch and land pull requests

The main session plans, splits, checks and merges; `implementer` agents code; `pr-reviewer` reviews. Lessons of phase 5's step A are built in.

## 0. Ask first when the work is heavy

The owner wants to be asked before any heavy implementation starts (2026-10-09): new crates or large code changes, bot experiments, training or league runs, anything that uses much of the owner's usage. Propose the plan, its pull requests and a rough cost, and wait for the go.

## 1. Settle what others will build on

Before any implementer starts:
- Write the decision record (skill `write-adr`) and any naming or convention the work depends on.
- Decide each pull request's title (ADR 0022) and branch (`claude/<topic>`).
- For several pull requests that touch the same files (often `docs/ROADMAP.md` lines next to each other, the architecture's lists, `README.md`), decide the order now and stack them: each branch starts from the one below it and its pull request's base is that branch. Each pull request ticks its own roadmap line and updates the docs for its own change. Bookkeeping that is not part of a change (a decision's date, a setting the owner made) rides with the next substantive pull request (skill `review-pr`, "Small bookkeeping").

## 2. Write each spec

An implementer gets, in its prompt:
- the branch to create or use, and what it starts from;
- the goal in two sentences, and what is out of scope;
- the exact behaviour, with examples: inputs and the expected outputs or files, including edge cases;
- the files to change and the docs to update;
- the checks to run and the commit subject and trailers to use (`Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>` and the session's `Claude-Session:` line).

Dispatch independent specs in parallel with `isolation: "worktree"`. To fix findings, resume the same agent with `SendMessage` rather than starting a new one: it keeps its context.

## 3. Review at the depth the risk calls for

Skill `review-pr`. One light review can cover a batch of documentation-only branches.

## 4. Open the pull requests

Skill `github-api`. The body says what and why (linking the roadmap item or decision record), what the review found and what changed, the check summary lines (`CLAUDE.md`'s commands), and for a stack: the order, "Create a merge commit, not squash", and that GitHub retargets each one when the one below merges. End with the attribution lines.

Then ask the owner for approval in the conversation, ending with the exact reply that approves, such as `merge #50` (or `merge #50 to #52` for a stack), so that the approval names its pull requests (ADR 0023).

## 5. Merge

Only what the owner approved in the conversation, naming its number or approving it in advance for a stated purpose (ADR 0023); otherwise confirm with him before merging. In stack order, each once its base is `main`, its head is the approved one and its required checks are green. Comment each approval on its pull request (skill `github-api`). Then update local `main`, delete local branches, and tick the roadmap if a pull request did not.

## 6. Afterwards

Clean up scratch worktrees. Note what went wrong or cost too much for the next daily report.
