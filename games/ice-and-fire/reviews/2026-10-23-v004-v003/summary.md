# Summary of the batch

ice-and-fire-v004 (candidate; its only change is the first-turn opening weights) against its parent v003, in 10 seat-swapped pairs at CodinGame's time limits (seed 151). All 20 games were decided, in 21.5 turns on average. Each bot won 10: v004 won 4 as red and 6 as blue, and v003 won 4 as red and 6 as blue. The blue seat won 12 of 20, too few games to mean much.

Games are cited by suffix: p0-0 is ice-and-fire-151-p0-0, and so on. Ply numbers follow the reviews.

## What decides these games

At the start of the middle game the winner's lead was only weakly ahead: material was higher for the winner in 13 of 20 games (mean +5.8). Five turns into the middle game the picture is clear: material was higher for the winner in 14 of 18 games (mean +128.4), army in 14 of 18 (+45.0), voronoi in 14 of 18 (+27.6), income in 13 of 18 (+7.9) and cells in 13 of 18 (+15.3). The HQ gap was lower for the winner in 15 of 18 (mean -30.8). At the start of the endgame the winner had less gold (higher in only 4 of 20, mean -8.7) and no more cut-off cells (higher in only 3 of 18, mean -5.5).

The decisive moments are cuts. The first large cut favoured the side that won in 16 of 19 games (49 moments). Lead changes (7 of 7) and HQ threats (18 of 18) also went that way, but an HQ threat is the finishing move, not the cause. Debt appeared in two games and favoured the winner once in two.

The recurring kinds of moment are these.

- **The ply 14 to 15 exchange.** In 10 games (p0-1, p1-1, p3-0, p3-1, p4-0, p4-1, p5-0, p5-1, p7-0, p7-1) reviewers judged red's ply 14 bad: red held 27 to 39 gold and either spent 20 to 30 on units that one cell could cut off, or kept the gold idle with a link open. Red won only 3 of those 10. Blue then made a large spend or push at ply 15 (30 to 47 gold) in 10 games and won 8 of them. In p5-1, red put 30 of 39 gold on three units behind (4,3) and (5,3). Blue trained four units at ply 15 with 46 gold, and red's lead went from +17 to -77.
- **One-cell links.** A group beyond a single link dies when an enemy level 1 trains onto the link, because cut units die at the next turn. In p1-0 at ply 19, blue's five trainings for 60 gold cut red's cells from 32 to 18. In p9-0 at ply 33, three trainings into the empty column (6,1) to (6,3) cut red's cells from 34 to 20. An empty link cell came before the cut in at least six games (p1-0, p1-1, p2-0, p3-0, p3-1, p9-0).
- **Upkeep.** A level 3 unit costs 20 upkeep. In p8-0 (v004 as red) and p8-1 (v003 as red), three level 3 units bought at ply 32 turned income from about +28 to about -29, and red went into debt at ply 39 or 40. Upkeep also cut income in p6-1 (red, ply 22: 22 to 4) and in p2-0 and p4-0 (blue, income 25 to 10 and 36 to 23).
- **The HQ chain.** Chains cost 60 to 200 gold, and the side that held the gold won the race (p6-0 red with 233 gold against a 190-gold chain; p8-1 blue with 200 gold at ply 41).

## Each bot, phase by phase

### v004 (candidate)

Early game:
- Good as red: the ply 10 mine (BUILD MINE 1 0), judged good in eight reviews (p0-0, p1-0, p2-0, p3-0, p5-0, p6-0, p8-0, p9-0).
- Bad as blue: no mine and 28 gold unspent at ply 13, flagged in p0-1, p7-1 and p8-1. Free owned spots were available. v004 built no mine in 8 of its 10 blue games (p0-1, p1-1, p3-1, p4-1, p5-1, p6-1, p7-1, p9-1). Its first came at ply 35 in p8-1, and at ply 11 in p2-1, which red captured at ply 24.

