---
id: E075
date: 2026-11-20
pull_request: "#NN"
parent: ice-and-fire-v030
hypothesis: Towers built only when the enemy's next turn could take what they guard, the gold kept for units and mines until then, beat ice-and-fire-v030.
change: The tower wall of the evaluation (E068) counts only the guarded frontier cells the enemy could take at its next turn were they not guarded, and switches screen two related changes (towers that guard nothing so counted lose their static value; the HQ guard of E026 against the next turn only).
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
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

Local, full games with `ice-and-fire-arena match` against v030's release
compiled with `rustc -C opt-level=3`, the variants built with
`cargo build --release`, time scale 0.430 and 6 ms of tolerance from
`scripts/calibrate.sh` at 3 games at a time on the session's 4 CPUs, seed
91:

| Variant | Switches | Elo over v030 [95%] | Pairs | Timeouts (variant / v030) |
| --- | --- | --- | ---: | ---: |
| base | none (control) | pending | | |
| wt | `WALL_THREAT` | pending | | |
| wti | `WALL_THREAT`, `IDLE_TOWER = 10` | pending | | |
| idle | `IDLE_TOWER = 10` | pending | | |
| hq0 | `HQ_GUARD_INCOMES = 0` | pending | | |
