---
id: E057
date: 2026-11-14
pull_request: "#328"
parent: ice-and-fire-v024
hypothesis: As blue, v023 and v024 keep 81% to 84% of their units next to another of ours at turns 5 to 7 (the top 10 54% to 68%) and 67% to 73% at turn 8 (35% to 39%): blue starts with two cells in its corner, and E051's crowd weight of 2 does not spread its units; a larger crowd weight when we play blue spreads them, takes more of the land between the HQs as blue, and beats v024.
change: None released. Screening only: the opening's rules using a crowd weight of 3, 4, 6 or 8 when we play blue (2 in v024, for both seats).
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-14 (a form after the study of v023's
CodinGame games, its 159 ranking games and 140 collected test games:
"Blue's opening, Mines turns 8-12, Exposure before the cut"), one of three
experiments run in parallel on `ice-and-fire-v024` (E057 to E059).

## Why

v023 against the top 10 (ranking and collected test games): it won 42%
as red (v022 14%) and 40% as blue (v022 42%). In its losses, in both
seats, it is a little behind at turn 10 (26 to 28 active cells against 30
to 33; a mine built in 13% to 24% of the games against 42% to 48%) and far
behind at turn 15 (29 to 30 cells and income 22 to 24 against 46 to 51 and
36 to 44), after a large cut (median turn 15) or a threat on its HQ
(turn 20); before the cut it leaves more cells the enemy can take than in
its wins at the same turn (median 4 against 2 to 3; as blue 10 or more in
27% of these losses, 2% of the wins).

In 40 arena pairs of v024 against v023 (seed 24024, scratch scan of
the records): as blue, 81% to 83% of the units stood next to another at
turns 5 to 7 in both releases, as red 32% to 60%; v024 already spends at
turn 6 as blue (8.5 gold, v023 0.5).

## The change

`opening/src/econ.rs`: `Params::blue_crowd`, the crowd weight when our
side is blue (`None`: `crowd`, in `Params::V003`); `bots/plan/src/main.rs`:
`OPENING_BLUE_CROWD` (`None`: v024, which the base commit keeps).

## Screening

Against `ice-and-fire-v024` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). Run 43596123750, seed 57057,
800 pairs each over two runners:

| Variant | Commit | Crowd as blue | Elo against v024 | Score |
| --- | --- | ---: | --- | ---: |
| bc4 | `1c6cbff` | 4 | −8.3 [−23.5, +6.9] | 48.8% |
| bc3 | `673a5ce` | 3 | −12.6 [−26.6, +1.4] | 48.2% |
| bc6 | `d0eb77f` | 6 | **−22.2 [−38.2, −6.3]** | 46.8% |
| bc8 | `5099347` | 8 | **−42.3 [−59.4, −25.5]** | 43.9% |

No variant gained; the owner dropped it ("Drop both (Recommended)").

## What was learned

A larger crowd weight as blue lost, the more so the larger: blue's
crowded units at turns 5 to 7 are not a matter of this weight. Blue starts
with two cells in its corner and trains near them; pushing its units apart
there breaks the territory's shape (as E051's heavy weights did for both
seats). The knob was removed: the pull request holds the journal entry
alone.
