# Frontline units, the race for the centre and roaming room: a plan (2026-11-20)

The owner, on 2026-11-20, after E076 (`ice-and-fire-v031`, the fall back
along the border):

> I actually think we could break down the early game / beginning of middle
> game strategy into three goals:
> - rush towards the most valuable cells as fast as possible (if there are
>   multiple objectives, make sure we quickly have one unit dedicated to each
>   objective, this unit should move only towards this direction, and newly
>   created units should be created only after a unit moved), we then have
>   two categories of units: frontline unit (one for each objective), back
>   line units. Note that a frontline unit can become a back line unit once a
>   closest unit was trained.
> - make sure to be there faster than the enemy, and maximize the area there.
>   It may then become a local fight that can be solved independently of the
>   rest of the board (I can be wrong on this)
> - also make sure our units won't end up in the middle of our territory, to
>   do so I think it is worth leaving empty cells within our territory so that
>   our units can roam afterwards. I noticed zasmu does something similar
>
> Propose a very detailed plan to do it, it is a very complex task that can
> be achieved in multiple steps / PRs

This note restates the three goals as definitions the code can follow,
gathers what the games already say about them, explains why earlier tries
in this direction lost, and lays out the work as steps, each a pull
request with its own check, so that each step can be stopped if its
evidence says so.

## The three goals as definitions

### Goal 1: one frontline unit per objective, rushing

- **Objective:** a cell whose holder decides land. Candidates: the middle
  chokes (`ice_and_fire_opening::solo::middle_chokes`), then the cells of
  the contested band (within 2 rings of the line even between the HQs,
  `eval::goal_zones`), the choice the owner made for E076's rush.
- **Value of an objective:** the land it decides, as the owner put it (we
  lose the cell to the opponent and the land behind it shrinks, so it is not
  one cell): the number of cells nearer to our active land than to the
  enemy's (Voronoi, in steps over valid cells) when we hold the objective,
  less the same when the enemy holds it. Objectives are ranked by value;
  low-value band cells are dropped (a threshold to tune).
- **Frontline unit:** at most one per objective. Each turn it steps along a
  shortest path toward its objective and never sideways or back (a
  different shortest path is allowed, and so is taking a cell on the way).
  It stops only on its objective, or when every step is blocked.
- **Trainings after the moves:** in a turn, the frontline units move first;
  trainings are placed after the moves, on the cells next to where a
  frontline unit now stands, nearest its objective. A new unit then starts
  as far forward as it can (zasmu trains at the tip after moving in 93% of
  its game-turn-4 trainings, we in 68%).
- **Handover:** when another unit of ours, a new one included, is nearer to
  an objective than the objective's frontline unit, it becomes the
  frontline unit, and the old one becomes a back-line unit.
- **Back-line unit:** every other unit (goal 3 says what it does).

### Goal 2: there first, and the most area there

- **Race table:** for each objective, the turn our nearest frontline unit or
  trainable cell reaches it, against the turn the enemy's nearest unit or
  its land reaches it (its trainings included, with its gold).
- **Commit or drop:** an objective we reach first, or at the same turn,
  gets a frontline unit; one the enemy reaches first is dropped, or
  defended where it is already ours (E076's `RUSH_DEFEND`, behind a switch
  that is off).
- **Area at the objective:** once there, the units near an objective fight
  for the land around it. The owner's question is whether such a local
  fight can be solved apart from the rest of the board. What links regions:
  the shared gold, cuts (one capture can cut land off elsewhere), and units
  that can reach two regions. Step 1 measures how local the fights are
  before any code depends on it.

### Goal 3: room for the back line

- **Pockets:** neutral cells enclosed by our land, which the enemy cannot
  reach without crossing our land (fence first, fill later). A pocket cell
  is as good as ours (it is in our Voronoi share and safe), and it is a
  cell a back-line unit can still take, so back-line units never stand idle
  inside our land.
- **Back-line policy:** a back-line unit moves to the nearest pocket cell,
  or to the nearest front cell where one of our cells is uncovered, and
  never into a cell where nothing is left to take within a few steps.
