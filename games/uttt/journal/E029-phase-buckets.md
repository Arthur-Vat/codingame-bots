---
id: E029
date: 2026-10-16
pull_request: "#149"
parent: uttt-v016
hypothesis: Output buckets by game phase as well as by target (30 instead of 10) let the network weigh the same accumulators differently in the opening, the middle game and the end, so it judges positions better at almost no cost in speed, and the bot is stronger.
change: None released. The NNUE's output bucket became the target plus 10 times the game's phase, by the number of closed small boards (0, 1 or 2, 3 or more), and the MCTS bot searched as `uttt-v016` does with the network of Train run 38883498642 (width 256, 30 buckets, 8 epochs on run 38799715918's data).
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Proposed to the owner on 2026-10-16, after E027, as a cheap way for a network to be "better in some contexts" (his question): the error by ply shows the end of the game far easier to judge than the middle (squared error 0.051 after ply 32 against 0.077 to 0.079 between plies 8 and 31). He asked for it the same day: "Also can't you try PUCT, output buckets by game phase now?"

**The change.** `uttt_engine::nnue` gained `PHASES = 3` and `phase(board)`: 0 while no small board is closed, 1 with one or two, 2 with three or more. In 300 self-play games of `uttt-v016`'s search at 2,000 iterations (6 random opening plies), these held 35%, 25% and 40% of the positions. The bucket became `target + 10 · phase`. The byte layout got a new magic (`UTTTNN02`), and networks of the first version were still read, each bucket serving all three phases, with the same evaluations (tested). The trainer needed only its symmetries to keep the phase. The output layer grows from 5,130 to 15,390 parameters: the packed network takes 32,512 characters instead of 27,264, and an evaluation costs the same, apart from counting the closed boards.

**The fit** ([run 38883498642](../training/runs/38883498642/report.md)) is fit-only on E024's data (run 38799715918, the same split and copies dropped), with `uttt-v016`'s settings: 8 epochs, target result. Compared with `uttt-v016`'s network on the same held-out positions:

| | Phase buckets (30) | `uttt-v016`'s (10) |
| --- | --- | --- |
| Squared error, all 4.8M held-out positions | 0.0653 | 0.0652 |
| Cross-entropy, nats | 0.6180 | 0.6176 |
| Move agreement, 49,860 positions spread evenly | 45.9% | 45.9% |
| Squared error after ply 32 | 0.0510 | 0.0509 |
| Calibration gap | 0.0105 | 0.0110 |

No better anywhere: by ply, the two networks differ by at most 0.0006 in error and 0.8 points in agreement, either way.

**Screening against `uttt-v016`** at CodinGame's limits (Claude's 4-core session, both bots compiled with Rust 1.90.0, 4-ply openings, seed 1, 3 games at a time, 100 pairs): 50 wins, 92 draws, 58 losses, -13.9 Elo [-45.2, +17.2]. Each bot had 3 timeouts; 99.9% of answers came within 95.5 ms for the candidate and 94.1 ms for `uttt-v016`. The bundle took 76,321 characters.

**Decision.** Dropped without an SPRT: the network predicts exactly as well as `uttt-v016`'s, and the screen shows no gain.

**What follows.** The first layer already sees the closed boards (as won or drawn), so the accumulators carry the game's phase, and one output layer reads it well enough: splitting the output by phase adds weights without adding knowledge. Capacity for "contexts" would have to come from the first layer, which is the width question (E025, E027), and that waits for more data. The engine and trainer changes are in this pull request's first commit (3b57788), the screened bot in the second (a0b89a7), and both are reverted in the third, which holds this record; the network stays on the run's branch `claude/train/38883498642` until it is pruned.
