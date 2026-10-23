# Summary of the batch

Bots: ice-and-fire-v005 (the current release) against its parent ice-and-fire-v004. 1,000 games in 500 seat-swapped pairs (seed 234), at CodinGame's time limits. v005 won 567 and v004 433 (56.7% for v005). Median length is 18 turns. The report has metrics only: there are no reviews in this batch and no game names or plies.

## What decides these games

Economy and reach toward the enemy HQ decide the games from about turn 10. Shares below are the share of still-running decided games in which the winner's value was higher, unless marked "lower wins".

- **Material** (the lead metric) is 73% at turn 10, 83% at turn 12, 90% at turns 16 and 20.
- **Income** is 74%, 83%, 86% and 85% at turns 10, 12, 16 and 20. **Cells** are 71%, 82%, 86% and 85%.
- **hq_gap** (gold still lacking to reach the enemy HQ) is the strongest signal of reach: lower wins 75% at turn 11, 79% at turn 12, 88% at turns 16 and 20.
- **reach** (steps to the enemy HQ): lower wins 83% at turn 16.
- **cut_off** (owned cells cut off from the HQ): lower wins 71% from turn 12.
- **voronoi** (land nearer one's own cells): 67% at turn 10, 83% at turn 16.
- **Gold** says little early (the winner had less gold at turn 8 in 54% of games) and tells from turn 11 (71%).
- The side ahead in material at turn 10 won 731 of 994 decided games (about 74%).
- One turn before the end, nearly all economy metrics are above 90%. The end state repeats the economy, so the signal worth acting on is earlier.

Potential cuts (max_cut, joints, total_cut) never reach the 70% threshold over the batch (max_cut only at turn 38). Actual cut-offs do.

The largest recurring swing in the turn means is material change at turn 12: v005 +19.4 against v004 +5.7 over all games, and +26.6 against -4.1 in the games where v005 was blue and v004 red.

## How the slices split

**Seats.** Blue won 605 of 1,000. Each bot wins about 21 points more as blue: v004 won 53.8% as blue (269 of 500) and 32.8% as red (164 of 500); v005 won 67.2% as blue (336 of 500) and 46.2% as red (231 of 500). v005 is ahead of v004 by 13.4 points in each seat, so its edge does not depend on seat. But as red v005 is not clearly above 50%.

**Length.** This is the main disagreement with the whole batch. v005 won 60.6% of short games (226 of 373, up to 16 turns), 59.7% of regular games (176 of 295, 17 to 21 turns) and only 49.7% of long games (165 of 332, 22 turns or more). The gap between short and long is about 11 points, with a 95% margin near 7. The edge belongs to short and regular games.

Long games show different tendencies. At five turns before the end, gold (89%), income (92%), can_cut (80%), inner_units (83%), far_cells (83%) and reach (lower wins 82%) lead. At one turn before the end, mines (74%), towers (65%), tower_cover (70%) and hq_guard (58%) lead, and the winner lost fewer cells in its last turn in 62% of games. Short games at five turns before the end show cells (84%) and income (70%), and gold gives no signal (54%).

**Leader at turn 10.** The turn-10 leader won 731 games and lost 263. When it lost, the winner came from behind: v005 in 172 of those games and v004 in 91. So 30% of v005's wins came from behind, against 21% of v004's. v005 won 69% of its wins from a turn-10 lead, against 79% for v004. The report does not give each bot's lead conversion, so this is a difference in wins, not a conversion rate.

**Seat and bot together.** When v005 was red, at turn 10 its far_cells averaged 0.2 against v004's 2.9 (v004 blue), its material change 9.9 against 23.2, and its room 7.2 against 17.6. When v005 was blue, its far_cells averaged 1.4 against v004's 0.6, and its material change 22.2 against 12.5. The red-seat v005 falls behind in tempo around turn 10.

**Winner-conditioned rows.** The "won" slices for joints and empty_joints (v004 wins show more joints at five turns before the end, 87%; v005 wins show fewer, 28%) follow from which bot won, since v004 always has more joints. They are bot identity, not evidence of cause.

## How v005 differs from v004

At turn 10 (v004 / v005 means): total_cut 60.5 / 7.7, max_cut 23.0 / 3.3, empty_joints 5.4 / 1.6, joints 7.0 / 2.4. The shape term does what it aims at. Yet joint counts do not decide who wins, while actual cut-offs do: cut_off is 3.9 / 2.7 at turn 12, and v005 has fewer.

v005 expands less toward the enemy: far_cells 0.8 / 1.7 at turn 10 and room 8.9 / 14.9. Its economy is larger from about turn 12: material 296 / 279 at turn 12 and 432 / 408 at turn 20; gross income 33.3 / 31.4 at turn 12 and 44.2 / 41.9 at turn 20. It spends more on buildings at turn 8 (9.5 / 4.8), and v004 keeps 67.5% of its gold unspent at turn 8 against v005's 30.2%. Army and unit counts are close. Whether the shape term causes the economic lead is not shown here.

## How far to trust the metrics

- There are no reviews, so no judgments and no agreement figures.
- Length and winner are outcomes, so slices by them are post hoc. Slices by winner mix bot identity.
- The turn-1 "tells" in the short-game slice (economy metrics at 73%) and the leader-lost slice (hq_gap and reach at 77%) rest on the first turn and have no per-turn values in the report. Treat them as unverified.
- Ties count half. The one-turn-before figures are near-trivial because the winner is ahead, so they show outcome, not cause.
- Sample size falls with turn: 999 games at turn 10, 405 at turn 20. Later tells rest on fewer games.
- Overall v005 share is 56.7% (about 53.6% to 59.8%). Blue's share of wins is 60.5%. The long-game share of 49.7% has a margin near 5.4 points.

## Ideas for the next experiment

1. **A seat-aware opening.** Evidence: v005 wins 46.2% as red and 67.2% as blue. As red, its far_cells, room and material change at turn 10 are well below v004's (0.2 against 2.9, 7.2 against 17.6, 9.9 against 23.2). Try a red-only turn-1 plan that claims more ground toward the enemy HQ, and keep the blue plan. Test seat by seat so the blue result is not diluted.

2. **A shape term aimed at real cut-offs.** Evidence: total_cut, max_cut and joints never reach the threshold overall, and in the seat slices they show 45% to 62%. cut_off tells from turn 12 (71%) and v005 already has fewer. Try penalising cut-offs and exposure (exposed30, can_cut) instead of the joint count, or a lower joint weight, and watch far_cells and room so the expansion does not fall further.

3. **A late-game term for long games.** Evidence: v005's edge is gone in long games (49.7% over 332). There, inner_units, far_cells and can_cut lead five turns before the end, and towers, tower_cover and hq_guard lead one turn before. Room drops to 3.7 for v005 by turn 20. Try a late term, after about turn 15, that weighs can_cut and HQ-guard units when room runs out. This is the least certain idea: the long-game signals mix cause and effect, so read per-game reviews of long games before building it.
