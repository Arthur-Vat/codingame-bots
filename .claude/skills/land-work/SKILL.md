---
name: land-work
description: Plans and lands a piece of work as one or more pull requests - settle conventions first, write specs for implementer agents, plan the stack and merge order, review at the right depth, and merge on the owner's approval. Use for any work that needs more than one pull request or any implementer agent.
---

# Land a piece of work

The main session plans, splits, checks and merges; `implementer` agents code; `pr-reviewer` reviews. Lessons of phase 5's step A and of the 2026-10-11 retrospective (phase 6) are built in.

## 0. Ask first when the work is heavy

The owner wants to be asked before any heavy implementation starts (2026-10-09): new crates or large code changes, bot experiments, training or league runs, anything that uses much of the owner's usage. Propose the plan, its pull requests and a rough cost, and wait for the go.

## 1. Settle what others will build on

Before any implementer starts:
- Write the decision record (skill `write-adr`) and any naming or convention the work depends on.
- Decide each pull request's title (ADR 0022) and branch (`claude/<topic>`).
- Prefer independent pull requests, each starting from `main`. For several pull requests that cannot avoid touching the same lines (often `docs/ROADMAP.md` lines next to each other, the architecture's lists, `README.md`), decide the order now and stack them linearly: each branch starts from the one directly below it and its pull request's base is that branch. Never merge a sibling branch into another: after the first merges, GitHub then reports conflicts on the rest (#62 to #76). When `main` moves, merge it into each remaining branch, lowest first, and wait for green again before asking for approval. Each pull request ticks its own roadmap line and updates the docs for its own change.
- Before parallel implementers start, write down every interface, format or layout that more than one of them uses (JSON shapes, routes, type and function names, command-line flags, file layouts). Each spec quotes it word for word; an implementer that finds it must change stops and reports instead of adapting it. Bookkeeping that is not part of a change (a decision's date, a setting the owner made) rides with the next substantive pull request (skill `review-pr`, "Small bookkeeping").

## 2. Write each spec

An implementer gets, in its prompt:
- the branch to create or use, and what it starts from;
- the goal in two sentences, and what is out of scope;
- the exact behaviour, with examples: inputs and the expected outputs or files, including edge cases;
- the files to change and the docs to update;
- the known risks, each handled or marked as not applying, with why:
  - input validation, including imported or user-supplied files;
  - edge cases: empty, largest, malformed;
  - timing races: asynchronous interfaces, servers, tests that wait;
  - Windows: paths, line endings, starting processes, shell scripts;
  - writes to GitHub (comments, labels, branch deletions, workflow dispatches): a dry-run mode, on by default, whose output is checked once before the first real run;
- the checks to run and the commit subject and trailers to use (`Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>` and the session's `Claude-Session:` line).

Dispatch independent specs in parallel with `isolation: "worktree"`. To fix findings, resume the same agent with `SendMessage` rather than starting a new one: it keeps its context.

## 3. Review at the depth the risk calls for

Skill `review-pr`. One light review can cover a batch of documentation-only branches.

## 4. Open the pull requests

Skill `github-api`. The body says what and why (linking the roadmap item or decision record), what the review found and what changed, the check summary lines (`CLAUDE.md`'s commands), and for a stack: the order, "Create a merge commit, not squash", and that GitHub retargets each one when the one below merges. End with the attribution lines.

Then ask the owner for approval in the conversation, in two turns (owner, 2026-10-09), in small batches of one to three pull requests (owner, 2026-10-11): ask as soon as a batch is written (opened, its required checks green and its review handled, skill `review-pr`), and merge it before opening or stacking more on top. Never ask about a pull request still being written.

1. The full brief, for each pull request: what it changes, its checks, its review and what was fixed, its risks and the head it would merge at. End the message by asking, in plain text, whether he wants to consider merging. No form in this turn: a form opens before the message can be read. His yes is not an approval: it names no pull request.
2. Once he says yes, a form (the question tool) whose options name the pull requests (ADR 0023): merge them as they are, such as `Merge #50, #51`, and, only when the order matters (a stack, shared lines, one building on another), merge them in order, such as `Merge #50, then #51`, with the reason in the option's description. Recommend one option. Add an option to hold. A choice in the form is an answer in the conversation.

Without the question tool, the second turn asks for the exact reply that approves, such as `merge #50`.

## 5. Merge

Only what the owner approved in the conversation, naming its number or approving it in advance for a stated purpose (ADR 0023); otherwise confirm with him before merging. In stack order, each once its base is `main`, its head is the approved one and its required checks are green. Comment each approval on its pull request (skill `github-api`). Then update local `main`, delete local branches, and tick the roadmap if a pull request did not.

## 6. Afterwards

Clean up scratch worktrees. Measure the work's token use (skill `session-usage`) and give the owner its one line. Note what went wrong or cost too much for the next daily report. Between batches or steps, follow "Suggest a fresh session" in skill `handoff`.
