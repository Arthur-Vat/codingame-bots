---
id: E023
date: 2026-10-29
pull_request: "#219"
parent: ice-and-fire-v009
hypothesis: In v009's games a cut the enemy could first afford with one more income was made within one turn about a quarter of the time; counting a cell the enemy can take at its next turn at 0.75 of a cell's value makes the search keep its territory out of reach and beats v009.
change: The plan bot's evaluation takes 7/4 (a quarter of a cell's value of 7, 4 as a cell and 3 as its income) off a side's score for each of its active cells that the enemy's chains of trainings reach for the enemy's gold plus its income.
release: ice-and-fire-v010
sprt: accepted
elo: +43.2 [+20.8, +65.8] at CodinGame's limits
pairs: 348
full_time: +43.2 [+20.8, +65.8] (the SPRT's own, ADR 0042), no timeout in 696 games, slowest answer 612.8 ms (the first turn)
decision: promoted
cg_rank: not reported
---

Chosen by the owner on 2026-10-29 after E024 ("E023, safe territory";
E021 and E024, both dropped, are on their own branches until merged),
from the study of towers and threats
([research/towers-and-threats.md](../research/towers-and-threats.md)),
which proposed the weight (0.75 of a cell's value) without measuring it.
The study measured cuts with the review tool's cut finder, far too slow
for the evaluation, so the owner chose two cheap definitions to screen as
two variants ("Both, as two variants"), for the enemy's next turn only
("Next turn only first (Recommended)"): the study's decay of 0.6 a turn
over one or two more incomes waits for this one's result.

## The change

`bots/plan/src/eval.rs`, `threatened`: a side's threatened cells are its
active cells that the enemy can take with chains of trainings costing at
most the enemy's gold plus its income (the gold it has at its next turn,
as the pressure term counts it; unit moves left out, as there). The
evaluation adds `7 × (enemy's threatened − own threatened) / 4`, in
integers, rounded toward zero (a difference of 1 cell scores 1, of 2
scores 3). A cell is worth 7 to its side in the shared evaluation (4 as an
active cell, 3 for its income of 1), so a threatened cell counts about 0.75
of that. The code's doc comment says "7/4 each"; it stays as frozen in the
release.

The shared tactics gain `chain::reach_within`, the cells a side's chains
reach within a budget, on the same bitboard search as `cost_to` (both now
call one `flood`). A test checks it against the Dijkstra of `Reach::new`
on random positions, with and without moves, at six budgets; the shared
`evaluate` keeps its 1.0 µs (`tactics/examples/timing.rs`). A plan test
checks the term on a corridor: blue with 20 gold threatens one cell, with
30 two, and red threatens blue's HQ.

The search test of E014 (`a_search_that_learns_nothing_new_climbs_from_other_first_actions`)
moves from seed 3's position to seed 2's, with the same assertions, as the
owner chose ("Move it to seed 2 (Recommended)"). On seed 3's position the
new evaluation ends the 17th climb on an empty plan, which stops the
search as it is built to; v009's evaluation stops as early on another
position (seed 1, ply 20: 18 climbs), and seed 2's reaches the 32 climbs
under both.

The branch takes in minifier level 2 (#218). The plan bot's bundle is
76,652 characters (87,977 with level 1); it plays exactly as its readable
copy in the 4 games of `scripts/check-minified.sh`'s comparison (500
fixed iterations, seed 7), and against the screened level-1 binary it
scored +3.5 [−24.6, +31.6] over 100 pairs (seed 25027), with no fault.

## Screening

Against `ice-and-fire-v009` at CodinGame's limits (50 ms a turn, 1,000 ms
the first, no tolerance), 400 seat-swapped pairs each on seed 25025, two
games at a time on a 4-core machine; the best again on 600 pairs with seed
25026. Bundles built with `cg-bundler` (minifier level 1) and rustc
1.90.0; read with `ice-and-fire-review screening`. "Reach" is the change
above; "reach and cut-offs" also counts, on top of the threatened cells,
the most cells that taking one of them cuts off from the side's HQ
(`ice_and_fire_opening::solo::severed`). v009's numbers in the same games
in parentheses.

