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
3. **The number of distinct end territories (sets of cells) with the most cells,** by a breadth-first enumeration while a layer holds at most `--cap` states (20,000 in the run below); past that, the start is reported as having more.

A depth-first search stops after `--limit` states expanded (20,000 below; each costs about a millisecond, since it makes every turn of the state) with the best plan found so far, marked unproven: a lower bound on the optimum. The searches try the turns that hold the most of the zone first. **The best plans are kept** (`--plans`), the commands of each turn as CodinGame reads them, so that later work need not search again: `--replay` plays them back through the engine and checks their values, `--from` starts later searches from them, and `--bench` measures any opening against them in seconds (the owner's choices, form of 2026-11-19).

Beside it, the same model plays v029's opening (the solo planner, untimed), the same with the assignment's steps (B of [E072](../../journal/E072-frontier-assignment.md)), the opening's rules alone and the assignment alone, and the suite gives zasmu's own active cells at turn T + 1 (its games are two-sided, so that number is only a pointer).

**Checks.** `--check` runs, on 3, 4 and 5 turns of 6 starts, a full enumeration without any cut and asserts that the pruned enumeration and the depth-first searches give the same maximum, the same optimal end states and the same best zone counts (a zone of every other cell) within 0 and 1 cells; it passes. A first version bounded a state's zone cells by those within one step a turn of its land; on 5 turns it found one zone cell fewer than the full enumeration on one start, since a unit's step and a training beyond it take two steps in a turn. The bound now allows a step for each training the gold can pay. A version that kept whole engine states gave the same counts as the one that rebuilds them from their keys (409, 6,580 and 62,393 end states on three of them).

## Results

Study run 44626073005 (the study workflow, #368 and #370), at the solver's commit 4493de6 with `--limit 20000 --cap 20000`, on 12 runners for up to 300 minutes each: 78 of the 94 starts (all 44 of zasmu's red starts, 34 of its first 50 blue ones; the rest were cut by the time limit), at T = 6, 7 and 8. Its files are in [2026-11-20-optimal-early-plans/](2026-11-20-optimal-early-plans/): `oracle.csv` (a row a start and horizon), `oracle.plans.tsv` (1,638 best plans; `--replay` plays all of them back with their stored values) and `bench.md`.

**What is proven.** The most cells: every start at turns 6 and 7, 72 of 78 at turn 8. The most middle chokes at the most cells: every start at turn 6, 76 of 78 at turn 7, 68 of 78 at turn 8. The most cells of the middle band: 72 of 78 at turn 6, 39 at turn 7 and 13 at turn 8; within 1 or 2 cells of the most, fewer still (61 to 4 of 78). Elsewhere the value is the best plan found, a lower bound, so the gaps to it below are at least what they say. (A value is proven only when the most cells it is measured from is; the run's files were written before that rule, and 20 of their rows were corrected to unproven.)

### The most land, and our openings

The mean most cells at the end of turn T, and each opening's share of the starts where it holds as many, with its mean shortfall (the side alone, the other passing); zasmu's mean active cells at the start of game turn T + 1 in its real games, from the suite, beside it:

| T | Seat | Starts | Most cells | v029's opening | B | The rules alone | The assignment alone | Zasmu at T + 1 (two-sided) |
| ---: | --- | ---: | ---: | --- | --- | --- | --- | ---: |
| 6 | red | 44 | 16.86 | 82%; 0.18 short | 34%; 1.30 | 16%; 2.02 | 82%; 0.20 | 16.75 |
| 6 | blue | 34 | 19.79 | 79%; 0.26 | 68%; 0.53 | 56%; 1.03 | 65%; 0.47 | 19.68 |
| 7 | red | 44 | 22.73 | 68%; 0.59 | 34%; 2.09 | 16%; 3.36 | 75%; 0.59 | 21.95 |
| 7 | blue | 34 | 25.79 | 68%; 0.50 | 59%; 0.85 | 44%; 1.68 | 50%; 0.74 | 24.70 |
| 8 | red | 44 | 29.59 | 36%; 1.30 | 23%; 2.93 | 7%; 4.61 | 59%; 0.84 | 26.45 |
| 8 | blue | 34 | 33.65 | 41%; 1.35 | 29%; 1.68 | 21%; 2.74 | 32%; 1.26 | 29.23 |

v029's opening holds the most land at turn 6 on four starts in five, a fifth of a cell short on average; it falls behind later because its 9-turn plans do not aim at land at turn 7 or 8, and from turn 7 the bot's search plays in its place anyway. Zasmu, in its two-sided games, holds the side-alone maximum at turn 6 to within 0.11 of a cell, and falls behind it from turn 7 as the sides meet.

### How many plans reach it

Distinct end territories with the most cells, on the starts where the enumeration stayed under 20,000 states a layer (the others have more):

| T | Seat | Counted | More than the cap | Distinct optimal territories: min / median / max |
| ---: | --- | ---: | ---: | --- |
| 6 | red | 22 | 22 | 15 / 401 / 7,995 |
| 6 | blue | 6 | 28 | 508 / 3,812 / 6,222 |
| 7 | red | 6 | 38 | 135 / 2,427 / 5,848 |
| 7 | blue | 1 | 33 | 101 |
| 8 | both | 0 | 78 | |

The most land is reached by hundreds to thousands of different territories on the starts counted, and by more on most starts: which of them a side ends on is a choice the most-land goal leaves free.

### Middle chokes and the middle band at the most land

On the maps with at least one middle choke: the most middle chokes a plan can hold with the most cells (and within 1 and 2 cells of it), what v029's opening holds, and the share of starts where it holds the most a plan can hold *with its own cells*; then the same for the cells of the middle band (within 2 steps of even between the HQs), all maps:

| T | Seat | Maps with chokes | Most chokes within 0 / 1 / 2 cells | v029's chokes | v029 at the most for its cells | Middle cells within 0 / 1 / 2 cells | v029's middle cells |
| ---: | --- | ---: | --- | ---: | ---: | --- | ---: |
| 6 | red | 28 | 0.82 / 0.82 / 0.82 | 0.11 | 29% | 0 / 0 / 0 | 0 |
| 6 | blue | 19 | 2.63 / 2.68 / 2.68 | 1.21 | 32% | 2.00 / 2.00 / 2.00 | 1.06 |
| 7 | red | 28 | 3.18 / 3.18 / 3.18 | 1.46 | 20% | 2.89 / 2.89 / 2.89 | 1.75 |
| 7 | blue | 19 | 4.89 / 5.00 / 5.00 | 2.58 | 17% | 5.38 / 5.65 / 5.65 | 3.71 |
| 8 | red | 28 | 4.86 / 4.89 / 4.89 | 3.07 | 14% | 7.32 / 7.32 / 7.32 | 4.52 |
| 8 | blue | 19 | 5.47 / 5.58 / 5.63 | 4.05 | 31% | 9.50 / 10.18 / 10.24 | 6.38 |

The bench (`--bench`, every start, each opening against the best plans at its own cells), middle chokes short of the most at turn 6 / 7: v029's opening 0.71 / 1.68 (red) and 1.42 / 2.33 (blue); the assignment alone 0.50 / 1.72 and 0.72 / 2.00; middle cells short at turn 6 (blue): v029 0.94, the assignment 0.24.

- **Land and the middle chokes do not compete early.** Allowing 1 or 2 cells less land buys almost no more middle chokes at turns 6 to 8, nor middle cells at turn 6 (both mostly proven): the plans with the most land can already hold the most of them. For the middle band at turns 7 and 8 the values within 1 or 2 cells are mostly lower bounds, and as blue at turn 8 they rise (9.50 to 10.24), so there the trade is not settled.
- **v029's opening leaves the middle on the table.** With the land it holds, plans exist that hold 1.9 to 7 times its middle chokes at turns 6 and 7 (0.82 against 0.11 as red at turn 6, 2.63 against 1.21 as blue, 3.18 against 1.46 as red at turn 7, 4.89 against 2.58 as blue) and 1.45 to 1.9 times its middle cells; it holds as many chokes as possible on a sixth to a third of the starts.
- **The assignment comes closer, and still lost.** The joint assignment of E072 is nearest the best plans on chokes and middle cells at turn 6, as it was built to be, yet lost the duels cut at turn 12 (−21 to −121): holding the middle in a side-alone plan is not yet holding it against an opponent who comes for it, nor keeping a land the search can defend from turn 7.

## B: the assignment inside the beam

[E072](../../journal/E072-frontier-assignment.md) offered the assignment's turns to the solo beam as extra steps, with the opening playing until game turn 6, 7 or 8, judged by duels cut at turn 12 against v029. The beam chose them, since they score better by its own objective, and they lost 50 to 90 Elo. Playing v029's opening longer lost too (−18 and −24 at turns 7 and 8), and a weight on the middle chokes alone was level at turn 6. With the pilot above, the picture so far: the opening finds the most land; what it lacks is a sound way to choose among the many plans that reach it, and its 9-turn objective with a passing opponent is not that way.

## What follows

The full run settles the first question of this note: our opening finds the most land; what it lacks is the choice, among the many plans that reach it, of those that hold the middle, and for the middle chokes (and the middle band at turn 6) that choice costs no land. Next steps, for the owner to choose:

1. **A lexicographic choice in the opening (small):** the solo beam keeps its land-first objective but breaks ties by the middle chokes, then the middle cells, held at the end of its plans; `--bench` tells in seconds whether it closes the gap above without losing land, then the duels cut at turn 12 whether that wins. Unlike the additive choke weight of E072's B (b3, level at turn 6), it gives up no land.
2. **The two-sided check (step 3 of phase 2):** the same choice judged against an opponent that expands too, since the assignment shows that holding the middle alone does not win; the stored best plans give its targets.
3. **Proofs where they matter (later):** a run with `--from` on the committed plans and a larger limit turns the middle band's lower bounds at turns 7 and 8 into proofs, if a decision ever rests on them.

## Reproduce

```sh
# The run: through the study workflow (scripts/study.sh), or on one machine
cargo run --release -p ice-and-fire-opening --example oracle -- --suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 50 --turns 6,7,8 --limit 20000 --cap 20000 --jobs 4 --csv oracle.csv --plans oracle.plans.tsv
# Its plans played back, and the openings measured against them
cargo run --release -p ice-and-fire-opening --example oracle -- --replay games/ice-and-fire/research/codingame/2026-11-20-optimal-early-plans/oracle.plans.tsv
cargo run --release -p ice-and-fire-opening --example oracle -- --bench games/ice-and-fire/research/codingame/2026-11-20-optimal-early-plans/oracle.plans.tsv --jobs 4
# The check against full enumeration
cargo run --release -p ice-and-fire-opening --example oracle -- --suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu --per-seat 3 --turns 4 --check
```
