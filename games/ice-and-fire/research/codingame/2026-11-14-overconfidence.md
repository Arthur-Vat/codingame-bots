# Is the search overconfident? Expected against actual, v023 to v026

Asked by the owner on 2026-11-14: "I would like to have an analysis of the time where our bot is overly confident (the opponent response is far more severe than expected). [...] My intuition is that our turn is correct, however we do not understand the opponent threat well enough."

Written by Claude from the rows of `ice-and-fire-review codingame-surprise` (README, "Review").

**In short:** on the turn it plays, the search is not overconfident about the opponent's reply. It is pessimistic: it expects about twice the damage the real reply does. The real reply is worse than every modeled one in about 1 ply in 10, and by 50 or more (three units, twelve cells) in 2%. Those large misses are almost all cuts made with unit moves, which the cut finder behind the modeled replies does not search; games with one were lost 71% of the time, against 47% without. Threats on our HQ are almost never a surprise of one turn: they appear after a slow decline, in positions the evaluation already scored well behind. Where the evaluation is overconfident is against the top 10: at an equal evaluation we lose about two games in three, with more gold and less land than in the games we win.

## What is measured

For each of our plies from the start of the middle game, the plan bot's search (v026, 600 tries, no deadline, as `codingame-judge`) scores the plan we played the way it scores any plan: the worst evaluation, from our side, at our next start of turn, over the replies it models (its memory of replies, then the plan's own: a chain to our HQ, the 3 best cuts of up to 5 trainings, a greedy turn, and a short climb of the opponent's own). That is what it **expected**. The opponent's real reply is applied the same way and evaluated the same way: what **happened** (*actual*). *Pass* is the evaluation if the opponent had done nothing. The **surprise** is actual − expected; below zero, the opponent found a reply worse for us than every modeled one. *Modeled damage* is expected − pass, *actual damage* actual − pass.

Units are the evaluation's: an active cell 4, a turn of income 3, a gold 1, a unit 10 a level, a tower 10. The evaluation also counts half a win against us when the opponent can pay a chain of trainings to our HQ at its next turn; plies where a value holds that term are counted apart.

Data: the collected CodinGame games on the branch `claude/codingame-games` at 063a143 of `ice-and-fire-v023` to `v026` (717 games, 375 lost; 9,542 of our plies). The positions of v023 to v025's games are judged by v026's search. The command, per release:

```sh
cargo build --release -p ice-and-fire-bot-plan --features analysis
ice-and-fire-review codingame-surprise --data DIR --me Tarleya \
  --judge target/release/ice-and-fire-bot-plan --release ice-and-fire-v026 --jobs 4 --out OUT
```

## 1. The kinds of plies

| Kind | Plies | Share |
| --- | ---: | ---: |
| Normal: the values are material | 8,264 | 87% |
| HQ threat foreseen, and it came | 834 | 9% |
| HQ threat foreseen, it did not come | 391 | 4% |
| HQ threat not foreseen | 53 | 0.6% |

A threat on our HQ appears in 393 games, 369 of them lost: nearly every loss (369 of 375) ends this way. In 365 of the 393, the threat appears *between* two of our plies: at the previous ply the search expected no threat and the reply did no more than expected (median surprise +31), but we were already far behind (median evaluation after the reply −231, about 58 cells), and at the next ply every plan of ours leaves the opponent a chain it can pay. In only 28 games did the threat come from a reply the search did not foresee.

## 2. The surprise on normal plies

| Plies | Below expected | ≤ −20 | ≤ −50 | 5% | 25% | Median | 75% | 95% |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 8,264 | 12% | 4.2% | 1.9% | −15 | +7 | +26 | +53 | +124 |

The modeled damage ranks the plies fairly well (Spearman correlation with the actual damage 0.60), but it is about twice the damage done, at every level of danger:

| Modeled damage | Plies | Mean modeled | Mean actual | Actual worse than modeled | Actual damage under 10 |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 to −20 | 1,480 | −9 | +7 | 11% | 84% |
| −20 to −50 | 2,494 | −35 | −11 | 10% | 48% |
| −50 to −100 | 2,537 | −71 | −34 | 8% | 27% |
| −100 to −200 | 1,342 | −135 | −76 | 11% | 16% |
| beyond −200 | 341 | −296 | −194 | 9% | 9% |

