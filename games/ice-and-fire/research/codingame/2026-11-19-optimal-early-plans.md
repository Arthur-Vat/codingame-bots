# Optimal early plans, and what the opening should choose among them (2026-11-19)

Phase 2 of the early-game rework ([2026-11-19-early-game.md](2026-11-19-early-game.md)), step 1. The owner asked, on 2026-11-19, to step back from quick results: to lay out the mathematical problem of the early game, the evaluation functions in play and the options, and to find out how many optimal plans each map has, with our openings playing until turn 6, then 7, then 8 ("Do not rush for quick results, try to make something solid and sound"; form: "Go as planned").

## The problem

**One side alone.** Until the territories meet, a side's turn is: each unit steps onto a neighbour or stays, then it trains units on cells next to its active land. Without mines, each turn's state is its cells `C`, its units `U` and its gold `g`, and

- a unit takes at most one new cell a turn, and a training takes the cell it lands on;
- at its next start of turn, `g ← g − 10·(trainings) + |C| − |U|` (income: active cells less a level-1 unit's upkeep of 1).

So a unit trained at game turn τ takes at most `T − τ + 1` cells by the end of turn T, and

> **cells(T) = cells at the start + Σ over units (T − τ_u + 1) − wasted moves**

where a wasted move is a unit that takes no new cell in a turn (it stays, or walks on its own land). Holding the most land at turn T splits into two problems:

1. **The economy:** a schedule of trainings (τ₁ ≤ τ₂ ≤ …) under the gold recursion; buying as soon as the gold allows is never worse for the cells.
2. **The geometry:** whether the units can each take a new, distinct cell every turn, despite void cells, dead ends and paths that cross.

Before contact the map leaves the geometry much freedom, so we expected many different plans to reach the same, most land; the question that matters is then which of them to choose. The solver below measures exactly that.

**With the opponent.** The middle chokes are contested (zasmu holds one by turn 7 in 93-94% of its games, [2026-11-19-middle-chokes.md](2026-11-19-middle-chokes.md)), so a side alone overvalues land the enemy reaches first and trainings late in the opening. A two-sided search (step 3 of phase 2) answers that; this note keeps to the side alone, which is what our opening plans with.

## The evaluation functions in play

| Where | Horizon and opponent | Function |
| --- | --- | --- |
| The opening, game turns 1 to 6 (solo beam, `ice_and_fire_opening::solo`) | 9 of our turns, the opponent passing | 3 × active cells + income + gold − 2 × (cells one capture cuts off + joints + line ends) − 6 × steps to the enemy HQ + cells nearer us than the enemy |
| The middle game, from game turn 7 (`bots/plan/src/eval.rs`) | Our turn, the worst of the opponent's remembered replies | The shared material and threat evaluation and about ten terms (debt, shape, threatened cells, mines, standing threat, upkeep of strong units, tower wall) |
| The judge (cut duels, [2026-11-19-zasmu-likeness.md](2026-11-19-zasmu-likeness.md)) | Game turn 12, both sides played | Zasmu-likeness: a logistic log-likelihood ratio on active cells, income, Voronoi share, middle-choke share, units, gold and more, per seat |

Three objectives, three horizons and two models of the opponent meet in the first 12 turns.

## The solver

[`opening/examples/oracle.rs`](../../opening/examples/oracle.rs) solves the side-alone problem exactly under the engine's rules: each turn every combination of unit steps (or staying) and every set of level-1 trainings, the other side passing, with no mines, towers or higher levels (no opening builds them this early). States that are the same for the rest of the game merge: our cells, our units' places and our gold. Every search cuts with one upper bound on the cells a state can still reach, the economy above with every unit taking a new cell every turn and the gold spent as soon as it allows:

```text
each turn left: k = gold / 10; cells += units + k; units += k; gold += cells − units − 10 k
```

A move that takes no cell costs at least one cell, so such moves are tried only when the bound leaves room. For each start (a map and seat of zasmu's in the opening suite [`openings/suite-v1.tsv`](../../openings/suite-v1.tsv)) and each horizon T:

1. **The most cells at the end of turn T:** a depth-first search for one cell more than the best of the openings below, again until none exists.
2. **The most middle chokes, and the most cells of the middle band (within 2 steps of even between the HQs), within 0, 1 and 2 cells of the most:** branch and bound, a state cut when what it holds plus what lies within the turns left of its land (at most the cells it can still take) cannot beat the best found.
3. **The number of distinct end territories (sets of cells) with the most cells,** by a breadth-first enumeration while a layer holds at most a million states; past that, the start is reported as having more.

Beside it, the same model plays v029's opening (the solo planner, untimed), the same with the assignment's steps (B of [E072](../../journal/E072-frontier-assignment.md)), the opening's rules alone and the assignment alone, and the suite gives zasmu's own active cells at turn T + 1 (its games are two-sided, so that number is only a pointer).

**Checks.** `--check` runs, on 3, 4 and 5 turns of 6 starts, a full enumeration without any cut and asserts that the pruned enumeration and the depth-first searches give the same maximum, the same optimal end states and the same best zone counts (a zone of every other cell) within 0 and 1 cells; it passes. A first version bounded a state's zone cells by those within one step a turn of its land; on 5 turns it found one zone cell fewer than the full enumeration on one start, since a unit's step and a training beyond it take two steps in a turn. The bound now allows a step for each training the gold can pay. A version that kept whole engine states gave the same counts as the one that rebuilds them from their keys (409, 6,580 and 62,393 end states on three of them).

## A first look: 4 starts

The full run, zasmu's 44 red and 50 of its blue starts at T = 6, 7 and 8, is about an hour on 4 processors: it runs on GitHub's runners through the study workflow, and its results will complete this note. A pilot on 2 red and 2 blue starts:

| Start | T | Most cells | Distinct optimal territories | v029's opening | Assignment alone | Zasmu at T + 1 |
| --- | ---: | ---: | --- | ---: | ---: | ---: |
| red 1 | 6 | 17 | 129,169 | 17 | 17 | 17 |
| red 2 | 6 | 17 | 84,509 | 17 | 17 | 17 |
| blue 1 | 6 | 20 | more than a million states in a layer (not counted) | 20 | 20 | 20 |
| blue 2 | 6 | 18 | 775 (517 of them hold a middle choke) | 18 | 18 | 18 |
| red 1 | 7 | 23 | more than a million states in a layer (not counted) | 23 | 23 | 23 |
| red 2 | 7 | 23 | more than a million states in a layer (not counted) | 23 | 23 | 21 |
| blue 1 | 7 | 26 | more than a million states in a layer (not counted) | 26 | 26 | 25 |
| blue 2 | 7 | 24 | 2,560 | 23 | 24 | 23 |
| red 1 | 8 | 30 | more than a million states in a layer (not counted) | 30 | 30 | 28 |
| red 2 | 8 | 30 | more than a million states in a layer (not counted) | 30 | 29 | 23 |
| blue 1 | 8 | 34 | more than a million states in a layer (not counted) | 34 | 34 | 30 |
| blue 2 | 8 | 32 | 2,294 | 27 | 31 | 27 |

What the pilot suggests, to be checked on the full run:

- **The maximum looks degenerate:** on the three starts where they were counted, 775 to 129,169 distinct territories hold the most land by turn 6 (2,294 to 2,560 at turns 7 and 8 on the one counted there); the others passed a million states in a layer of the enumeration before its end, their territories not counted. If that holds on the full run, which of them a side picks is a free choice, so the opening's evaluation decides it. Four starts are a pointer, not a rate.
- **Our opening already holds the most land at turns 6 and 7** on all but one start (one cell short there). At turn 8 it is up to 5 cells short on one start: its plans look 9 turns ahead and do not maximise land at turn 8.
- **Zasmu, playing two-sided, matches the side-alone maximum at turn 6** on these starts and falls behind it later, as contact begins.

The pilot's figures on middle chokes are left out: they came from a search whose cut on the zone was too tight (a training chained after a unit's step reaches two steps in one turn), found by the review and fixed before the full run.

## B: the assignment inside the beam

[E072](../../journal/E072-frontier-assignment.md) offered the assignment's turns to the solo beam as extra steps, with the opening playing until game turn 6, 7 or 8, judged by duels cut at turn 12 against v029. The beam chose them, since they score better by its own objective, and they lost 50 to 90 Elo. Playing v029's opening longer lost too (−18 and −24 at turns 7 and 8), and a weight on the middle chokes alone was level at turn 6. With the pilot above, the picture so far: the opening finds the most land; what it lacks is a sound way to choose among the many plans that reach it, and its 9-turn objective with a passing opponent is not that way.

## What follows

Once the full run is in:

1. **If most maps have cell-optimal plans that hold more middle chokes or middle cells than v029's,** the opening should choose among equal-land plans by contested value, lexicographically (cells first, then chokes and the middle) or with weights set from the measured price of a choke in cells, not guessed ones.
2. **If taking a choke always costs cells,** that price is the choke's weight, and the two-sided search (step 3) must say whether it pays against an opponent who contests it.
3. **If v029's opening is already at the optimum of both,** the gap to zasmu is two-sided (contact and the race at turns 7 to 12) and step 3 comes next.

## Reproduce

```sh
cargo run --release -p ice-and-fire-opening --example oracle -- --suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 2 --turns 6,7,8 --jobs 4 --csv pilot.csv
cargo run --release -p ice-and-fire-opening --example oracle -- --suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 3 --turns 4 --check
```
