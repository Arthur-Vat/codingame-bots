# A type planner learnt from zasmu, from game turn 5 (2026-11-28)

After E103 (a lazier model of the opponent's reply, −73.5), E104 (a
stronger one, level) and E105 (a frugal one, −5.7 to −9.4), the owner asked
for a model of play like zasmu's, by unit type, for both sides (2026-11-28):

> For units assume you should first model unit type, then for these have a
> deterministic action or a model-based set of action. It should depend on
> the timing in the game and other metrics. [...] I think this approach could
> apply to us and the opponent.

then, after the measures below:

> I don't want a small fix, I prefer a large design rework. [...] We should
> consider from turn 5. We are not actually level because we reach turn 6 by
> being too crowded and then have a slightly worse position. Our units find
> it difficult to grab territory after.

His choices in forms on 2026-11-28: the tables guide both climbs, ours and
the opponent's, measured alone and together; our climb samples whole turns
from the tables and offers each unit only its type's likeliest kinds
(pruning our own options cannot hide a danger from us), while the
opponent's climb stays complete, ordered by the tables, with a few sampled
replies beside it (a missed reply is a danger we do not see: E103, E105); hand-written types refined by the
data; learnt from zasmu only; the shape "a type planner and the search"; the
new design from game turn 5; cells chosen by a matching that keeps units
apart; progress measured by the land and units at game turn 12 and by the
crowding at turns 5 to 12. This note is the design, for his review before
any code. It builds on the role-planner note of 2026-11-27 (branch
`claude/ice-and-fire-role-planner`), whose four principles stand.

## What the data says

