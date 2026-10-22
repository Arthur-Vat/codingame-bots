---
id: E004
date: 2026-10-22
pull_request: "#185"
parent: ice-and-fire-v003
hypothesis: v003's opening with weights chosen per map on the first turn, among v003's and a portfolio of 16, for the middle game that a heuristic score rates best (the economy 3 turns into the middle game, then the Voronoi share) against v003's opening simulated, beats v003's fixed weights.
change: Until the middle game the `plan` bot plays v003's opening with the weights whose rollout against v003's opening scores best, chosen on the first turn and again when our side differs from the prediction; the plan search takes over for good at the middle game instead of at a gap of 3.
release: ice-and-fire-v004
sprt: accepted
elo: +51.9 [+27.1, +77.3] at CodinGame's limits
pairs: 364
full_time: +51.9 [+27.1, +77.3] (the SPRT's own, ADR 0042), no timeout in 729 games, slowest answer 27.5 ms
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-10-22, after E003: "keep focusing on the early
game, by trying to spend time correctly during the first turn: the opening's
weights aren't tuned; the 1,000 ms of the first turn are still unused. 1.
Identify potential strategies 2. Benchmark them only in the early game:
consider we reached the middle game when one player can reach the opponent
territory 3. Then pick the best strategies and replace the release early
game with selected early games 4. Pick the best one as a new release ... I
want to have something very efficient." The plan, agreed in forms the same
day: a score of hand-built heuristics, never fitted; the turn-1 players S0
(v003), S1 (tuned weights), S2 (weights chosen per map), then a dynamic
program, beam search, simulated annealing and the cross-entropy method, each
kept only if it beats S2 at equal time; the best 1 or 2 inside the bot, an
arena check against v003, the best through the SPRT.

## The tools (#184)

The crate `ice-and-fire-opening` holds v003's opening (`econ`), the start
of the middle game (the player to act can pay, with its gold, a chain of
trainings to an enemy cell) and the score. Its harness, the example
`early`, plays openings against each other in process, from both seats of
many source maps, up to the middle game, and scores both sides there; the
arena's opening benchmark scores real games the same way. Details:
[research/early-game.md](../research/early-game.md#the-middle-game-score-e004).

**The score**, from heuristics, after the owner's direction: the economy
first (gold + 3 × income, the gold held 3 turns into the middle game), then
the Voronoi share (1 gold a cell, a turn of its income), less the enemy's
best cut on its next turn (3 a cell and 12 a mine, their income until the
horizon, and the units' value), debt, and a payable chain to the HQ. The
rankings below are the same at 2 and 5 turns.

## Three scores

The score went through three versions; only the last is used below.

1. **The plan's first score** (gold + (20 − turn) × income + free cells left
   − the enemy's cuts after banking 1 to 3 turns), changed after comparing
   it with E003's games, then optimised. Its best openings lost to v003 at
   CodinGame's limits (seed 51, 400 pairs each): the beam search 44.4%,
   −39.3 Elo [−62.9, −16.0]; S2 46.4%, −25.2 [−48.1, −2.5]. The plans were
   made for the middle game but the plan search took over at a gap of 3,
   before it; with the hand-over moved to the middle game (seed 61, 200
   pairs) v003's own opening scored +6.9 [−25.0, +39.0], the beam −1.7
   [−34.3, +30.8] although it led by 47 gold at the middle game of those
   games. The owner judged the method wrong: a score must define a good
   position from heuristics, not be shaped by played games. The hand-over
   at the middle game was kept, so that the opening is played up to the
   position it was chosen for.
2. **Voronoi first**, at 10 gold a cell (a training), then the economy, as
   the owner first directed. The beam search that maximised it led v003 by
   15 Voronoi cells at the middle game of real games, with 11.5 gold
   against 29.0, and lost: 36.4%, −97.1 Elo [−120.3, −74.7] (seed 81, 400
   pairs); the score's sign agreed with the winner in 36.5% of the games.
   The owner then set the economy first.
3. **The economy first**, above.

## Screening

Rust 1.90.0 for the bots; the harness at 600 ms for the first turn and 35 ms
for a new choice later, four games at a time.

**S1**, coordinate search over econ's ten weights on maps 1 to 1,000
against econ, fill and rush (the mean of the three scores): v003's weights
+19.2, the search's +22.5, with `contest_cap` 16 (v003: 12), `hq_div` 2 (4),
`min_turns_left` 7 (6), `safe_spot` 8 (3), `contact_offset` 3 (2): v003's
weights are near the best fixed set by this score.

**S2**, a portfolio of weight sets, one chosen per map on the first turn by
a rollout against econ: greedily from 120 sets around v003's and S1's, on
maps 1 to 1,000; 16 sets reach +25.9 (32 sets +25.9; the best set for each
game, an oracle, +34.3).

**The players at equal time** (600 ms on the first turn), on maps 2,001 to
2,300 from both seats, the score of each against econ, fill and rush (± the
95% interval, about 1 to 2):

| Player | econ | fill | rush | Mean |
| --- | ---: | ---: | ---: | ---: |
| v003's econ (S0) | 0 | +20.6 | +37.2 | +19.2 |
| S1 | +3.9 | +22.9 | +40.3 | +22.4 |
| **S2 (16 sets), about 17 ms** | +9.6 | **+23.8** | **+41.7** | +25.0 |
| Dynamic program over purchases (P2) | +14.5 | +16.8 | +36.6 | +22.6 |
| Beam search over whole turns (P1), width 150 | **+22.2** | +17.1 | +36.3 | **+25.2** |
| Simulated annealing (P4) | +19.9 | +18.2 | +37.4 | +25.2 |
| Cross-entropy method (P5) | +14.6 | +20.4 | +38.8 | +24.6 |

No planner beats S2 against every opponent: those that plan whole turns
beat it against econ, the model they plan against, and lose to it against
fill and rush. NRPA (P6) was not tried. The Hungarian moves of P3 were not
built: the planners' gains come from the moves' weights and the purchases,
mixed turn by turn from the portfolio (in the first attempt, the beam over
v003's weights alone scored +28 against econ, with 4 sets +42, with 16
sets +45). Under the first score, the beam's width hardly mattered (width
60, 150 and 400 within 2 points; 150 finishes in about 260 ms, 400 in 552
ms).

**The best two inside the bot**, against v003 at CodinGame's limits
(seed 91, 400 pairs each, two games at a time on a 4-CPU machine):

| Bot | Score | Elo | At the middle game: score, its sign agrees with the winner | Faults |
| --- | --- | --- | --- | --- |
| S2 | 453 wins, 347 losses (56.6%) | +46.3 [+22.2, +70.8] | +16.6, 57.9% | 3 timeouts (v003: 0) |
| Beam search | 368 wins, 432 losses (46.0%) | −27.9 [−51.6, −4.4] | +26.4, 45.1% | 4 timeouts (v003: 3) |

The timeouts all came in the plan search of the middle game, which is
v003's, on a busy machine (v003 had 1 to 3 per 800 games in the session's
other runs). At its first start of turn in contact, S2 has 39.4 gold against
v003's 31.4, with 3.3 units against 3.6 and the same cells and income
(20.7): it buys a little less and keeps the gold for the first fights.

All screening code is in commit 92d96bd of the pull request; the next
commits keep the portfolio's choice in the bot and the beam in the crate.

## The candidate

The `plan` bot (bundle 89,885 characters): lethal first; until the middle
game, v003's opening with the weights chosen on the first turn from v003's
and the 16 of the portfolio (`ice_and_fire_opening::planned`,
`Planner::Choose`, about 20 ms of the first turn's 600), chosen again when
our side differs from the prediction; then, for good, v003's plan search.

**The first head (834c450)** passed the SPRT: accepted after 325 pairs,
57.6% of 651 games, +52.8 Elo [+27.9, +78.3], LLR 2.95, no timeout, crash
or invalid answer (answers after the first: median 4.1 ms, 99.9% 26.5 ms,
slowest 30.4 ms); smoke test 200 of 200 against `random`. Its review then
found that the opening could come back after the middle game: the test of
the middle game (can the player to act pay a chain to an enemy cell?) was
made afresh each turn, so a later turn short of gold played the opening,
which ignores threats (55 such turns in 30 games of the bot against itself,
54 of them with the enemy able to strike). The opening now ends for good at
the middle game: at our first start of turn that is one, or when the
choice's prediction, on track to its end, has reached it at the enemy's
start of turn. The SPRT runs again on the fixed head.

**SPRT** (head 9f37be7, CodinGame's limits: time scale 1, no tolerance,
against `ice-and-fire-v003`): accepted after 364 pairs, 57.5% of 729
games, +51.9 Elo [+27.1, +77.3], LLR 2.97. Smoke test: 200 of 200 against
`random`. No timeout, crash or invalid answer; answers after the first:
median 2.5 ms, 99.9% 25.6 ms, slowest 27.5 ms.

**League** of v004 (800 pairs each, CodinGame's limits): 55.9% against
v003, 83.7% against v002, 96.9% against v001, 96.5% against `cutter`,
99.8% against `guard` and `chain`, 99.7% against `bronze`; rated +2019.4
against v003's +1979.4 (random at 0).

**Decision:** promoted, `ice-and-fire-v004`.