(The modeled damage is a worst case, so some of the gap is the nature of a minimum; but the opponent's reply is not that worst case in about 90% of plies.) Against the top 10 the modeled and actual damage are a little larger (−70 and −36) than against the others (−62 and −25); in our losses than in our wins (−82 and −43, against −54 and −23); the share of surprises of −20 or less is 4.6% against the top 10, 3.3% against the others, 5.0% in losses, 3.5% in wins.

## 3. The large misses

The 159 normal plies with a surprise of −50 or less (2%), next to the rest (v026's games for the first rows, all four releases for the last line):

| Plies (v026) | Count | Cells taken | Cells cut off | Units lost | Trainings | Moves | Some cut | Real reply among the modeled |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Surprise ≤ −50 | 27 | 3.6 | 8.4 | 43 | 3.3 | 4.0 | 93% | 0% |
| −50 to 0 | 141 | 1.4 | 1.4 | 15 | 1.1 | 3.5 | 47% | 0% |
| 0 or more | 1,217 | 1.0 | 0.9 | 10 | 0.9 | 4.0 | 30% | 6% |

Over the four releases, 144 of the 159 large misses cut cells off, 131 used unit moves, and 49 trained a level-3 unit. The modeled cuts come from the cut finder (`ice_and_fire_tactics::cut`), which searches chains of trainings only; cuts that need a unit's move are seen only if the opponent's short climb or its greedy turn happens to find them. Games with a large miss (or an unforeseen HQ threat) were lost 71% of the time (167 games), against 47% without (550 games).

## 4. The evaluation against the result

Share of games lost, by the evaluation after the reply, game turns 8 to 17:

| Evaluation | Against the top 10 | Against the others |
| --- | ---: | ---: |
| −150 to −50 | 92% (648 plies) | 83% (119) |
| −50 to 0 | 76% (597) | 73% (131) |
| 0 to +50 | 65% (734) | 40% (220) |
| +50 to +150 | 40% (978) | 16% (484) |
| +150 to +400 | 8% (506) | 5% (433) |

Against the rest of Legend the evaluation is roughly calibrated (even near 0 to +50); against the top 10 we need about +100 to be even. Part of that is the stronger player outplaying us later, which no evaluation of the position can show. Part is in the position: at an equal evaluation (−50 to +50, turns 8 to 17), the plies of games we lost had, against those we won, more gold and less land (ours less theirs at our next start of turn):

| Games | Plies | Gold | Income | Active cells | Units (training cost) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Won | 569 | +15 | −4 | −5.7 | −12 |
| Lost | 1,113 | +26 | −6 | −9.0 | −22 |

Fitted to the result (a logistic fit on these four differences, turns 8 to 17), a cell is worth about 13 gold and a turn of income about 10, where the evaluation counts 4 and 3; given the evaluation, more gold of ours goes with more losses. This agrees with the opening study ([2026-11-14-opening-study.md](2026-11-14-opening-study.md)): we lose the land of the middle. But refitted weights predicted no better and played worse before (E017), and weighing gold less lost in the opening (E034, E056): a fit to results is not a better evaluation for the search.

## What follows

The owner's intuition holds for a narrow case and not in general. The search does not misjudge the usual reply: it overrates it. What it misses are **cuts made with moves**, rare but costly, and the slow loss of land that makes our HQ indefensible a few turns later, which the evaluation sees as a material deficit but not as the danger it is against the top 10.

Proposals, each an experiment screened against v026 (the owner's choice):

1. **Move-and-cut replies** (medium; the time roadmap's "move-and-cut plans"): add to the modeled replies the cuts that start with one or two unit moves, for example the cut finder run from the state after each enemy unit's best move toward our land. Aimed at the large misses: 2% of plies, in games lost 71% of the time. It costs search time.
2. **A less pessimistic reply score** (small, one constant): score a plan by a mix of its worst modeled reply and its pass value (or second-worst reply), so that the search stops paying for threats that do not come. The modeled damage is twice the real one, and at an equal evaluation our losses hold more gold and less land, which is what a search that over-defends would do. Risk: more real large misses; screen two or three mixes.
3. **Not proposed:** refitting the evaluation's weights to results (E017), or a land term in the middle game (E061 lost).

## Limits

- The search judged each ply with 600 tries and no deadline; on CodinGame it searched until its 45 ms deadline, so its plans and its memory of replies may differ.
- v023 to v025's positions are judged by v026's search, not by the bot that played them.
- The evaluation's units are not gold: "twice the damage" compares evaluations, and a cell's or a unit's worth in them is the evaluation's.
- 522 of the 717 games are against the top 10, so the overall figures lean on them.
