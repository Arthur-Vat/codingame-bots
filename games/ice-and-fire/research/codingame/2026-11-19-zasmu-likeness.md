# The zasmu-likeness score, an early-game judge (2026-11-19)

Phase 0 of the early-game rework ([2026-11-19-early-game.md](2026-11-19-early-game.md)). The owner approved it on 2026-11-19 ("Ok for phase 0, but try to fit it on zasmu games"; form: "Zasmu-likeness score"): the early score is how close a side's position at game turns 8 to 12 is to zasmu's own positions, by seat and map; a win model serves only to check that zasmu-likeness predicts wins, on CodinGame and in self-play full games; then an arena mode that ends games at turn 12, scored by it. If zasmu-likeness did not predict full-game wins, the plan stopped here for his decision. It does, so the judge is in use.

## The score

- **Features** of a side at the start of its ply ([`early/src/score.rs`](../../early/src/score.rs)): its active cells, Voronoi share, share of the middle chokes (on maps with at least 2 of them; 0 elsewhere), units and those of level 2 and 3, gold, income, cells, mines, towers, centre cells, steps to the centre (at most 20), share of all the chokes, and two features of the map (its valid cells, and whether it has 2 or more middle chokes), so that maps that zasmu met more often do not count as zasmu-like.
- **Model:** for each seat, a logistic regression on standardized features with an L2 penalty (scikit-learn's default `C = 1`; the Rust fit's weights are within about 0.01 of scikit-learn's), telling zasmu's sides from the other players' sides at the start of their ply of game turn 12. Our own sides are left out of the fit. The score is the log-likelihood ratio, the model's log-odds less its seat's prior log-odds (the log-odds of zasmu among the seat's fitted sides: −2.90 red, −2.25 blue), so that red's and blue's scores compare: higher is more like zasmu. `ice-and-fire-review early-fit` fits it from the data branch and writes [`early/src/score/weights.rs`](../../early/src/score/weights.rs).
- **Data:** the data branch `claude/codingame-games` at 5788694 (7,030 ranking and test games, all against our releases). Fitted sides at turn 12: red 1,021 (53 of them zasmu's), blue 5,992 (571 zasmu's). Red is thin: CodinGame's test games always seat the tested bot first, so zasmu plays red only in ranking games.
- **Turn 12, alone:** fits at turns 8 to 11 tell zasmu apart less well and predict the winner less well (below), and summing the scores of turns 8 to 12 predicted the winner worse than turn 12 alone (AUC 0.741 against 0.810 in the games without zasmu).

What it rewards (the standardized weights at turn 12, red / blue): income +0.93 / +1.37, active cells +0.64 / +1.14, the middle chokes +0.61 / +0.70, level-3 units +0.31 / +0.53, the Voronoi share +0.26 / +0.31; and against it, gold kept −1.26 / −0.89 and units, the active cells held equal, −0.56 / −0.49. So: more land and income from fewer units, gold spent, the middle chokes taken. The weight of cells held equal to the active cells is −0.94 for blue (cells cut off from the HQ count against it).

## Does it tell zasmu apart?

