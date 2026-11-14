---
id: E059
date: 2026-11-14
pull_request: "#329"
parent: ice-and-fire-v024
hypothesis: Before the large cut that decides most of v023's losses to the top 10 (median turn 15) it leaves more cells the enemy can take than in its wins at the same turn (median 4 against 2 to 3, as blue 10 or more in 27% of these losses); a threatened cell costing more, in every turn or only from turn 10 to 15, makes the search guard its cells before the cut and beats v024.
change: None released. Screening only: the plan bot's evaluation charging a threatened cell 6 or 7 quarters rather than 5 in every turn, or 7 or 8 only from game turn 10 to 15.
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

E047 charged more for threatened cells only when a side led and lost
heavily (the search stopped expanding); E023 and E037 set 7 then 5.

## The change

`bots/plan/src/eval.rs`: `EXPOSURE_QUARTERS` (`THREAT_QUARTERS`, 5: v024,
which the base commit keeps) and `EXPOSURE_TURNS`, the game turns in which
a threatened cell costs it.

## Screening

Against `ice-and-fire-v024` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). Run 43596132848, seed 59059,
800 pairs each over two runners:

| Variant | Commit | Quarters | Turns | Elo against v024 | Score |
| --- | --- | ---: | --- | --- | ---: |
| q7m | `16dd465` | 7 | 10 to 15 | +1.3 [−10.8, +13.4] | 50.2% |
| q6 | `5151d76` | 6 | all | −2.2 [−14.4, +10.0] | 49.7% |
| q7 | `1399916` | 7 | all | −5.2 [−18.7, +8.2] | 49.2% |
| q8m | `ceff111` | 8 | 10 to 15 | −6.9 [−19.1, +5.2] | 49.0% |

No variant gained; the owner dropped it ("Drop both (Recommended)").

## What was learned

Charging more for the cells the enemy can take, in every turn or only
before the usual turn of the decisive cut, changed nothing measurable, as
E047 found for a side that leads. Our greater exposure before the cut in
the losses goes with being behind on land and income by then (the turn 10
to 15 gap), more than with how the evaluation weighs a threatened cell.
E058 (mines early in the middle game) gained in the same round. The knob
was removed: the pull request holds the journal entry alone.
