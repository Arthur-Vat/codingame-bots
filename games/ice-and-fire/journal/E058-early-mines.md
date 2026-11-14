---
id: E058
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v024
hypothesis: In v023's losses to the top 10 its first mine comes around game turns 16 to 20 and the top 10's around 10 to 12, and by turn 10 the top 10 have a mine in 42% to 48% of these games against our 13% to 24%; E033's bonus grows with the territory, which is still small then; a safe mine counting more until turn 12 or 15 builds the economy when it pays most, and beats v024.
change: The plan bot's evaluation counts a safe mine 4, 6 or 8 more until game turn 12, 15 or 18, or one more per 3 active cells rather than 4 until turn 12.
release: ice-and-fire-v025
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

Against `ice-and-fire-v024` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; one timeout of e8's opponent in 1,600 games, no other fault).
"Until" is `EARLY_MINE_TURN`, "more" `EARLY_MINE_BONUS`, "per" the active
cells per extra point (`EARLY_MINE_PER_CELLS`, 4 as E033's).

First round, run 43596128673, seed 58058, 800 pairs each over two runners:

| Variant | Commit | Until | More | Per | Elo against v024 | Score |
| --- | --- | ---: | ---: | ---: | --- | ---: |
| e4t15 | `183b31a` | 15 | 4 | 4 | **+24.4 [+11.0, +37.8]** | 53.5% |
| e4 | `f764bd1` | 12 | 4 | 4 | **+15.2 [+1.9, +28.6]** | 52.2% |
| e8 | `7ca4d5d` | 12 | 8 | 4 | **+13.9 [+0.5, +27.3]** | 52.0% |
| p3 | `852a269` | 12 | 0 | 3 | +4.8 [−8.7, +18.2] | 50.7% |

The leader again with its neighbours, run 43598714015, seed 58059, 1,600
pairs each over four runners:

| Variant | Commit | Until | More | Elo against v024 | Score |
| --- | --- | ---: | ---: | --- | ---: |
| e6t15 | `0a44da2` | 15 | 6 | **+30.3 [+20.7, +39.8]** | 54.3% |
| e4t18 | `f647fec` | 18 | 4 | **+21.5 [+12.1, +31.0]** | 53.1% |
| e4t15 | `183b31a` | 15 | 4 | **+18.5 [+9.2, +27.8]** | 52.7% |

A safe mine counting more in the first turns of the middle game gained in
every form with a fixed bonus; counting it per fewer cells did little, as
the territory is still small then. The candidate is e6t15
(`EARLY_MINE_TURN = 15`, `EARLY_MINE_BONUS = 6`), the best on 1,600 pairs,
whose bundle is the same, byte for byte, as the screened commit's. The
test of E033's bonus reads its positions early in the game, so its
expected values now add E058's 6.

## SPRT

Pending.