- **Leaving cells empty:** our side does not fill a pocket while a front
  cell or an objective still needs units; the pockets are filled when the
  back line has nothing better to do.

I read "empty cells within our territory" as neutral pockets inside our
land (the owner may mean empty cells we own: open question 3).

## What the games already say

From the 76 CodinGame games of v029 and v030 against zasmu (9 won),
compared within a seat where it matters (zasmu is blue in 69 of them):

| Measure | Us | zasmu |
| --- | --- | --- |
| Band cells held at game turn 10, red / blue | 5.0 / 7.9 | 8.4 / 12.8 |
| First game turn holding a band cell, red / blue (median) | 8 / 7 | 7 / 6 |
| Neutral cells sealed off by its land at game turns 9 to 11 | about 3 | 9 to 10 |
| Trainings at the tip of its land after the turn's moves, game turn 4 | 68% | 93% |
| Steps from a unit to its nearest own unit at game turn 8, red / blue | 2.05 / 1.65 | 2.44 / 2.39 |
| Unit turns standing still at game turns 11 to 14 | 43% | 21% |
| Units lost by game turn 9 (76 games) | 57 | 20 |

(Seat-controlled for the band cells and the spread; zasmu as red and us as
blue are 7 games each. The other rows mix seats.)

So zasmu reaches the centre at about the same turn as we do, but holds
more of it, fences more land than it fills, trains forward, spreads its
units, and keeps them moving: the three goals in the numbers. These are
correlations in games against us; Step 1 measures the goals directly.

## Why earlier tries in this direction lost, and what differs

- **E072 (frontier assignment, Hungarian on arrival turns, dropped):** it
  replaced the opening (game turns 1 to 6), planned against an enemy that
  does not move, and was judged by duels cut at game turn 12; it lost −21
  to −121. Here: roles persist across turns (a unit keeps its objective);
  the race counts the enemy's units and gold; the first behaviour change
  is in the middle game (turns 7 to 14), where E067, E068, E070 and E076
  gained; and the judge is full games.
- **E061 (the neutral land each side reaches first, as an evaluation term,
  dropped) and E076's land term (`VORONOI` 2 loses −22 and −36):** a
  weight on land alone pulls every unit at once. Here the land value ranks
  objectives, and roles decide who moves where.
- **E076's rush from the tip (screened, never better than the candidate):**
  chains of trainings with no unit dedicated to them, offered to a one-turn
  search that values a choke as one cell; offered in 303 of 304 positions,
  played whole in 7. Here the rush is the frontline units' moves followed by
  trainings at their tips, a plan for the whole turn, not one more action.
- **E066 (blocked units counted as idle, dropped) and E060 (idle units
  walk to land, accepted):** a cost per idle unit helps (E060), but did not
  tell units where to go. Here back-line units get pockets as targets.

## Design: a strategic layer above the search

The search stays the judge: it plays the turn, checks plans against the
opponent's replies and decides. The new layer proposes, each turn:

1. **Objectives** with their values (Step 2).
2. **The race table** (Step 3).
3. **Roles:** for each unit, frontline (with its objective) or back line,
   kept between turns in a memory beside the opening's (`Planned`), with
   the handover rule (Step 3).
4. **The strategic turn:** the frontline units' steps, then the trainings at
   their tips that the gold pays, then the back-line units' steps toward
   pockets or uncovered front cells, as one plan (Step 4).
5. That plan is offered to the search as a macro action and as the plan its
   climbs start from, and the evaluation gets terms that agree with the
   goals (an objective held, a pocket kept), so that the search can still
   prefer a cut, a defence or a lethal chain when the board demands it.

Code: a module `strategy` in `games/ice-and-fire/bots/plan/src/` (files
`strategy/objectives.rs`, `strategy/race.rs`, `strategy/roles.rs`,
`strategy/turn.rs`), standard library only, the bundler's conventions,
every behaviour behind a switch that is off until a step's experiment turns
it on. Images of positions (as in E076: the objectives, the race, the roles
and the strategic turn drawn on the board) come with each step for the
owner, from an analysis mode of the bot.

