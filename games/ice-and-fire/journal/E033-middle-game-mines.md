---
id: E033
date: 2026-11-12
pull_request: pending
parent: ice-and-fire-v014
hypothesis: The top 10 of CodinGame build a mine in about a fifth of their games at every turn from turn 9 and v014 almost never does, because its evaluation scores a mine below the gold it costs; counting a mine above its income, where the enemy cannot take it at once, makes the search build mines in the middle game and beats v014.
change: The plan bot's evaluation adds a bonus for each of a side's mines on an active cell that the enemy cannot take at its next turn.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Chosen by the owner on 2026-11-12 ("Mines experiment (Recommended)"), the
first of the experiments proposed by the report on 2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
"What to try in our bot").

## Why

From turn 9 to turn 15 the top 10 build a mine in about 20% of their games
at every turn, the rest of Legend in 6% to 9%, and v014 in 0% to 2% against
the top 10; under 1% of our spending goes to the economy in the middle
game, against 14% of theirs. Our search scores their turns with a mine 35
below its own choice, against 22 for their plain turns. The report warns
that the top 10 are already ahead when their mines show, so the mines may
follow their lead as much as make it: only an experiment says.

v014's search cannot gain by a mine. The shared evaluation
(`ice_and_fire_tactics::eval`) counts gold at 1 and income at 3, so a mine
is 15 gold less and 4 of income more, 12 − 15 = −3, before the pressure
and threatened-cell terms, which both count the gold spent against it too.
A climb adds only actions that raise the plan's score. In the games
below v014 builds 2.8 to 3.0 mines a game, nearly all in the opening, by
its rules (a mine in 36% of games at its turn 6, 74% at turn 7, 28% at
turn 8), then in 1% to 3% of games a turn from its turn 10 to 14 and 5% to
10% later.

## The change

`bots/plan/src/eval.rs`: each mine of a side on one of its active cells
counts `MINE` more, in both sides' scores, unless the enemy can take its
cell within `MINE_SAFE_TURNS` of its turns: with 1, the mine's cell is not
among the side's threatened cells of E023 (those the enemy's chains of
trainings reach for its gold plus its income), which the evaluation
already finds; with 2, nor among those reached for its gold plus two
incomes, searched only when a side has a mine that still counts. A mine
pays its 15 gold back in four turns, and one the enemy takes is destroyed.

A test checks the term on a quiet position (a mine built far from the
enemy adds `MINE` to its side and takes as much from the other) and on a
corridor (a mine on a cell the enemy can take next turn adds nothing).