Cross-validated AUC (5 folds by game) of zasmu's sides against the other players', by game turn, red / blue (turn 12's all-features figures from `early-fit`, the rest from scikit-learn with other folds, which gave 0.818 / 0.813 at turn 12):

| Game turn | 8 | 9 | 10 | 11 | 12 |
| --- | ---: | ---: | ---: | ---: | ---: |
| All features | 0.721 / 0.760 | 0.753 / 0.759 | 0.766 / 0.787 | 0.790 / 0.828 | 0.803 / 0.814 |
| Territory only (active cells, Voronoi, middle chokes) | 0.621 / 0.590 | 0.664 / 0.578 | 0.690 / 0.649 | 0.739 / 0.713 | 0.715 / 0.675 |

The mean score at turn 12 by group (out of fold for the fitted players, red / blue): zasmu +0.71 / +0.66; the players ranked 2 to 7 −0.71 / −0.77; the other players −1.26 / −0.89; v029 −1.13 / −0.63; v028 −1.23 / −0.87; v026 −1.48 / −0.76; v014 −2.89 / −2.03. Our releases climb in the order of their strength, and v029 stands with the other players, below the players ranked 2 to 7.

## Does it predict wins on CodinGame?

The AUC of the red-minus-blue difference at game turn 12 for red's win, beside the same difference of active cells (the strongest single measure of [2026-11-19-early-game.md](2026-11-19-early-game.md)):

| Games | Games | Zasmu-likeness | Active cells |
| --- | ---: | ---: | ---: |
| Without zasmu | 6,389 | 0.810 | 0.847 |
| With zasmu | 624 | 0.842 | 0.788 |

By turn, in the games without zasmu (scikit-learn, the same fit): 0.532 at turn 8, 0.578 at 9, 0.699 at 10, 0.781 at 11, 0.810 at 12 (active cells 0.670, 0.721, 0.775, 0.816, 0.847). So in games against us zasmu-likeness predicts the winner well, but a little less than the active cells alone: what makes a position zasmu's is not all what wins it.

## Does it predict wins in self-play full games?

1,000 full games between releases (`ice-and-fire-arena match`, 100 seat-swapped pairs a pairing, time scale 0.487 and tolerance 6 ms from `scripts/calibrate.sh`, records kept), read by `ice-and-fire-review early-check`. Games decided before turn 12 left out:

| Pairing | Full games: Elo of the first | Games | Zasmu-likeness AUC | Active cells AUC |
| --- | --- | ---: | ---: | ---: |
| v029 vs v026 | +139.0 [+90.1, +193.8] | 198 | 0.810 | 0.858 |
| v029 vs v014 | +346.1 [+282.8, +434.2] | 199 | 0.966 | 0.928 |
| v029 vs v020 | +354.5 [+287.2, +451.1] | 198 | 0.936 | 0.810 |
| v026 vs v022 | +139.0 [+86.8, +197.9] | 198 | 0.828 | 0.840 |
| v022 vs v014 | +135.0 [+87.7, +187.6] | 197 | 0.879 | 0.884 |
| All | | 990 | 0.890 | 0.867 |

It predicts the full game's winner in self-play, better than the active cells over all pairings and less well between the two closest releases (v029 and v026).

## Duels cut at game turn 12

`ice-and-fire-arena-early` plays a game until both sides have started their ply of turn 12 and gives the win to the higher score (a game decided earlier keeps its result; its own records, replayed, score the same: `early-check` AUC 1.000 on them). The same five pairings, 200 pairs each, new seeds, same time settings:

| Pairing | Full games (100 pairs) | Cut at turn 12 (200 pairs) |
| --- | --- | --- |
| v029 vs v026 | +139.0 [+90.1, +193.8] | +107.5 [+70.8, +146.8] |
| v029 vs v014 | +346.1 [+282.8, +434.2] | +288.1 [+249.3, +333.8] |
| v029 vs v020 | +354.5 [+287.2, +451.1] | +232.8 [+194.9, +276.2] |
| v026 vs v022 | +139.0 [+86.8, +197.9] | +149.3 [+114.6, +187.0] |
| v022 vs v014 | +135.0 [+87.7, +187.6] | +105.6 [+73.2, +140.0] |

Every pairing comes out in the order of full games, with margins a little smaller on the whole (the middle game, where v029's gains over v020 partly lie, is cut off). The cut games take about a tenth of a full game's time (2,000 of them in 406 s on 4 CPUs, against about 36 minutes for the 1,000 full games), so about six times the evidence (squared z-score) in the same time for v029 against v026.

**The seats.** The review of the pull request found that the first version compared the two seats' log-odds, whose priors differ by 0.65 in blue's favour only because CodinGame seats its test games so: in a mirror match of v029, blue won 75% of the cut games, against 65% of 80 full games, and 50 of 60 pairs split. With the priors taken away (the score above), blue wins 66% of a cut mirror of 400 games (151 of 200 pairs split), and 56% to 66% of the cut games of the five pairings, against 50% to 64% of their full games. Blue's real edge in this game stays, as it should; the seat-swapped pairs of the arena cancel it.

## Limits

- **Every CodinGame game is against us,** so "zasmu's positions" are zasmu's positions against our releases, and the other players' are theirs against us. Red's model rests on 53 of zasmu's games.
- **It is a likeness, not a value.** A planner tuned to it could reach positions that look like zasmu's at turn 12 (gold spent, much land) and lose later, which is why a candidate it favours still goes through the SPRT on full games (the plan's acceptance), and why `early-check` should be run again on the planner's own full games.
- **The cut arena compresses margins**, more for pairings whose difference lies in the middle game: it measures early-game progress, not strength. Many of its pairs split one game each, as blue's edge decides close positions.

## Reproduce

```sh
cargo build --release -p ice-and-fire-review -p ice-and-fire-arena
target/release/ice-and-fire-review early-fit --data DATA --me Tarleya --weights games/ice-and-fire/early/src/score/weights.rs
target/release/ice-and-fire-arena match --bot A=... --bot B=... --pairs 100 --records FULL
target/release/ice-and-fire-review early-check FULL/*.json
target/release/ice-and-fire-arena-early match --bot A=... --bot B=... --pairs 200
```

`DATA` is the `ice-and-fire` folder of the data branch. The per-turn figures and the territory-only fits came from scratch Python over `codingame-early`'s CSV (not committed).
