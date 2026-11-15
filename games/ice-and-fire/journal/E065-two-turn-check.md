---
id: E065
date: 2026-11-14
pull_request: pending
parent: ice-and-fire-v026
hypothesis: The middle game's search judges a plan one reply deep, and the large removals it foresees are the price of positions built earlier (stretched land, the opponent banking gold); scoring its best few plans two plies further (the worst replies, our quick follow-up, then the opponent's best cut with its next turn's gold) before choosing chooses plans whose position holds one turn longer, and beats v026.
change: None released. Screening only: the plan bot's final choice scores its best plans by the final score again, each after its worst replies, a short climb of ours and the opponent's chain to our HQ or best cut with the gold of its next turn, and plays the best of them.
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-14 ("the two-turn check of the final choice
as the next experiment"), the first recommendation after the study of the
missed threats
([research/codingame/2026-11-14-overconfidence.md](../research/codingame/2026-11-14-overconfidence.md),
section 3d).

## Why

The study found that the large removals the search foresees are not
parryable on the turn they come: the cut was on the board before our turn,
and the search finds no better plan. They follow from positions built
earlier, which a score one reply deep does not tell apart. E063 and E064
showed that modeling more or milder replies at the first ply loses; this
experiment keeps the first ply as it is and looks one turn further, only
for the few plans that the search already ranks best.

## The change

`bots/plan/src/search.rs`, after the final choice and its check (E027):
the `DEEP_PLANS` best plans by the final score, the chosen one first, are
scored again while the turn's time remains (`Search::deep_score`). For
each of a plan's `DEEP_REPLIES` worst replies (the memory's and its own),
the reply is played, then a climb of ours of up to `DEEP_EVALS` tries
against no reply, then the opponent's strike with the gold of that turn:
a pass, its chain to our HQ or one of its `DEEP_CUTS` best cuts, whichever
leaves the lowest evaluation at our next start of turn
(`Search::follow`). A plan's two-turn score is the lowest over those
replies. The plan with the best two-turn score is played (`DEEP_BLEND`:
the best sum of the final and the two-turn score), the chosen one on a
tie; a check the time cuts before its second plan changes nothing.

On the speed bank (200 positions, 600 tries, this session's machine) the
check with 4 plans, 2 replies and 60 tries adds 1.45 ms to a search of
17.2 ms (the mean), and changes the answer in 83 positions of 200.

## Screening

Against `ice-and-fire-v026` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048), 800 pairs each over two runners. Every variant keeps the 60 tries
of the follow-up and one cut of the opponent unless said; *margin* is by
how much another plan's value must beat the chosen plan's
(`DEEP_MARGIN`); the variants c and d stop the climbs at 50% of the way
from the soft to the hard limit rather than 75%, to leave the larger
check its time, and try 150 tries of the follow-up.

Run 43652545999, seed 65065 (one timeout of c in 1,600 games, no other
fault):

| Variant | Commit | Plans | Replies | Chosen by | Margin | Elo against v026 |
| --- | --- | ---: | ---: | --- | ---: | --- |
| a | `7cec6c4` | 4 | 2 | two-turn score | 0 | +5.6 [−7.6, +18.9] |
| b | `2b8cdc5` | 4 | 2 | sum | 0 | +5.2 [−7.2, +17.6] |
| c | `81a9c27` | 5 | 3 | two-turn score | 0 | −1.7 [−15.4, +11.9] |
| d | `1f039b3` | 5 | 3 | sum | 0 | −0.4 [−13.5, +12.7] |
| e | `36b1b09` | 3 | 1 | sum | 0 | +8.7 [−3.9, +21.3] |

Run 43656527721, seed 65066 (no fault of a variant; three timeouts of
v026):

| Variant | Commit | Plans | Replies | Chosen by | Margin | Elo against v026 |
| --- | --- | ---: | ---: | --- | ---: | --- |
| f | `f443746` | 4 | 2 | two-turn score | 20 | −5.2 [−17.5, +7.1] |
| g | `193d736` | 4 | 2 | two-turn score | 50 | +2.2 [−8.2, +12.6] |
| h | `4292bb1` | 3 | 1 | two-turn score | 20 | −9.1 [−22.5, +4.2] |
| i | `d296b1d` | 3 | 1 | sum | 20 | +5.2 [−7.2, +17.7] |
| j | `6719f81` | 2 | 1 | sum | 0 | +0.9 [−11.8, +13.6] |
| k | `7ebf277` | 5 | 1 | sum | 0 | −4.3 [−17.4, +8.7] |
| e2 | `b5e96b9` | 3 | 1 | sum | 0 | −7.8 [−20.7, +5.0] |

No variant is outside its interval, and e played twice gives +8.7 then
−7.8: over the twelve runs (9,600 pairs) the mean is −0.1. The check is
not too slow: a variant's median answer is 1.5 to 3 ms longer than
v026's on the same runner (it uses the time v026 leaves before the hard
limit), with one timeout in 19,200 games.

## What was learned

- **Looking one turn further for the best few plans does not pay in this
  form.** The check changes the plan played often (in 83 of the 200
  positions of the speed bank for the variant a, 70 for e), yet no
  variant gains: the two-turn score, made of a climb of ours against no
  reply and one cut of the opponent's, tells the search's best plans
  apart no better than the final score does. Larger checks (c, d) and
  margins (f to i) changed nothing.
- **Time is not what limits it**: the check fits in the 4.5 ms the turn
  leaves after the climbs. The owner's condition for a defence reserve
  ("only if that check is too slow") is not met, so none was started.
- The follow-up of ours does not defend (it climbs against no reply),
  which may make every plan look alike two plies on; a follow-up against
  the opponent's cuts was not tried.

The code stays reachable in this pull request's commits (the variants
above; `df7ede0` holds the last one, e2); the branch's head holds the
journal entry alone. Dropped by the owner on 2026-11-15 ("Drop E065").
