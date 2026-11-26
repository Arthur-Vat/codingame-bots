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

Pending.
