# A Code of Ice and Fire: what decides games between our bots

A qualitative study of games between `ice-and-fire-v004` and `ice-and-fire-v003`, asked by the owner on 2026-10-22: "simulate maybe 100 games ... try to interpret what is going on, why the bots seem to be playing good or bad ... these qualitative assessments could help defining what a good or bad early game is, and then could help benchmark best early games strategies". He chose text boards on about 30 maps (form, 2026-10-22).

## Method

- **Games:** 100 seat-swapped pairs of v004 against v003 at CodinGame's limits (arena, seed 121; v004 55.0%, +34.9 Elo [−10.9, +81.9]). Of the 100 maps, 28 were won by v004 from both seats and 18 by v003 from both: on those 46 maps the play decided the game, not the seat. Those 92 games are the sample; 88 of them were replayed without error.
- **Tool:** `cargo run --release -p ice-and-fire-arena --example boards -- GAME.json [--past N] [--every K]` replays a record and prints, at each red start of turn, both sides' gold, income, active cells, units, mines, Voronoi share, and what the enemy's best cut would take with 30 and with 50 gold, then the board and both sides' commands.
- **Reading:** boards of 15 games on 13 maps (p0, p3, p11, p15, p29, p30, p40, p43, p51, p59, p61; 8 lost by v004, 7 won), turn by turn around the opening's end and around the decisive event, both seats; the numbers of every turn for all 88 games.

## What the games show

1. **The opening ends level.** At game turns 7 to 9, the end of the opening, games v004 went on to win and games it went on to lose look the same, seat by seat: gold, income, cells, units, mines, Voronoi share, and what a cut of 30 or 50 gold could take from each side all agree within noise (v004 as red at turn 7: +6.9 and +7.1 gold over v003, −10.7 and −9.6 Voronoi cells, in wins and losses). The difference appears between turns 9 and 11: in lost games v004 (red) is 13.9 cells, 4.8 units and 30.5 Voronoi cells behind by turn 11; in won games it is level.
2. **A game is decided by one cut.** 69 of the 88 games turn on a single turn in which one side loses at least 30% of its territory (and 6 cells or more), mostly between turns 11 and 20. At the start of the turn before it, the side that is cut has 21 gold on average, the side that cuts 31.
3. **What is cut is a neck.** Both bots grow territory as one-cell-wide lines and "fingers" from the HQ, and later as large regions joined to the HQ through a corridor one or two cells wide, often along a map edge. The cut is two to four level 1 trainings across that corridor; everything beyond it goes inactive and its units die. Examples: on map p15, v004 (blue) held 32 cells hanging from column 10 and lost 28 of them at turn 15 to three trainings by its own HQ; on p51 a north-east region of 28 cells went at once; on p29 and p61 (v004 red) two fingers along rows 1 and 3, rooted at column 1, were cut at turns 9 to 11. The winners did the same to the loser's necks (p40, p59, p43).
4. **The neck forms after the opening.** The fingers of the first turns are short; the decisive necks grow between turns 10 and 18, while the plan search extends the territory. Its evaluation pays 4 gold a cell and guards only against the replies the opponent can pay now (its chain, its best cuts with its current gold, bronze's turn), so a region that a cut of the opponent's gold two or three turns later would sever looks safe.
5. **Other ways to lose.** Two of the losses without a single cut (p3, p82) were frozen fronts: Voronoi shares fixed for ten turns, both sides banking, v004's income 15 to 17 against v003's 27 to 33 because of level 2 units (upkeep 4) and towers on the front; v003 banked over 200 gold and won the late exchanges. On a cramped map (p11), v004's three units bunched at the front and left its own side empty; v003 walled the front and flowed around into v004's half (Voronoi 45 to 25 in six turns). Both bots buy about the same per game: 22 level 1, 3.5 level 2 and 0.9 level 3 trainings, 1.7 towers, 1.3 mines.

The neck on map p15 (v004 blue, `x`; v003 red, `o`), at the start of turn 14 and after v003's turn 14 (`1` to `3` and `A` to `C` are units, `;` and `:` cells cut off from their HQ):

```
turn 14: v003 gold 44, v004 gold 3        turn 15: v004 left with 4 cells
Hmo####Axxx#                              Hmo####::::#
.oo##AxB..x.                              .oo##:::..:.
.ooooo.;..x.                              .ooooo.o..:.
.o...2x;:.x.                              .o...o2o:.:.
.moo:ox;;xx.                              .moo:o:oo::.
.oooooxAxxx.                              .ooooo:::::.
.oooooA;;Ax.                              .ooooo:oo::.
.1ooooo:..x.                              1oooooo1..:.
.....111xxx.                              ....1ooo11:.
.........xx.                              ......1..11.
.......##xxx                              .......##xxx
#....####..Q                              #....####..Q
```

## What it means for the early game

- **No score of the end of the opening can separate these games**: the positions are equivalent by every measure tried. That is why the turn-1 experiments of E005 found nothing: more time or another weighting of the same position cannot see a difference that is not there. Against v004 at CodinGame's limits (seed 111, 200 pairs each): hill climbing on the opening's weights to 300 ms −3.5 Elo [−26.6, +19.6]; the economy counted at 1 turn −27.9, at 5 turns −1.7; a Voronoi cell worth 0 gold −5.2, 3 gold −24.4; gold counted twice −36.6; the choice judged after 2 turns of simulated middle game −36.6 [−70.2, −3.7].
- **What the opening can still change is the shape it hands over**: whether the territory is a set of thin lines with single roots, or a compact block whose roots are wide and covered by units. A compact opening is under test (a weight that favours free cells touching our territory; results in E005).
- **The larger gain is in the early middle game**: the decisive necks grow there, under the plan search.

## Proposed heuristics, for the owner's decision

For "a good position after the early game", in the order the games rank them:

1. **No neck.** The territory a cut of a standard budget could sever (the cut finder with 30 gold, about one turn of income at the middle game, whatever gold the enemy holds now) should be small; equivalently, the joints between our HQ and our regions should be at least two cells wide or covered by our units.
2. **Gold in hand when the territories meet**, relative to the enemy's: the side that cuts first has about 10 more.
3. **Income kept**: no level 2 or 3 units or towers unless a cut or a chain needs them.
4. **Our own side claimed**: the free cells nearer us taken, units spread rather than bunched at the front.
5. Then the Voronoi share and the economy, as in the current score.

Heuristic 1 applies at least as much to the plan search of the middle game (its evaluation and its replies) as to the opening.
