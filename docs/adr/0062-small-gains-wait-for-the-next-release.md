# 0062. Small gains wait for the next release

- Status: accepted (owner, 2026-11-28)
- Date: 2026-11-28
- Supersedes: [0012](0012-evaluation.md), decision 11 for accepted candidates below the threshold of decision 1
- Scope: framework

## Context

ADR 0012 makes every accepted candidate a release: one experiment, one pull request, one release, merged whole (decisions 9 and 11). Each release is also a CodinGame submission (skill `deliver-release`), whose ranking takes hours and shows small differences poorly.

On 2026-11-28 E116 (A Code of Ice and Fire, pull request #411) was accepted at +7.9 [+1.8, +14.0] Elo over 4,400 pairs, after screenings that had said +14 to +21. The owner judged that so small a gain is not worth a release of its own ("I don't think a very small gain is worth a release version") and chose in a form to combine it with the next experiment that gains ("Combine with E120 if it gains"), and to make this a standing rule recorded in an ADR ("Standing rule via an ADR").

## Decision

1. **An accepted candidate whose SPRT estimate is below +15 Elo waits.** Its pull request is not merged: it stays open, its journal entry complete (hypothesis, screening, SPRT) with `decision: waiting`, until the combined pull request of decision 3 carries it, and is then closed, its commits reachable as ADR 0021 keeps them. The owner may still choose to release it on its own.
2. **The next experiment builds on what waits.** Its branch starts from the waiting branch (or merges it), and it is screened against the waiting candidate's bot, so that its own gain is measured on top of the waiting one.
3. **A combined release carries every waiting gain and the new one.** It is one pull request with one release (ADR 0012, decision 9, unchanged) and every journal entry it carries. Its SPRT tests the combined release against the current release.
4. **Each experiment keeps its own evidence.** A waiting entry keeps its own SPRT; when its gain ships, its `release:` names the combined release and its `decision:` becomes `promoted`. The new experiment's entry gives its screening against the waiting candidate and the combined SPRT.
5. **A combined release below the threshold waits in turn.** Its gains go on waiting with the next experiment, unless the owner releases them.

## Consequences

- Fewer, larger releases, and fewer CodinGame submissions, each with a gain more likely to show in the ranking.
- A pull request can carry more than one experiment, which CLAUDE.md and `docs/WORKFLOW.md` must say. Each experiment still has its own entry and its own evidence, so the journal can tell which part gained.
- The combined SPRT tests the sum, not each part. A part that adds nothing on top of the other shows only in its screening against the waiting candidate.
- Code on `main` lags the best bot while a gain waits. An experiment that does not build on the waiting branch measures itself against the wrong bot; decision 2 is there to prevent it.
- The threshold is the SPRT's estimate at CodinGame's limits, not its interval. A new ADR can change it.