Middle game:
- Good as blue: cutting strikes at plies 15 to 23 (p1-1, p3-1, p4-1, p5-1, p9-1) and a 100-gold cut at ply 33 (p6-1).
- Bad as blue: spending into a reach it could not hold. In p7-1, 40 gold hung off one link at (5,4) at ply 15, leaving 1 gold at ply 19. In p9-1, four units went into red's reach at ply 15 and four died at ply 16. In p1-1, five units moved forward at ply 19 with 26 gold unspent.
- Good as red: retakes of cut cells and corridor trainings (p0-0 at ply 24, p1-0 at plies 14 and 16, p5-0 at plies 16 and 18, p7-0 at plies 16 and 18).
- Bad as red: units left on or beyond a link. In p1-0 at ply 18 the neck cell (3,5) was empty and a tower stood inside the group that was cut. In p3-0 the east link was open at ply 14, and at ply 16 red spent 50 of 54 gold into a dead end. In p4-0 at ply 16, 30 of 50 gold went into a pocket.

Endgame:
- Good as red: HQ chains paid from a bank (p0-0 at ply 30, p3-0 at ply 22, p6-0 at ply 42, p8-0 at ply 84) and a cut of 11 blue cells at ply 20 in p6-0.
- Bad as red: towers bought where cut cells needed reconnecting (p7-0 at plies 26 and 28, p4-0 at ply 20, p2-0 at ply 40), a WAIT with 34 gold (p1-0 at ply 36), and three level 3 units at ply 32 (p8-0).
- Good as blue: HQ chains (p1-1 at ply 29, p4-1 at plies 19 to 21, p5-1 at ply 25, p8-1 at ply 41).
- Bad as blue: spending to zero and leaving 9 cells cuttable (p6-1 at plies 75 to 81), and a level 1 on an empty cell within a level 2's reach (p2-1 at ply 21).

### v003 (parent)

Early game:
- Good as blue: expansion by moves and a training at plies 9 to 11 (p3-0, p4-0, p8-0, p9-0).
- Good as red: the ply 10 mine, judged good in six reviews (p0-1, p2-1, p3-1, p4-1, p5-1, p7-1). The one exception is p1-1.
- Bad as red: the ply 14 problem, flagged in p0-1, p1-1, p3-1, p4-1, p5-1 and p7-1.

Middle game:
- Good as blue: level 1 trainings through the enemy's corridor, cutting 5 to 15 cells (p1-0 at ply 19, p2-0 at plies 17, 23 and 27, p3-0 at plies 15 and 17, p5-0 at plies 15 to 21, p7-0 at plies 17, 25 and 27, p9-0 at plies 29 to 35).
- Bad as blue: moving units off the corridor (p0-0 at ply 23), a raid that left the gap at (9,9) open (p3-0 at ply 19), five units pushed into the centre band (p7-0 at ply 15), and holding 25 gold for a turn (p9-0 at ply 17).
- Good as red: strikes and level 2 retakes (p0-1 at ply 16, p2-1 at plies 22 and 32, p3-1 at ply 20, p7-1 at plies 16, 20 and 24, p6-1 at ply 58 with 140 gold).
- Bad as red: units placed in pockets joined by one or two link cells (p9-1 at plies 20 to 24), and a WAIT with 110 gold and can_cut 4 (p8-1 at ply 22).

Endgame:
- Good as blue: level 3 cuts on towers and HQ chains (p2-0 at plies 29 and 33, p4-0 at ply 23, p5-0 at plies 25 to 31, p9-0 at ply 41).
- Bad as blue: no mine until ply 35 to 41, or never (p1-0, p4-0, p5-0, p6-0, p7-0, p9-0). Level 3 purchases that drained upkeep (p4-0: upkeep 9 to 32; p2-0: upkeep 32, income 25 to 10; p8-0 at plies 61 and 81, which ended in debt at ply 82). Three mines for 45 gold at ply 37 (p7-0).
- Good as red: cuts at plies 72, 76 and 80 while blue had 0 to 11 gold (p6-1), and level 2 reconnections (p0-1 at plies 30 and 36, p2-1 at plies 32 and 44).
- Bad as red: three level 3 units at ply 32 (p8-1, which ended in debt at ply 40), and a level 3 at ply 22 that cut income from 22 to 4 (p6-1).

## How far to trust the reviews

