---
id: E021
date: 2026-10-29
pull_request: "none yet"
parent: ice-and-fire-v009
hypothesis: v009's pressure term counts a threat to an HQ almost the same whether its side can pay for it next turn or in 8 turns, so v009 builds towers against far threats; counting a threat by how soon it can be paid for beats v009.
change: None released. Local screening only: the plan bot's pressure term replaced by −V × (1 − 0.42^t), t the incomes still missing beyond the next turn, at V = 50, 100 and 200; and E022's control, v009 without the static value of a tower.
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Approved by the owner on 2026-10-29, as the first experiment of the study
of towers and threats ([research/towers-and-threats.md](../research/towers-and-threats.md)),
to run in parallel with other work.

## The change

The study found that the pressure term, `clamp(gold + income − cost, −200,
0)` for each side's cheapest chain to the enemy HQ, pays about as much for
a threat 5 or more turns away as for one 2 to 4 turns away, and that 48% of
v009's towers answer such far threats. Its proposal, taken as written:

    pressure = −V × (1 − r^t),  t = max(0, cost − gold − income) / max(income, 1)

with r = 0.42 (the rate at which v009's threats were carried out within 5
turns), t fractional, 0 when the side can pay at its next turn, and −V when
the HQ is out of reach. The same function counts both sides, so our own
attack is also valued only when it is near.

The code is commit d4ebd07 on this branch (reverted by the next commit, so
the branch carries the entry alone): the shared evaluation takes its
pressure term as a function (`ice_and_fire_tactics::eval::evaluate_with`),
the plan bot passes its own (`bots/plan/src/eval.rs`, constants `PRESSURE`
and `DECAY`), the analysis mode's "no pressure" variant removes the bot's
term, and tests check the term's values (0, −58, −82, −93 at t = 0 to 3
with V = 100, −100 for an unreachable HQ) and that `evaluate_with` changes
only the pressure term. Each variant changes one constant. The minified
bundle is 87,824 characters.

## Screening

Against `ice-and-fire-v009` at CodinGame's limits (50 ms a turn, 1,000 ms
the first, no tolerance), 400 seat-swapped pairs each on seed 23023, two
games at a time on a 4-core machine; the best again on 600 pairs with seed
23024. Bundles built with `cg-bundler` (minified) and rustc 1.90.0, as
CodinGame does; read with `ice-and-fire-review screening`. Each row gives
the variant's numbers, then v009's in the same run in parentheses: towers,
level 1 to 3 units trained and gold kept a game; timeouts; median and
maximum answer after the first.

| Variant | Elo against v009 | Score as red | Score as blue | Towers | Mines | Level 1 / 2 / 3 | Gold kept | Timeouts | Median, max ms |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | ---: | --- |
| V = 100 | −17.4 [−39.0, +4.1] | 34.8% | 60.2% | 2.91 (3.11) | 2.63 (1.83) | 24.6 / 3.4 / 1.3 (22.8 / 4.0 / 1.4) | 18.7 (20.2) | 6 (4) | 13.8, 101.8 (13.1, 92.2) |
| E022: no static tower value | **−46.3 [−67.1, −25.8]** | 34.5% | 52.2% | 2.58 (3.50) | 1.83 (1.91) | 24.8 / 4.5 / 1.4 (26.7 / 4.4 / 1.5) | 27.9 (22.3) | 2 (1) | 14.6, 109.1 (14.4, 104.0) |
| V = 200 | +1.7 [−18.6, +22.1] | 36.2% | 64.2% | 2.98 (2.94) | 2.60 (2.03) | 23.6 / 3.2 / 1.3 (22.6 / 3.6 / 1.5) | 21.8 (19.2) | 3 (2) | 12.8, 60.4 (13.6, 67.0) |
| V = 50 | **−97.1 [−120.2, −74.9]** | 22.5% | 50.2% | 2.67 (3.37) | 2.45 (2.00) | 24.5 / 3.7 / 1.3 (25.8 / 4.3 / 1.3) | 15.2 (19.8) | 1 (1) | 14.7, 102.3 (13.8, 107.7) |
| V = 200, 600 pairs, seed 23024 | −5.2 [−21.8, +11.4] | 33.0% | 65.5% | 3.01 (3.03) | 2.70 (1.93) | 23.7 / 3.3 / 1.3 (22.7 / 3.7 / 1.5) | 21.7 (20.5) | 8 (4) | 12.3, 126.7 (12.5, 135.0) |

The 99th percentile of every bot's answers was 32.0 ms, and its slowest
first answer 636 to 677 ms. Timeouts were 20 in the variants' 4,400 games
against 12 for v009 in the same games, all below the SPRT's 1%.

## What was learned

- **The discount does not stop the far towers.** At every V the variants
  built about as many towers as v009 (2.9 to 3.0 against 2.9 to 3.1 at
  V = 100 and 200). The study already found that only part of v009's
  towers come from the pressure term (the static +10 makes about half of
  them, the opponent's replies others), and a tower usually blocks
  trainings on the front whatever the HQ threat.
- **The scale matters more than the shape.** V = 50 is clearly weaker,
  V = 100 a little weaker, V = 200 even with v009 on 1,000 pairs (−2.4
  over both runs). A discounted term counts far threats like a full one:
  at V = 200 anything 3 or more incomes away is −185 to −200, close to the
  old term's floor of −200. What the old term adds is a steep, large
  reward for bringing our own attack near; with a smaller V it fades, and
  the variants build more mines (2.5 to 2.7 against about 2.0) instead.
- **The static tower value is worth keeping** (E022's control): without it
  v009 builds a quarter fewer towers, keeps more gold unspent (27.9
  against 22.3) and loses 46 Elo. This and E020 together: towers valued
  much more or much less than +10 both lose.
- Not tried, as the study proposes them only after a winning V: r = 0.6;
  V above 200 would be close to the old term again.
