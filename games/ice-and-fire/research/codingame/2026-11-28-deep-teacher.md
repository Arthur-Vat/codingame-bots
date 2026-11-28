# A deeper teacher: our next turn in the search (2026-11-28)

The owner asked (2026-11-28) for "a much stronger offline bot to learn
from". E117 measured the simplest teacher, our search given more time and
caps, against `ice-and-fire-v034` at 1,600 pairs each (screening run
[46158259035](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46158259035)):

| Variant | What it changes | Elo over v034 [95%] |
| --- | --- | ---: |
| `control` | nothing | −2.2 [−7.9, +3.5] |
| `c1000` | 1,000 climbs at CodinGame's time | −0.9 [−8.5, +6.7] |
| `tk3` | 3 times the time, the climbs, the reply tries, the reply memory and the reply cuts; the opponent's cuts one cell longer | +17.6 [+7.3, +27.9] |
| `tk10` | the same, 10 times; cuts two cells longer | +23.3 [+12.4, +34.1] |

Ten times the search buys about 23 Elo. tk10 answers in 43 to 75 ms on its
median turn (v034 about 20): it runs out of first actions to start from
(40 of 54 turns in a local game) rather than of time. More of the same
search, wider, does not make a teacher. It is also no use to the release
(`c1000`).

The search is one turn deep: our plan, the opponent's replies, then the
evaluation. What our next turn can do after a reply, a counter-cut, a
recapture, a chain that the reply opened, is left to the evaluation's
terms. Depth has never been measured. The owner chose (2026-11-28, "Design a
deeper teacher") this note, for his review before any code.

## The design

A switch in the plan bot, off on main and in every release, that changes
only how the turn's candidate plans are compared:

1. **Today's search, as now.** `think` climbs, keeps its plans and their
   replies (the shared memory and each plan's own), and scores them.
2. **The candidates.** The `N` best plans by today's final score (`N` from
   4 to 8), and always the one today's search would play.
3. **Our next turn at each leaf.** For each candidate and each of its
   replies (the memory's and its own, at most about 10), the state after
   the reply is already the start of our next turn (`after_reply` ends both
   turns, income and upkeep included). From it, a whole turn search of
   ours, `think` itself with a smaller budget (its climbs, its replies from
   the opponent, its final score), gives the leaf's value: the score of the
   plan our next turn would play. A finished game is `±WIN` as now.
4. **The deep score.** A candidate's score is the worst of its leaves (the
   opponent picks the reply), and the bot plays the best candidate; a tie
   keeps today's choice.
5. **Budget.** A teacher's turn gets a fixed time, divided evenly over the
   leaves: with `N` = 8 and 10 replies, 80 next-turn searches of 20 to
   50 ms each, 2 to 4 seconds a turn. Offline only; the arena gives the
   room with the screening's `extra_tolerance_ms` (E117).

This is four half-turns (ours, theirs, ours, theirs) where today's search
has two, at the same evaluation. The candidates come from today's search,
so depth only re-ranks what width finds: the measure is whether looking
one turn further chooses better.

## The measures

| Step | What | Check that can stop the work |
| --- | --- | --- |
| 1 | The switch and the leaf search, at a fixed number of tries (`CG_FIXED_ITERS`) so that games replay | Identical answers with the switch off (`judge`, byte for byte); how often the deep choice differs from today's |
| 2 | Screening against v034: `d4` and `d8` (`N` = 4 and 8), and tk10 again beside them at about the same time per turn | Depth beats width at the same time, and the deep teacher beats v034 by much more than +23 |
| 3 | If it does: the teacher's games and scores as data (E116's fit on teacher self-play, its scores as targets, its moves at game turns 9 to 14) | Each use is its own experiment, screened as usual |

Step 2's cost: a teacher's turn is 100 to 200 times a v034 turn, so a game
takes 2 to 4 minutes where E117's control took about 8 seconds. 800 pairs
a variant over 20 runners, four games at a time on each, take about 1 to 3
hours and measure to about ±10 Elo.

## Risks

- **The same evaluation at the leaves.** Depth helps where a turn's result
  shows only a turn later; it cannot fix a wrong value (the mine's
  double-counted gold, E111 and E113).
- **The opponent's model counts twice.** Its replies pick the worst leaf; a
  weak or pessimistic model (E103, E105) misleads twice as often. The leaf
  searches use the same model as today's, which E104 found level with a
  stronger one.
- **Too few candidates.** Depth only re-ranks today's plans; if the best
  plan is never among them, the teacher cannot find it. Step 1 counts how
  often the candidates differ.
- **Time.** A teacher is screened at 800 pairs rather than 1,600; a gain
  under about +20 would not be told from tk10's.
- **Size.** About 100 lines in `search.rs`; the bundle stays within
  100,000 characters, as every switch has.
