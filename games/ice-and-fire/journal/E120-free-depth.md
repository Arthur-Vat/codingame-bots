---
id: E120
date: 2026-11-28
pull_request: "#NN"
parent: ice-and-fire-v034
hypothesis: About half of the middle game's turns end before their time, at the cap of climbs or out of first actions to start from; using that time to re-rank the two best plans by a whole turn search of ours after each of their two worst replies looks one turn further at no cost to today's search, and beats v035's weights alone (E116) and ice-and-fire-v034.
change: After today's search stops on its own with time left, its two best plans, today's choice first, are each searched again after their two worst replies by a whole turn search of ours (with its own replies from the opponent) in the time left, and the plan with the better worst leaf is played; a plan whose leaves cannot all be searched is not compared. Released with E116's weights (ADR 0062).
release: ice-and-fire-v035
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner asked (2026-11-28) for "a much stronger offline bot to learn
from". Three measures led here:

- **More time does not make our search much stronger.** With 10 times the
  time and caps it beat v034 by +23.3 [+12.4, +34.1], with 3 times +17.6,
  and 1,000 climbs at CodinGame's time −0.9 (screening run
  [46158259035](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46158259035),
  1,600 pairs each). Wider search runs out of first actions to start from
  (40 of 54 turns in a local game at 10 times), not of time.
- **Depth does.** Re-ranking the 4 or 8 best plans by a whole turn search
  of ours after each of the opponent's replies (four half-turns instead of
  two, about 1 s a turn, offline) beat v034 by +70.9 [+56.4, +85.7] and
  +92.9 [+77.0, +109.2]; 44 times wider search at the same time, +16.5
  (run [46163041336](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46163041336),
  800 pairs each). E018 and E065 had looked one turn further with our next
  turn as fixed answers or as a climb against no reply, and gained nothing;
  the difference is a next turn that defends against the opponent's
  replies.
- **Depth must not take time from today's search.** Cutting today's search
  to 30 to 50% of the turn to make room for a deep step lost: 50% with 3
  candidates and climb leaves +5.4 [−4.6, +15.5], 40% −10.0, 30% −44.5,
  50% with whole-search leaves −13.0 (run
  [46176208077](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46176208077),
  1,600 pairs each, at CodinGame's real limits).

So the deep step here runs only in the time today's search leaves unused.
On 4 local pairs, 53% of middle-game turns had such time (median 8.6 ms at
time scale 0.548, about 16 ms at CodinGame's speed; 90th percentile 21.6),
and the deep step changed the move on 46% of them.

## The change

On the branch `claude/ice-and-fire-e120-free-depth`, which carries the
switches of the three measures above (all off) and E116's weights:
`LEAN_DEPTH` in `bots/plan/src/search.rs` is
`Lean { share: 100, candidates: 2, replies: 2, leaf: Leaf::Think, free: true }`.
Today's search runs with its limits unchanged; when it stops before them,
`lean_step` takes today's choice and the next best plan, ranks each one's
replies (the memory's and its own) by a fresh evaluation after the reply,
and searches our next turn after the two worst with `think` (deep step
off), the time left divided by the leaves left. A leaf with less than
`MIN_THINK_US` (1 ms at CodinGame's speed) is skipped and its plan not
compared. The plan played is today's unless the other one's worst leaf
scores strictly better. The bot answers by its usual deadline.

With the switch off, the bot's answers are those of E116's
`ice-and-fire-v035` (`judge --tries 600` on 2,031 positions, byte for
byte).

## Screening

Against E116's `ice-and-fire-v035` (the weights alone), at CodinGame's
real limits:

| Run | Variant | Candidates × replies, leaf | Pairs | Elo over E116's v035 [95%] |
| --- | --- | --- | ---: | ---: |
| [46180507731](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46180507731) | `control` | off | 1,600 | +2.6 [−3.3, +8.5] |
| | `f2t` | 2 × 2, whole search | 1,600 | +8.9 [−0.7, +18.5] |
| | `f3t` | 3 × 3, whole search | 1,600 | +5.2 [−4.9, +15.3] |
| | `f2r3t` | 2 × 3, whole search | 1,600 | +8.9 [−0.3, +18.2] |
| | `fx` | 3 × 2, whole search or a climb when short | 1,600 | +9.6 [−0.0, +19.2] |
| [46182014587](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46182014587), seed 1201 | `control` | off | 3,200 | +1.8 [−2.2, +5.9] |
| | `fx` | | 3,200 | +3.5 [−3.4, +10.3] |
| | `f2t` | | 3,200 | +5.1 [−1.6, +11.9] |

Commits: `control` `03fee7f`, `f2t` `107b19b`, `f3t` `18e9b41`, `f2r3t`
`64c40cf`, `fx` `a7f13f1`. Faults: one timeout of `f2t` and one of v035
in the second run's 6,400 games each, within the 1% the SPRT allows.

Every deep variant was ahead in both runs; over both, `f2t` averages
about +6 over 4,800 pairs, the controls about +2. The owner chose
(2026-11-28, "Combine weights + f2t") to release it with E116's weights as
one candidate (ADR 0062), the SPRT judging the sum against v034.

## SPRT

Pending.
