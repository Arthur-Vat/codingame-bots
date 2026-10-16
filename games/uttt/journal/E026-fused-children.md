---
id: E026
date: 2026-10-16
pull_request: "#145"
parent: uttt-v016
hypothesis: Evaluating a node's children from its own accumulators, each child's move rows added as the output layer reads them, gives exactly the same search more iterations per turn, so the bot is stronger at CodinGame's limits.
change: `Network::child_logit` and `evaluate_child` read the parent's accumulators plus the move's one or two rows per seat lane by lane, and `NnueBoard::child_scores` uses them, so a child's accumulators are no longer copied, updated and stored; the bot's settings and network are unchanged.
release: none
sprt: rejected
elo: -17.6 [-34.8, -0.4] at CodinGame's limits
pairs: 89
full_time: -17.6 [-34.8, -0.4] (the SPRT's own, ADR 0020), no timeout in 178 games
decision: dropped
cg_rank: not reported
---

Step D3 of [the roadmap](../ROADMAP.md#d-scale): ideas 3 and 4 of 2026-10-15, as the owner agreed on 2026-10-16 ("I agree with the plan, go on with the implementation").

**What was tried.**
- **Idea 3: one pass per child, without the copy.**
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

**Faster with Rust 1.97 only.** In the engine's speed example (20 positions of the bot's games, 20,000 iterations each), three runs alternating with the code before it, on Claude's session:
- before: 382,354 to 407,081 iterations per second;
- now: 460,930 to 492,240, about 18% more iterations in the same time.

The children's moves now cost only their rules, 0.5% to 5.8% of the time. Evaluating the children, their rows included, takes 41.7% to 46.5%, and the walk's updates 28.6% to 30.1%.

**Screening against `uttt-v016`** at CodinGame's limits (Claude's 4-core session, 4-ply openings, seed 1, 3 games at a time, 100 pairs): 44 wins, 115 draws, 41 losses, +5.2 Elo [-14.3, +24.8]. The candidate had 5 timeouts and `uttt-v016` 6, on a busy machine. As in E022, the same play at more iterations should gain a few Elo, which only an SPRT can show.

The bundle took 72,017 characters; the 1,561 more were the new functions (the bundle drops comments).

**SPRT** ([run 38873342751](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38873342751)), at CodinGame's limits, 4-ply openings, seed 1:

```
SPRT against uttt-v016:
uttt-v017 vs uttt-v016: 178 games, 89 complete pairs
  uttt-v017: 30 wins, 109 draws, 39 losses (score 47.5%)
  Elo of uttt-v017 over uttt-v016: -17.6 [-34.8, -0.4] (95%, from pairs)
  pairs by uttt-v017's points (0, 1/2, 1, 3/2, 2): [0, 15, 68, 6, 0]
  uttt-v017 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v016 faults: 0 timeouts, 0 crashes, 0 invalid answers
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR -2.95 (bounds -2.94, 2.94) after 89 pairs
Verdict: REJECTED: uttt-v017 is not stronger than uttt-v016
```

**Why.** The speed above was measured with this session's Rust, 1.97, while CI and CodinGame compile with 1.90.0, which installed in this session on 2026-10-16. Built with 1.90, six alternating runs of the speed example each:
- the code before E026 ran 397,038 to 476,475 iterations per second, about 423,000;
- E026's ran 406,004 to 478,928, about 444,000.

That is about 5% more, within the machine's noise. Rust 1.97 compiles today's loop slower than 1.90 does, so most of the 18% was the newer compiler's loss, not E026's gain. With the same moves and about 5% more iterations, the true gain is probably a few Elo at most. An SPRT for +10 rejects that, and its -17.6 is most likely chance; play is identical at fixed iterations, so the candidate cannot judge positions worse.

**Lesson.** Measure speed with Rust 1.90.0 (`cargo +1.90.0 run --release -p uttt-engine --example speed`), the version CodinGame compiles with. A newer compiler can make the baseline slower and a change look better than it is. The speed rows of the [README](../README.md#engine-speed) and E022's measurements were most likely taken with a newer compiler too: until this session, `CLAUDE.md` said sessions could not install 1.90.0.

The code stays in this pull request's commits (0d34da7 and c06f4a3): it is exact, and with Rust 1.90 not slower.
