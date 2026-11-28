---
id: E108
date: 2026-11-28
pull_request: "#408"
parent: ice-and-fire-v034
hypothesis: zasmu trains at game turns 5 and 6 and holds one more unit from turn 6, and builds its first mines at turn 7; an opening that trains whenever it can at turns 5 and 6 and builds no mine before turn 7 keeps up with it and beats ice-and-fire-v034.
change: Switches in the opening (a forced training at game turns 5 and 6 when the turn bought none and the gold pays) and in the actions offered (no mine before a game turn), and the opening's payback horizon.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

On the 1,038 games against zasmu, every ply: zasmu trains 0.90 and 1.00
units at game turns 5 and 6, ours 0.12 and 0.82, and holds 3.9 units at turn
6 against our 3.1; our first mine comes at turn 6 (0.14 a turn), zasmu's at
turn 7. The owner chose (2026-11-28) to test the opening's economy.

## Change

On the branch `claude/ice-and-fire-e108-opening-economy` (from `main`). The
opening (`opening/src/econ.rs`) buys a unit when it pays back within
`OPENING_HORIZON` (18, E056) and buys no mine (`OPENING_SAFE_SPOT` is
`u8::MAX`): the mines before game turn 7 are the search's, once
`middle::reached`. New: `Params.train_from` / `train_to` (a forced training in
`econ::forced_training`, on a free cell next to our land, when the turn
bought none and the gold pays; the bot's `OPENING_TRAIN_TURNS`), and
`actions::MINE_FROM` (no mine offered before that game turn, for both sides'
climbs). With the switches off the answers are those of `main` (`judge`, 600
tries, 2,031 positions: byte-identical; 12 whole games at a fixed number of
iterations: identical answers). The bundle: 69,773 characters.

Against v034 in self-play the opening already trains at game turns 5 and 6
(3.5 units at turn 6 for both bots in local games): the gap against zasmu
comes from the game zasmu plays, not from an opening rule.

## Screening

Against `ice-and-fire-v034`, 1,600 pairs each, 4 shards
([run 46144246115](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46144246115)):

| Variant | Change | Elo over v034 [95%] |
| --- | --- | ---: |
| `control` | none | +1.1 [−4.8, +6.9] |
| `train56` | a training at game turns 5 and 6 | −12.6 [−21.4, −3.8] |
| `mine7` | no mine before game turn 7 | +1.7 [−4.3, +7.7] |
| `train56mine7` | both | −8.7 [−18.0, +0.6] |

A horizon of 21 lost in 40 local pairs (−43.7 [−101, +11]) and had lost in
E056: it was left out of the screening. Commits: `control` `f3449d1`,
`train56` `f612ee6`, `mine7` `ccddd34`, `train56mine7` `85cc035`, `horizon21`
`be5f2dc`. One timeout of the control; no other fault.

## What was learned

Dropped: the forced trainings cost a little, the later mines change nothing.
The code stays on the branch (commit `f2fb7d5c`, the switches off).
