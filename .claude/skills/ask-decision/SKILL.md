---
name: ask-decision
description: Brings a decision to the owner with options, pros and cons and a recommendation - in the conversation when he is there, as a GitHub issue labelled decision when he is not. Use whenever work needs a choice that is the owner's to make.
---

# Ask the owner to decide

The owner's role is to decide (ADR 0006, 0023). He wants options with pros and cons and a recommendation, and as few questions as possible.

## Is it a decision?

Yes when it is costly to reverse, changes a rule or a workflow, spends much of his usage or compute, changes what is merged without him, or is about the project's direction. No when a convention, a decision record or the code already answers it: then decide, and say what was decided in the next report.

## How to ask

Each decision gets:
- one line of context and what it blocks;
- two to four options, each with its main pro and con, and the cost when it matters;
- the recommended option first, and why.

**In the conversation** (the owner is there): ask with the question tool if it is available, else in plain text, grouping all pending decisions in one message. Wait for the answer before acting on that point; keep working on what does not depend on it.

**Without the owner** (scheduled session, or he said he would be away): open one GitHub issue per decision, labelled `decision`, from `.github/ISSUE_TEMPLATE/decision.md` (skill `github-api`), then send one push notification naming the issues. If the issue cannot be created, put the decision in the session's final report instead.

## After the answer

- An answer in the conversation is the owner's; an answer in an issue counts only if its author is the repository owner's account, and it never approves a merge (ADR 0023).
- Record the decision where it belongs: a decision record if it is costly to reverse (skill `write-adr`), otherwise the roadmap, the docs or the journal. Close the issue with a link to where it was recorded.