All 1,038 games against zasmu on the data branch (one agent id; we won 84),
every ply from game turn 1: 22,789 of zasmu's positions and replies, and
ours in the same games. Scratch tools (the analysis build's `zunits` and
`zunits_us`, the review's `codingame-surprise` with every ply kept).

**The game turns at turns 9 to 12.** zasmu's land minus ours, the median
over the games: −2 at game turn 5, −1 at 8, +1 at 9, +3 at 10, +7 at 12, +12
at 16; zasmu 5 cells ahead or more in 2% of the games at turn 8, 61% at 12.
At game turn 15 we win 27% of the games where we lead by 3 cells or more,
7% of the level ones, 4 to 5% of those we trail. zasmu's income passes ours
at game turn 10 (29 against 27); ours stays at 29 from turn 11 while
zasmu's reaches 48 at turn 16. By turn 16 zasmu holds 3.0 mines, we 1.5.

**It is mostly expansion, not fights.** From game turn 7 to 12 zasmu gains
4.9 to 2.8 cells a turn, we 4.1 to 1.4; zasmu captures 0.4 to 1.4 of our
cells a turn.

**Crowding starts in the opening.** Per unit, at the start of its side's
turn:

| Game turn | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 12 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Own units next to it, zasmu / us | 0.00 / 0.48 | 0.63 / 0.91 | 0.50 / 0.46 | 0.50 / 0.71 | 0.34 / 0.73 | 0.33 / 0.53 | 0.36 / 0.47 | 0.38 / 0.46 |
| Free neutral cells next to it, zasmu / us | 1.86 / 1.59 | 1.88 / 1.69 | 2.13 / 1.79 | 2.06 / 1.80 | 1.70 / 1.23 | 1.47 / 0.96 | 1.16 / 0.84 | 0.75 / 0.58 |
| No free neutral cell next to it, zasmu / us | 0% / 6% | 4% / 2% | 3% / 2% | 2% / 6% | 9% / 17% | 19% / 33% | 30% / 44% | 50% / 59% |
| Units, zasmu / us | 2.0 / 2.0 | 3.0 / 2.9 | 3.9 / 3.1 | 4.9 / 3.8 | 5.1 / 4.7 | 5.5 / 5.0 | 6.0 / 5.0 | 6.5 / 5.2 |

zasmu trains at game turns 5 and 6 (0.9 and 1.0 a turn) where our opening
trains 0.1 and 0.8 and builds a mine; it holds one more unit from turn 6.

**What each type does** (hand types on the position: a pusher next to the
enemy's land; a guard on or next to one of its joints within 3 steps of the
enemy; a reserve within 3 steps; else back; actions: capture, a step towards
the enemy's land onto a neutral or an own cell, sideways, away, or none).
At game turns 9 to 12, zasmu's pushers capture 15 to 24% of the time and
ours 5 to 9%; zasmu's units behind the front stand still 1 to 4% of the
time, ours 10 to 30%; zasmu's take neutral land 75 to 89% of the time, ours
50 to 64%. A pusher that can capture does so 56 to 77% of the time, more
with more of our units within 2 steps; a back unit with less than 40 gold
steps onto neutral land 45 to 60% of the time and, with 80 or more, stands
still 52% of the time. With 80 gold or more, 47% of zasmu's replies spend
nothing; at a turn where it can pay a chain to our HQ it trains about 8
units at once.

**The tables predict zasmu.** Each unit's action from a table of the other
games' counts (games held out, backing off to a coarser key below 20
cases), from game turn 7: the most common action 36% right (62% in three
guesses); by type and level 38% (72%); with the gold band 46% (80%); with
the local fight (can capture, units of each side and our towers within 2)
50% (84%); with the position (distances to the enemy and to the nearest
joint, on a joint) 51% (86%).

## The design

A layer before the search, for each side to act, from game turn 5 (the
opening plays turns 1 to 4). The search stays the judge of every turn,
against the opponent's replies (principle 1 of the role-planner note).

1. **Types.** Every unit of the side gets a type from the position: pusher,
   guard, reserve, back, each with its level, and the back units split by
   whether a free neutral cell is next to them. The split is refined where
   the data shows a type mixing behaviours.
2. **Tables.** For each type, the shares of zasmu's action kinds, by turn
   band (5–6, 7–8, 9–12, 13–20, 21 and later), gold band (under 40, 40 to 79,
   80 and more, and a killing turn apart), local fight and position, backed
   off to coarser keys where the data is thin. Learnt offline in Python
   from the data branch, encoded in Rust as compact tables (the bot has
   about 30,000 characters left of its 100,000).
3. **Spending.** For each turn band and gold band, zasmu's trainings by
   level and cell type (capture, front, near, far), its towers by cell type
   (on or next to a joint, front, near, far) and its mines, as shares; they
   order the trainings and buildings the climbs offer, and give the plan its
   purchases.
4. **Cells by a matching.** A kind of action becomes a cell by a matching
   of the units to the candidate cells of their likeliest kinds, with a cost
   for crowding (a cell next to another unit's cell, two units heading for
   the same part of the frontier) and paths over 2 to 3 turns reserved so
   that units do not block each other (the role-planner note's step 3; the
   Hungarian method in `ice_and_fire_opening::assign`). No planned step
   uncovers a cell the enemy can enter (principle 2).
5. **Sampled turns.** Each unit's kind drawn from its type's table, the
   kinds turned into cells by the matching, and purchases drawn from the
   spending table, make one whole turn; K of them (K about 32) are drawn,
   each scored against the remembered replies, and the best few start our
   climbs (forced starts, E029). The search's time goes to promising, varied
   turns rather than to one plan it can climb away from (E077's plan was
   rejected in 99% of turns).
6. **Our climb, pruned.** In every climb of ours, each unit is offered only
   the cells of its type's likeliest kinds (the three likeliest hold 86% of
   zasmu's actions from turn 7), tried in the tables' order; the lethal
   chain, cuts and blockers stay offered.
7. **The opponent's model.** Its climb stays complete (nothing pruned),
   ordered by the same tables from its seat; a few turns sampled for it the
   same way are added to its replies, so that the model is more like a real
   player without missing what the climb finds.

## Stages

Each stage is one experiment, with its journal entry and a screening
against v034 (the owner's choice: measures at every stage), and the
measures the owner chose: our land lead and units at game turn 12, and the
crowding at turns 5 to 12, in local games at CodinGame's speed against v034,
and later in the collected games against zasmu.

| Stage | What | Check that can stop the work |
| --- | --- | --- |
| 1 | The tables and the spending table built offline, encoded in Rust, and the bot's typing checked against the Python one on the data branch's positions | The encoded tables predict zasmu's held-out replies as well as the Python ones (51% and 86% from turn 7; to be measured from turn 5) |
| 2 | Our climb pruned to the tables' likeliest kinds and ordered, from game turn 7 (the opening as now); variants by the number of kinds kept | Crowding and land at turn 12 move towards zasmu's figures; no loss in screening |
| 3 | The opponent's climb ordered, and sampled replies added | Screening: each alone and both together |
| 4 | The matching and the sampled turns as the climbs' starts | Crowding at turns 7 to 12 near zasmu's; how often the move played comes from a sampled turn; no loss in screening |
| 5 | The new design from game turn 5 (the opening plays turns 1 to 4) | Units at turn 7 and land at turn 12; screening |
| 6 | The spending table in the plan | Income at turn 12; screening; then the SPRT of the whole against v034 |

Stage 1 changes no play; stages 2 and 3 are small in code; stage 4 is the
largest (the matching and the paths); stage 5 needs the opening's handover
(`middle::reached`, E067's `HANDOVER_TURN`) moved.

## Risks

- **A weaker model of the opponent** (E103, E105): its climb is never
  pruned, and stage 3 measures the opponent's side alone.
- **Our pruning hides a good move**: the three likeliest kinds miss 14% of
  zasmu's actions; stage 2 screens the number of kinds kept, and the
  tactical actions (lethal chain, cuts, blockers) stay offered.
- **The search rejects the plan** (E077: rejected in 99% of turns): the
  sampled turns are many and varied, our climb is pruned to the same kinds,
  and stage 4 counts how often the move played comes from a sampled turn.
- **Time**: the typing and the matching run every turn and in every climb's
  ordering; the typing is a few floods, the ordering a table lookup, the
  matching once per turn. Stage 2 measures the search's tries a turn.
- **zasmu's style may not suit our search**: its play is learnt with its
  own search behind it; the measures at turn 12 show early whether moving
  like it gains land with ours.
- **Seat bias**: zasmu's test games all have it in the second seat; its
  first-seat games come from replays (ADR 0061, pull request #403) and the
  85 ranking games where it had it.
- **Size**: the tables must fit the bot's 100,000 characters with the rest.
