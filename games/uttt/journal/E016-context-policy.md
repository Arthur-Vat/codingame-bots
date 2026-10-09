---
id: E016
date: 2026-10-09
pull_request: "#37"
parent: uttt-v010
hypothesis: The bytes v010 leaves free (about 24 KB of 100 KB) hold a larger move policy whose better predictions make playouts, and the order of children, better.
change: None released. Larger move models were fitted on 400,000 games of v010's own search and compared offline; the cheapest that predicts better (E015's weights plus a weight for the destination board's pattern and role) was built into a bot and screened against v010.
release: none
sprt: not run, local screening only
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

The owner's request on 2026-10-09: improve the best approach so far,
E015's pattern policy, by using the whole file.

## Data and models

Self-play on GitHub (run 37895007043): 20 jobs of 20,000 games of v010's
search (pattern playouts, children ordered by the policy), 10,000
iterations per move, 6 random opening moves. That is 400,000 games and
16.4 million searched positions, 50 times E015's data. The models were
fitted by `uttt-trainer fit-patterns --models all` in fit-only runs
37901943775 and 37903237938, holding every 20th game out (821,775
positions):

| Model | Weights | Held-out cross-entropy | P(search's favourite) | Packed digits |
| --- | --- | --- | --- | --- |
| Uniform | 0 | 1.9026 | 0.167 | |
| 32 classes (E011) | 32 | 1.8062 | 0.210 | |
| patterns (E015, v010's features) | 26,275 | 1.7331 | 0.238 | 17,630 |
| 7 kinds of destination | 36,785 | 1.7322 | 0.238 | 21,201 |
| patterns, two phases (before and from move 30) | 52,550 | 1.7338 | 0.236 | 29,202 |
| patterns + destination's pattern | 27,858 | 1.7008 | 0.254 | 18,710 |
| patterns + destination's pattern and role | 32,604 | 1.6953 | 0.256 | 21,186 |
| the same, two phases | 58,879 | 1.6959 | 0.255 | 32,293 |
| rich: 7 kinds + destination's pattern + own board's role | 38,380 | 1.6934 | 0.260 | 20,848 |
| large: 7 kinds + destination's pattern and role + own board's role by pattern | 64,134 | 1.6891 | 0.261 | 31,726 |

- v010's features are saturated: on 8,000 games their held-out
  cross-entropy was 1.7332, on 400,000 it is 1.7331. More data, more
  kinds of destination or a second phase add nothing.
- What helps is new information about the board the opponent is sent to:
  its pattern, seen by the opponent, and whether it would decide the
  game. The same gain appeared on the 8,000 local games.
- Packing the weights (`cg_core::packed`, a Huffman code per table and
  kind, in base64) takes 3 to 4 bits per weight instead of 6: even the
  large model would fit in the file.

## The cost in playouts

A playout draws each of its first 16 moves by weighing every free cell
of the board it plays in. E015's weight is one lookup; the destination's
pattern and role need the destination board's state after the move.
Computed directly, a draw took 2 to 3 times as long as E015's. With a
per-board cache along the playout and a branch-free loop (archived in
#38), context playouts ran at 58%
of E015's speed, and the bot searched about 20% fewer iterations per
move (15,200 against 19,000 at 18 ms; 62,000 against 68,000 at 90 ms).

## Screening against v010

Local, 2-core sandbox, 4-ply openings; "1 game at a time" removes the
two games' competition for the processor, which hurt the larger bot
more.

| Bot | Setting | Pairs | Elo against v010 |
| --- | --- | --- | --- |
| v010's features refit on the 400,000 games | 20 ms | 300 | -0.6 [-23.2, +22.1] |
| large, all three tables in playouts (8,000 games) | 5,000 iterations each | 100 | +27.9 [-16.3, +72.9] |
| large, all three tables in playouts (8,000 games) | 20 ms | 100 to 200 | -85 to -110 |
| large, priors only, E015's playouts (8,000 games) | 20 ms | 200 | -23.5 [-53.3, +6.0] |
| destination's pattern and role (8,000 games) | 5,000 iterations each | 100 | +15.6 [-30.7, +62.5] |
| the same | 15,000 iterations each | 100 | +22.6 [-19.6, +65.5] |
| the same, 15,000 against v010's 19,000 | iterations | 100 | -8.7 [-51.4, +33.8] |
| the same | 20 ms, 1 game at a time | 100 | -38.4 [-79.8, +1.9] |
| destination's pattern and role (400,000 games) | 20 ms, 1 game at a time | 150 | -49.0 [-84.4, -14.5] |
| the same | 100 ms, 1 game at a time | 80 | +19.6 [-24.7, +64.5] |
| the same | 100 ms, 1 game at a time | 100 | -26.1 [-65.5, +12.6] |

At equal iterations the better policy gains about 15 to 25 Elo; the
iterations it costs take that back, and more at 20 ms. At CodinGame's
limits the two runs average about -6 Elo over 180 pairs: no candidate
worth an SPRT. Iterations, not bytes, limit this kind of model.

## What is kept

- In `main` through #36: `selfplay --patterns` (self-play with the
  bot's own search), `fit-patterns --models` (the comparison above), and
  the Train workflow's `patterns` stage with fit-only runs on earlier
  data.
- Archived in #38, closed without merging so that bots do not carry
  unused code (commit 3c22deb, `git fetch origin pull/38/head`): the
  context policy with its cache and tests, checked move by move against
  the trainer's definitions, and `cg_core::packed`, for a later model
  that earns its bytes.
