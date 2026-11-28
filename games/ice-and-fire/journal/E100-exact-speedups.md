---
id: E100
date: 2026-11-27
pull_request: "#399"
parent: ice-and-fire-v033
hypothesis: The same search made about 45% cheaper, with every answer unchanged, gets about 1.8 times the tries in each turn of CodinGame's time and beats ice-and-fire-v033.
change: Four changes that leave every score and answer as they are; the lethal chain's search is skipped when the gold cannot pay for the shortest path to the target; the start of a turn no longer marks the cells again; a capture grows the seat's marked land from where it was; and the shape term finds its joints with bitwise operations.
release: ice-and-fire-v034
sprt: accepted
elo: +32.7 [+15.7, +49.7]
pairs: 480
full_time: +32.7 [+15.7, +49.7]
decision: promoted
cg_rank: Legend, 2nd of 92 (2nd of 936 overall), score 32.55, ranking ended 2026-11-28: submitted through the API on the owner's approval (submission 41836441)
---

## Why

The owner asked how the search spends its time and how to make it
cheaper (2026-11-27). E099's screening found that half the time loses
32 Elo against v033, so time is worth strength. A profile of v033's
search with callgrind (41 positions of v033's CodinGame games, 600 tries
each: 9.89 billion instructions) found where the time goes, and four
changes that leave every score unchanged. The owner chose to run the
SPRT directly, without a screening.

## Change

1. **The lethal chain's lower bound** (`tactics/src/chain.rs`):
   `chain_to` (the cheapest chain of trainings to a cell, with up to 8
   variants through enemy towers, each built on `Reach`) took 33% of the
   instructions: the search offers the lethal chain at every step of every
   climb, ours and the opponent's, and looks for it in every reply search.
   Every chain trains each cell not ours on its path, 10 gold at least
   each, so when the gold cannot pay 10 for each cell not ours on the
   shortest path to the target (`trainings_needed`, a breadth-first search
   on bitboards where our own cells cost nothing), no chain can, and the
   search is skipped: 9.89 to 7.03 billion instructions.
2. **No second marking at the start of a turn** (`engine/src/state.rs`):
   `start_turn` marked both sides' active cells again, two floods from the
   HQs, while every capture (`apply`) already keeps them marked; a debug
   build checks that they are: 7.03 to 6.47 billion.
3. **A capture grows the marked land** (`engine/src/state.rs`): after a
   capture, the seat's land only grew, so its active cells grow from those
   already marked (`grow`), a step or two, rather than flooding again from
   the HQ; the opponent's, which may have split, is flooded again as
   before; a debug build checks the result: 6.47 to 6.12 billion.
   Both rest on the marks being right. A position read from text keeps
   the marks it was given (the rules-check bot compares them with
   CodinGame's), so `State::marked` says whether they are the cells joined
   to each HQ: true from `State::new`, after every start of turn and for a
   read position whose marks are those; for one whose marks are not (a
   doctored input in the rules-check bot's tests), captures and the next
   start of turn flood the cells from the HQs as before. A new engine test
   covers it.
4. **The shape term with bitwise operations** (`bots/plan/src/eval.rs`,
   the owner's suggestion of image-processing operators): the cells
   within 3 steps of the enemy are its land dilated three times; a test
   on bit planes for every cell at once (`possibly_cut`: a cell whose held
   neighbours stay joined on the ring of 8 cells around it cannot cut
   anything off, digital topology's test of a removable cell) leaves
   about 4 of the 17 candidates; each one left is taken away and the land
   flooded from the HQ, the cells not reached being those it cuts off
   (`weakness_near`), in place of the depth-first search: 6.12 to 5.48
   billion. Growing the pieces round the removed cell in step was slower
   (5.96).

In all, 9.89 to 5.48 billion instructions for the same searches (−45%).

## Checks

- The same answers and the same scores (`judge`, 600 tries) on all 2,031
  positions of v033's 159 CodinGame ranking games, after each change.
- `weakness_near` against the depth-first search: no difference on
  160,204 lands (the positions and random plies from each); a new unit
  test compares them on many positions and on parts cut by random holes.
- Every test of the game's crates and bots passes with the debug checks
  on (the rules-check bot's included); the analysis build with debug
  assertions met no failed check on the 2,031 positions.
- The engine's speed example (`--example speed`, 2 s a row, two runs
  each, the same shared machine): random turns played 119,242 and 133,044
  a second on main against 148,994 and 149,882; whole random games 860
  and 958 against 1,065 and 1,073.

## SPRT

On the pull request at `26ea7d5`, in blocks of 10 pairs over 8 runners
calibrated to CodinGame's speed and real limit
([run 46052864652](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46052864652)):

```
ice-and-fire-v034 vs ice-and-fire-v033: 960 games, 480 complete pairs
  ice-and-fire-v034: 525 wins, 0 draws, 435 losses (score 54.7%)
  Elo of ice-and-fire-v034 over ice-and-fire-v033: +32.7 [+15.7, +49.7] (95%, from pairs)
  pairs by ice-and-fire-v034's points (0, 1/2, 1, 3/2, 2): [50, 0, 335, 0, 95]
  ice-and-fire-v034 faults: 1 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v033 faults: 3 timeouts, 0 crashes, 0 invalid answers
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.74 (bounds -2.94, 2.94) after 480 pairs
Verdict: ACCEPTED: ice-and-fire-v034 is stronger than ice-and-fire-v033
```

The smoke test against `random`: 200 wins in 200 games, no fault.

## What was learned

Speed alone, with every decision the same at equal tries, is worth
about 33 Elo here, as E099's half-time screening (−32.2) foretold. The
speed is not all used: in 8 local self-play pairs of each release at
CodinGame's speed, v034's tries a turn from game turn 7 rose 43% (602
against 421), not the 80% the instructions allow, since 26% of its turns
reach the cap of 32 climbs (`MAX_CLIMBS`, 5% for v033) and 75% run out of
first actions (57%), so that 31% of its turns end before 30 ms (8%). A
higher cap is a change of the search's choices, an experiment of its own.

The rules reviewer found no case where the engine's results differ from
before, and proposed a test of how `State::read` sets `State::marked`
(the new engine test sets the field by hand): left for a test-only pull
request after this one, since a code commit here would run the SPRT
again.
