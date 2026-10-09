---
name: review-pr
description: Chooses the review depth a pull request needs, runs the independent reviewer agents, and handles their findings before the owner is asked to approve. Use for every pull request Claude opens.
---

# Review a pull request

Every pull request gets an independent review before the owner is asked to approve it, except small bookkeeping (below). The reviewer must not have seen the work: run it as a separate agent, never as the session that made the change.

## Choose the tier

| Change | Tier | Agents |
| --- | --- | --- |
| Documentation only: docs, journal entries, READMEs, training reports | light | `pr-reviewer` (light) on Sonnet: pass `model: "sonnet"`; one review may cover several such branches |
| Workflows, scripts, permissions, `CLAUDE.md`, decision records, `.claude/` | deep | `pr-reviewer` (deep) |
| Bots, engine, search, bundler, arena | bot | `pr-reviewer` (bot), plus `rules-reviewer` when `games/<game>/engine` or `games/<game>/referee` changes |
| A candidate release | bot | as above; the SPRT judges strength |

The tier sets depth, not whether to review. A change made only by a script (a regenerated index, for instance) may get the light tier even inside a deeper pull request.

## Small bookkeeping

Documentation changes of at most about 20 lines that only record what the owner decided in the conversation, or what already happened (roadmap ticks, dates, status lines in the docs), get no separate review. Decision records and `CLAUDE.md` are never bookkeeping: they keep the deep tier (owner's decision, 2026-10-09). The merge still needs the owner's approval. Put them in the next substantive pull request when there is one. When one must go alone, run `scripts/check-docs.sh` and say in the body that it had no review, and why.

## Brief the reviewer

Give the branch, the intent in two or three sentences (what the owner asked for, which decision record), the tier, and what is expected to fail and why (for a stacked pull request: what the branches below it provide). Do not pass your own reasoning or doubts: the review should be independent.

Keep reviews cheap, since they draw on the owner's usage:
- paste the diff into the brief when it is under about 300 lines, so the reviewer need not explore;
- name the few files it may need beyond the diff;
- ask for a light review in about 10 tool calls and 200 words, a deep or bot review in about 30 tool calls and 400 words.

## Handle the findings

- Verify any finding that would change the design before acting on it.
- Fix blockers and real issues; fix cheap nits; for the rest, say in the pull request why they stay.
- A finding that needs a decision goes to the owner (skill `ask-decision`), not into a guess.
- Put a short "Review" section in the pull request body: the verdict, what was fixed (with commit SHAs) and what was kept.
- Then ask the owner to approve: the full brief first, then a form naming each pull request (skill `pr-train`, step 4).

Do not re-run the full review after small fixes; re-run it only if the fixes changed the design.
