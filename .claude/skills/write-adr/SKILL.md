---
name: write-adr
description: Writes a decision record in docs/adr after the owner has decided, with its scope, index row and the status lines of the records it supersedes or completes. Use whenever a decision is costly to reverse or changes a rule, a workflow or the owner's role.
---

# Write a decision record

## First, the owner decides

A record writes down a decision the owner made. Propose it first (skill `ask-decision`): the problem, two or three options with pros and cons, and a recommendation. Write the record once the owner has chosen, and quote the date of his answer in the status line.

## The record

- Number: the next one after the highest in `docs/adr/`, four digits. File `docs/adr/NNNN-short-name.md`, the name matching the title.
- Start from `docs/adr/template.md`. Header lines, each starting with `- `, in this order: `- Status: accepted (owner, YYYY-MM-DD)`, `- Date: YYYY-MM-DD`, then `- Supersedes:`, `- Completes:` or `- Puts in place:` if any (naming the record and the decision numbers), then `- Scope:`.
- Scope (ADR 0022): `framework`, or a game's folder name (`uttt`) when every decision binds that game only. A record with any decision that binds the framework is `framework`.
- Context: the facts that force the decision, with sources and dates; what the owner asked. Decision: numbered, each one sentence in bold followed by the detail. Consequences: what becomes easier, what harder, what must now be done.
- Plain, short English. Nothing about the owner beyond "the owner".

## The other files

- `docs/adr/README.md`: a row in the scope's group, in number order, with the title and status.
- A superseded or completed record: change only its `Status:` line (for example `accepted; decision 2 superseded by [0024](0024-....md)`) and its row's status in the index. Never edit its text otherwise.
- `CLAUDE.md`, `docs/WORKFLOW.md`, `docs/ARCHITECTURE.md` and `docs/ROADMAP.md` where the decision changes what they say.
- Run `scripts/check-docs.sh`; it checks the index, the scope and the links.

## Merging

A decision record is reviewed at the deep tier (skill `review-pr`) and merged only with the owner's approval of that pull request (ADR 0023), even when other classes of change are delegated.
