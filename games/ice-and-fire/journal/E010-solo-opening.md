---
id: E010
date: 2026-10-23
pull_request: "#NN"
parent: ice-and-fire-v004
hypothesis: An opening planned for our side alone over its first 7 turns, which keeps the territory and the income of v004's but grows it compact (few cells that one capture cuts off), together with the same shape term in the middle-game evaluation, beats v004.
change: Until the middle game the `plan` bot plays v003's opening rules with weights for a compact shape and a pull toward the enemy HQ, chosen turn by turn by an anytime beam search over our next 7 turns as if the opponent passed (about 610 ms on the first turn); the middle-game evaluation takes 2 from a side's score for each of its joints and each cell of the largest group one capture cuts off.
release: ice-and-fire-v005
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-23: "I still think the early game is really
bad, the position is weak when entering the middle game. When there a
multiple options to spawn units is it possible to try to have them grouped
(while obviously trying to maximize the territory, then revenue?) Thin
lines are also weak and should be penalized. Could you focus on the early
game first? [...] There must not be two players, just one player focusing
on his early game alone!", then "Could you add a strategy which penalizes
inside moves? Also try to go towards the opponent HQ when possible.
Remember to not only do a greedy approach, but try to do a long term
planning (by using more time for the search, maybe using beam search?)",
and, on a planned board of map 3, "Can you do a beam search closer to the
time limit too?".

## One player alone

The early game is studied as one player alone: red plays 7 turns (the
middle game starts at our eighth turn in the median game) while blue
passes, and the board at the start of turn 8 is judged. The tool is the
opening crate's example `solo` (`cargo run --release -p ice-and-fire-opening
--example solo -- --help`); it prints a table of averages over maps and
seats, and the boards side by side.

The shape of a territory is measured by its **joints** (active cells whose
loss cuts other cells off from the HQ, found by a depth-first search from
the HQ), the **most cells one capture cuts off**, its **line ends** (cells
with at most one own neighbour) and its **reach** (steps to the enemy HQ).

The opening's rules (`econ`) gained five weights for a candidate cell:
`group` (a bonus for each own neighbour), `thin` (a penalty when only one
touches it), `spot` (a bonus for an own mine spot), `inside` (a penalty for
a cell with no free neighbour, where a unit would be stuck) and `toward` (a
pull toward the enemy HQ). v003's and v004's weights set them to 0.

The planner (`opening/src/solo.rs`) combines, turn by turn, 27 weight sets
(group 0, 2 or 4, thin 0, 3 or 6, toward 0, 1 or 2; inside 4 when a shape
weight is on) with 8 ways of buying. A beam search over whole turns, the
opponent passing, keeps the best nodes of each turn by the position's
value: 4 a cell, 3 an income, 1 a gold, 1 for each cell of the land nearer
us than the enemy (half the Voronoi count), less 2 for each joint, each
cell that one capture cuts off and each line end, and 6 for each step to
the enemy HQ. The search is anytime: widths 2, 4, 8 and so on until the
deadline; past the deadline it rates only the top node, which keeps it
within about 5 ms of its deadline (it overshot by up to 120 ms before).

The Voronoi term came from the owner's reading of map 3's board ("this
seems suboptimal right?"): without it the plan grew a band along the top
edge.

Over 100 maps from both seats (200 openings), at the start of turn 8:

| Policy | Score | Cells | Income | Gold | Joints | Most cut off by one cell | Line ends | Steps to enemy HQ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| v004's rules | 129.9 | 22.0 | 21.4 | 25.2 | 9.2 | 18.6 | 2.7 | 11.5 |
| Compact rules (group 2, thin 6) | 134.7 | 19.8 | 16.8 | 20.6 | 1.1 | 3.4 | 0.5 | 13.9 |
| Planned, 600 ms | 185.3 | 22.2 | 21.5 | 25.7 | 0.6 | 1.5 | 0.2 | 11.9 |

The compact rules alone lose cells and income; the plan keeps both and
removes nearly every joint. More time helps little past 600 ms: the score
is 178.5 at 50 ms, 183.0 at 150 ms, 184.4 at 300 ms, 185.8 at 600 ms and
186.0 at 900 ms. Weighting income 5 or 8 instead of 3 traded a cell for a
little income; 3 was kept (territory first). The boards of maps 1 to 10
were shown to the owner as a page.

## In the bot

`Planner::Solo` (`opening/src/planned.rs`) plans 7 turns on the first turn,
follows the plan while our side is as it predicted (the opponent's moves
rarely touch us before contact), and plans again within the turn's time
when it differs or runs out. The opening ends at the middle game, as v004's
does. The first turn's budget is 800 ms, of CodinGame's 1,000; the plan
stops at 76% of it, about 610 ms.

## Screening

All at CodinGame's exact limits, two games at a time, against
`ice-and-fire-v004`, Elo with its 95% interval:

| Variant | Seed | Pairs | Elo |
| --- | ---: | ---: | --- |
| Planned opening, v004's middle game | 1010 | 200 | +17.4 [−14.1, +49.2] |
| The same | 2010 | 600 | −11.0 [−29.5, +7.4] |
| The same, gold weighted 3 | 3010 | 400 | −34.0 [−58.3, −10.0] |
| The same, gold weighted 2 | 3010 | 400 | −10.4 [−33.2, +12.3] |
| The same (the base below) | 3010 | 400 | +16.5 [−6.0, +39.1] |

The planned opening alone came out level with v004. In 15 recorded games it
reached the middle game with about 3 more cells and 3 more income than v004
but 23 less gold, spent on units; the owner chose to value the bank, which
made it worse. He then asked for about ten variants, played on the same 400
pairs (seed 3010) as the base:

| Variant of the base | Elo |
| --- | --- |
| Gold weighted 0 | −9.6 [−32.8, +13.6] |
| No pull toward the enemy HQ (reach 0) | +8.7 [−13.9, +31.3] |
| Pull doubled (reach 12) | −4.3 [−28.2, +19.4] |
| No Voronoi term | +2.6 [−19.0, +24.2] |
| Voronoi tripled | −23.5 [−46.9, −0.3] |
| No shape penalties (joints, cut, line ends 0) | −91.5 [−116.2, −67.7] |
| Shape penalties doubled | −19.1 [−41.9, +3.5] |
| Plans of 4 turns | −78.6 [−103.1, −54.8] |
| Plans of 10 turns | +11.3 [−11.3, +34.0] |
| Income weighted 6 | −3.5 [−26.0, +19.0] |

None beat the base. Two lost heavily: without the shape penalties, and
with plans of 4 turns. Shape decides much, but the middle-game search did
not see it. The owner chose to add it there: each side's joints plus the
most cells one capture cuts off, times a weight, taken from its score
(`bots/plan/src/eval.rs`). On the same 400 pairs: weight 1 +6.1 [−16.1,
+28.3], **weight 2 +25.2 [+2.4, +48.3]**, weight 4 −1.7 [−24.9, +21.4].
Weight 2, best of 14 tries on those maps, was checked on 600 pairs of new
maps (seed 4010): 656 wins, 544 losses (54.7%), **+32.5 Elo [+14.0,
+51.2]**, 3 timeouts in 1,200 games (v004 0 in that run, 1 to 3 in the
others on this machine), slowest answer 623.7 ms (the first turn), answers
after the first 7.1 / 32.2 / 35.2 ms (median, 99%, 99.9%).

## The candidate

`ice-and-fire-v005` is the `plan` bot with the solo opening (objective as
above, 7 turns) and the middle-game shape weight 2. The bundle is 98,396
characters of the 100,000 allowed.
