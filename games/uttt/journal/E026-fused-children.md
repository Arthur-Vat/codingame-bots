---
id: E026
date: 2026-10-16
pull_request: "#145"
parent: uttt-v016
hypothesis: Evaluating a node's children from its own accumulators, each child's move rows added as the output layer reads them, gives exactly the same search more iterations per turn, so the bot is stronger at CodinGame's limits.
change: `Network::child_logit` and `evaluate_child` read the parent's accumulators plus the move's one or two rows per seat lane by lane, and `NnueBoard::child_scores` uses them, so a child's accumulators are no longer copied, updated and stored; the bot's settings and network are unchanged.
release: uttt-v017
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Step D3 of [the roadmap](../ROADMAP.md#d-scale): ideas 3 and 4 of 2026-10-15, as the owner agreed on 2026-10-16 ("I agree with the plan, go on with the implementation").

**What was tried.**
- **Idea 3, kept: one pass per child, without the copy.**
  - Before, each child copied the parent's 1 KB of accumulators, added its move's rows, stored them, and the output layer read them back.
  - Now `squared_dot_1` and `squared_dot_2` read the parent's lanes, add the move's one or two rows, clamp, and multiply in the same loop.
  - A child whose position is over, or whose side to move has a move that wins the game, now adds no rows at all, since it is not evaluated.
- **Idea 4, dropped: a loop shaped for SSE2's `pmaddwd`.**
  - Lanes written by eight, in pairs summed into four `i32`s, made the compiler shuffle even and odd lanes apart: eight shuffles per eight lanes on half-empty registers. In a scratch benchmark of the output layer alone (256 lanes, basic x86-64), it took about 120 to 150 ns against 50 to 60 ns for today's loop.
  - Four other plain-Rust shapes were no faster than today's loop either.
  - SSE2's intrinsics (`_mm_madd_epi16`) would do it in about 7 instructions per eight lanes instead of 11. But calling them needs an `unsafe` block even though SSE2 is always on for x86-64, so they would need an ADR. Today's loop is kept, and the fused loop has its shape.

**Exact.** The integer sums are the same, so every evaluation is the same bit for bit:
- **Engine tests:** `children_evaluated_from_the_parent_equal_their_own_evaluation` compares `child_logit` and `evaluate_child` with `logit` and `evaluate` of the accumulator `play` makes. It covers every legal move of every position of 100 random games, at widths 16, 256 and 12 (lanes beyond the last group of eight). The extreme network's test also evaluates every child.
- **Arena, at a fixed 20,000 iterations per answer** (seed 7, 4-ply openings, 50 pairs, time limits scaled by 30 so that no answer timed out): the candidate against `uttt-v016` played the same 100 games, 5,419 turns, as `uttt-v016` against itself. The lines of every answer, the winners and the ends were identical.
- **Control:** `uttt-v015` in the candidate's seat differed in all 20 games at 2,000 iterations.

**Faster.** In the engine's speed example (20 positions of the bot's games, 20,000 iterations each), three runs alternating with the code before it, on Claude's session:
- before: 382,354 to 407,081 iterations per second;
- now: 460,930 to 492,240, about 18% more iterations in the same time.

The children's moves now cost only their rules, 0.5% to 5.8% of the time. Evaluating the children, their rows included, takes 41.7% to 46.5%, and the walk's updates 28.6% to 30.1%. The [README](../README.md#engine-speed) has the rows.

**Screening against `uttt-v016`** at CodinGame's limits (Claude's 4-core session, 4-ply openings, seed 1, 3 games at a time, 100 pairs): 44 wins, 115 draws, 41 losses, +5.2 Elo [-14.3, +24.8]. The candidate had 5 timeouts and `uttt-v016` 6, on a busy machine. As in E022, the same play at more iterations should gain a few Elo, which only an SPRT can show.

The bundle takes 72,017 characters. The 1,561 more are the new functions and their comments, which the bundle keeps.
