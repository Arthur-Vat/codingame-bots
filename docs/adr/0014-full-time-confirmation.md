# 0014. Confirm accepted candidates at CodinGame's time limits

- Status: accepted; the "faults" part of decision 3 superseded by [0015](0015-tolerate-rare-timeouts.md) for timeouts; superseded for games in Legend by [0020](0020-full-time-sprt-in-legend.md)
- Date: 2026-10-07
- Puts in place: decision 3 of [ADR 0013](0013-evaluation-time-limits.md)

## Context

ADR 0013 keeps evaluations at a fifth of CodinGame's time limits and asks for full-time measurements once a release reaches Gold league. The first release, `uttt-v001`, went from Bronze to Legend on its own on 2026-10-07, ranked 182 of 443, so that time has come.

Two ways to bring full time into the decisions were weighed with the owner:

- **A.** Keep the SPRT at 20 ms, then confirm an accepted candidate with a fixed match at 100 ms.
- **B.** Run the whole SPRT at 100 ms. Every decision would use CodinGame's conditions, but an average test would take about 2.5 hours instead of 30 minutes, and the longest would pass GitHub's 6-hour job limit, forcing a lower cap on pairs and more inconclusive tests.

## Decision

The owner chose A.

1. **The SPRT still decides at 20 ms plus 5 ms of tolerance** (ADR 0013).
2. **An accepted candidate then plays its baseline 500 pairs at CodinGame's exact limits** (time scale 1, no tolerance, the same openings): `CONFIRM_PAIRS` in `evaluation.env`. On GitHub's runners this takes about 20 minutes and measures the gain at full time within about ±25 Elo.
3. **The confirmation can reject:** a candidate that is clearly weaker at full time (the 95% interval of its Elo advantage entirely below 0) or that faults there is rejected, even though the SPRT accepted it. Otherwise the full-time result is recorded in the pull request's comment and the journal entry.
4. **If confirmations start contradicting the SPRT,** for example gains at 20 ms that vanish at 100 ms several times, Claude proposes moving the SPRT itself to full time (option B) in a new ADR.

## Consequences

- Every release has a measured gain at CodinGame's conditions, not only at 20 ms.
- An experiment takes about 20 minutes longer when it is accepted, and no longer when it is rejected.
- The confirmation is a safety net, not a second test of strength: with 500 pairs it can miss a small loss, but not a large one.