Time: objectives, race and roles are computed once a turn (a few
breadth-first searches over 144 cells), not at each evaluation; the search
keeps its 45 ms.

## Steps

Each step is one pull request. The ones that change the bot are one
experiment each (screening, then the SPRT on the candidate). A step whose
check fails stops the plan there for the owner's decision.

### Step 1: measure the three goals in the games (no bot change)

- **What:** a command of the review tool, `codingame-roles`, over the
  CodinGame games (the data branch) and over arena records, per side and
  game turn up to 15:
  - objectives and their values; for each, the turn each side first holds
    it, and who held it at turn 12;
  - frontline units, recognised as units that step toward one objective for
    at least two turns in a row, and back-line units; how many objectives
    have a frontline unit, and how soon after the first contact;
  - trainings placed after moves at the tip;
  - pockets: neutral cells enclosed by the side's land, and how many
    back-line unit turns end inside the land with no cell to take within 2
    steps;
  - locality: for each capture and kill of game turns 8 to 15, the distance
    to the nearest objective, and how often one turn's captures touch two
    regions or cut land off in another region.
- **Groups:** zasmu, the players ranked 2 to 7, our releases v029 to v031,
  by seat.
- **Check:** zasmu shows the three goals more than we do (one frontline
  unit per objective, trainings at the tip, more pockets and fewer idle
  back-line units); and the fights are local enough for Step 6 (most
  captures within 2 steps of one objective, few cuts across regions). If
  zasmu does not, or the fights are not local, the owner decides how to go
  on.
- **Also:** filmstrips of a few games with roles drawn, for the owner to
  confirm the definitions.
- **Size:** about 400 lines in `games/ice-and-fire/review/src/`, tests on
  hand-made positions; a research note with the tables.

### Step 2: objectives and their value (no behaviour change)

- **What:** `strategy::objectives(state, seat) -> Vec<Objective>`, each
  with its cell, kind (choke or band) and value (the land it decides: the
  Voronoi count of the side with the cell held by it less with it held by
  the enemy, the rest of the board as it is). Ranked by value; a threshold
  drops the low ones.
- **Tests:** values on hand-made maps (a single corridor: the choke decides
  everything behind it; a symmetric map: mirrored objectives have equal
  value); the value is 0 for a cell neither side's land can reach first.
- **Check:** in the zasmu games, the objectives zasmu takes first are
  among our top-valued ones (Step 1's tables); images for the owner.

### Step 3: the race and the roles (no behaviour change)

- **What:** `strategy::race(state, objectives)`: our arrival turn (a unit
  walking, or a training at our nearest trainable cell then walking)
  against the enemy's (its units walking, its trainings with its gold).
  `strategy::roles`: assigns at most one frontline unit to each objective
  we win or tie, by arrival turn then value (greedy first; the Hungarian
  method of `ice_and_fire_opening::assign` if greedy proves poor); keeps
  roles between turns; applies the handover rule. The memory sits beside
  `Planned` in the bot's loop (`session.rs`).
- **Tests:** one frontline unit per objective at most; a frontline unit's
  objective does not change unless it is reached, lost or handed over; the
  handover picks the nearer unit; roles survive a turn in which nothing
  changes.
- **Check:** played over the zasmu games as if we were zasmu, the roles
  match zasmu's own frontline units (Step 1's recognition) in most turns;
  images for the owner.

### Step 4: the strategic turn in the middle game (E077)

