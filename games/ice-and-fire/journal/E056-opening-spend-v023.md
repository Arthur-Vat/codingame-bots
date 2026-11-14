---
id: E056
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v023
hypothesis: The top 10 spend more than we do at turns 6 and 7 (11 and 16 gold as red against our 5 and 8); E046 found that weighing gold less in our opening lost (about -50), but its units crowded each other, so that more units took no more land; with v023's units spread (E051, E055), each new unit finds a cell of its own, so spending more in the opening (gold weighed less in the plan, or trainings that pay back by a later turn, or units expected to take more cells after contact) takes more land and beats v023.
change: The opening's plan weighs gold 0 or half as much as v023 (every other weight doubled), or its purchases count a unit as paying back by game turn 18 or 21 rather than 15, or a unit as taking 2 cells after the territories meet rather than 0.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14 (a form after v023: "Spend more on v023
(Recommended)").

## Why

The scratch study of the first 10 turns against the top 10 (in E051's
entry) found the top 10 spending more at turns 6 and 7, and our units
crowding each other. E055 (`ice-and-fire-v023`) spread them: a cell is
worth 2 less to the opening for each of our units next to it, and the
plan looks 9 turns ahead. E046 tried spending more on v021 by weighing
gold half as much (−49) or allowing mines (−25), and E034 found gold
weighed 0 worse (−24.8), all with crowding units.

## The change

`bots/plan/src/main.rs`: `OBJECTIVE` (the opening plan's weights; gold 1
in v023), and `OPENING_HORIZON` and `OPENING_AFTER_CONTACT`, set on every
weight set of the opening (`Params::horizon`, the game turn by which a
purchase must pay back, 15; `Params::after_contact`, the cells a unit is
expected to take after the territories meet, 0). The base commit is v023.

## Screening

Pending.
