---
id: E034
date: 2026-11-13
pull_request: "#294"
parent: ice-and-fire-v015
hypothesis: v015's opening builds a mine early (at its turn 6 or 7 in most games, where the rest of Legend trains) and plans its turns by a score that weighs income heavily; an opening that favours expansion, with no mine and income weighed less than land, reaches the middle game with more land and beats v015 (candidate `ice-and-fire-v016`).
change: The opening uses no mine spot (its safe spot set to 255) and its planner's score weighs cells 3 and income 1 rather than 4 and 3 (gold 1, voronoi 1 and reach 6 unchanged).
release: ice-and-fire-v016
sprt: accepted
elo: +31.0 [+13.1, +49.1] at CodinGame's limits
pairs: 640
full_time: +31.0 [+13.1, +49.1] (the SPRT's own, ADR 0042), no fault of v016 in 1,280 games (v015 1 timeout), slowest answer 626.6 ms (the first turn)
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-11-13, after E033 ("I think the evaluation
should favor more the expansion (voronoi and such metrics) than economy in
the early game", then "do the next experiment. Try many variants in
parallel"). It is also the third experiment proposed by the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
"The first mine").

## Why

The opening ([`opening/`](../opening/)) plays until the middle game, by
rules (E003) whose weights and purchases the solo planner (E010) chooses
turn by turn with a beam search over our next 7 turns, as if the enemy
passed. Its score of the position reached weighs cells 4, income 3, gold
1, closeness to the enemy HQ 6, and the land nearer our cells than the
enemy's (`voronoi`) only 1; its purchases buy a mine whenever it pays back
more than a training by the horizon. E033 left it unchanged: in E033's local
matches with game records (seed 33040), v015 built a mine at its own
turn 6 in 29% of games and at turn 7 in 67% to 68%, as v014 did. In the report the rest of Legend builds that early mine in
about 6% of games, and from turn 10 our wins and losses differ most in
cells and voronoi. The report could not tell whether the early mine hurts
(it goes with the map), and earlier changes to the opening were dropped
(E011, E013), so it is screened.

## The change

`bots/plan/src/main.rs`:

- `OPENING_SAFE_SPOT`, new, sets the opening's `safe_spot` (a mine spot is
  used only when the enemy's territory is more than that many steps away)
  in every weight set of the planner: `u8::MAX`, so the opening builds no
  mine (v003's 3 before). The middle game's search still builds them, and
  E033 makes it build them on a large territory.
- `OBJECTIVE`, the planner's score of the position at the end of the early
  game: cells 3 and income 1, rather than 4 and 3; gold 1, voronoi 1,
  reach 6 and the shape terms unchanged.

A test plays the bot's turns from the start of four of the referee's maps
and checks that no turn of the opening builds a mine; with v015's safe
spot it fails at seed 1 (a mine at (1, 0)).

The candidate's code plays the same games as the screened commit
`1b4b119` at 500 fixed tries (12 games against v015, 511 turns, compared
as `scripts/check-minified.sh` compares records).

## Screening

All against `ice-and-fire-v015` on GitHub (`screen.yml`, 800 pairs each
over two calibrated runners; no timeout, crash or invalid answer on
either side in any match). Each variant is a commit of this pull
request. "No mine" is a safe spot of 255, "safe 5" of 5; the weights are
the planner's score's (v015: cells 4, income 3, gold 1, voronoi 1, reach
6), only those named changed.

First round, runs 43425890486 (seed 34001) and 43425893737 (seed 34002):

| Variant | Commit | Elo against v015 | Score |
| --- | --- | --- | ---: |
| no mine | `03b96a3` | **+22.6 [+7.0, +38.3]** | 53.2% |
| safe 5 | `6ae691b` | +8.7 [−0.1, +17.5] | 51.2% |
| voronoi 3 | `1610fe7` | +1.3 [−13.1, +15.7] | 50.2% |
| voronoi 6 | `973808c` | **−17.4 [−33.0, −1.9]** | 47.5% |
| voronoi 10 | `553f874` | **−29.2 [−45.4, −13.0]** | 45.8% |
| no mine, voronoi 3 | `339a977` | −0.9 [−16.8, +15.1] | 49.9% |
| no mine, voronoi 6 | `fbe2ac1` | −2.2 [−17.7, +13.4] | 49.7% |
| safe 5, voronoi 3 | `6032904` | −6.5 [−20.9, +7.9] | 49.1% |
| income 2 | `35f5af6` | +10.4 [−2.8, +23.7] | 51.5% |
| income 1 | `8e67817` | **+21.7 [+7.2, +36.4]** | 53.1% |
| income 1, voronoi 3 | `184379e` | −1.3 [−16.8, +14.2] | 49.8% |
| cells 6 | `c3da14b` | −6.1 [−18.4, +6.2] | 49.1% |
| cells 6, voronoi 3 | `edba09b` | **−14.8 [−29.3, −0.3]** | 47.9% |
| gold 0 | `f5f5660` | +2.2 [−12.4, +16.8] | 50.3% |
| no mine, income 1 | `adaed51` | **+43.7 [+27.9, +59.6]** | 56.2% |
| no mine, cells 6, voronoi 3 | `a858da1` | −4.8 [−20.6, +11.0] | 49.3% |

Second round, run 43428591718 (seed 34003), the leader's neighbours:

| Variant | Commit | Elo against v015 | Score |
| --- | --- | --- | ---: |
| no mine, income 0 | `f92e99b` | **+29.6 [+13.9, +45.4]** | 54.2% |
| no mine, income 2 | `dba8742` | **+20.4 [+4.8, +36.2]** | 52.9% |
| no mine, income 1, gold 0 | `109a217` | **−24.8 [−40.1, −9.6]** | 46.4% |
| no mine, income 1, reach 4 | `e3b05f4` | **+27.4 [+11.9, +43.0]** | 53.9% |
| no mine, income 1, reach 9 | `12b46b1` | +1.7 [−13.5, +17.0] | 50.2% |
| safe 5, income 1 | `6d7a788` | −3.5 [−17.5, +10.6] | 49.5% |
| no mine, income 1, cells 3 | `1b4b119` | **+39.7 [+24.4, +55.1]** | 55.7% |
| income 0 | `e9956aa` | −11.7 [−25.8, +2.3] | 48.3% |

Confirmation of the first round's three best, run 43428595394 (seed
34004), 1,600 pairs each over four runners:

| Variant | Commit | Elo against v015 | Score | All its pairs |
| --- | --- | --- | ---: | --- |
| no mine, income 1 | `adaed51` | **+21.7 [+10.8, +32.7]** | 53.1% | about +29 over 2,400 |
| no mine | `03b96a3` | **+20.4 [+9.2, +31.7]** | 52.9% | about +21 over 2,400 |
| income 1 | `8e67817` | **+18.3 [+7.8, +28.8]** | 52.6% | about +19 over 2,400 |


Second confirmation, run 43431048562 (seed 34005), 1,600 pairs each over
four runners:

| Variant | Commit | Elo against v015 | Score | All its pairs |
| --- | --- | --- | ---: | --- |
| no mine, income 1, cells 3 | `1b4b119` | **+38.8 [+27.4, +50.4]** | 55.6% | about +39 over 2,400 |
| no mine, income 0 | `f92e99b` | **+22.4 [+11.1, +33.7]** | 53.2% | about +25 over 2,400 |
| no mine, income 1, reach 4 | `e3b05f4` | **+32.9 [+21.6, +44.2]** | 54.7% | about +31 over 2,400 |
| no mine, income 1, cells 3, reach 4 | `98c4051` | **+39.5 [+28.1, +50.9]** | 55.7% | 1,600 only |

Every match on GitHub was free of timeouts, crashes and invalid answers.

## The candidate

`ice-and-fire-v016`: no mine in the opening, income 1, cells 3, the best
over 2,400 pairs (+39.7, then +38.8); adding reach 4 scored the same and
is one change more. Frozen by `scripts/new-release.sh` (41,005
characters; the same answers as its readable copy in 4 games).

In a local match against v015 with game records (150 pairs, seed 34010,
time scale 0.575, tolerance 7 ms: +39.5 [+2.2, +77.8], no fault), v016
built no mine before its own turn 8, and its first mine at its turn 11 in
the median game (247 of 300 games), then one in 18% to 22% of games at
every turn from its turn 10 to 15, about as many a game as v015 (3.21
against 3.16). v015, in these games, built its first at turn 7 (a mine in
35% of games at turn 6, 68% at turn 7).

## What was learned

- **The early mine was the cost, not the mine.** Without it the bot builds
  as many mines a game, four turns later, once its territory is larger:
  the pattern the report found for the top 10 (a mine in about 20% of
  games at every turn from turn 9). No mine alone was +20 (2,400 pairs).
- **Land, yes; voronoi weight, no.** Weighing income less (1, or 0 with no
  mine) and cells a little less scored +18 to +39: the planner's score
  then leans on what it already counts as expansion, the pull toward the
  enemy HQ (reach 6) and the land nearer us (voronoi 1). Raising voronoi
  itself to 3, 6 or 10 was level to clearly weaker (+1, −17, −29), and
  cancelled the gain of no mine or of income 1 when added to them; cells 6
  was level to weaker too. Gold must stay (gold 0, −25 with no mine).
- **The two halves add up, partly.** No mine (+20) and income 1 (+19)
  gave +29 together, and +39 with cells 3.
- **Screening many variants in parallel.** 25 variants in five runs on
  GitHub took about an hour; the first round's leader dropped
  from +43.7 to +21.7 on new pairs, hence the confirmations.

## SPRT

Accepted on the pull request (run 43434902614, at aeb6787), at CodinGame's
speed and real limit, in blocks of 10 pairs over 8 calibrated runners:

```
Smoke test against ice-and-fire-random: 200 wins in 200 games, no fault.
ice-and-fire-v016 vs ice-and-fire-v015: 1280 games, 640 complete pairs
  ice-and-fire-v016: 697 wins, 0 draws, 583 losses (score 54.5%)
  Elo of ice-and-fire-v016 over ice-and-fire-v015: +31.0 [+13.1, +49.1] (95%, from pairs)
  pairs by ice-and-fire-v016's points (0, 1/2, 1, 3/2, 2): [114, 0, 355, 0, 171]
  ice-and-fire-v016 faults: 0 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v015 faults: 1 timeouts, 0 crashes, 0 invalid answers
  slowest answer: ice-and-fire-v016 626.6 ms, ice-and-fire-v015 627.6 ms
Checked 8 rounds of 8 blocks (64 blocks counted, 16 left out)
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.14 (bounds -2.94, 2.94) after 640 pairs
Verdict: ACCEPTED: ice-and-fire-v016 is stronger than ice-and-fire-v015
```

The SPRT's +31.0 is within the screening's +39 over 2,400 pairs (an SPRT
that stops early on a lucky run tends to read high, the screening's chosen
leader too). The nightly league rates the release.