- **What:** the strategic turn of the design, game turns 7 to 14, offered
  to the search as a macro action and as the start of its climbs. Variants,
  each behind a switch:
  - offered only, or also the climbs' start;
  - trainings at the tips only after the moves (the owner's rule), or also
    before when the gold would otherwise stay unspent;
  - an evaluation term for the objectives: for each objective, its value
    times (held by us 1, by the enemy −1, else our race margin capped at 1).
- **Judge:** cut duels at game turn 12 for speed, then full games on
  GitHub (`screen.yml`, 800 pairs, three seeds for the leader, as E076),
  then the SPRT against v031.
- **Also measured:** Step 1's measures on the candidate's games against
  v031: band cells at turn 10, frontline coverage, trainings at the tip.

### Step 5: the back line and its room (E078)

- **What:** back-line moves toward pockets or uncovered front cells, as
  part of the strategic turn; an evaluation term for pockets (each pocket
  cell counts as a cell of ours, so filling one gains nothing while a front
  cell is free); the rule that pockets are filled last.
- **Judge:** as Step 4, against Step 4's release (or v031 if Step 4 was
  dropped).
- **Also measured:** idle back-line unit turns at game turns 11 to 14 (we
  43%, zasmu 21%); pockets held at turns 9 to 12.

### Step 6: local fights (E079, only if Step 1 finds them local)

