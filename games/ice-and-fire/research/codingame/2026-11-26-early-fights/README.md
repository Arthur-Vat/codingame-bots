# v032's lost fights before game turn 12

The owner (2026-11-26): we still lose too many fights in the early and
middle game; we sometimes expand too much and lose our units, where we
would play safer and keep our territory after the opponent's play. This
study measures it in v032's CodinGame games (E083).

## Data and method

v032's 1,950 CodinGame games on the branch `claude/codingame-games`,
converted to records by `ice-and-fire-review codingame` (all 1,950 agree
with our engine) and replayed by `arena/examples/plies.rs` (the board at
the start of every ply up to game turn 13). `fights.py` reads, after each
of our plies before game turn 12, what the enemy's next ply took: our
cells it captured, our active cells it cut off from our HQ, the cells we
had just taken, our units; and each large loss (3 or more cells in one
enemy ply).

```sh
ice-and-fire-review codingame --out records data/ice-and-fire/logs/*.json
target/release/examples/plies 13 records/*.json > plies.jsonl
python3 fights.py plies.jsonl data/ice-and-fire/index.jsonl records
```

## Results

| Per game, to game turn 12 | Won (1,248) | Lost (679) |
| --- | --- | --- |
| Cells captured by the enemy | 2.8 | 4.6 |
| Cells cut off from our HQ | 1.7 | 4.7 |
| Cells taken in our ply and lost at once | 0.6 | 0.4 |
| Units lost | 2.29 | 3.17 |

- The fights start at game turn 8; from then the cells lost per enemy ply
  grow from 0.3 to 2.3 at turn 12, while v032 keeps 0 to 7 gold (median)
  and the enemy 18 to 42.
- 922 games of the 1,950 have a large loss before turn 12, 1,410 in all
  (46, 173, 295, 446 and 450 at game turns 8 to 12), 48% of them in our
  lost games. In the median one, v032 has 32 gold at the start of its ply
  before (at least 15, a tower, in 98% of them), spends it down to 5 (15 or
  more left in 9%), and the enemy, with 38, trains 2 units (0.72 of level 2
  or more a large loss, on average) and takes 4 cells, 2 of them cut off.
  The cells lost were taken a turn before (median).
- v032 holds more land than its opponents at game turn 12 (41 cells to 35,
  medians): it spreads wide and thin, spends all its gold on it, and keeps
  nothing against the enemy's reply; the opponent keeps a reserve and cuts.

Pictures, action by action, of six of the worst cases and three typical
ones against the top players:
https://claude.ai/artifact/6bqSJZVKwnJtQ9oA7ZDna2.
