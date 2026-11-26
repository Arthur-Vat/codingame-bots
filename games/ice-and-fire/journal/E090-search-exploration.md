---
id: E090
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: A search that also starts from spending splits (towers where they most lower the enemy's best cut, then trainings), that can remove or swap an action when nothing more improves its plan, and that re-climbs from its best plan with actions removed, finds mixes of purchases the greedy climb misses, and beats ice-and-fire-v032.
change: Three settings of the search (`SPLITS`, `SWAPS`, `PERTURB`, `search/explore.rs`), each at its v032 value v032.
release: none
sprt: not run: no variant above 0 in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The owner (2026-11-26) found v032's towers poorly placed and asked how the
search weighs its spending: with 50 gold, does it compare 3 towers, 2
towers and 2 units, 1 tower and 3 units, 5 units, and saving? It does not:
it climbs, adding one action at a time, the one that raises the plan's
score most against the opponent replies it models, and stops when none
does (saving is the default). A mix of purchases is found only if each step
pays alone; a purchase made early is never undone; and its restarts vary
only the first action.

A study of the towers first (`analysis` command `towers`, on a bank of the
towers built written by `arena/examples/tower_bank.rs`), on the 6,150
towers v032 built up to game turn 13 in its 1,950 CodinGame games and
zasmu's 672:

- 19% of v032's towers stand 3 or more steps from any enemy cell or unit
  (zasmu's 11%), but 79% of those still lower the cells the enemy can take
  (its chains of trainings reach that far), and they save 4.1 cells of the
  enemy's best cut on average. 55% of all v032's towers stand on a cell a
  unit left (E070, E076), 40% guard (E026).
- Against every other cell where the tower could have stood, v032's is the
  best against the enemy's best cut 79% of the time (zasmu's 76%); 7% of
  its towers lower neither the threat nor the cut (zasmu's 8%).
- v032 exposes more: the enemy's best cut takes 5.1 cells before its tower
  (zasmu 3.9), and the towers come later (32% at game turns 5 to 9, zasmu's
  52%; the opening, to game turn 6, builds none).

The owner chose to change how the search explores (options 1 to 3 of six
proposed).

## The change

`bots/plan/src/search/explore.rs`, used by `search.rs` for our own search
only (not the opponent's model):

- `SPLITS` (0 in v032): after the first climb has filled the memory of
  replies, the spending splits of the turn: for each number of towers up to
  `SPLIT_TOWERS` (3), each tower on the cell offered that leaves the
  enemy's best cut smallest, then each number of trainings up to
  `SPLIT_TRAININGS` (5), the trainings in the order of their gain alone.
  The `SPLITS` best against the memory start the next climbs, which add
  moves and more.
- `SWAPS` (off in v032): when no action improves a climb's plan, the best of
  removing one of its actions or replacing one of its purchases by another
  purchase, if it scores higher, and the climb goes on from there, at most
  `MAX_SWAPS` (3) times.
- `PERTURB` (0 in v032): once the climbs repeat, the next climbs start from
  the best plan so far without its purchases, then without each of its
  first commands, `PERTURB` starts in all.

## Before the screening

The `codingame-judge` run of each setting on 921 of v032's CodinGame plies
from game turn 7 (decided positions left out), 600 tries a search: the
share of plies where the search finds a plan it scores above the one v032
played, and how many tries a millisecond it makes on the bench bank:

| Setting | Above v032's play | Tries a ms |
|---|---|---|
| v032 | 9.4% | 21.6 |
| SPLITS 4 | 22.7% | 21.0 |
| SWAPS | 22.3% | 38.0 |
| PERTURB 4 | 13.6% | 19.7 |
| all three | 23.5% | 25.3 |

## Screening

Each variant against `ice-and-fire-v032`, 400 pairs, full games, no
faults on either side (run 45879356488):

| Variant | Settings | Elo |
|---|---|---|
| splits4 | `SPLITS` 4 | -22.6 [-44.5, -0.9] |
| splits8 | `SPLITS` 8 | -47.2 [-68.7, -26.1] |
| swaps | `SWAPS` | -0.0 [-21.3, +21.3] |
| perturb4 | `PERTURB` 4 | -5.2 [-24.8, +14.3] |
| splits4-swaps | `SPLITS` 4, `SWAPS` | -44.5 [-66.1, -23.3] |
| all | `SPLITS` 4, `SWAPS`, `PERTURB` 4 | -30.5 [-51.0, -10.1] |

The spending splits lose, the more of them the more; the swaps and the
perturbations are level with v032. The plans the splits find score higher
by the search's own measure (22.7% of plies above v032's play, against
9.4%) yet lose games: they are judged against the replies the first climb
found, and they take the climbs that would have found others, so a plan
fitted to a few replies wins the choice and meets the replies left out.
Finding plans the evaluation rates higher does not, alone, make the bot
stronger.

## Result

Dropped (the owner, 2026-11-26), for E091: a repair of the latest plan
against its worst reply, with a fresh reply search after each repair.
