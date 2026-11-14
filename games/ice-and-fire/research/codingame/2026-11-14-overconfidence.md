# Is the search overconfident? Expected against actual, v023 to v026

Asked by the owner on 2026-11-14: "I would like to have an analysis of the time where our bot is overly confident (the opponent response is far more severe than expected). [...] My intuition is that our turn is correct, however we do not understand the opponent threat well enough."

Written by Claude from the rows of `ice-and-fire-review codingame-surprise` (README, "Review").

**In short:** on the turn it plays, the search is not overconfident about the opponent's reply. It is pessimistic: it expects about twice the damage the real reply does. The real reply is worse than every modeled one in about 1 ply in 10, and by 50 or more (three units, twelve cells) in 2%. Those large misses are large cuts (13 of our cells removed on average, against 2 in other replies) that the modeled replies could not hold: about half needed unit moves, which the cut finder behind the modeled replies does not make, and most of the rest were beyond that finder's reach; games with one were lost 71% of the time, against 47% without. Threats on our HQ are almost never a surprise of one turn: they appear after a slow decline, in positions the evaluation already scored well behind. Where the evaluation is overconfident is against the top 10: at an equal evaluation we lose about two games in three, with more gold and less land than in the games we win.

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
| 8,264 | 10% | 4.2% | 1.9% | −15 | +7 | +26 | +53 | +124 |

The modeled damage ranks the plies fairly well (Spearman correlation with the actual damage 0.60), but it is about twice the damage done, at every level of danger:

| Modeled damage | Plies | Mean modeled | Mean actual | Actual worse than modeled | Actual damage under 10 |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 to −20 | 1,480 | −9 | +7 | 11% | 84% |
| −20 to −50 | 2,494 | −35 | −11 | 10% | 48% |
| −50 to −100 | 2,537 | −71 | −34 | 8% | 27% |
| −100 to −200 | 1,342 | −135 | −76 | 11% | 16% |
| beyond −200 | 341 | −296 | −194 | 9% | 9% |

