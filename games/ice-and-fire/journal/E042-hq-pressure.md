---
id: E042
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games the top 10, after 28% of their long chains (4 or more trainings), could pay at their next turn for a chain of trainings to our HQ (we after 4% of ours), and the cuts that would have killed three quarters of those chains mostly left our HQ open (122 of 296 lost by force in v018's search); v018's pressure term stops counting once a side can pay for that chain, so counting a standing threat on the enemy HQ makes the search keep one, which ties the enemy to its defence and protects our chains, and beats v018.
change: The plan bot's evaluation counts a side that can pay, at its next turn, for its cheapest chain of trainings to the enemy HQ a bonus, flat or growing with the gold it would have left.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Yes, screen it"), after the study of
our replies to the top 10's long chains in v014's CodinGame games (E038's
entry gives the study of chains and trades it continues).

## Why

When a top-10 player trained 4 or more units in a ply, the best of 8
cuts available to us at our next ply would have killed three quarters of
those units in 42% of cases; v014 did in 25%, and v018's search would in
28%. On the 296 positions where such a cut existed and v018 would kill
under half, v018's judge found the cut losing by force in 122 (it leaves
our HQ to a chain the enemy can pay for), every plan losing in 60, and
the cut 126 below v018's own plan (median) in the other 114: the search
is right not to punish. After their long chains the top 10 could pay for
a chain to our HQ at their next turn 28% of the time; we after ours 4%.

The shared evaluation's pressure term counts how far a side is from
paying for its cheapest chain to the enemy HQ (at most 200), and nothing
once it can: being able to strike is worth no more than being exactly
short of it.

## The change

`tactics/src/eval.rs`: `margin`, a side's gold and income less its
cheapest chain to the enemy HQ, and `evaluate_with_margins`, the shared
evaluation with both margins as its pressure term found them (no work
more). `bots/plan/src/eval.rs`: `standing(margin)`, 0 for a side that
cannot pay, else `HQ_BONUS` plus `HQ_MARGIN_HALVES` halves of the margin
up to `HQ_MARGIN_CAP` (100); it counts for each side (both 0 in v018,
which the base commit keeps). A test checks the bonus and the margins.

## Screening

Pending.