- **What:** a region per contested objective (the cells within a few steps
  of it and the units in it); a small search per region (all moves and
  trainings of the region's units over one or two turns, against the
  enemy's units there); the gold shared between regions by their value;
  the regions' best plans offered to the main search, which checks the
  cuts that cross regions.
- **Judge:** as Step 4.
- **Risk:** if one cut can decide two regions (Step 1's measure), the
  decomposition misleads; the main search's check is the safeguard.

### Step 7: the opening by roles (E080)

- **What:** the same layer from game turn 1: frontline units for the
  objectives from the first training, trainings after moves at the tips.
  Offered to the opening's planner as one more plan beside its rule sets
  (`ice_and_fire_opening::planned`), so it plays only when the planner
  rates it best.
- **Judge:** `oracle --bench` (no land lost against the stored best plans),
  then cut duels, full games and the SPRT.
- **Why last:** the opening is level with zasmu within a seat at game
  turns 6 and 8, and the openings that bought more (E046, E056) or assigned
  units to the frontier (E072) lost; the gap opens at turns 8 to 12.

### Step 8: CodinGame and the next goals

- Submit the releases on the owner's approval (skill `deliver-release`);
  read their ranking games with Step 1's command and the capability suites;
  compare band cells, frontline coverage and idle units with zasmu's.
- Then the later goals of the zasmu study: banking for the strike and the
  defence against it.

## Order and size

| Step | Pull requests | Changes the bot | Size |
| --- | --- | --- | --- |
| 1 Measure | 1 (tool and note) | no | medium |
| 2 Objectives | 1 | no (switch off) | small |
| 3 Race and roles | 1 | no (switch off) | medium |
| 4 Strategic turn (E077) | 1 experiment | yes | large |
| 5 Back line (E078) | 1 experiment | yes | medium |
| 6 Local fights (E079) | 1 experiment | yes | large |
| 7 Opening by roles (E080) | 1 experiment | yes | large |
| 8 CodinGame | per release | no | small |

Steps 2 and 3 can share a pull request if the owner prefers fewer; each
experiment stays alone (one hypothesis per pull request).

## The owner's answers (forms, 2026-11-20), as defaults

The owner answered the open questions and added: "I gave logical answers,
however it may be challenged. Make sure to propose multiple variants or
totally different implementations/approaches when relevant". So each
answer is the default, and the steps screen the other options against it.

| Question | Default (the owner's answer) | Variants screened against it |
| --- | --- | --- |
| Value of an objective | The land it decides (the Voronoi swing) | Chokes above every band cell, then by land; nearness to the line even between the HQs; the land weighted by the cells' own value (a mine spot more) |
| How many objectives at once | Those above a value threshold, as many as we have units for | One per unit, the most valuable first; at most 2 or 3; a threshold that falls as the game goes on |
| Room for the back line | Neutral pockets fenced in by our land | Corridors of owned empty cells from the back to the front; both |
| A frontline unit's path | Any shortest path, one step nearer every turn | A step aside to take a free cell when it costs at most one turn and the race is still won; one path fixed when the objective is given |
| Trainings in a turn | After the frontline moves, at their tips; elsewhere only with gold that would stay unspent | Strictly after the moves and at the tips; no rule, the tips offered and the search free |
| First step | Measure (Step 1) | (Steps 2 to 4 directly; a quick prototype first) |

## Variants by step

Beyond the table above, each step's experiment screens, on GitHub, the
variants that could reasonably win (up to 8 a round, three seeds for a
leader, as E076):

- **Step 2 (objectives):** the land counted to the end of the map or only
  within a few steps of the objective; mine spots counted more; objectives
  recomputed every turn or kept until reached or lost.
- **Step 3 (race and roles):** greedy assignment by arrival then value,
  against the Hungarian method (`ice_and_fire_opening::assign`) on arrival
  turns weighted by value; the handover when a unit is nearer by one step,
  or by two (fewer switches); the enemy's arrival with its trainings paid
  from its gold, or its units only.
- **Step 4 (strategic turn, E077):** the plan offered as one macro action,
  as the start of the search's climbs, or both; an evaluation term for the
  objectives on or off, at a few weights; game turns 7 to 12 or 7 to 14.
- **Step 5 (back line, E078):** back-line units to the nearest pocket, to
  the nearest uncovered front cell, or the nearer of the two; the pocket
  term on or off; pockets kept only while a front cell is free, or always
  until game turn 14.
- **Step 6 (local fights, E079):** the region's radius (2 or 3 steps);
  one turn or two of local search; the gold shared by value, or by need
  (the gold the region's best plan uses).
- **Step 7 (opening by roles, E080):** the roles offered as one more plan
  to the opening's planner, or played as the opening outright.

## Two other approaches, to compare with the frontline layer

The owner chose (form, 2026-11-20) to lay out two different approaches as
alternatives; each would be a track of its own, judged by the same cut
duels and full games, so that the three can be compared on the same
measures (Step 1's).

### A. A search over several of our turns

- **What:** the middle game searches over two or three of our turns with
  the enemy's replies between them, so that a rush, a fall back or a
  fence emerges from the search instead of from rules. Candidates: a beam
  over whole-turn plans (the current climb's best plans as the beam), or
  Monte Carlo tree search over turns with the climb as its playout policy.
- **What we know:** E065 checked the final choice two turns ahead and was
  level (the check was cheap: 1.5 to 3 ms a turn); M1 (beam) and M2 (two
  turns) were proposed on 2026-11-14 and not tried. Time is the limit: 45 ms
  a turn, the evaluation 63% of the search's time.
- **Steps:** (A1) a two-turn beam behind a switch, its speed measured;
  (A2) screened against v031; (A3) Monte Carlo tree search only if A2
  shows depth pays.

### B. A two-sided early planner (the race as a game)

- **What:** plan both sides' units from game turn 7 to 12 or 14 as a race
  with simultaneous moves and arrival times: each side's units and
  affordable trainings to objectives, with the enemy planning too, solved
  exactly on small regions (the oracle's exact solver,
  `opening/examples/oracle.rs`, as a start) or by search (best response
  against the enemy's best plan, a few rounds).
- **What we know:** the early-game plan of 2026-11-19 proposed a two-sided
  planner judged by cut duels; the oracle found the most land by turn 6 is
  held by tens of thousands of territories, so exact solving must stay
  regional or short.
- **Steps:** (B1) the race game on one region, solved exactly, checked
  against the zasmu games (does its plan match zasmu's arrival?); (B2) as
  the plan offered to the search, screened against v031.

### How the three compare

| | Frontline layer (Steps 1 to 8) | A. Multi-turn search | B. Two-sided planner |
| --- | --- | --- | --- |
| Encodes the goals | as rules and roles | finds them by search | finds them by solving the race |
| Main risk | rules that misfire in positions they did not foresee | too slow in 45 ms | too slow or too regional |
| First evidence | Step 1, no code in the bot | A1's speed | B1 on zasmu's positions |
| Size | large, in eight steps | medium to large | large |

Step 1's measures serve all three. The recommended order is Step 1 first,
then the frontline layer's Steps 2 to 4, with A1's speed check alongside,
since it is cheap; B waits for Step 1's answer on how local the fights are.
