---
id: E058
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v024
hypothesis: In v023's losses to the top 10 its first mine comes around game turns 16 to 20 and the top 10's around 10 to 12, and by turn 10 the top 10 have a mine in 42% to 48% of these games against our 13% to 24%; E033's bonus grows with the territory, which is still small then; a safe mine counting more until turn 12 or 15 builds the economy when it pays most, and beats v024.
change: The plan bot's evaluation counts a safe mine 4 or 8 more until game turn 12 or 15, or one more per 3 active cells rather than 4 until turn 12.
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

## The change

`bots/plan/src/eval.rs`: `EARLY_MINE_TURN` (0: none, v024, which the base
commit keeps), `EARLY_MINE_BONUS` and `EARLY_MINE_PER_CELLS`: until that
game turn a safe mine counts the bonus more and one more per that many
active cells (E033's `mine_bonus` otherwise).

## Screening

Pending.
