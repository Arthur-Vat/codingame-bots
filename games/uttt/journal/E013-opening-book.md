---
id: E013
date: 2026-10-08
pull_request: "#31"
parent: uttt-v008
hypothesis: An opening book of deeper searches for the bot's first moves adds strength, since only its first move gets the first turn's long search, and only its second move inherits much of it.
change: None released. What such a book could bring at most was measured first, with `uttt-trainer head-start`: v008's search against itself, one side searching 20 times longer on its first 4 moves.
release: none
sprt: not run, measurement only
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

The owner asked whether an opening book, or a structure built during the
long first turn, could add strength. The bot already keeps its search
tree from turn to turn. Measured on 16 games of v008's search
(600,000 iterations on the first move, then 60,000 per move): on its
second move, the bot still holds 16% of its first search's visits when
it moved first and 8% when it moved second, 59% and 44% of that move's
tree. From the third move on, about 1% is left. So the first turn's long
search mostly helps the first two moves, and a book would help the
next few.

The most a book could bring: in each pair of games from the empty board,
the same side searched 20 times longer on its first 4 moves (1,200,000
iterations each, the first move included), then as usual; the other side
had the bot's usual budgets throughout. 60,000 iterations per move is
about what the bot runs in 90 ms on this sandbox in the early game, and
600,000 on the first move about its 900 ms. Locally, 2 threads:

| Seed | Pairs | Wins, draws, losses for the head start | Elo |
| --- | --- | --- | --- |
| 1 | 50 | 37, 23, 40 | -10.4 [-63.3, +42.0] |
| 2 | 55 | 46, 33, 31 | +47.7 [+1.5, +95.6] |
| 3 | 55 | 41, 33, 36 | +15.8 [-34.1, +66.3] |
| 4 | 55 | 36, 27, 47 | -34.9 [-83.8, +12.7] |
| All | 215 | 160, 116, 154 | +4.8 [-19.9, +29.6] |

Twenty times more search on the first four moves is worth about +5 Elo,
at most about +30. A book could hold deeper searches than that, but the
head start also left a larger tree for the fifth move, which a book
would not. No book is worth its file space and build for now. The
command stays in the trainer, so the measure can be taken again if the
bot changes how it uses the first moves, for example with a value
network.
