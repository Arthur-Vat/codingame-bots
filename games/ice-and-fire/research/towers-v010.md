# Towers and threats in v010's games

The same study as [towers-and-threats.md](towers-and-threats.md), on `ice-and-fire-v010`, the candidate of E023 (pull request #219: a cell the enemy can take at its next turn counts 0.75 of its value), to see what that change did to towers before the next tower experiment. The owner asked for it on 2026-10-30 ("Study v010's towers (Recommended)"). No change to the bot.

## Method

The same commands, games and seed as v009's study, with rustc 1.90.0 and v010's release:

```sh
rustc +1.90.0 --edition 2021 -C opt-level=3 -o v010 games/ice-and-fire/releases/ice-and-fire-v010.rs
target/release/ice-and-fire-arena match --bot v010=./v010 --bot v010b=./v010 \
  --pairs 500 --seed 22022 --jobs 2 --records records
ice-and-fire-review towers --jobs 4 --csv csv records/*.json > towers.md
ice-and-fire-review screening --jobs 4 records/*.json > screening.md
```

1,000 games of v010 against itself, 500 seat-swapped pairs at CodinGame's limits (50 ms a turn, 1,000 ms the first, no tolerance), two games at a time on a 4-core machine. The records stay out of the repository; the seed rebuilds them. The position bank was not redrawn.

## v010 against v009

| Self-play, 1,000 games | v009 | v010 |
| --- | ---: | ---: |
| Red's wins | 41.7% | 35.6% |
| Towers a side a game | 3.66 | 3.99 |
| Towers built against threats 5 or more enemy turns away, or never | 48% | 49% |
| Towers that never blocked a training or a unit | 29.5% | 34.8% |
| Towers later cut off from their HQ | 16.2% | 13.0% |
| Towers later lost | 15.7% | 13.7% |
| Builder's gold when it builds (median) | 44 | 53 |
| Enemy's gold then (median) | 9 | 16 |
| Gold kept a turn (the two bots) | 25.6, 27.0 | 30.8, 31.5 |
| Large cut (6+ cells) in reach now, 6+ cells lost that turn | 39.6% | 35.2% |
| The same, first in reach with one more income, lost within one turn | 23.0% | 22.1% |
| A threat to the HQ, decay a turn of "taken within 5 turns" (fit) | 0.420 | 0.395 |
| Timeouts (the two bots) | 2, 1 | 13, 10 |

What it says:

1. **E023 did what it was meant to.** A large cut in reach is made less often (35% against 40%), towers are cut off or lost less often, and both sides bank more gold, which also widens their own reach.
2. **The towers are not better placed.** v010 builds about 9% more towers, as large a share of them against far threats, and more that never block a training or a unit (35%). Their raise of the enemy's chain is the same as v009's (median 20); their gain in the pressure term has a median of 20 against 10, with close means (14.5 against 14.1).
3. **The threats to the HQ decay as before** (0.395 a turn against 0.420): E021's reading still holds.
4. **The timeouts are this machine's:** the SPRT of v010 on pull request #219, on GitHub's runners, had none in 696 games, and the screenings had 6 in 2,000.

## For the next tower experiment

Changing what a tower is worth lost or tied four times: E020's towers +15, and E021, E022's control and E024, whose entries are on the branches `claude/ice-and-fire-e021-discounted-pressure` and `claude/ice-and-fire-e024-tower-cap` until merged. The owner chose next to change where towers go: offer towers on the cells where they most shrink the enemy's reach into our territory, the measure v010's evaluation already scores, aiming at the third of the towers that block nothing.
