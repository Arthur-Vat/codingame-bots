# Summary of the batch

v022 (ours) against zasmu, the top 1 of the ladder: 20 decided games (10 ranking, 2 test-type and 8 replay in the batch's naming; ranking games are on random maps, test games on fixed maps, replays on a ranking map with v022 as blue). v022 was red in 10 games and blue in 10. zasmu won all 20 (10 as red, 10 as blue) and v022 none. Games lasted 22.6 turns on average. v022 lost on turns 17 to 26 as red (mean 22.4) and turns 16 to 38 as blue (mean 22.8). Games are cited by the number in their file name.

## What decides these games

The metrics below are zasmu's (the winner's) values against v022's, from `summary-data.md`:

- Start of the middle game (20 games): zasmu was higher in total_cut (100%, mean +71.2), army (+20.5), material (+20.4), cells (+4.7) and income (+2.6). It kept less gold: kept_share was lower in 20 of 20 (mean -81.3). Mines were zero for both sides in every game.
- Five turns into the middle game: income was higher in 20 of 20 (+16.6). Mines were higher in 18 of 20 (equal in 2). Gold was lower in 19 of 20, and kept_share lower in 17 of 20.
- Start of the endgame: material higher in 20 of 20 (+191.1) and gross income in 20 of 20 (+18.2). hq_gap was lower in 17 of 20 (mean -34.6). This one follows from the win; it is not a cause.

The context from the brief, over all 150 games and not these 20: in v022's losses at turn 15 it has 27 to 30 active cells and income 23 to 24, against 45 to 47 and 39 to 44 for the opponents. At turn 10 the opponents had a mine in 70% to 90% of games, against 10% to 20% for v022.

Decisive moments recur in four kinds. An HQ threat is flagged in 18 games, and zasmu's side won all 18 moments. Swings occur in 15 games (31 moments, 73% for the favoured side). Lead changes occur in 7 games (10 moments, 86%). Large cuts occur in 3 games (4 moments, 67%). Three examples:

- A cut paid for in upkeep. In 921353008 at ply 26 a level 3 cut 4 cells, but income fell from 27 to 11. In 921452369 at ply 23 two level 3 units took income from 22 to -6 with 18 gold left. In 921371154 at plies 34 and 40, three level 3 units took income to -1.
- A strike on a thin spine. At ply 28 in 921352754 and 921451652, zasmu spent 140 gold on nine trainings and cut v022 from 36 active cells to 4. In both, v022 had kept 79 gold on ply 27.
- The HQ chain. Once hq_gap reached 0, zasmu spent 170 to 290 gold in one turn (921353008 at ply 47, 921367560 at ply 45, 921452289 at ply 74).

## Each bot, phase by phase

### v022

Early (plies 0 to 13):
- Good: two level 1 units on turn 1 (921367312, 921368697). On ply 13, a level 2 onto zasmu's pair at (5,6) and (5,7) (921363064, 921452139, 921452289).
- Bad: spending nothing with 10 to 20 gold on plies 8 to 12, while zasmu expanded (921365300, 921367312, 921368092, 921368697, all red). As blue, spending nothing with 13 gold on ply 11 (921452289), and keeping 18 of 28 gold on ply 13 (921452771).

Middle (plies 14 to 30):
- Bad, as red: gold idle while the centre is contested, 28 to 66 gold kept on plies 14 to 20 (921368697, 921371154, 921367312, 921367560).
- Bad, as red: mines late or never. The first mine came at ply 14 (921369551, 921371154), 18 (921368092), 26 (921358588), 30 (921367560), 32 (921353008), 38 (921367312), or never (921365300). zasmu had four mines by ply 27 in 921365300 and six by ply 28 in 921371154.
- Bad, as red: level 3 units with 20 upkeep each. Income fell from 22 to 5 (921365300, ply 18), from 25 to 10 (921369551, ply 22) and from 27 to 11 (921353008, ply 26).
- Bad, as blue: a level 3 at ply 23, paid with the whole economy: 921364673 and 921452369 (income 22 to -6), 921363064 and 921452289 (35 to 20).
- Bad, as blue: cut groups hanging on one link, cut back at once by zasmu. Ply 15 in 921352754, 921451652, 921451841 and 921366477; ply 17 in 921452289 (a strip).
- Good: retakes or cuts by training onto an enemy cell. As red, 50 gold of trainings at ply 20 (921358588). As blue, two level 2 trainings at ply 29 retook the line (921352754, 921451652).