The buckets hold 8,194 of the 8,264 normal plies: 23 whose pass value holds the HQ term are left out (here and in the comparisons by opponent and result below), and in 47 the modeled worst reply scores above the pass (expected > pass). The ratio of modeled to actual damage runs from about 1.5 to 3, and in the first bucket the reply even leaves us better off than a pass on average (the evaluation counts our income at our next start of turn either way, and a reply's purchases can cost the opponent more than they take). The modeled damage is a worst case, so some of the gap is the nature of a minimum; but the opponent's reply is not that worst case in about 90% of plies. Against the top 10 the modeled and actual damage are a little larger (−70 and −36) than against the others (−62 and −25); in our losses than in our wins (−82 and −43, against −54 and −23); the share of surprises of −20 or less is 4.6% against the top 10, 3.3% against the others, 5.0% in losses, 3.5% in wins.

## 3. The large misses

The 160 normal plies with a surprise of −50 or less (1.9%), next to the rest (v026's games, the command's report):

| Plies (v026) | Count | Cells taken | Cells cut off | Units lost | Trainings | Moves | Some cut |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Surprise ≤ −50 | 27 | 3.6 | 8.4 | 43 | 3.3 | 4.0 | 93% |
| −50 to 0 | 141 | 1.4 | 1.4 | 15 | 1.1 | 3.5 | 47% |
| 0 or more | 1,217 | 1.0 | 0.9 | 10 | 0.9 | 4.0 | 30% |

Units move in nearly every reply, so moves alone do not tell the large misses apart; their size does (over the four releases, 13.0 of our cells removed on average, against 1.9 in replies that did not surprise). A real reply that surprises is almost never one of the modeled ones (0% of the large surprises, 1% of the small ones): had it been modeled, its value could not fall below the worst modeled one, except when the same commands come in another order, or a pass meets the no-reply value of an empty memory, which is taken before the opponent's income.

Why the modeled replies did not hold them, read on the board for each reply (all four releases; the causes are tested in this order): a reply *needs its moves* when, played without them (its trainings and buildings alone, in order), it removes fewer than half the cells; otherwise the cut finder is run from the reply's start with the opponent's gold, as the search runs it (chains of up to 5 trainings, its 3 best cuts) and wider (up to 8 trainings, 20 cuts), and compared with the cells the reply removed.

| Cause | Surprise ≤ −50 (160) | −50 to 0 (662) | 0 or more (7,442) |
| --- | ---: | ---: | ---: |
| No cell of ours removed | 6% | 35% | 48% |
| Moves needed | 49% | 33% | 22% |
| Trainings alone; the search's cut finder finds as many cells | 6% | 17% | 22% |
| Trainings alone; only the wider finder finds as many | 11% | 5% | 4% |
| Trainings alone; neither finder finds as many | 28% | 11% | 3% |

So the large misses are cuts the modeled replies could not contain for two reasons, in about equal parts: they needed unit moves before the trainings (49%, twice the share in other replies), or they were cuts of trainings alone that the cut finder does not reach even when widened (28%, against 3%), or reaches only when widened (11%). In 50 of the 160 the reply trained a level-3 unit (8% of the replies that did not surprise), and in 21 it trained more than 5 units (0.5%). Games with a large miss (or an unforeseen HQ threat) were lost 71% of the time (167 games), against 47% without (550 games).

## 3b. The shape of the large misses

Asked by the owner on 2026-11-14 ("try to characterize those missed threats"). The 160 large misses, next to the 2,220 replies that also cut cells off but did not surprise, and the 5,884 other replies (all four releases, normal plies). A **joint** is one of our active cells whose capture alone cuts others off from our HQ (`solo::severed`, the shape the evaluation counts); *depth* is the most steps from the enemy's land, before the reply, to a cell the reply removed.

| Measure | Large misses | Cuts that did not surprise | Other replies |
| --- | ---: | ---: | ---: |
| Our cells removed (mean), of which cut off | 13.0, 9.4 | 5.2, 3.0 | 0.8, 0.2 |
| Our cells it captured to do so (mean) | 3.6 | 2.2 | |
| It cut cells off without capturing a joint (a double link broken) | 38% | 11% | |
| It captured a joint | 54% | 89% | 6% |
| Cells that joint cut off (mean, when one was captured) | 6.3 | 2.9 | 2.8 |
| Our joints at the end of our turn (mean) | 6.2 | 5.3 | 4.0 |
| Depth of the removed cells (median steps behind our front) | 3 | 2 | 1 |
| Depth of the captured cells (median; mean) | 2; 2.8 | 1; 1.5 | |
| Nearest captured cell to our HQ (median steps) | 7 | 10 | |
| Removed cells we had taken in our last turn | 22% | 34% | 25% |
| A tower of ours lost (captured or cut off) | 38% | 18% | 5% |
| A tower of ours captured | 16% | 9% | |
| Level-3 trainings (mean) | 0.4 | 0.2 | 0.0 |
| Trainings (mean) | 3.3 | 1.8 | 0.5 |
| Opponent's gold at its turn (median) | 46 | 40 | 37 |
| Share of it spent | 78% | 66% | 37% |
| Opponent spent 10 gold or less the turn before | 26% | 17% | |
| Our gold left after our turn (median) | 8 | 8 | 14 |
| Our towers on active cells (mean) | 2.6 | 2.7 | 2.6 |

What sets the large misses apart:

- **They break double links.** Only 54% of them capture a joint, against 89% of ordinary cuts; 38% cut a region off without capturing any single weak point, by capturing several cells (3.6 on average) that together sever it. The evaluation's shape term counts joints and the largest region one capture cuts off; it does not see a region held by two or three thin links, and the cut finder, which searches single chains of trainings from the enemy's land, finds such cuts only when one chain happens to cross all the links.
- **They strike deeper.** The removed land is larger and lies deeper, which partly restates its size; but the captures themselves also reach further: 2 steps behind our front (mean 2.8, against 1.5 for ordinary cuts), and within 7 steps of our HQ (10). The land removed was mostly not taken in our last turn (22% of it, against 34% in ordinary cuts), so these are not punishments of a fresh overextension. A tower of ours is captured in 16% of them (9% of ordinary cuts) and lost, captured or cut off, in 38% (18%); they train 0.4 level-3 units each on average (0.2).
- **They spend more of a slightly larger bank.** The opponent holds 46 gold (median; 40 before ordinary cuts) and spends 78% of it (66%); in a quarter of them it spent 10 gold or less the turn before (17%).
- **Our side looks ordinary:** the same gold left (8) and towers (2.6) as before ordinary cuts, a few more joints (6.2 against 5.3).
- **Some opponents do it more:** _iahelka in 4.0% of our plies against it, Waffle3z 2.5%, Pduhard- 2.3%, karliso and _iaz 2.0%, zasmu 1.3%, reCurse 0.9% (opponents with 300 plies or more).
- **They are mostly isolated:** 15 of the 160 follow another at our previous ply; 22 games have two or more.

Examples of the largest in v026's games (CodinGame replays): [921571527](https://www.codingame.com/replay/921571527) turns 12 and 13 against karliso (29 then 25 cells removed, with 114 then 51 gold, 1 and 3 towers lost), [921571436](https://www.codingame.com/replay/921571436) turns 10 and 11 against _iahelka (22 cells removed at turn 10 without capturing a joint), [921571992](https://www.codingame.com/replay/921571992) turn 23 against Pduhard-.

**What follows.** The structure these replies exploit is a *cheapest cut*: the set of our cells whose capture, at its price (10 for an empty cell, 20 or 30 for one held by a unit, 30 through a tower), separates the most land from our HQ. A max-flow on our territory's graph (144 cells) finds exactly and fast the cheapest set of cells whose capture isolates a *given* region from our HQ, double links included; trying candidate regions (for example each region beyond our joints and thin links, or the land within a few steps of the enemy's) against the opponent's gold would say what it can sever next turn. It leaves out the chain of trainings needed to reach the cut cells, which would have to be added to the price. It could serve both as a term of the evaluation (the land the opponent can afford to cut off, beyond the single joints counted today) and as a reply generator (the cut cells, then the chain of trainings that reaches them). This is a proposal, not tested: E064 showed that adding replies costs more than it saves unless they are the ones a strong opponent plays.

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

