---
id: E059
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v024
hypothesis: Before the large cut that decides most of v023's losses to the top 10 (median turn 15) it leaves more cells the enemy can take than in its wins at the same turn (median 4 against 2 to 3, as blue 10 or more in 27% of these losses); a threatened cell costing more, in every turn or only from turn 10 to 15, makes the search guard its cells before the cut and beats v024.
change: The plan bot's evaluation charges a threatened cell 6 or 7 quarters rather than 5 in every turn, or 7 or 8 only from game turn 10 to 15.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
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

Pending.
