# Towers and threats in v009's games

How much is a tower worth, and how urgent is a far threat? In one of the owner's games, v008 built a tower worth +20 to its evaluation only through the pressure term: the enemy's cheapest chain to its HQ went from 140 to 180 gold while the enemy had 3 gold and +17 income, so the threat was many turns away. The pressure term (`ice_and_fire_tactics::eval`, the shortfall `gold + income − cost` clamped to [−200, 0]) counts a far threat almost like a near one, and each active tower also counts a static +10. [E020](../journal/E020-level-two-units.md) showed that valuing towers more is clearly weaker. This note is phase 0 of the study the owner approved on 2026-10-29: data and tools for the next experiments (E021 to E025), no change to the bot.

## Method

Everything below comes from these commands, at the head of pull request #215 (the tools), with rustc 1.90.0:

```sh
rustc +1.90.0 --edition 2021 -C opt-level=3 -o v009 games/ice-and-fire/releases/ice-and-fire-v009.rs
target/release/ice-and-fire-arena match --bot v009=./v009 --bot v009b=./v009 \
  --pairs 500 --seed 22022 --jobs 2 --records records
ice-and-fire-review towers --jobs 4 --csv csv records/*.json > towers.md
ice-and-fire-review screening --jobs 4 records/*.json > screening.md
ice-and-fire-review bank --jobs 4 --out bank.txt --count 5000 --seed 22022 records/*.json
cargo +1.90.0 run --release -p ice-and-fire-bot-plan --features analysis -- \
  bank --positions bank.txt --tries 600 --out bank.tsv > bank.md
```