- The judgments agree with the change in material at 78% for good plays (112 of 144) and 93% for bad plays (71 of 76). They are strongest in the middle game (bad 100%, good 84%) and the endgame (bad 91%, good 83%). The early game is the weakest: 64% for good plays (36) and 62% for bad plays (8), where the mean change is only +3.5. Twelve judgments of a side's last turn were left out.
- The material metric itself misleads in places. A debt wipe raises it: in p8-0 the lead read flat at ply 81 while blue's 12 units died. Summary-data reads upkeep as a sign of strength (80% at the endgame), but p8-0, p8-1, p2-0 and p2-1 show upkeep of 32 to 63 with negative income losing games. Read upkeep alongside income.
- The rules' flags often sit one ply after the play that caused them. In p5-0 the flagged swing at ply 22 was caused by blue's ply 21; red's ply 22 won back 108 of it. Similar cases appear in p0-0 (ply 26), p1-1 (ply 22), p6-0 (ply 22) and p7-0 (ply 22). The "first one's side won" figures therefore partly measure results.
- Contradictions the batch does not settle:
  - Idle gold at ply 13: judged bad in p0-1, p7-1 and p8-1, but good in p4-1 and p5-1, where the 28 banked gold paid for the 40-gold strike at ply 15.
  - The ply 10 mine: judged bad in p1-1 (the lead rose from +6 to +18 on the next turn), though the same reviewer notes the income gain lasted. Eight other reviews judged the same play good.
  - Blue's 40-gold spend at ply 15: judged bad in p7-1 and p9-1, where red's ply 16 level 2 counter cut the units, and good in p1-1, p3-1, p4-1 and p5-1, where the cut held. Verdicts follow the next turn, not the spend.
  - Towers: p2-0, p4-0, p5-1 and p9-0 treat a cell next to an enemy tower as enterable only by level 3. In p9-1 a level 1 stood next to red's tower at (3,0) at ply 32, and the reviewer says the file does not resolve it. The tower analysis in those games depends on this rule, so check it before relying on those verdicts.
- Fifteen of 20 reviews say the file shows no closing board and infer the HQ capture from the last commands.
- No review can judge the opening: the reviewers saw plays, not weights. All 20 describe the same opening (two level 1 trainings on turn 1, then a mine at ply 10 for red). This batch gives no evidence about the opening change.

## Ideas for the next experiment

Each needs the owner's approval before a run.

1. **Mines for the blue seat.** v004 built no mine in 8 of 10 blue games. Income five turns into the middle game predicted the winner in 13 of 18 games. Free owned spots were available at ply 13 in p0-1, p7-1 and p8-1. Change: when no mine stands and an owned spot is free, build one at 15 gold. The trade-off: p4-1 and p5-1 credit the banked 28 gold at ply 13 with the strike at ply 15 that won, so the experiment must keep the bank near 40 gold. v003 also rarely mined as blue (6 of 8 known), so the change may help both bots.
2. **Guard links before spending.** Red's ply 14 play was flagged in 10 games. Change: before a spend that puts units beyond a one-cell link, keep a unit on each empty link cell the enemy can reach with its next-turn gold (current gold plus income). Evidence: p3-0 (east link open at ply 14), p1-1 (cells (2,4), (2,5) and (3,5) empty at ply 20; 7 cells cut at ply 21) and p2-0 (links vacated at plies 16, 22 and 26, each cut).
3. **Upkeep check on purchases.** Change: refuse a purchase that leaves income below zero, unless it reconnects cut cells or pays for the HQ chain. Evidence: p8-0 (v004 as red, three level 3 units at ply 32, income +29 to -28, debt at ply 39), p8-1 (the same play by v003, debt at ply 40), and p6-1 (level 3 at ply 22, income 22 to 4 for ten turns).
4. **Reconnect before towers.** Change: when cut-off cells exist, spend on reconnecting them before towers or new front units. Evidence: p7-0 (v004 as red bought towers at plies 26 and 28 while 7 and 9 cells stayed cut, and lost), p5-0 (v004 as red spent 20 gold on a level 2 that reconnected nothing at ply 28) and p9-1 (v003 as red's level 2 at ply 24 did not restore the link).
