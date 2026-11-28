---
id: E111
date: 2026-11-28
pull_request: "#410"
parent: ice-and-fire-v034
hypothesis: zasmu builds 0.41 mines a turn at game turns 9 to 14 against our 0.15; valuing a safe mine more in those turns, and the free mine spots a side holds a little, makes us build and take more and beats ice-and-fire-v034.
change: Two switches in the evaluation (both sides): a safe mine counts `MID_MINE_BONUS` more from game turn 9 to 14, and each free mine spot on a side's active cells that the enemy cannot take counts `SPOT_BONUS` until game turn 14.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The game-phase comparison with zasmu (2026-11-28): at game turns 9 to 14 it
builds 0.41 mines a turn, we 0.15; by turn 16 it holds 3.0 against our 1.5.
The owner chose mines at turns 9 to 14 and "a very small nudge to grab mine
slots".

## Change

On the branch `claude/ice-and-fire-e111-mines` (from `main`), in
`eval::mines`: `MID_MINE_BONUS` (0) added to E058's early bonus of a safe mine
from `MID_MINE_FROM` (9) to `MID_MINE_TO` (14); `SPOT_BONUS` (0) for each free
mine spot on a side's active cells that is not exposed, until `SPOT_UNTIL`
(14). Off, the evaluation is unchanged.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46149023325](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46149023325)):

| Variant | `MID_MINE_BONUS` | `SPOT_BONUS` | Elo over v034 [95%] |
| --- | ---: | ---: | ---: |
| `control` | 0 | 0 | +3.7 [−2.1, +9.4] |
| `m4` | 4 | 0 | −3.5 [−13.0, +6.0] |
| `m8` | 8 | 0 | −0.4 [−10.1, +9.3] |
| `spot1` | 0 | 1 | +4.1 [−5.4, +13.6] |
| `m4spot1` | 4 | 1 | −2.4 [−12.3, +7.6] |

Commits: `control` `655f509`, `m4` `ac262e6`, `m8` `148b21f`, `spot1`
`859c7cc`, `m4spot1` `fd800d0`. One timeout of the control; no other fault.

## What was learned

Dropped: all level. In 30 local pairs of `m8` against v034 at CodinGame's
speed, `m8` built 3.17 mines a game against v034's 3.37: the bonus did not
make us build more mines. The reason, measured on 800 of our turns at game
turns 9 to 14 with 15 gold and a free safe spot where we built none: a mine
added to the plan our search chose lowered its score (median −11 where the
gold allowed it): the shared evaluation counts gold once as material and
again in its pressure term (how far a side is from paying its chain to the
enemy HQ), so 15 gold spent costs about 30 while a mine's 4 of income brings
about 16. Our evaluation saves for a strike, which a mine's bonus does not
outweigh. That led to the invest strategy (E113).

The code stays on the branch (commit `655f5096`, the switches off).
