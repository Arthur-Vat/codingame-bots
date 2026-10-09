---
id: E015
date: 2026-10-08
pull_request: "#35"
parent: uttt-v009
hypothesis: A move policy with one learned weight per small-board pattern, cell and destination predicts the search's choices better than 32 move classes, so its playouts judge positions better and its order of children is better.
change: Playouts draw their first 16 moves, and nodes order their children, with 26,275 learned weights (`PatternPolicy`, `search::PatternBoard`) instead of E011's 32 class weights; the weights travel in the bot as one base64 character each.
release: uttt-v010
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: pending
---

The owner's request on 2026-10-08: use the bytes the bot leaves free
(v009 is 41.5 KB of 100 KB) for many more weights, with an efficient
encoding, and the owner chose the move policy over a larger value
network.

## The model

A move's feature is the pattern of the small board it is played in, seen
by the player to move (each cell empty, its own or the opponent's), the
cell, and where the move sends the opponent. The 8 symmetries of the 3×3
grid map a pattern and cell to the same weight, so only canonical pairs
of open boards count: 5,255 of them. Each comes with one of 5
destinations: a free choice, or a board where the opponent, the player,
both or neither can win a small board at once. That makes 5,255 × 5 =
26,275 weights, against 32 classes before; the classes' features
(winning the board, blocking, the centre, free choice, giving a board)
are all functions of this feature.

A move is drawn with probability proportional to `e^θ` of its feature,
as before with integer weights below 2^24; a game-winning move is still
played first. The table from (pattern, cell) to its canonical number is
built when the bot starts, in about 4 ms.

## Encoding

Each log-weight θ is rounded to a quarter between -8 and 7.75 and written
as one character of the base64 alphabet: 6 bits per weight, 26,275
bytes in all. Rounding costs little: on held-out positions the
cross-entropy goes from 1.7334 to 1.7352 nats. The bundled bot is 75.7 KB.

The readable source and the paste-ready file are two versions of the
same code: the repository keeps the documented sources, and the bundler
strips comments, indentation and blank lines into the file pasted on
CodinGame.

## Fit

`uttt-trainer fit-patterns` fits θ to the visits of searched positions by
softmax cross-entropy and Adam (4 epochs, batches of 256 positions, step
0.01, penalty 1e-5), holding every 20th game out. The data, played
locally on 2026-10-08 at 10,000 iterations per move with 6 random opening
moves:

- 6,000 games with v008's playouts (`selfplay --policy
  games/uttt/bots/mcts/src/weights.rs`, seeds 500 and 501);
- 2,000 games with decisive playouts (seeds 101 and 202).

That is 320,437 fitted positions (2.4 million moves) and 17,072 held
out; 25,710 of the 26,275 features occur in the fitted positions.

| Model, on held-out positions | Weights | Cross-entropy, nats per position | Probability of the search's favourite move |
| --- | --- | --- | --- |
| Uniform | 0 | 1.8787 | 0.172 |
| Move classes, fitted on the same data | 32 | 1.7937 | 0.210 |
| Patterns | 26,275 | 1.7334 | 0.234 |
| Patterns, rounded to quarters | 26,275 | 1.7352 | 0.234 |

## Screening

Locally against v009 (2-core sandbox, 4-ply openings, 20 ms per move
unless said). The fitted θ is divided by a temperature before encoding;
sharper weights play better, as with E011's classes.

| Temperature | Policy moves per playout | Seed | Pairs | Elo against v009 |
| --- | --- | --- | --- | --- |
| 1 | 16 | 31 | 100 | -26.1 [-69.6, +16.6] |
| 0.5 | 16 | 31 | 100 | +43.7 [-0.5, +89.3] |
| 0.5 | 16 | 47 | 300 | -26.1 [-49.7, -2.8] |
| 0.33 | 16 | 31 | 100 | +54.3 [+8.8, +101.8] |
| 0.33 | 16 | 47 | 300 | +22.0 [-2.5, +46.8] |
| 0.33 | 16 | 59 | 100 | +15.6 [-26.1, +57.9] |
| 0.25 | 16 | 31 | 100 | +15.6 [-24.4, +56.1] |
| 0.33 | 8 | 47 | 300 | +24.9 [+0.5, +49.7] |
| 0.33 | whole playout | 47 | 300 | +9.8 [-13.1, +32.9] |

The candidate keeps temperature 0.33 and 16 policy moves: about +27 over
its 500 pairs at 20 ms. The screens at one temperature disagree more than
their intervals suggest (0.5: +44 then -26), so the SPRT decides.

## More weights, tried offline

Three richer destination kinds were fitted on the same data, without
going into a bot. They add whether the destination board would win the
whole game for the opponent, whether a free choice lets the opponent win
the game at once, and whether the move's own board would win the game
for either side:

| Destination kinds | Weights | Held-out cross-entropy |
| --- | --- | --- |
| 5 (the candidate) | 26,275 | 1.7334 |
| 7 | 36,785 | 1.7350 |
| 14 | 73,570 | 1.7372 |
| 28 | 147,140 | 1.7387 |

None predicts better: with 8,000 games, the data, not the bytes, limits
the model. Fitted on 3,000 and 6,000 of the games, the patterns' lead
over the classes grew from 0.056 to 0.064 nats, so more games should
help, and larger models may need many more. The encoded weights' entropy
is 4.2 bits per weight: an entropy coder would shrink the 26,275
characters to about 18,400 if bytes ever run short.

## First result

From the first SPRT comment on #35 (commit 760697b): smoke test 200 wins in 200 games; SPRT
accepted after 943 pairs (789 wins, 412 draws, 686 losses), LLR 3.00;
confirmation at CodinGame's limits 377 wins, 280 draws, 343 losses, no
timeout for either bot.

At 20 ms the gain, +19.2, is below the local screening's +27 but its
interval excludes 0. At full time it is +11.8, interval from -5.7 to
+29.4: v010 is not weaker, as ADR 0014 asks, and the estimate is a little
above E014's +9.7 at the same limits. Answers took as long as v009's
(99.9% within 92.1 ms, against 92.3), so the larger policy costs no
search speed.

#33 (the larger value network and `MixBoard`) was merged into `main`
first. Merging it here put its engine code into the bundle, unused by
this bot: 77,026 bytes instead of 75,677. The release was rebuilt from
the merged sources, as the release check requires, and the SPRT runs
again on it.
