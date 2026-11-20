---
id: E075
date: 2026-11-20
pull_request: "#376"
parent: ice-and-fire-v030
hypothesis: Towers built only when the enemy's next turn could take what they guard, the gold kept for units and mines until then, beat ice-and-fire-v030.
change: None released. Screening only: the tower wall of the evaluation (E068) counting only the guarded frontier cells the enemy could take at its next turn were they not guarded; towers that guard nothing so threatened losing their static value; the HQ guard of E026 against the next turn only.
release: none
sprt: not run: level or weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

The owner chose this experiment on 2026-11-20 (form; E074, the opening's
target kept by the search, deferred after it). The evidence, from the
`places towers` measurement (branch `claude/jolly-keller-qol8es`, 3ba2a63)
on about 166 full games of v030 against v029:

- v030 builds 7.3 to 8.4 towers a game, its first in the middle game and
  its first mine at median game turn 9 to 10;
- 52 to 56% of its towers spare nothing at the enemy's best cut of its next
  turn (`score::exposure` with and without the tower), and about 6 in 10
  spare at most a cell while our gold at the next turn would have paid for
  them then (about 70 gold a game, at game turns 8 to 15);
- 20 to 27% spare 2 to 9 cells, 12 to 19% more.

This is an upper bound on waste: `exposure` looks one enemy turn ahead,
at chains of up to 5 trainings.

Why the search builds them: a tower costs 15 gold, and the evaluation gives
it back at once, whatever the enemy can do. The shared evaluation counts an
active tower 10, and since E068 (`eval::WALL`) each frontier cell it guards
counts 8 more, so a tower beside one frontier cell already scores +3, and
beside two +11. The actions offer a tower only where it lowers the cells the
enemy can take at its next turn (`actions::guards`, E026) or guards the HQ
within two of the enemy's incomes (`HQ_GUARD`), or with a step (E070); but
once offered, the static value carries it even where the enemy's best cut is
elsewhere.

## Change

Switches, all off in v030:

- `eval::WALL_THREAT`: the tower wall counts `WALL` (8) only for the guarded
  frontier cells that the enemy could take at its next turn were they not
  guarded: its chains of trainings for its gold plus income, its units of
  level 2 or more stepping first, as `eval::reach`, but past our towers
  (`ice_and_fire_tactics::chain::reach_past_towers`, new: levels 1 and 2
  enter what level 3 may, except the cells of units they do not beat).
- `eval::IDLE_TOWER`: each active tower whose cell and neighbours hold none
  of those cells counts this much less (10 cancels the shared evaluation's
  static value).
- `actions::HQ_GUARD_INCOMES`: the enemy's incomes beyond its gold within
  which a tower that raises its chain to our HQ is offered (2 in v030; 0
  for its next turn only).

## Screening

Full games against v030 on GitHub (`screen.yml`, run 44757582435), 800
pairs a variant over 2 calibrated runners, the workflow's own seed; and
locally (`ice-and-fire-arena match` against v030's release compiled with
`rustc -C opt-level=3`, the variants built with `cargo build --release`,
time scale 0.430 and 6 ms of tolerance from `scripts/calibrate.sh` at 3
games at a time on the session's 4 CPUs, seed 91, 400 pairs):

| Variant | Switches | GitHub, 800 pairs [95%] | Local, 400 pairs [95%] |
| --- | --- | --- | --- |
| base | none (control) | – | +11.3 [−8.0, +30.7] |
| wt | `WALL_THREAT` | −0.4 [−12.8, +11.9] | −13.9 [−31.8, +3.9] |
| wti | `WALL_THREAT`, `IDLE_TOWER = 10` | −14.3 [−26.3, −2.4] | −2.6 [−21.5, +16.3] |
| idle | `IDLE_TOWER = 10` | −10.0 [−22.2, +2.2] | −8.7 [−25.7, +8.3] |
| hq0 | `HQ_GUARD_INCOMES = 0` | −1.7 [−11.1, +7.6] | +6.1 [−11.0, +23.2] |

No fault on GitHub; locally 1 to 3 timeouts a side for the variants. The
local control ran while the session analysed CodinGame games alongside
(14 and 16 timeouts), so the local column is noisy by about ±17. The new
reach did not slow the bot: its answer times match v030's.

## What was learned

Making towers cheaper in the evaluation does not pay. Taking their static
value away from towers that guard nothing the enemy could take loses about
10; counting the wall only where a cell is threatened, or guarding the HQ
only against the next turn, is level. The towers that spare nothing at the
enemy's best next cut earn their cost some other way, likely over more
than one enemy turn. The owner then asked (2026-11-20) for an analysis of
zasmu's games before any new change; it led to E076, whose candidate
builds towers where units fall back.

Dropped. The code (`tactics::chain::reach_past_towers`, the switches
`eval::WALL_THREAT`, `eval::IDLE_TOWER`, `actions::HQ_GUARD_INCOMES` and
their tests) stays reachable at commit 16f0c7a of the branch
`claude/ice-and-fire-e075-towers-in-time`.
