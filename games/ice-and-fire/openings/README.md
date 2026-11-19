# Opening suites

Step 4 of phase 0 of the early-game rework ([research/codingame/2026-11-19-early-game.md](../research/codingame/2026-11-19-early-game.md)): the top players' positions of game turns 5 to 10 and 12 on CodinGame, so that a bot can be played from the same map, seat and opponent and its early game compared with theirs. The judge of the plan's phase 2 is the cut arena ([research/codingame/2026-11-19-zasmu-likeness.md](../research/codingame/2026-11-19-zasmu-likeness.md)); a suite says, beside it, how far a planner is from the top players' own positions.

## The suite

[`suite-v1.tsv`](suite-v1.tsv) holds 3,668 games: every game of zasmu and of the players ranked 2 to 7 on 2026-11-19 other than us (Pduhard-, karliso, _iaz, reCurse, _iahelka) against one of our releases that reached game turn 12, from the data branch `claude/codingame-games` at 5788694. Each line gives the game, its kind, the map's seed (as our generator takes it), the player's seat, the player, the release it met, then the player's measures at the start of its plies of game turns 5, 6, 7, 8, 9, 10 and 12 (active cells, Voronoi share, share of the middle chokes, units, gold; [`early/src/score.rs`](../early/src/score.rs)), and its zasmu-likeness at turn 12.

By seat (red / blue): zasmu 44 / 554 games, Pduhard- 60 / 563, karliso 51 / 559, _iaz 50 / 553, reCurse 58 / 571, _iahelka 63 / 542. Most are test games (3,017), on 50 maps (the ten of [`codingame-seeds.txt`](../codingame-seeds.txt) among them, 1,140 games), which seat us red, so blue is the players' usual seat. The games make 809 distinct starts (map, seat and release met): 651 from ranking games, 158 from test games. The suite is frozen: a new suite is a new file, never an edit.

## Running it

```sh
cargo run --release -p ice-and-fire-review -- early-suite --data DATA --me Tarleya --players zasmu,Pduhard-,karliso,_iaz,reCurse,_iahelka [--comment TEXT] > suite.tsv
cargo run --release -p ice-and-fire-review -- early-openings --suite games/ice-and-fire/openings/suite-v1.tsv --bot NAME=BINARY --opponent ice-and-fire-v014=BINARY [--opponent ...] [--time-scale S] [--jobs 4] > openings.md
```

`early-openings` plays the bot in the player's seat on the start's map against the release the player met (a compiled release for each, named as the suite names it; starts without one are left out), up to the start of the bot's ply of game turn 12, once per start. It prints, for each player and seat, the player's mean beside the bot's at turns 5, 8, 10 and 12, each start weighing once, the zasmu-likeness at turn 12, and the share of starts where the bot holds as many active cells as the player at turn 10. The opponent is the same release, not the same moves: it answers what the bot does, as it answered the player. The suite's releases are compiled from `releases/` and, for the probe `ice-and-fire-e069-k1`, from `probes/`.

## v029 from the same starts

`ice-and-fire-v029`, all 809 starts, time scale 0.487 and its releases compiled with `rustc -C opt-level=3` (162 s on 4 CPUs). The player's mean / v029's mean:

| Player | Seat | Starts | Active cells, turn 10 | Active cells, turn 12 | Middle chokes, turn 10 | Voronoi, turn 10 | Gold, turn 12 | Zasmu-likeness, turn 12 |
| --- | --- | ---: | --- | --- | --- | --- | --- | --- |
| zasmu | red | 44 | 30.0 / 28.2 | 36.6 / 32.9 | 0.42 / 0.24 | 0.52 / 0.44 | 39 / 54 | +0.83 / −1.35 |
| zasmu | blue | 210 | 33.9 / 32.1 | 41.1 / 38.5 | 0.40 / 0.31 | 0.57 / 0.53 | 48 / 59 | +0.65 / −0.17 |
| karliso | red | 51 | 30.9 / 27.6 | 36.3 / 32.3 | 0.39 / 0.22 | 0.50 / 0.43 | 47 / 56 | +0.38 / −1.78 |
| karliso | blue | 210 | 34.4 / 32.1 | 39.0 / 38.4 | 0.38 / 0.30 | 0.58 / 0.53 | 52 / 59 | +0.06 / −0.29 |
| _iaz | red | 50 | 29.8 / 28.7 | 36.5 / 33.1 | 0.37 / 0.26 | 0.54 / 0.45 | 39 / 54 | −0.22 / −1.12 |
| _iaz | blue | 201 | 31.6 / 31.9 | 38.5 / 38.2 | 0.35 / 0.33 | 0.58 / 0.53 | 41 / 62 | −0.86 / −0.29 |
| Pduhard- | red | 60 | 30.3 / 28.5 | 35.9 / 33.3 | 0.33 / 0.24 | 0.52 / 0.45 | 48 / 52 | −1.29 / −0.98 |
| Pduhard- | blue | 210 | 32.8 / 32.2 | 38.0 / 38.3 | 0.41 / 0.32 | 0.58 / 0.53 | 62 / 60 | −1.64 / −0.34 |
| reCurse | red | 58 | 29.6 / 28.8 | 32.6 / 33.7 | 0.25 / 0.22 | 0.51 / 0.44 | 48 / 54 | −0.83 / −1.01 |
| reCurse | blue | 219 | 31.1 / 32.0 | 34.4 / 38.3 | 0.30 / 0.32 | 0.56 / 0.53 | 58 / 60 | −0.88 / −0.22 |
| _iahelka | red | 63 | 28.8 / 28.8 | 32.3 / 34.4 | 0.22 / 0.19 | 0.48 / 0.44 | 46 / 55 | −1.23 / −1.02 |
| _iahelka | blue | 208 | 32.4 / 31.8 | 39.7 / 37.8 | 0.25 / 0.32 | 0.52 / 0.53 | 50 / 59 | −0.80 / −0.26 |

From zasmu's own starts, v029 holds about 2 active cells fewer at turn 10 and 2.6 (blue) to 3.7 (red) fewer at turn 12, a quarter (blue) to almost half (red) fewer of the middle chokes, a Voronoi share 0.04 (blue) to 0.08 (red) lower, and keeps 11 (blue) to 14 (red) more gold. It holds as many active cells as zasmu at turn 10 on 37% of zasmu's blue starts and 34% of its red ones (the last table of the tool's report, not shown above). The gap is widest as red, where the suite is thinnest (44 starts). Against the players ranked 2 to 7 the picture is mixed: v029 is level with or ahead of reCurse in active cells by turn 12, and of _iahelka as red, but behind karliso, and as red behind all of them in Voronoi share and middle chokes. This is the plan's target in figures: from these starts, the planner of phase 2 should reach zasmu's columns.
