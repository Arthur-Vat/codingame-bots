---
id: E020
date: 2026-10-29
pull_request: "#213"
parent: ice-and-fire-v008
hypothesis: In v008's games a side with more level-2 units than the evaluation expects wins less often, and one with more towers wins more often; valuing level-2 units below their training cost (they cost 4 gold of upkeep a turn against 1), and towers higher, makes the search defend better and beats v008.
change: The plan bot's evaluation counts each level-2 unit on an active cell 15 less than its training cost, 5 instead of 20, for both sides. Towers keep their weight, since valuing them more lost in screening.
release: ice-and-fire-v009
sprt: accepted
elo: +39.8 [+18.6, +61.3] at CodinGame's limits
pairs: 377
full_time: +39.8 [+18.6, +61.3] (the SPRT's own, ADR 0042), no timeout in 754 games, slowest answer 632.0 ms (the first turn)
decision: promoted
cg_rank: not reported
---

Chosen by the owner on 2026-10-29 after a study of v008's losses ("Study
v008's losses (Recommended)", then "Towers up, level 2 down
(Recommended)").

## The study

1,000 games of `ice-and-fire-v008` against itself at CodinGame's limits
(500 seat-swapped pairs, seed 21021), turned into the review tool's game
files and metrics, and measured as E019's study of v007 was:

| | v007 (E019's study) | v008 |
| --- | ---: | ---: |
| Median game length | 19 turns | 23 turns |
| Red's wins | 37.1% | 40.3% |
| The first large cut's maker wins | 89.1% (822 of 923) | 85.0% (693 of 815) |
| First large cuts beyond what trainings alone take | 87.3% (median 7 cells more) | 70.6% (median 4) |
| Middle-game cells taken beyond trainings alone | 35% | 23% |
| Cuts of 8 or more cells after the side cut had grown | 93% (median 7 cells) | 86% (median 4) |
| Held-out log loss of the evaluation's score and the seat | 0.292 | 0.352 |
| Most that one metric lowers it | 0.004 (`moves`) | 0.012 (`level2`) |

E019 did what it was meant to: fewer cuts by moves, less overextension.
The evaluation now explains less of who wins, and four metrics add to it
two to three times more than any did for v007: a side's level-2 units, beyond
the opponent's, are worth about 23 less than the evaluation counts them
(`level2`, log loss lowered by 0.012), the cells its towers cover about 5
more each (`tower_cover`, 0.009), its units near its own HQ about 32 less
(`hq_guard`, 0.008, a sign of being under threat rather than a cause),
and its towers about 18 more (`towers`, 0.007). v008 builds more of both
than v007 (at turn 22, 2.2 level-2 units and 3.3 towers a side against
1.9 and 2.8), and its upkeep is higher and its income lower.

## Screening

Against `ice-and-fire-v008` at CodinGame's limits, 400 seat-swapped pairs
each, seed 20200, two games at a time; the best again on 600 pairs with
seed 20201. "Towers +15" counts a tower 25 instead of 10; "level 2 −N"
counts a level-2 unit 20 − N; "cover" adds 2 for each active cell a
tower covers.

| Variant | Elo against v008 | Score as red | Score as blue | Variant's timeouts (v008's) | Median answer (v008's) |
| --- | --- | ---: | ---: | ---: | --- |
| towers +15 | **−82.3 [−105.4, −59.9]** | 35.0% | 41.8% | 3 (2) | 20.8 ms (24.4) |
| towers +15, level 2 −10 | **−50.7 [−74.7, −27.2]** | 39.8% | 45.8% | 0 (0) | 20.5 ms (24.6) |
| the same and cover | **−72.2 [−96.4, −48.8]** | 36.5% | 43.0% | 6 (0) | 18.5 ms (24.5) |
| level 2 −5 | **+37.5 [+17.8, +57.5]** | 44.8% | 66.0% | 1 (0) | 20.2 ms (21.7) |
| level 2 −10 | **+49.0 [+27.4, +70.9]** | 47.5% | 66.5% | 3 (3) | 20.1 ms (23.5) |
| level 2 −10, 600 pairs, seed 20201 | **+39.0 [+21.6, +56.5]** | 46.0% | 65.2% | 3 (6) | 18.7 ms (22.2) |
| level 2 −15 | **+59.6 [+38.8, +81.0]** | 49.2% | 67.8% | 2 (2) | 19.1 ms (23.9) |
| level 2 −15, 600 pairs, seed 20201 | **+55.5 [+37.4, +73.8]** | 49.8% | 66.0% | 6 (3) | 19.2 ms (24.5) |
| level 2 −20 | **+54.3 [+32.7, +76.3]** | 49.2% | 66.2% | 2 (3) | 19.7 ms (24.6) |

Valuing towers more is clearly weaker, and pulls the combinations down
with it: the towers the study saw with winning sides are a sign of a lead
more than a cause of one. Valuing level-2 units less is clearly stronger,
the more so up to a discount of 15 (+57 over its 1,000 pairs), level at
20. The candidate takes 15, as the owner chose ("Candidate v009, discount
15 (Recommended)"). Its timeouts, 8 in 2,000 games against v008's 5 in
the same runs, are within the SPRT's 1%.

## The change

`bots/plan/src/eval.rs` subtracts `LEVEL2_DISCOUNT` (15) times the
difference of the two sides' level-2 units on active cells. A level-2 unit thus counts 5 while its side can pay its
upkeep (the debt term still takes twice its cost otherwise). The plan
test of the debt rule asserts the evaluation exactly; its last position
holds a blue level-2 unit, so its expected value adds the discount for
red (the owner: "Add the new term (Recommended)"), and a new test checks
the term itself.

The bundle had 42 characters to spare: the plan bot's input loop, a copy
of the policy crate's, is now that crate's `io::turns`, which `io::play`
also uses. The loop also refuses a count over 144 lines (one per cell),
as the plan bot's did, for the policy's bots too. The bundle is 99,066
characters; it replays identically under the determinism check, and
against the screened binary it scored +10.4 [−15.0, +36.0] over 200
pairs.

## SPRT

Accepted on the pull request (run 40888044130, at a2c4b72), at CodinGame's
limits:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v009 vs ice-and-fire-v008: 754 games, 377 complete pairs
  ice-and-fire-v009: 420 wins, 0 draws, 334 losses (score 55.7%)
  Elo of ice-and-fire-v009 over ice-and-fire-v008: +39.8 [+18.6, +61.3] (95%, from pairs)
  pairs by ice-and-fire-v009's points (0, 1/2, 1, 3/2, 2): [49, 0, 236, 0, 92]
  ice-and-fire-v009 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v008 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): ice-and-fire-v009 19.6, 32.0, 34.3 ms; ice-and-fire-v008 24.3, 31.9, 34.3 ms
  slowest answer: ice-and-fire-v009 632.0 ms, ice-and-fire-v008 619.4 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.00 (bounds -2.94, 2.94) after 377 pairs
Verdict: ACCEPTED: ice-and-fire-v009 is stronger than ice-and-fire-v008
```

## League

The league of the release (at a2c4b72, 800 pairs a pairing) scored v009
60.1% against v008 (962 wins, 638 losses), 65.4% against v007, 72.9%
against v006 and 76.9% against v005, and rated it +2250.0 ± 182.5,
against v008's +2211.1 ([LEADERBOARD.md](../LEADERBOARD.md)).
