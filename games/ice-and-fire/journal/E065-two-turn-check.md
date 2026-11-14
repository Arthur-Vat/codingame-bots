---
id: E065
date: 2026-11-14
pull_request: pending
parent: ice-and-fire-v026
hypothesis: The middle game's search judges a plan one reply deep, and the large removals it foresees are the price of positions built earlier (stretched land, the opponent banking gold); scoring its best few plans two plies further (the worst replies, our quick follow-up, then the opponent's best cut with its next turn's gold) before choosing chooses plans whose position holds one turn longer, and beats v026.
change: The plan bot's final choice scores its best plans by the final score again, each after its worst replies, a short climb of ours and the opponent's chain to our HQ or best cut with the gold of its next turn, and plays the best of them.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
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
