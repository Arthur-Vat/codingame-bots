---
id: E032
date: 2026-10-16
pull_request: "#152"
parent: uttt-v017
hypothesis: E030's PUCT settings (temperature 0.1, weight 0.25) were chosen from six coarse settings at 20 ms; at CodinGame's limits the search runs about five times the iterations, so sharper priors with a larger weight should send them to the network's best children better, and the bot is stronger.
change: PUCT's prior temperature `VALUE_TEMPERATURE` from 0.1 to 0.05 and its weight `PRIOR_WEIGHT` from 0.25 to 0.5; nothing else.
release: uttt-v018
sprt: accepted
elo: +42.0 [+20.0, +64.3] at CodinGame's limits
pairs: 162
full_time: +42.0 [+20.0, +64.3] (the SPRT's own, ADR 0020), no timeout in 324 games, 99.9% of answers within 93.0 ms
decision: promoted
cg_rank: not reported
---

Proposed to the owner on 2026-10-16 among four next steps after [E031](E031-fit-target.md), and chosen the same day ("Tune PUCT (Recommended)"). [E030](E030-puct.md) said: "The temperature and weight were screened coarsely at 20 ms (six settings), so finer tuning at full time may add more."

**The settings** (`cg-search`'s `Mcts::value_temperature` and `prior_weight`, E030): a node's children get the priors `softmax(estimate / T)`, and selection adds `W · prior · sqrt(parent visits) / (visits + 1)` to UCB1's bound. A lower temperature puts more of the prior on the children the network rates best; a higher weight lets the priors steer longer before the visits take over.

**Screening against `uttt-v017`** at CodinGame's limits (Claude's 4-core session, both bots compiled with Rust 1.90.0 and `-C opt-level=3` from `uttt-v017.rs` with only the two constants changed, 4-ply openings, seed 1, 3 games at a time, 100 pairs each), in the order played, each batch chosen from the previous ones:

| Temperature | Weight | Wins, draws, losses | Elo |
| --- | --- | --- | --- |
| 0.05 | 0.25 | 52, 104, 44 | +13.9 [-12.8, +40.8] |
| 0.2 | 0.25 | 51, 95, 54 | -5.2 [-33.2, +22.7] |
| 0.1 | 0.15 | 40, 99, 61 | -36.6 [-63.7, -10.0] |
| 0.1 | 0.4 | 55, 99, 46 | +15.6 [-10.8, +42.3] |
| 0.07 | 0.35 | 54, 107, 39 | +26.1 [-3.4, +56.0] |
| 0.1 | 0.6 | 51, 106, 43 | +13.9 [-16.8, +44.9] |
| 0.1 | 1.0 | 46, 98, 56 | -17.4 [-46.3, +11.3] |
| 0.05 | 0.5 | 59, 105, 36 | +40.1 [+10.7, +70.1] |
| 0.03 | 0.5 | 62, 101, 37 | +43.7 [+18.3, +69.5] |
| 0.05 | 0.75 | 57, 105, 38 | +33.1 [+4.6, +62.0] |
| 0.03 | 0.75 | 68, 98, 34 | +59.6 [+31.4, +88.7] |
| 0.07 | 0.5 | 68, 86, 46 | +38.4 [+15.3, +61.7] |
| 0.02 | 0.75 | 49, 109, 42 | +12.2 [-13.0, +37.4] |
| 0.03 | 1.0 | 75, 81, 44 | +54.3 [+24.3, +85.1] |
| 0.02 | 1.0 | 69, 81, 50 | +33.1 [+5.5, +61.2] |
| 0.01 | 1.0 | 60, 87, 53 | +12.2 [-21.0, +45.5] |

Every setting with a temperature from 0.03 to 0.07 and a weight from 0.5 to 1.0 scored +33 to +60, each interval above 0; a weight below 0.25, a temperature of 0.01 or 0.02 at weight 0.75 lost most of it (0.02 at weight 1.0 kept +33.1). All 16 played the same 100 openings, and the best of 16 noisy results overstates its own gain, so the two best centres of that region were played again on 150 new openings (seed 2), the same way:

| Temperature | Weight | Wins, draws, losses | Elo | Timeouts (candidate, v017) |
| --- | --- | --- | --- | --- |
| 0.03 | 0.75 | 98, 139, 63 | +40.7 [+19.6, +62.1] | 0, 0 |
| 0.05 | 0.5 | 99, 143, 58 | +47.8 [+23.3, +72.7] | 1, 0 |

Over their 250 pairs the two are even (about +48 and +45). The candidate takes 0.05 and 0.5: the setting closer to E030's, and one whose second result was not the maximum it was picked for. 99.9% of its answers came within 94.9 ms on seed 2 (94.9 ms for `uttt-v017`). The bundle takes 71,871 characters, as `uttt-v017`'s.

The analysis program and the Train workflow's nnue self-play search with the new settings, as the newest release does (their tests check it).

**SPRT** ([run 38932514058](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38932514058), on 13121ab), at CodinGame's limits, 4-ply openings, seed 1:

```
Smoke test against uttt-random:
uttt-v018 vs random: 200 games, 100 complete pairs
  uttt-v018: 200 wins, 0 draws, 0 losses (score 100.0%)

SPRT against uttt-v017:
uttt-v018 vs uttt-v017: 324 games, 162 complete pairs
  uttt-v018: 109 wins, 145 draws, 70 losses (score 56.0%)
  Elo of uttt-v018 over uttt-v017: +42.0 [+20.0, +64.3] (95%, from pairs)
  pairs by uttt-v018's points (0, 1/2, 1, 3/2, 2): [2, 23, 81, 46, 10]
  uttt-v018 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v017 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): uttt-v018 90.1, 91.0, 93.0 ms; uttt-v017 90.1, 91.0, 93.0 ms
  slowest answer: uttt-v018 94.6 ms, uttt-v017 95.2 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.97 (bounds -2.94, 2.94) after 162 pairs
Verdict: ACCEPTED: uttt-v018 is stronger than uttt-v017
```

An earlier SPRT on b92a390, the same bot before the review's test fix changed the code's hash, also accepted it: +46.3 Elo [+23.1, +69.9] after 151 pairs ([run 38931445429](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38931445429)). Both agree with the screening (+47.8 on seed 2).

**What follows.** PUCT's first settings, picked at 20 ms, left about 40 Elo at full time: with five times the iterations, the search gains from trusting the network's ranking of the children more (sharper priors, twice the weight). Settings tuned at 20 ms should be checked again at full time.
