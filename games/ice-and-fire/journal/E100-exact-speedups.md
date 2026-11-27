---
id: E100
date: 2026-11-27
pull_request: "#399"
parent: ice-and-fire-v033
hypothesis: The same search made about 45% cheaper, with every answer unchanged, gets about 1.8 times the tries in each turn of CodinGame's time and beats ice-and-fire-v033.
change: Four changes that leave every score and answer as they are; the lethal chain's search is skipped when the gold cannot pay for the shortest path to the target; the start of a turn no longer marks the cells again; a capture grows the seat's marked land from where it was; and the shape term finds its joints with bitwise operations.
release: ice-and-fire-v034
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
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

Pending.
