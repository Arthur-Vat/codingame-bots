# A role for every unit: a planner over several turns (2026-11-27)

The owner, on 2026-11-27, after E096 and E097 (the units behind the front
sent to the far free land, by the evaluation and by a fixed rule, neither
stronger than `ice-and-fire-v033`):

> Could we design a full planner for all units, assigning them a role?

and, before it:

> Ideally I think the movement of units could be nearly eliminated from the
> search. At least units very far from the fight should be eliminated. [...]
> I assume we could have long term planning.

This note is the design, for the owner's review before any code. It says
what the planner is for, what earlier work teaches about how such a layer
must sit beside the search, the roles and how units get them, how the plan
looks several turns ahead, what it costs in time, and the steps, each with a
check that can stop the work.

## What it is for

The owner's notes on 14 of v033's CodinGame losses
(https://claude.ai/artifact/DjRr4oRMMBTXFtyrsrEGtP) and E096's measures on
v032's 2,350 test games against the top 10:

- **Crowding in the centre** (7 of 14 losses): at game turns 8 to 12 our
  units have 0.67 of their own units next to them, zasmu's 0.33 and
  karliso's 0.39; later, 49% of our units stand still at game turns 13 to 18
  against 30 to 34%. The top players move their units around inside their
  land more than we do (30 to 35% of their moves end on a cell the unit had
  visited, against our 17%): their units are repositioned, ours are parked.
- **Cuts we did not see coming** (6 of 14): the cell-value study found
  that 49 to 65% of the cells lost in a ply are cut off rather than taken,
  and that a cell's price with cuts against the enemy's gold predicts its
  loss well (AUC 0.85 to 0.93,
  [cell values](2026-11-27-cell-values.md)).
- **A weaker economy and too few mines** (6 of 14): mines at game turns 8
  to 12, us 0.8, zasmu 1.5, karliso 1.1.
- **zasmu's formation** (the owner): one unit and two next to it, then the
  three take land and a tower covers them exactly.

So the planner's first job is the front: where the units stand, which
joints are guarded, which unit answers a cut, and how the units move
together. Far land comes last: E096 and E097 show that it does not decide
games.

## What earlier work teaches

| Work | What it did | Result | Lesson for the planner |
| --- | --- | --- | --- |
| E095 (v033) | The opening's order of plans: near the whole frontier | **+49.8**, accepted | A plan-level change, made before the search plays, can win |
| E077 | A strategic turn (one frontline unit per objective, trainings at its tip) offered to the search | Rejected by the search in 99% of turns; forced, it lost units: 34% of its moves left a cell the enemy then took, cutting land off; back units with a safe step as the climbs' start: +14.8 on one seed, parked | The search must stay the judge; a role move must never uncover a cell the enemy can enter |
| E072 | A frontier assignment (Hungarian) replacing the opening | −21 to −121 | Planning against an enemy that does not move fails |
| E080 | The turn-1 planner (fronts, entrances, sub-goals), Python prototype | Far weaker than v020 in its duels | Fights decide; the plan needs the enemy's replies |
| E096 | Far land, home units, unit room as evaluation terms | Level or worse | Terms do not move the units the way we mean |
| E097 | Units behind the front moved by a fixed matching | Level or worse; offered, never taken | Far land is not the lever; removing those units saves 2 to 3 of 30+ options a step |

Four principles follow:

1. **The search stays the judge of every turn**, against the enemy's
   replies (E019, E027). The planner proposes; it never plays a move the
   search has not checked.
2. **No role move uncovers our land**: a step that leaves a cell the
   enemy can enter, and so cut what lies behind it, is never planned
   (E077's safe step, E070's step and guard).
3. **The plan looks several turns ahead, the search one**: the planner's
   value is in what one-turn search cannot see, which is where a unit should
   be in three turns, and which joint will be attacked when the enemy's gold
   allows.
4. **The plan knows the enemy**: its reach with its gold and income, its
   affordable cuts (the cell-value study's price with cuts), not a passive
   enemy (E072, E080).

## The roles

Every unit of ours has one role, kept between turns and changed only by
the rules below. A newly trained unit is trained for a role.

| Role | What the unit does | When it is needed |
| --- | --- | --- |
| **Guard** | Stands on a joint of our land (a cell whose loss cuts land off) that the enemy can reach within two turns, or next to it, and moves only to another guarding cell | Each joint whose loss would cut more than a few cells and whose price with cuts is within the enemy's gold plus two turns' income: the cuts we did not see coming |
| **Reserve** | Waits one or two steps behind the front where the enemy's best cut would enter, ready to take back the cut cell or to cut back | One or two per front, by the value of the enemy's best cuts there |
| **Pusher** | Takes contested cells at the front, in a formation: two or three units abreast, a tower planned on the cell behind them (zasmu's pattern) | The contested cells whose land is worth the most (E077's objectives, valued by the land they decide), while the gold pays the tower |
| **Sweeper** | Takes free cells behind the front, each on its own cell and path (E097's matching) | Only when no guard, reserve or pusher task needs the unit: E096 and E097 show that the far land is the least of the needs |

Units that are not needed by any task keep the sweeper role; a level-2 or
level-3 unit is never a sweeper. Towers and mines are not roles: the guard
and pusher tasks ask for towers, and the economy asks for mines (a later
step).

## How units get their roles

Each turn, before the search:

1. **Tasks**: list the guard, reserve, pusher and sweeper tasks with their
   cells and values: the land a guarded joint keeps, the land a pushed cell
   decides, the land a reserve can take back. The values use the existing
   machinery: the enemy's reach and affordable cuts (`eval::reach`,
   `eval::threatened`, the cell-value study's price with cuts) and E077's
   objectives and race (`strategy::objectives`, `strategy::race`).
2. **Matching**: units (and the trainings the gold pays) to tasks by
   arrival turn and value, with the Hungarian method already in
   `ice_and_fire_opening::assign`. A unit keeps its task unless another
   unit would reach it two turns sooner, or the task disappears.
3. **Paths over several turns**: each unit's path to its task over the
   next 3 to 5 turns, planned together so that no two units step on the
   same cell in the same turn and no unit walks another's path (a
   reservation table in space and time, the standard way to plan many
   agents on one grid: the owner's collaborative snake). A path step that
   would uncover a cell the enemy can enter is forbidden (principle 2).
4. **The plan for this turn**: every unit's first step, the trainings the
   tasks ask for, the towers the guard and pusher tasks ask for; and two or
   three variants of it (without the push, with the reserve one step
   forward, with the tower first).

## How the plan reaches the search

The search's climbs start from the plan and from each variant (E077's
"first climb's start", the only form that gained there) and then do what
they do now: add or change single actions while the score against the
enemy's replies improves. So the search can still fix a plan that exposes
a cell, play a lethal chain or answer a cut. The roles and paths are kept
for the next turn and checked against what the search played: a unit the
search moved off its path keeps its task and has its path planned again.

**Would it save time?** The planner itself runs once a turn: a matching of
at most 10 units to at most 40 tasks, and paths on 144 cells over 5 turns,
about a millisecond. Leaving unit moves out of the search entirely is not
proposed: E077 shows the search must be able to correct a role's move.
What saves the search's time is that its climbs start from a whole plan
instead of from nothing; the measure of that is the number of climbs a turn
and the plan's share of the commands finally played.

## Steps

Each step is one pull request with a check; the owner decides at each check.

1. **The unseen cuts and the crowding, measured (no bot change).** On
   v033's 159 ranking games and the owner's 14 annotated losses: for each
   cut that took 4 or more of our cells, whether its joint was guarded the
   turn before, whether a unit stood within two steps to take it back, and
   the enemy's gold against its price; for the crowded turns, which units
   had no task. Also zasmu's units sorted by the same role rules in its
   games against us. **Check**: most of our decisive cuts hit unguarded
   joints the planner would have guarded, and zasmu's units fit the roles.
2. **The planner in an analysis mode (no behaviour change).** Tasks,
   matching and paths, drawn on the owner's games in the review page
   (roles as unit colours, paths as arrows), for the owner to check the
   roles turn by turn.
3. **Guards and reserves (an experiment).** The plan of these roles only,
   as the climbs' start, from game turn 7. **Judge**: screening in full
   games against v033, then the SPRT.
4. **Pushers in formation (an experiment).** The push with a tower behind,
   after step 3's release.
5. **All roles, trainings born into roles, sweepers last (an experiment).**
6. **Paths over more turns and mines (an experiment each)**, if the
   earlier steps gain.

## The owner's decisions (2026-11-27)

1. **First roles**: guards and reserves (the cuts we did not see coming).
2. **Horizon**: paths over 3 turns; 5 can come in a later step.
3. **The search's freedom**: a unit the search plays off its path keeps its
   role and task; the planner plans its path again from where it stands.
   Roles change only by the planner's rules (a unit two turns nearer, a
   task gone).
4. **From game turn 5**, just before contact, not 7 and not 1. The opening
   plays game turns 1 to 6 today (E095's order) and the search from turn 7
   (E067; handing over at turn 6 lost −26.8 there, −6.1 in E096). So step 3
   screens two ways in: the search, started from the planner's plan, from
   game turn 5; and the opening kept at game turns 5 and 6 with its units'
   steps checked against the roles (no step that uncovers a guarded joint).
5. **The centre, held but not crowded** (the owner): we should get the
   centre without packing it. So the pusher tasks ask for at most two or
   three units abreast at a contested cell, a guard stands on or next to its
   joint and nowhere else, and a unit with no task near the centre is a
   sweeper, never a third unit beside two others. Step 1 measures how many
   units we keep within two steps of the line even between the HQs against
   zasmu's, so that the cap has a number.

## The questions as they were asked

1. **The first roles to build**: guards and reserves first (the unseen
   cuts), or pushers in formation first (the crowding and zasmu's pattern)?
2. **How far ahead**: paths over 3 turns first (cheaper, and the enemy's
   replies are less certain further out), or 5?
3. **The search's freedom**: may the search change a unit's role for the
   next turn when it plays it off its path, or does the planner keep roles
   and only replan paths?
4. **The opening**: should the planner also run from game turn 1 (E080's
   direction) once it plays well in the middle game, or stay from turn 7?
