---
id: E087
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: A search that models the opponent's cuts that start with its units' steps, charges a plan for its second-worst reply as well as its worst, and is offered pairs of purchases near the front, stops opening weaknesses the opponent punishes and spends better, and beats ice-and-fire-v032.
change: Replies with steps before a cut, a charge on the second-worst reply, and spending bundles in the search; each setting at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E086's entry ("A unit that left the line"): at game turn 10 of race2's game
on map 925707781, v032's search moved unit 11 out of our line, and v032
answered with a step of its unit next to our line followed by a cut (a
level-2 unit killing ours, the cell left empty taken, unit 11 cut off and
killed). The search's replies did not hold that kind: its cut finder chains
trainings only, and E037's threat lets only units of level 2 or more step
first. The owner (2026-11-26): all those kinds of cuts must be considered
in the opponent's replies; and with K gold the search should weigh many
ways to spend: several units, several towers, both, or saving.

With the replies below, the search models the real cut on that position
(it leaves us at 97 to 106), but rates a cut in the north worse (73), the
same worst reply whether unit 11 moves or stays (64 if it stays): the move
still wins by 9. The worst reply alone does not charge a plan for opening a
second weakness; hence the second-worst charge.

## The change

In `bots/plan/src/`:

- `search.rs`, `REPLY_STEPS`: bit 1, all of the opponent's movable units
  step toward our land (onto a cell of ours they may take, else a free cell
  next to our land), then its 2 best cuts (chains of up to
  `STEP_CUT_LENGTH`, 4, trainings); bit 2, each such step alone (the first
  `STEP_REPLIES`, 8, captures first), then its best cut.
- `search.rs`, `SECOND_QUARTERS`: a plan's score, the worst evaluation
  after its replies, less this many quarters of its second-worst reply's
  damage (the evaluation of the plan's end with no reply less the
  evaluation after that reply).
- `actions.rs`, `BUNDLE_CANDIDATES`: the first this many single trainings
  and towers on cells next to the enemy's land or to a cell it can take at
  its next turn, paired, each pair the gold pays for offered as one action;
  saving stays the climb's default (nothing added).

## Games and screening

Variants: steps1 (`REPLY_STEPS` 1, 58298d300a2b), steps3 (3, d42ffc643e34),
bundles6 (`BUNDLE_CANDIDATES` 6, fd1ee360beef), second2 (`SECOND_QUARTERS`
2, 7941e16e1f94), steps3-second2 (008d31e697d4), steps3-bundles6
(6b2b89ab3116), steps3-second2-bundles6 (1a211d7ec786).

A local check of the heaviest (steps3-second2-bundles6) in full games
against v032, 8 pairs at CodinGame's limits (seed 87): no fault, answers
after the first 41.7 ms median, 45.5 at 99%, 47.3 at 99.9% (v032 30.8, 43.7,
45.4): close to the 50 ms limit.

Games against v032 on E086's 20 maps, both seats, cut at game turn 12, 40
each, measured as E086's (v032 against itself the baseline):

| Bot | Middle cells lost, turns 6 to 10 | All cells lost, turns 6 to 10 | Holds its centre at turns 8 / 10 | Cells at turn 12, its / v032's |
| --- | --- | --- | --- | --- |
| v032 | 0.47 | 0.85 | 64% / 86% | 35.5 / 36 |
| steps1 | 0.75 | 1.23 | 64% / 89% | 35.5 / 39 |
| steps3 | 0.72 | 1.15 | 64% / 86% | 34.5 / 41 |
| bundles6 | 0.70 | 1.10 | 61% / 86% | 36 / 37 |
| second2 | 0.50 | 0.75 | 58% / 81% | 34 / 38.5 |
| steps3-second2 | 0.50 | 0.75 | 61% / 81% | 35.5 / 37.5 |
| steps3-bundles6 | 0.68 | 1.10 | 67% / 86% | 36 / 39.5 |
| steps3-second2-bundles6 | 0.80 | 1.40 | 56% / 78% | 34 / 38 |

The second-worst charge is the only change that loses no more cells than
v032 (0.75 against 0.85 in all, 0.50 against 0.47 in the middle), at a
cost in land (34 to 35.5 cells at turn 12, v032 37.5 to 38.5); modelling
more replies (steps) alone makes the search lose more and gain less land.
40 games are few: the screening is needed.