| Variant | Elo against v009 | Score as red | Score as blue | Towers | Mines | Level 1 / 2 / 3 | Gold kept | Timeouts | Median, max ms |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | ---: | --- |
| reach and cut-offs | **+30.5 [+10.3, +50.9]** | 44.5% | 64.2% | 4.00 (3.72) | 2.07 (1.84) | 24.5 / 3.7 / 1.8 (24.4 / 4.1 / 1.7) | 38.5 (25.9) | 1 (2) | 15.6, 111.0 (13.7, 72.5) |
| reach | **+52.5 [+32.4, +72.9]** | 48.5% | 66.5% | 4.03 (3.62) | 2.23 (1.90) | 25.3 / 3.8 / 1.8 (24.0 / 4.1 / 1.7) | 35.6 (25.8) | 2 (1) | 12.3, 93.3 (13.2, 96.1) |
| reach, 600 pairs, seed 25026 | **+53.1 [+36.2, +70.3]** | 45.5% | 69.7% | 4.04 (3.65) | 2.15 (1.77) | 25.6 / 3.7 / 1.7 (24.4 / 4.1 / 1.6) | 35.5 (23.7) | 4 (2) | 13.9, 87.3 (14.8, 111.7) |

Every bot's answers were at most 31.9 ms in 99% of turns; the slowest
first answers were 636 to 690 ms. The candidate's timeouts, 6 in its
2,000 games of "reach", are within the SPRT's 1%.

## The candidate

The owner chose it as the candidate on 2026-10-30 ("Go on with the reach
only candidate"): `ice-and-fire-v010`, frozen by `scripts/new-release.sh`
(76,652 characters, the same answers as its readable copy in 4 games).

## What was learned

- **Keeping territory out of the enemy's reach is worth about +50.**
  Over its 1,000 pairs, "reach" scored +52.9 against v009. It keeps more
  gold unspent (35.5 against 23.7 a turn): a side's gold widens its reach
  and so threatens more enemy cells, and spending it where that leaves
  cells exposed scores worse. It builds more towers (4.0 against 3.6) and mines, and a
  few fewer level-2 units.
- **The cut-offs add nothing.** Counting the cells a threatened joint cuts
  off scored 22 less and was 2 ms slower: the shape term (E010) already
  counts joints and the largest cut-off, and the search plans against the
  opponent's best cuts.
- Next, from the study: the decay of the threat over one and two more
  incomes (0.6 a turn), and the weight itself (0.75 was proposed, not
  measured).

## SPRT

Accepted on the pull request (run 41018447625, at 33563b6, the same code
as the head), at CodinGame's limits:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v010 vs ice-and-fire-v009: 696 games, 348 complete pairs
  ice-and-fire-v010: 391 wins, 0 draws, 305 losses (score 56.2%)
  Elo of ice-and-fire-v010 over ice-and-fire-v009: +43.2 [+20.8, +65.8] (95%, from pairs)
  pairs by ice-and-fire-v010's points (0, 1/2, 1, 3/2, 2): [45, 0, 215, 0, 88]
  ice-and-fire-v010 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v009 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): ice-and-fire-v010 18.8, 31.7, 32.3 ms; ice-and-fire-v009 19.6, 31.7, 32.2 ms
  slowest answer: ice-and-fire-v010 612.8 ms, ice-and-fire-v009 613.7 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.98 (bounds -2.94, 2.94) after 348 pairs
Verdict: ACCEPTED: ice-and-fire-v010 is stronger than ice-and-fire-v009
```

## League

The league of the release (run 41019647778, at 96d9bcb, 800 pairs a
pairing) scored v010 56.9% against v009 (911 wins, 689 losses), 61.9%
against v008, 69.4% against v007, 73.3% against v006, 81.2% against v005,
83.1% against v004 and 99.8% against bronze, and rated it +2286.0 ± 182.5,
against v009's +2248.0 ([LEADERBOARD.md](../LEADERBOARD.md)). v010 lost 2
of its 11,200 games by a fault.