- **Games:** 1,000 games of v009 against itself, 500 seat-swapped pairs at CodinGame's limits (50 ms a turn, 1,000 ms the first, no tolerance), seed 22022, two games at a time on a 4-core machine. The records stay out of the repository; the seed rebuilds them.
- **Chains:** the enemy's cheapest chain to an HQ is the one the pressure term counts: `cost_to` without moves, on the position's costs (a tower's protection taken as lasting). "Turns to afford" at the start of a side's turn is the smallest k ≥ 0 with gold + k × income ≥ cost (0: it can pay now).
- **Towers:** a tower is one the rules accepted. Its effect on the chain is measured just before and just after its own command; "the enemy's turns to afford" counts from the enemy's next turn (1). "Blocked a training" and "blocked a unit" are generous: at a later enemy turn, a cell the tower alone protects could otherwise have been entered at level 1 or 2, by a training (next to the enemy's territory, with the gold) or by an adjacent enemy unit; they do not say the enemy wanted to.
- **Cuts:** the review tool's cut measure: the most enemy active cells among the 8 best cuts of up to 5 trainings, with the gold of now, and of one and two more incomes on the same position.
- **Bank:** 5,000 positions drawn among the 14,169 middle-game plies (from the middle game's first ply to the endgame's, as the trace cuts them). Each is searched with a fixed 600 tries and no deadline, so the result does not depend on the machine. 600 is v009's median at CodinGame's limits on this machine: 598 tries over 269 middle-game turns of 6 games (`--pairs 3 --seed 5 --show-bot-stderr`, the debug line's `tries`).

## The games

Red won 41.7%; 997 of the 1,000 games ended by taking an HQ, so "the HQ was taken in the game" also says who won. The screening report of the batch, the reference for later screenings against v009:

| Bot | Games | Score | As red | As blue | Towers | Mines | Level 1 | Level 2 | Level 3 | Gold kept | Kept share | Timeouts | Median ms | 99% ms | Max ms | Max first ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| v009 | 1000 | 47.7% | 39.4% | 56.0% | 3.60 | 2.01 | 25.30 | 4.19 | 1.60 | 25.6 | 52.1% | 2 | 19.4 | 32.1 | 75.2 | 619.5 |
| v009b | 1000 | 52.3% | 44.0% | 60.6% | 3.73 | 2.20 | 25.76 | 4.31 | 1.67 | 27.0 | 52.0% | 1 | 19.7 | 32.0 | 76.4 | 843.5 |

The two copies differ by −16.0 Elo [−33.4, +1.3], within noise.

## Towers

v009 builds many towers: 7,328 in 1,000 games, 3.7 a side a game (3.1 by red, 4.3 by blue); 157 of the 2,000 sides built none. Winners and losers build as many (3.65 and 3.68 a game).

| When built and with what | Quartile 1 | Median | Quartile 3 |
| --- | ---: | ---: | ---: |
| Game turn | 11 | 14 | 18 |
| Builder's gold at the start of its turn | 31 | 44 | 72 |
| Builder's income | 22 | 27 | 33 |
| Enemy's gold | 4 | 9 | 29 |
| Enemy's income | 20 | 26 | 32 |
| Enemy's chain to the builder's HQ, just before | 100 | 130 | 160 |
| The same just after the tower | 110 | 140 | 180 |
| The same after the builder's whole turn | 120 | 150 | 180 |
| The tower's raise of that chain | 0 | 20 | 20 |
| The tower's gain in the pressure term | 0 | 10 | 20 |

By how far the enemy was from affording its chain just before the tower (its turns; 1 = its next turn):

| Enemy's turns to afford | Towers | Mean pressure gain | Raised the chain | Blocked a training | Blocked a unit | Enemy afforded the old chain | Cut off | Gone | Builder won | Builder lost its HQ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 70 | 15.6 | 92.9% | 78.6% | 72.9% | 100.0% | 5.7% | 4.3% | 1.4% | 98.6% |
| 2 | 729 | 17.7 | 81.6% | 68.2% | 62.3% | 69.1% | 11.5% | 11.2% | 9.2% | 90.8% |
| 3-4 | 3020 | 13.8 | 64.6% | 72.4% | 67.5% | 54.4% | 22.4% | 15.9% | 31.9% | 67.9% |
| 5-9 | 2428 | 14.9 | 67.2% | 69.4% | 64.1% | 28.5% | 14.3% | 17.9% | 68.6% | 31.2% |
| 10+ | 949 | 10.5 | 62.3% | 64.2% | 58.3% | 8.0% | 7.1% | 14.3% | 88.2% | 11.6% |
| never | 132 | 12.2 | 56.1% | 51.5% | 69.7% | 17.4% | 5.3% | 13.6% | 85.6% | 14.4% |
| all | 7328 | 14.1 | 66.9% | 69.6% | 64.8% | 41.1% | 16.2% | 15.7% | 49.8% | 50.1% |

Median enemy turns after the build, among the towers it happened to: first blocked a training 1, first blocked a unit 1, the enemy had the gold of the old chain 6, cut off 5, gone 6. 2,165 towers (29.5%) never blocked a training or a unit.

What it says:

1. **The owner's case is the common one.** 3,509 towers (48%) were built when the enemy was 5 or more turns from affording its chain, or never at its income. Their mean gain in the pressure term (10.5 to 14.9) is about that of towers against threats 2 to 4 turns away (13.8 to 17.7): the term pays for a far threat as for a near one. Against those far threats the enemy later reached the gold of the chain the tower raised in only 22.6% of the cases (8.0% at 10 turns or more).
2. **Towers against far threats are built by the side ahead.** Their builder won 69% (5 to 9 turns) to 88% (10 or more) of the games; towers against threats 1 or 2 turns away were built by sides that went on to lose about 90% of their games. Towers follow the state of the game more than they change it, as E020 found.
3. **Towers stand on the front whatever the threat.** About 70% block something at the enemy's next turn in every bucket, and 30% never block anything; the distance of the HQ threat does not tell a useful tower from a useless one. 16% of the towers are later cut off from their HQ and 16% lost, a median five or six enemy turns later.

## Threats to the HQ

At the start of each ply, the turns until the player to act could pay its chain to the enemy HQ, against whether and when it took that HQ (its turns from this one, 0 being this one):

| Turns to afford | Plies | Taken in the game | Relative | Within k + 2 | Relative | Within 5 turns | Relative |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 727 | 100.0% | 1.03 | 100.0% | 1.51 | 100.0% | 1.23 |
| 1 | 2693 | 96.8% | 1.00 | 66.1% | 1.00 | 81.4% | 1.00 |
| 2 | 5740 | 84.7% | 0.87 | 28.1% | 0.43 | 36.1% | 0.44 |
| 3 | 4908 | 63.7% | 0.66 | 11.9% | 0.18 | 11.9% | 0.15 |
| 4 | 4779 | 49.1% | 0.51 | 7.2% | 0.11 | 4.7% | 0.06 |
| 5 | 3336 | 38.5% | 0.40 | 5.0% | 0.08 | 2.1% | 0.03 |
| 6 | 2431 | 29.5% | 0.30 | 4.7% | 0.07 | 1.0% | 0.01 |
| 7 | 1950 | 36.3% | 0.37 | 6.8% | 0.10 | 1.0% | 0.01 |
| 8 | 1774 | 30.1% | 0.31 | 6.0% | 0.09 | 0.5% | 0.01 |
| 9 | 1003 | 23.4% | 0.24 | 5.6% | 0.08 | 0.7% | 0.01 |
| 10 | 694 | 15.1% | 0.16 | 5.0% | 0.08 | 0.9% | 0.01 |
| 11 | 669 | 24.7% | 0.25 | 6.4% | 0.10 | 0.7% | 0.01 |
| 12 | 1151 | 39.8% | 0.41 | 16.6% | 0.25 | 0.0% | 0.00 |
| 13 | 452 | 19.5% | 0.20 | 6.9% | 0.10 | 0.2% | 0.00 |
| 14 | 491 | 26.3% | 0.27 | 11.2% | 0.17 | 0.2% | 0.00 |
| 15 | 831 | 32.4% | 0.33 | 18.4% | 0.28 | 0.6% | 0.01 |

Maximum-likelihood fits over 1 to 15 turns, P(k) = p × r^(k − 1):

| Outcome | p | r |
| --- | ---: | ---: |
| Taken in the game | 0.955 | 0.845 |
| Taken within k + 2 turns | 0.360 | 0.780 |
| Taken within 5 turns | 0.815 | **0.420** |

- **A threat's urgency falls by about 0.42 a turn.** Taken within 5 turns: 81% of threats one turn from affordable, 36% at two, 12% at three, 5% at four, 2% at five, then about 1%. "Within k + 2" falls as fast at first (1, 0.43, 0.18, 0.11) and then stays at 5 to 7%: later, unrelated threats; its fit's 0.78 comes from that floor. "Taken in the game" is mostly who wins.
- **A capture is seen one or two turns ahead, and the attacker waits near there.** One turn before a capture the taker could afford its chain within one turn in 75.0% of the games, two turns before within two in 88.7%; the median turns to afford was 2 at every one of the 2 to 6 turns before the capture: the defender keeps the attacker about two turns away for several turns, until it breaks.
- **For the evaluation**, which looks at the position after our turn, the enemy's next turn adds one income: the eval's "enemy turns to afford" n is this table's k + 1, and the pressure term is 0 when n = 1.

## Cuts

| Cells of the best cut with the gold of now | Plies | Median cost | Median cells lost | Lost at least those cells | Lost at least half |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1-2 | 8772 | 20 | 0 | 31.1% | 40.1% |
| 3-5 | 10603 | 40 | 1 | 29.1% | 41.9% |
| 6-9 | 5530 | 50 | 2 | 28.1% | 44.3% |
| 10-19 | 3484 | 70 | 3 | 29.7% | 44.6% |
| 20+ | 894 | 100 | 7 | 30.6% | 44.6% |

A large cut (6 cells or more) in reach with the gold of now was followed by a loss of 6 or more cells in that turn in 39.6% of 9,908 plies (53.6% by the next turn). One first in reach with one more income: 23.0% within one turn, 35.9% within two (10,138 plies); with two more incomes: 14.4% within two turns (3,586). The best cut the tool finds is made in full only 30% of the time, whatever its size: the side to act often prefers something else, or the cut's cells are not all worth taking.

## The position bank

| Function (release build, rustc 1.90.0) | Median µs | Mean µs |
| --- | ---: | ---: |
| The bot's evaluation (`eval`) | 3.49 | 3.64 |
| The shared evaluation (`evaluate`) | 1.13 | 1.23 |
| Its pressure term (both sides) | 0.33 | 0.37 |

The pressure term is not 0 in 98.9% of the positions (mean absolute value 30.1), the tower term in 59.9% (10.2).

| Search, 600 tries | First command differs from v009's | Plan differs as a set of commands | Plans with a tower | Towers in all plans |
| --- | ---: | ---: | ---: | ---: |
| v009 | – | – | 1149 (23.0%) | 1279 |
| v009, twice the tries (control) | 2131 (42.6%) | 2074 (41.5%) | 1267 (25.3%) | 1462 |
| no pressure term | 4128 (82.6%) | 4606 (92.1%) | 584 (11.7%) | 617 |
| no static tower value | 1163 (23.3%) | 1455 (29.1%) | 574 (11.5%) | 611 |

- **The search is unstable by itself:** twice the tries changes 41.5% of the plans. Only changes well beyond that say something about a term.
- **The pressure term steers almost every plan:** without it 92.1% of the plans change. The most common change is a move that becomes a level-1 training (1,261 positions); empty plans become mines and trainings (59 + 49); towers in plans halve.
- **The static +10 makes about half of the towers:** without it the plans change no more than the control's (29.1%), but the towers in them halve (1,279 to 611), mostly into moves and level-1 trainings (158 and 72 of the first commands). The other half come from the pressure term and the replies.
- **The cost is no obstacle:** the pressure term is 0.33 µs of the bot's 3.49 µs; a discount of it adds next to nothing.

## Proposal for E021 to E025

**E021, time-discounted pressure.** Replace each side's pressure `clamp(gold + income − cost, −200, 0)` by

    pressure = −V × (1 − r^t),  t = max(0, cost − gold − income) / max(income, 1)

(t is the enemy turns beyond its next one, fractional; an unreachable HQ gives −V), with **r = 0.42** from the within-5-turns fit. The term then pays for a near threat and almost nothing for a far one:

| t (n − 1) | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| V = 200 | 0 | −116 | −165 | −185 | −194 | −197 | −199 |
| V = 100 | 0 | −58 | −82 | −93 | −97 | −99 | −99 |
| r = 0.6, V = 200 | 0 | −80 | −128 | −157 | −174 | −184 | −191 |

A tower that pushes the enemy from t = 5 to t = 6 then gains 1.5 at V = 200 (now about 15), one from t = 1 to t = 2 still 49. The near end is steeper than today's term (about 1 a gold; here about 6.7 a gold at t = 0 with V = 200 and an income of 26), so the scale V is the first thing to screen: **V = 100 first, then V = 200 and V = 50**, all at r = 0.42; r = 0.6 only if one of them wins. The same function counts our own pressure on the enemy, so our attack is also valued only when it is near.

**E022, controls.** No static tower value is the cheaper and the more informative one: it halves the towers in plans while changing no more plans than twice the tries does. Screen it right after E021's first variant. No pressure term at all changes 92% of the plans; keep it as a control of how much E021's term matters, not as a candidate.

**E023, safe territory.** A large cut first in reach with one more income is made within one turn 23% of the time, against 40% for one in reach now with the gold of the turn (0.396, 0.230, 0.144 at 0, 1 and 2 more incomes, about 0.6 a turn). So a cell the enemy can take at its next turn is lost about a quarter of the time: a first weight counts such cells at 0.75 of a cell's value, and the cut threat itself decays by r = 0.6 a turn rather than the HQ threat's 0.42.

**E024, a tower's value capped by a level-3 unit's extra cost.** A tower raises the enemy's chain by a median 20 (quartile 3: 20), which is a level-3 training's extra cost over level 1 (30 − 10). Cap what a tower adds (the static term plus its pressure gain) at 20.

**E025, towers on interior joints.** This study does not measure joints under towers, so it gives no parameter. The action generator offers towers only on border cells; the bank's positions can test, with the analysis mode, how often a joint tower would be chosen before a screening.

**Order to screen** (each against v009, 400 pairs as in E020, read with `ice-and-fire-review screening`): E021 at V = 100, E022 without the static tower value, E021 at V = 200 and V = 50, E024, then E023 and E025.
