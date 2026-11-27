---
id: E096
date: 2026-11-27
pull_request: "#397"
parent: ice-and-fire-v033
hypothesis: A better approach to the centre (the search a turn earlier, units that keep their own room instead of crowding, a unit or two kept at home, the far free land valued) lets our units take the free land far from the front as the top players do while holding the middle, and beats ice-and-fire-v033.
change: Three terms in the search's evaluation, each net of the enemy's and each 0 in v033 (`eval::far`, `eval::home`, `eval::room`), and the search's handover from the opening at game turn 6 rather than 7 (`search::HANDOVER_TURN`); one variant each.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The owner annotated 14 of v033's CodinGame losses (2026-11-27,
https://claude.ai/artifact/DjRr4oRMMBTXFtyrsrEGtP): too crowded in the
centre in 7, a weak economy in 6, cuts we did not see coming in 6. He asked
how we take our free land: near the front to consolidate, far from it for
the economy, and whether our units walk back over the same paths.

Measured on v032's 2,350 CodinGame test games against the top 10 (v033 plays
the same middle game; tool `review/examples/midgame.rs` on the session
branch `claude/pensive-darwin-879zmo`, not merged; we are red, the opponents
blue):

- **Crowding:** at game turns 8 to 12 our units have 0.67 of their own units
  next to them, zasmu's 0.33, karliso's 0.39, Pduhard-'s 0.37; 0.79 free
  cells next to each unit against 1.02 and 1.04. Later our units stand
  still: 49% do not move at game turns 13 to 18, against 34% and 30%.
- **Near the front** (4 steps or fewer), we take free cells faster than the
  top players (2.6 a turn at game turns 8 to 12 against 2.2), and the cells
  the enemy could take or cut off with 30 or 50 gold are level with zasmu's.
- **Far from the front**, we take free cells later: 0.72 a turn at game
  turns 8 to 12 against 0.96 for zasmu and 1.25 for karliso; and we have
  fewer mines (0.8 against 1.5 and 1.1).
- **Walking back** is not our problem: the top players move inside their
  land and onto cells they visited more than we do (30 to 35% of their
  moves at game turns 13 to 18 against our 17%); our units stand still
  instead.

The owner then asked whether this comes from our rush to the centre. Turn
by turn (tool `rush.rs`, rings from the line even between the HQs, the
middle being rings 0 and 1):

- The top players reach the middle at game turn 6 with about one training
  (0.9 a turn in the middle at turn 6); we reach it at turn 7, the search's
  first turn, with 1.5 trainings, and 86% of our moves at turn 7 head for
  the middle against 67% for zasmu.
- At game turn 8 we keep 0.84 units at home (ring 4 or more) against 1.47
  for zasmu and 1.12 for karliso, and hold fewer middle cells (4.5 against
  5.0 to 5.7).
- Game by game, the more of our units in the middle at turn 8, the fewer
  far free cells we take at game turns 8 to 12: 8.3 with none, 6.3 with
  one, 4.1 with two, 3.1 with three, 2.6 with four or more (28 to 905
  games each); zasmu's stays at 4.2 to 4.9 from two units up, on the same
  maps.

## Change

All in `bots/plan/src/eval.rs` and `search.rs`, each 0 or as v033 in the
control:

- `far` (`FAR`, `FAR_DIST` 4, `FAR_UNTIL` 14): each active cell more than 4
  steps from the enemy's land counts `FAR` more.
- `home` (`HOME`, `HOME_CAP` 1, `HOME_UNTIL` 12): a unit kept on such a cell
  with a neutral cell within 2 steps counts `HOME`.
- `room` (`ROOM`, `ROOM_RANGE` 4, `ROOM_CAP` 4, `ROOM_UNTIL` 16): each
  neutral cell within 4 steps of our units belongs to its nearest unit, and
  each unit's cells, up to 4, count `ROOM` each. Units that crowd share the
  same cells and count little; units that head for the same cells split
  them (the owner's collaborative snake: units should not walk the same
  paths), so the term looks several turns ahead rather than one.
- `HANDOVER_TURN` 6: the search plays from game turn 6. E067 found turn 6
  weaker with v026's search (−26.8); the search has changed since, and the
  top players train into the middle at that turn.

Speed (`ice-and-fire-bot-plan speed`, 2,000 tries on 300 middle-game
positions of v033's games, the faster of 2 runs): off 17.9 tries a ms, far
17.9, home 17.4, room 16.1, all three 14.9. With a term on, four tests that
compare the evaluation of two positions with different units would count
it; they are to be adjusted if such a variant becomes the candidate.

## Screening

800 seat-swapped pairs per variant against `ice-and-fire-v033`, whole
games at CodinGame's limits, 2 shards
([run 46012271222](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46012271222)):

| Variant | Commit | Change from v033 | Elo over v033 [95%] | Score |
| --- | --- | --- | ---: | ---: |
| `off` | `9981967` | none (control) | +0.9 [−10.1, +11.8] | 50.1% |
| `t6` | `c4aa443` | `HANDOVER_TURN` 6 | −6.1 [−20.7, +8.5] | 49.1% |
| `far4` | `02e08ce` | `FAR` 4 | −16.1 [−31.7, −0.5] | 47.7% |
| `far8` | `3449889` | `FAR` 8 | −64.1 [−80.4, −48.2] | 40.9% |
| `home8` | `5427e63` | `HOME` 8 | −3.5 [−17.4, +10.4] | 49.5% |
| `room1` | `992f490` | `ROOM` 1 | +1.7 [−13.2, +16.6] | 50.2% |
| `room2` | `419e5f3` | `ROOM` 2 | −25.7 [−41.2, −10.2] | 46.3% |

One timeout of far4 and one of v033 against far8 in 11,200 games; no other
fault.

No variant beats v033. The far-land term loses, the more the heavier
(far8 −64.1): paying for land far from the enemy pulls units and
trainings away from the middle, where the games are decided. The room term
is level at 1 and loses at 2, with its 10% of speed. Handing over at game
turn 6 and keeping a unit at home are level. So the gap the measures show
(far land taken later, crowded units) is not closed by valuing far land or
room in the search's evaluation, as the cell-value terms of E090 to E094
were not.

## What was learned

Dropped. The gaps the measures show are real (crowded units at game
turns 8 to 12, far land taken later, fewer units at home, a burst into the
middle at turn 7), but no term in the search's evaluation closes them:
valuing far land loses, and the more the heavier; giving each unit its own
room is level at a weight of 1 and loses at 2; keeping a unit at home and
the handover at game turn 6 are level. The owner then asked for a
structural change instead (2026-11-27): a layer before the search that
gives the units behind the front their own free cells and paths (E097).

The code stays reachable on the branch `claude/ice-and-fire-e096-centre-approach`
(commits `9981967` to `419e5f3`: `eval::far`, `eval::home` and
`eval::room` with their test, and the variants), and the measuring tools
(`review/examples/midgame.rs` and `rush.rs`) on `claude/pensive-darwin-879zmo`.
