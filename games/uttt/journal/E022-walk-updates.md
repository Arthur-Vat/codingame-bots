---
id: E022
date: 2026-10-15
pull_request: "#136"
parent: uttt-v014
hypothesis: Updating the NNUE's accumulators with each move as the search walks down its tree, instead of building them from scratch at every node it expands, gives exactly the same search more iterations per turn, so the bot is stronger at CodinGame's limits.
change: `NnueBoard` carries its accumulators, built once at the root (`NnueBoard::new`), and `Game::play` adds the move's rows (`Network::play`), so `child_scores` starts from them instead of `Network::refresh`; the bot's settings are unchanged.
release: uttt-v015
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Step D3, idea 2, of [the roadmap](../ROADMAP.md#d-scale). The owner asked for ideas 1 and 2 of D3 on 2026-10-15 ("Read the handoff and do D3's ideas 1 and 2"). Idea 1, the measurement, is #135: in the bot's search, building accumulators from scratch took at least 21% to 25% of the time.

Two ways were open: updating the accumulators move by move as the search walks down, or keeping checkpoints at nodes with many visits (the write-up's way). Move by move came first because it is simple and exact, and it gained enough for one experiment; checkpoints could still save part of the walk's updates later.

**Exact.** Accumulators are sums of integer rows. A move only turns inputs on, and every partial sum stays within `i16`, so updating them move by move gives exactly what `Network::refresh` builds. The search's scores are bit for bit the same, and it plays the same moves:

- the engine test `nnue_positions_follow_the_moves_exactly` compares, after every move of 30 random games, the carried accumulators with `refresh`, and the position with a plain `Board` given the same moves (board, side to move, score and legal moves);
- in the speed example, before the old code path was removed, a copy of the old `child_scores` (accumulators from scratch) and the new `NnueBoard` returned identical `SearchResult`s on 19 positions of the bot's games;
- in the arena, at a fixed 20,000 iterations per answer (seed 7, 4-ply openings, 50 pairs), the candidate against `uttt-v014` played the same 100 games, 5,399 turns, as `uttt-v014` against itself: the lines of every answer, the winners and the ends were identical. As a control, `uttt-v013` in the candidate's seat differed in all 20 games at 2,000 iterations.

**Faster.** On those 19 searches (276,349 iterations of the bot's search), the new code took 0.698 to 0.740 s where the old took 0.943 to 0.952 s, in three runs that each timed both: 22% to 27% less time. The speed example's rate went from about 305,000 to about 381,000 iterations per second. At CodinGame's limits, over 20 games played one at a time (seed 11), the candidate ran these iterations per turn against `uttt-v014`'s, by turn of the bot (medians): turns 1 to 4, 29,800 against 26,900 (+11%); 5 to 9, 29,600 against 24,700 (+20%); 10 to 14, 29,900 against 26,200 (+14%). Later turns are mostly proven positions, which stop early. Deeper trees at full time make the walk longer, so the gain per turn is smaller than in the speed example.

After the change, the speed example shares the time as: updating the accumulators on the walk, with the rules, about 20%; copying them for each child 3% to 4%; playing each child's move 19%; evaluating the children 36% to 39%; the rest 18%.

Screening against `uttt-v014` at CodinGame's limits (4-ply openings, seed 1, 3 games at a time on Claude's 4-core session): 50 pairs, 15 wins, 72 draws, 13 losses, +6.9 Elo [-21.9, +35.9]. Both bots timed out 4 times in 100 games, while a review agent was using the machine; one game at a time later, the candidate timed out once and `uttt-v014` three times in 20 games, with 99% of both bots' answers within about 99 ms: this machine's noise, not the candidate's. A gain of this size, a few Elo, is below what 50 pairs can see, so the SPRT judges it.

The bundle takes 70,456 characters (268 more than `uttt-v014`), leaving 29,544.

The SPRT's result goes here.
