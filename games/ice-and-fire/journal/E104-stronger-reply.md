---
id: E104
date: 2026-11-27
pull_request: "#404"
parent: ice-and-fire-v034
hypothesis: E103 lost about 70 Elo by making the opponent's climb lazier, so the quality of that one reply to each of our plans matters far more than its cost; a stronger climb of the opponent (fuller passes, or the best of several climbs, still one reply) makes the search trust fewer plans a real opponent punishes, and beats ice-and-fire-v034.
change: Switches in `Search::replies` that leave v034 as it plays; variants where the opponent's climb tries every action at its first step or at every step, or where the opponent climbs two or three times from different first actions and only the plan it rates best is kept.
release: none
sprt: not run: no variant gained in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

## Why

E103's candidate shared what the opponent's climbs of a turn learnt and
started each from an earlier plan: its replies were as good on the plans
finally played, for 29% less time, but it lost −73.5 in its SPRT, and each
switch alone about −65 in local games. Both made the opponent's climb
lazier, so its replies to the plans the search compares got weaker. The
owner chose to try the other way (2026-11-27): a stronger reply, one per
plan. E064 and E098 found that more replies make the search too cautious,
so every variant here keeps one reply from the opponent's own plan.

## Change

`bots/plan/src/search.rs`:

- `OPP_FULL` (0): 1, the opponent's climb tries every action at its first
  step; 2, at every step (`Search::full`, the opponent's search only).
- `OPP_PICK` (1): the opponent climbs that many times, each barred from the
  first actions of the earlier ones with a budget of its own, and only the
  plan its evaluation rates best is its reply.

With 0 and 1 the bot plays as v034 (the same answers and scores on the
2,031 positions of v033's CodinGame games at 600 tries). The time a search
at 600 tries on those positions (407 of them, one core): v034 14.58 ms,
`OPP_FULL` 1 14.92, `OPP_FULL` 2 17.42, `OPP_PICK` 2 17.19, `OPP_PICK` 3
19.59.

## Screening

Variants against `ice-and-fire-v034`, 1,600 seat-swapped pairs each, whole
games at CodinGame's limits, 4 shards
([run 46077199032](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46077199032)). E101 and E102 found a screening's
error near ±15 Elo, so only a larger effect is read from it; the best goes
to the SPRT.

| Variant | `OPP_FULL` | `OPP_PICK` | Elo over v034 [95%] |
| --- | ---: | ---: | ---: |
| `off` | 0 | 1 | −6.1 [−11.4, −0.8] |
| `oppfirst` | 1 | 1 | −4.8 [−10.8, +1.2] |
| `oppfull` | 2 | 1 | −3.3 [−12.9, +6.4] |
| `opp2pick` | 0 | 2 | −11.5 [−21.2, −1.8] |
| `opp3pick` | 0 | 3 | −2.4 [−12.4, +7.6] |

Commits: `off` `82ff8c5`, `oppfirst` `37d18ed`, `oppfull` `24062f8`,
`opp2pick` `d730f78`, `opp3pick` `4affe3e`. One timeout of `oppfull`; no
other fault.

## What was learned

Dropped. Every variant is level with the control (−6.1): a stronger climb
of the opponent, by fuller passes (−4.8, −3.3) or by the best of two or
three climbs (−11.5, −2.4), gains nothing, though a weaker one (E103) lost
about 70 Elo. The opponent's climb is past the point where more search
improves the reply that matters: it maximizes the opponent's evaluation
by our own measure, and climbing harder on that measure does not bring it
closer to what a real opponent plays. A better reply model needs another
measure of what the opponent wants, not more search on ours.

The code stays reachable on the branch `claude/ice-and-fire-e104-stronger-reply`
(commits up to `571b4396`: the switches `OPP_FULL` and `OPP_PICK`, and the
variants).