The owner's intuition holds for a narrow case and not in general. The search does not misjudge the usual reply: it overrates it. What it misses are **large cuts its cut finder cannot make** (after unit moves, or beyond its reach), rare but costly, and the slow loss of land that makes our HQ indefensible a few turns later, which the evaluation sees as a material deficit but not as the danger it is against the top 10.

Proposals, each an experiment screened against v026 (the owner's choice):

1. **Wider cut replies** (medium; the time roadmap's "move-and-cut plans"): add to the modeled replies (a) cuts that start with unit moves, for example the cut finder run after each enemy unit's step toward our land, and (b) a cut finder that reaches further than chains of 5 trainings. First, a study of the 45 large misses that no finder reaches (what shape their cut has) would say what (b) needs. Aimed at the large misses: 2% of plies, in games lost 71% of the time. It costs search time.
2. **A less pessimistic reply score** (small, one constant): score a plan by a mix of its worst modeled reply and its pass value (or second-worst reply), so that the search stops paying for threats that do not come. The modeled damage is twice the real one, and at an equal evaluation our losses hold more gold and less land, which is what a search that over-defends would do. Risk: more real large misses; screen two or three mixes.
3. **Not proposed:** refitting the evaluation's weights to results (E017), or a land term in the middle game (E061 lost).

## Limits

- Sections 2 and 4 (the damage buckets, the rank correlation, the fit, the tables by turn) and the four-release totals of section 3 were computed from the command's `rows.csv` with short scratch scripts, not by the command; its report gives the tables for one run.
- A modeled reply is listed once even when the memory and the plan's own replies both hold it; with an empty memory the search scores the end of our turn itself, which then counts as a modeled outcome.

- The search judged each ply with 600 tries and no deadline; on CodinGame it searched until its 45 ms deadline, so its plans and its memory of replies may differ.
- v023 to v025's positions are judged by v026's search, not by the bot that played them.
- The evaluation's units are not gold: "twice the damage" compares evaluations, and a cell's or a unit's worth in them is the evaluation's.
- 522 of the 717 games are against the top 10, so the overall figures lean on them.
