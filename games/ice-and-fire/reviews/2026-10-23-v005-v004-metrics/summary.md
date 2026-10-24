# Summary of the batch

Bots: ice-and-fire-v005 (candidate, opening planned on turn 1 for a compact territory toward the enemy HQ, plus a shape term that penalizes joints and cells one capture would cut off) against its parent ice-and-fire-v004. A Code of Ice and Fire, 1,000 games (500 seat-swapped pairs, seed 234, CodinGame time limits). v005 won 567, v004 433; all 1,000 decided, median length 18 turns. This is a metrics report (ADR 0044), so there are no reviews and no plies are discussed.

## What wins these games

The metrics tell the winner from turn 10 on, not before. Income, cells, material, gross income and the two change metrics tell from turn 10 at 71 to 74%. Gold, units, army, voronoi and level-1 units tell from turn 11 at 71 to 74%. At turn 8 nothing reaches 70%; the best are cells and material at 62%. By turn 12, income and material are at 83%, cells at 82% and voronoi at 77%. By turn 16, material is at 90% (winners average 480, losers 218), and lower hq_gap (the gold still lacking for the cheapest chain to the enemy HQ) goes with winning at 88%.

The side ahead on material at turn 10 (the report's lead metric) won 731 of the 994 games where it had a lead, 73.5%. The 263 games where the leader lost are the comebacks.

Cause or consequence: territory comes first. Cells (62% at turn 8, 71% at turn 10) and voronoi (67% at turn 10) are the earliest signals that are plausibly causes. Income and material are built from cells and mines by definition, so they largely follow from territory. Army and units follow from the gold the side could spend. Captured, lost and can_cut tell from turns 11 to 12 and most likely follow from the lead. Everything at one turn before the end is at 90% or more, which is a consequence by construction.

The shape metrics do not tell the winner early. Joints, total_cut and max_cut stay between 42% and 49% through turn 10 across all games. Their slice results flip with which bot won (the winner had more joints in 84% of v004-won games, fewer in 86% of v005-won games), so they mostly show the bots' styles, not a cause.

The slices disagree, so the overall 57% should not be read alone:
- Seat: v005 won 336 of 500 as blue (67%) and 231 of 500 as red (46%). v004 won 54% as blue and 33% as red. v005 gains 13 points over v004 in each seat. The seat gap is 21 points for each bot, and blue wins 605 of 1,000 overall.
- Length: v005 won 202 of 333 short games (61%), 198 of 333 regular games (59%) and 167 of 334 long games (50%). Its edge is in the first two thirds. In long games the metrics that tell the winner arrive late: gold from turn 17 (72%), cells, units and voronoi from turns 14 to 15.
- Turn-10 leader: v005 won 172 of the 263 games the leader lost (65%). The report does not say which bot led, so these counts show who won, not who led.

## How the two bots differ

Means at turn 10 (v004 / v005):
- Shape: total_cut 60.5 / 7.7, max_cut 23.0 / 3.3, joints 7.0 / 2.4, empty_joints 5.4 / 1.6. The shape term works as designed from turn 5 on (total_cut 19.3 / 6.5), and the gap holds in both seats.
- Land: voronoi 52.8 / 44.6, room 14.9 / 8.9, border 18.7 / 11.9. v005 holds less land nearer to its cells than the enemy's and leaves less neutral room beside its territory. Voronoi is one of the metrics that tell the winner, so this is a cost.
- Expansion: far_cells 1.7 / 0.8 at turn 10 and 3.7 / 2.7 at turn 12. Cells are equal (26.1 / 25.4 at turn 10, 37.9 / 38.8 at turn 20). Material is level at turn 10 (260 / 258) and pulls ahead from turn 12 (279 / 296), with material_change at turn 12 of 5.7 / 19.4.
- Spending: at turn 8 v005 keeps 30% of its gold against v004's 68%, and spends 9.5 against 4.8 on buildings. By turn 20 it holds more gold (67.1 / 55.4).

Seat differences (same bot, different seat):
- Material_change at turn 10: v005 as blue 22.2 against v004 as red 12.5; v005 as red 9.9 against v004 as blue 23.2. v005 gains economically as blue and trails as red.
- Far cells: v005 as red reaches 0.0 to 0.2 at turns 8 to 10, against v004 as blue at 2.9. v005 as blue reaches 1.4 at turn 10 and 4.7 at turn 12, against v004 as red at 0.6 and 1.3.
- Shape: the joint and cut gaps are the same in both seats, so the shape change does not depend on the seat. The expansion and economy gains do.

## What the report cannot say

- These are correlations from games that have already ended. Late-turn shares rest on fewer games: 797 games still run at turn 15 and 436 at turn 20. Some late shares come from survivors, such as joints at 72% for turn 28 in the v004-red slice.
- Blue starts with one more cell, so early tells can be seat effects. The 21-point seat gap is the same for each bot.
- The 56.7% has a 95% interval of about 53.6% to 59.8%. It covers one seed and one opponent.
- The report gives no per-length means for v005, so it cannot say what v005 does differently in long games.
- The opening plan and the shape term changed together, and both show from turn 5. The report cannot separate them.
- Lead and winner are linked by construction: material is both the lead and a winner tell.
- It has no plies, no moves and no reviews, so it cannot name the moves behind any gap.

## Ideas for the next experiment

1. Separate the two v005 changes. Run v005 with only the turn-1 opening, and with only the shape term, against v004 over seat-swapped pairs. The shape gap appears from turn 5, so this batch cannot tell which change wins games.
2. Keep the shape term but protect land. v005 holds less land (voronoi 44.6 against 52.8 at turn 10, room 8.9 against 14.9), and voronoi tells the winner at 77% by turn 12. Try a joint penalty that applies only where the cut cells would be lost to the enemy, or raise the weight of voronoi and room. The joint metrics stayed near 50% through turn 10, so the current penalty may cost more land than it protects.
3. Make the red opening expand. As red, v005 barely pushes outward (far_cells 0.2 against 2.9 at turn 10) and trails in material_change at turn 10. Test a seat-aware opening so red reaches out as blue does, and measure the win rate by seat.
4. Measure before changing the late game. v005 wins only half of the long games, and the report cannot say why. Before changing the late plan, log what v005 does after turn 20: its gold, its spending, and its cell losses.
