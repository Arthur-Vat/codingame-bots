---
id: E026
date: 2026-10-30
pull_request: "#222"
parent: ice-and-fire-v010
hypothesis: A third of v010's towers never block a training or a unit; offering towers only where they guard something (cells the enemy can take at its next turn, or the HQ against a chain the enemy can pay within two turns) stops the wasted ones and beats v010.
change: The plan bot's action generator offers a tower on a border cell only if it lowers the cells of ours the enemy can take at its next turn, or raises the enemy's chain to our HQ while the enemy can pay for that chain within two of its turns.
release: ice-and-fire-v011
sprt: accepted
elo: +24.9 [+8.8, +41.2] at CodinGame's limits
pairs: 433
full_time: +24.9 [+8.8, +41.2] (the SPRT's own, ADR 0042), no timeout in 867 games, slowest answer 615.5 ms (the first turn)
decision: promoted
cg_rank: not reported
---

Chosen by the owner on 2026-10-30 after the study of v010's towers
([research/towers-v010.md](../research/towers-v010.md)): "Towers where reach drops (Recommended)", the two rules screened as
variants ("Both as variants"), on border cells only ("Border only first
(Recommended)"). Screened against E023's v010 (pull request #219, merged on 2026-10-30).

## The change

`bots/plan/src/actions.rs`, `guards`: v010 offers a tower on every free
border cell that is no mine spot, and the search keeps those the
evaluation likes. Now a border tower is offered only if, built, it

- lowers `eval::threatened` for our side (the cells of ours the enemy's
  chains reach for its gold plus its income, E023's measure), or
- with `HQ_GUARD`, raises the enemy's cheapest chain to our HQ while that
  chain costs at most the enemy's gold plus two of its incomes (the
  threats the study of v009 found real: taken within 5 turns 81% of the
  time one turn away, 36% two turns away).

Each candidate cell costs one clone, one tower and one or two floods; the median
answer did not change. A test checks the rule through the generator: on a
corridor where blue can take red's last cell, the tower there is the one
offered; with the sides far apart, none is; and where blue cannot take
a red cell next turn but can pay its chain to red's HQ within two turns,
only the tower next to the HQ is offered (the HQ guard).

## Screening

Against `ice-and-fire-v010` (pull request #219) at CodinGame's limits
(50 ms a turn, 1,000 ms the first, no tolerance), 400 seat-swapped pairs
each on seed 26026, two games at a time on a 4-core machine; the best
again on 600 pairs with seed 26027. Bundles built with `cg-bundler`
(minifier level 2) and rustc 1.90.0; read with `ice-and-fire-review
screening`. v010's numbers in the same games in parentheses.

| Variant | Elo against v010 | Score as red | Score as blue | Towers | Mines | Level 1 / 2 / 3 | Gold kept | Timeouts | Median, max ms |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | ---: | --- |
| reach or HQ guard | +16.5 [−1.7, +34.8] | 38.2% | 66.5% | 3.07 (3.83) | 2.28 (1.97) | 24.3 / 3.9 / 1.7 (23.5 / 3.7 / 1.4) | 29.3 (28.4) | 9 (10) | 16.7, 225.0 (16.8, 146.5) |
| reach only | +10.4 [−9.8, +30.8] | 37.8% | 65.2% | 3.03 (3.99) | 2.29 (2.13) | 24.8 / 3.7 / 1.7 (23.8 / 3.9 / 1.4) | 28.6 (28.5) | 8 (8) | 16.4, 107.8 (16.6, 106.2) |
| reach or HQ guard, 600 pairs, seed 26027 | **+17.4 [+1.9, +33.0]** | 38.7% | 66.3% | 3.15 (3.95) | 2.27 (2.13) | 25.1 / 3.9 / 1.8 (24.1 / 3.8 / 1.6) | 30.1 (29.4) | 6 (7) | 16.3, 96.6 (16.8, 88.2) |

Every bot's answers were at most 32.1 ms in 99% of turns; the slowest
first answers were 640 to 681 ms. Both bots timed out alike, 6 to 10 in
800 to 1,200 games: this machine's, as in the study of v010, whose SPRT on
GitHub's runners had none in 696 games.

## The candidate

The owner chose it as the candidate on 2026-10-30 ("Candidate v011
(Recommended)"): `ice-and-fire-v011`, frozen by `scripts/new-release.sh`
(77,196 characters, the same answers as its readable copy in 4 games).

## What was learned

- **Fewer towers, a little stronger.** Over its 1,000 pairs the HQ-guard
  rule scored about +17 against v010 with a fifth fewer towers (3.1
  against 3.9 a game); the gold goes to mines and units, level 3 above
  all.
- **The HQ guard matters a little.** Without it the rule scored +10.4, as
  many towers dropped; the towers it adds back are those against a chain
  the enemy can pay within two turns.
- The gain is small for an SPRT at elo1 = 10; the candidate is the
  owner's to choose.

## SPRT

Accepted on the pull request (run 41033188671, at c9c5780), at
CodinGame's limits:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v011 vs ice-and-fire-v010: 867 games, 433 complete pairs
  ice-and-fire-v011: 465 wins, 0 draws, 402 losses (score 53.6%)
  Elo of ice-and-fire-v011 over ice-and-fire-v010: +24.9 [+8.8, +41.2] (95%, from pairs)
  pairs by ice-and-fire-v011's points (0, 1/2, 1, 3/2, 2): [38, 0, 326, 0, 69]
  ice-and-fire-v011 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v010 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): ice-and-fire-v011 20.0, 31.8, 32.3 ms; ice-and-fire-v010 20.5, 31.7, 32.3 ms
  slowest answer: ice-and-fire-v011 615.5 ms, ice-and-fire-v010 612.7 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.95 (bounds -2.94, 2.94) after 433 pairs
Verdict: ACCEPTED: ice-and-fire-v011 is stronger than ice-and-fire-v010
```

## League

The league of the release (run 41033188671, at c9c5780, 800 pairs a
pairing) scored v011 52.4% against v010 (839 wins, 761 losses), 59.9%
against v009, 64.3% against v008, 69.4% against v007, 76.5% against v006,
81.4% against v005 and 99.9% against bronze, and rated it +2301.0 ± 182.5,
against v010's +2285.6 ([LEADERBOARD.md](../LEADERBOARD.md)). v011 lost 1
of its 11,200 games by a fault.
