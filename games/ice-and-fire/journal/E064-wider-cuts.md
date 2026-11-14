---
id: E064
date: 2026-11-14
pull_request: "#NN"
parent: ice-and-fire-v026
hypothesis: In v026's CodinGame games the opponent's real reply is worse than every reply the search models by 50 or more in about 2% of our plies, games with such a miss were lost 71% of the time (47% without), and these misses are large cuts that the modeled replies cannot hold, about half because they need unit moves before the trainings and most of the rest because they lie beyond the cut finder's chains of 5 trainings; modeling longer cuts, and cuts after an enemy unit's step toward our HQ, makes the search defend against them, and beats v026.
change: The plan bot's search models the opponent's cuts as chains of up to 8 trainings rather than 5, or adds for the 2 or 4 enemy units nearest our HQ a reply made of the unit's step toward it then the best cut from there, or both.
release: none
sprt: pending
elo: none
pairs: none
full_time: none
decision: pending
cg_rank: none
---

Asked by the owner on 2026-11-14 (a form after the overconfidence study,
[research/codingame/2026-11-14-overconfidence.md](../research/codingame/2026-11-14-overconfidence.md):
"Both in parallel"), run beside E063.

## Why

The study of v023 to v026's CodinGame games (9,542 of our plies, judged by
v026's search; section 3 of the note): of the 160 plies whose real reply
scored 50 or more below the worst modeled one, 49% needed the reply's unit
moves (without them its trainings removed under half the cells; 22% of the
replies that did not surprise), 28% were cuts of trainings alone that the
cut finder does not find even with chains of 8 and its 20 best cuts (3%),
and 11% were found only with that wider finder (4%); 6% were within the
search's own finder. These replies removed 13.0 of our cells on average,
against 1.9 for replies that did not surprise.

## The change

`bots/plan/src/search.rs`, in the opponent's replies (`Search::replies`):
`REPLY_CUT_LENGTH`, the longest cut in trainings (`CUT_LENGTH`, 5, in
v026), and `MOVE_CUTS`, the number of enemy units, nearest our HQ first,
whose step toward our HQ is played before the best cut from there (0 in
v026, which the base commit keeps). More replies cost search time; the
screening plays at CodinGame's speed and real limit, so its result counts
that cost.

## Screening

Pending.
