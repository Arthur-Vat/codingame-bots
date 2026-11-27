---
id: E099
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v033
hypothesis: Removing or shrinking each part of the search in turn, at CodinGame's limits, shows what each part is worth against the time it takes; the search with half its time shows what a faster search would be worth; and a check of the plan chosen that starts only when it fits before the hard limit removes most turns that end past it without losing strength.
change: Switches in `search.rs` that leave v033 unchanged (`REPLY_OWN`, `LAZY`, `VERIFY_FITS`), and one variant commit per ablation, each against ice-and-fire-v033.
release: none yet
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner asked how the search spends its time and how to make it more
efficient (2026-11-27). A timer-instrumented v033 in 200 local games at
CodinGame's speed (page https://claude.ai/artifact/RqrWMARqi4Fw8d5DtETKeC)
found, from game turn 7: a median turn of 42.1 ms; the reply searches 43%
of the turn (the opponent's own climb alone 32%, about 1.3 ms a search);
scoring the tries 28%; generating the actions 18%, rising through the
game; the final choice and its check 4.5%. 3% of the plies ended past
45 ms (worst 54 ms, past CodinGame's 50), mostly because the check of the
plan chosen (E027's `VERIFY`) started a reply search near the hard limit.

Before speeding any part up, two questions: what a faster search is worth
at all (the same bot with half its time), and what each part is worth
against its time (an ablation of each, at the same limits).

## Change

Switches, each leaving v033 as it plays (the same answers on 300 positions
of v033's CodinGame games at 600 tries):

- `REPLY_OWN`: whether the replies hold the opponent's own climb (E019).
- `LAZY`: whether a climb's steps are lazy.
- `VERIFY_FITS`: the check of the plan chosen starts a reply search only
  when the longest reply search of the turn so far still fits before the
  hard limit; otherwise the best plan whose replies were searched is
  played.

## Screening

Two runs against `ice-and-fire-v033`, whole games at CodinGame's limits:
[run 46043171122](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46043171122)
(`off`, `half`, `vfix`: 800 pairs, 4 shards) and
[run 46043180478](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46043180478)
(the six ablations: 798 pairs, 3 shards). Commits: `off` `3bbf176`, `half`
`69daa07`, `vfix` `6d045b1`, `noopp` `94ca013`, `opp100` `6eca10b`, `cut1`
`2a461cd`, `mem4` `e2d41e0`, `noblock` `e1023b3`, `fullpass` `ea76eb3`.

| Variant | Change | Elo over v033 [95%] |
| --- | --- | ---: |
| `off` | none (control) | −5.2 [−14.7, +4.3] |
| `half` | `BUDGET_MS` 22.5 (half the time; the first turn unchanged) | **−32.2** [−46.5, −18.0] |
| `vfix` | `VERIFY_FITS` | +4.8 [−5.1, +14.7] |
| `noopp` | `REPLY_OWN` off: no opponent climb in the replies | **−93.6** [−110.0, −77.7] |
| `opp100` | `REPLY_EVALS` 100 rather than 300 | +3.0 [−6.9, +13.0] |
| `cut1` | `REPLY_CUTS` 1 rather than 3 | +7.0 [−6.7, +20.6] |
| `mem4` | `MEMORY_CAP` 4 rather than 8 | **−26.2** [−41.1, −11.4] |
| `noblock` | `BLOCK_TOWERS` off (E044's blockers) | **−40.2** [−54.3, −26.3] |
| `fullpass` | `LAZY` off: every action tried at every step | **−23.1** [−37.9, −8.4] |

Faults: one timeout of `off` and one of `opp100` (each in about 1,600
games), one of v033 against `half`; none for `vfix`.

What each part is worth, against the share of the turn it takes (the
timing page):

- **Time:** half the time loses 32 Elo, so a faster search is worth
  having: about 30 Elo a halving of the time near v033's speed.
- **The opponent's own climb** (E019; 32% of the turn) is worth about 94
  Elo: without it the search ends most turns early (median answer 24 ms
  against 43 on the same runner) and loses badly. Its budget does not
  matter: 100 tries play as 300 (+3.0), since a climb uses about 52 tries
  on average.
- **The memory of 8 replies** is worth 26 Elo over 4.
- **The blockers** (E044) are worth 40 Elo.
- **The lazy passes** are worth 23 Elo over trying every action at every
  step.
- **Three cuts in the replies** are worth nothing over one (+7.0, level).
- **`vfix`** is level (+4.8) and had no timeout.

## Local measurements

From a counting build of v033 in 100 local games against v033 at
CodinGame's speed (1,886 plies from game turn 7):

- The opponent's climb gives the most damaging reply in 82.5% of the
  reply searches, decides the score of 81.5% of our tries and the final
  score of 85.7% of the plans played; the three cuts together 9 to 12%,
  the greedy turn 2 to 4%, the lethal chain about 2%.
- 44% of the reply searches find exactly the replies an earlier search of
  the same turn found, 48% the same worst reply, and 50% of the
  opponent's climbs end on a plan found earlier in the turn.
- Of our tries, trainings of level 1 more than 3 steps from the enemy's
  land are 13.6% and are kept 0.8% of the time; level-3 trainings at the
  front 9.2%, kept 3.2%; moves are 42% of the tries and kept 16 to 28%.

A profile of the search (callgrind, 41 positions at 600 tries) found
`chain_to` at 33% of all instructions: the lethal chain is looked for at
every step of every climb, ours and the opponent's, each time with a
full `Reach` and up to 8 variants through enemy towers. A lower bound
(10 gold for each cell not ours on the shortest path to the target) skips
it when the gold cannot pay: the same answers and scores on all 2,031
positions of v033's games at 600 tries, and 21% less time a search at
fixed tries (27.0 to 21.3 ms). Bounding the evaluation's chain floods
gained nothing more.