Endgame (plies 31 on):
- Bad: idle gold at the end. 225 gold at ply 42 in 921368792, 72 to 126 at plies 36 to 40 in 921368092, 171 gold at ply 45 in 921452139, and 30 to 94 gold through ply 35 in 921452289.
- Bad: the HQ approach empty or the wrong towers. Towers on the HQ corner at (1,1) and (0,1)/(2,0) did not stop level 3 units (921368792 at ply 40, 921368092 at ply 46). No unit within two steps of the HQ (921364673 at ply 30, 921452289 at ply 74).
- Bad: level 3 units bought late, with 40 upkeep a turn for two of them (921368092 at ply 42, income to -9; 921368697 at ply 40, income 44 to 6; 921367560 at ply 38, income 24 to 5).

### zasmu

Early: two level 1 units onto the centre pair (5,6)/(6,6) or (6,5)/(7,5) at ply 12 (921352754, 921366477, 921451652, 921452771). Its own two units onto (5,6)/(5,7) at ply 12 were killed by v022's level 2 (921363064, 921452139, 921452289).

Middle: towers on the contact cells and mines on home spots by ply 14 to 22. Examples: a tower at (6,5) on plies 14 and 18 (921452369, 921352754), mines at (1,0) and (4,1) (921452369, 921364673, 921452139). It also trained level 2 units onto single links: at (6,6) on ply 16 (921366477), and at (5,8) on ply 20 (921452771).

Endgame: it spent most of its gold each turn until the middle game ended, then kept a reserve for a chain of 170 to 190 gold (921452369 and 921364673 at ply 30, 921366477 at ply 42). Its own level 3 strikes had costs too: upkeep rose to 49 and then 58 at ply 28 in 921352754 and 921451652, with income down to 8 and 11, and it recovered by spending the rest.

## How far to trust the reviews

`summary-data.md` says reviewers' judgments agree with the next-turn change of the lead in 97% of 114 bad plays and 94% of 107 good ones. Nine last-turn judgments were left out. That agreement is measured one turn ahead, so it misses later costs. In 921353008, the ply 26 cut was judged good, but the upkeep it caused cut income from 27 to 11 for five turns.

The reviews also disagree with each other:

- zasmu's level 3 on (4,6) at ply 21 in the game 921365300 is judged bad (lead 102 to 70, then recovered by ply 24). The same kind of play at ply 21 in 921367312 is judged good (lead +3 by ply 23, and blue's cut was "clearly in blue's favour").
- 921452289 judges v022's ply 43 chain as good, though it was 29 gold short. The link fell at ply 44 and the army died at ply 45.
- 921363064 calls the rules' ply 38 swing misleading: it counts the cost of trainings as material.
- 921368092 says the rules ended the game at ply 51, while it also says hq_gap was 0 at ply 49. When the chain was paid is not clear.
- In five reviews (921358588, 921369551, 921371154, 921452369, 921452771) the HQ capture is inferred, because the file has no board after the last ply.

## Ideas for the next experiment

1. Mines before towers and units. Build a mine on every free mine spot before any other spend from turn 7. In the red games v022 built its first mine at ply 14 or later, or never (921365300, 921353008, 921358588, 921367312, 921367560, 921368092, 921369551, 921371154), while zasmu had four to six by ply 27 (921365300, 921371154). As blue, two to three spots stayed idle for many turns (921452289, 921452369).
2. A gold cap during the middle game. End a turn with no more than about 30 gold while a free mine spot or an attackable contact cell is open. The idle gold recurs in all 20 games (kept_share lower for zasmu in 20 of 20 at the middle start, mean -81.3).
3. A level 3 gate on income. Buy a level 3 only when income after its 20 upkeep stays positive, or when it answers an enemy level 3 at a tower. Otherwise prefer level 2 at single links. The bad income drops follow level 3 buys in 8 of 10 red games and 4 of 10 blue games (921452369, 921371154, 921353008).
4. Guard the HQ approach and the links before a strike. Keep a level 2 or 3 within two steps of the HQ from about ply 30, and each turn compare the chain cost with the opponent's hq_gap. Towers on the HQ corner did not stop level 3 units (921368792, 921368092). Cut groups on a single link were lost in five blue games (921352754, 921451652, 921451841, 921366477, 921452289).
