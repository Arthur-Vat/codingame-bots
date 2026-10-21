---
id: E002
date: 2026-10-21
pull_request: "#182"
parent: ice-and-fire-v001
hypothesis: A search over the whole turn, as the contest's winner did (hill climbing over single actions, replayed against a memory of the opponent's replies), plays better than the fixed phases of the ladder's `banker`, which only look at the opponent through a few rules.
change: A new bot, `plan` (games/ice-and-fire/bots/plan), that takes a lethal move or chain if there is one, else climbs over moves, trainings, towers, mines, the best cuts and a lethal chain, scoring each plan against the opponent's replies it remembers.
release: ice-and-fire-v002
sprt: accepted
elo: +126.4 [+83.8, +173.0] at CodinGame's limits
pairs: 109
full_time: +126.4 [+83.8, +173.0] (the SPRT's own, ADR 0042), no timeout in 218 games, slowest answer 26.5 ms
decision: promoted
cg_rank: not reported
---

The first version of the search, not a tuned one: the weights are those of
`ice_and_fire_tactics::eval::evaluate`, the cuts those of `cuts`, and the
bot's own code is `games/ice-and-fire/bots/plan` (a few small files). The
ladder and tactics crates are unchanged.

## The bot

One turn, within `40 ms × CG_TIME_SCALE` from the arrival of the turn's first
input line (every turn, the first included):

1. **Lethal first:** a movable unit next to the enemy HQ steps in, else
   `chain_to(state, HQ[enemy], gold)`; if it wins, it is played.
2. **Actions** of a state: every legal single command (each move of a movable
   unit to a neighbouring cell it may enter; each training at the lowest level
   that may enter a cell, so a higher level only where it kills a unit or
   passes a tower; each tower on a border cell of ours, each mine on a free
   mine spot), and as macro actions the top 4 of `cuts(state, 4, gold, 4)`
   with more than one training, and the lethal chain if any. Each is checked
   by applying it on a clone.
3. **Score of a plan** = the worst `evaluate` (our seat) over the opponent's
   replies in the memory, after our plan, the end of our turn, the reply (each
   command kept only if accepted) and the end of its turn. An empty memory
   scores the plan's own state. The replies are tried most damaging first and
   scoring stops once a plan cannot beat the best so far.
4. **Hill climbing:** from the start state, the action that scores best is
   added while it improves the score. A step is lazy (see below).
5. **Memory:** after a climb, the opponent's replies to the plan (its lethal
   chain, its top 3 cuts, bronze's `greedy_turn`) are added if new, and the 8
   most damaging kept. Another climb runs against the memory, and so on while
   time remains and the plans differ. The plan that scores best against the
   whole memory is played.

**Time.** Trying every action at every step costs too much at 10 ms, so a step
tries the actions in the order of the gain each had when last tried (new ones
first) and stops when the next one's old gain is no more than the best gain
found; when that finds nothing, one full pass tries the rest, so a climb ends
on a plan that no single action improves. A later climb starts from the best
gains the actions had in the earlier climbs. No new climb or reply search
starts after 60% of the budget; the rest is for choosing the plan. The search
is also limited by a number of plans scored, so that the tests are
deterministic (`Limit::max_evals`); the bot passes none.

No `HashMap::new` or `RandomState`: the one map (the actions' gains) uses a
fixed hasher, since the studio's WASI has no `random_get`. The bot compiled for
`wasm32-wasip1` and run by Node's WASI answers a first turn.

## Screening

Rust 1.90.0 bundles, `ice-and-fire-arena match`, seat-swapped pairs, seed 1,
two games in parallel on a 4-CPU machine (a shared virtual machine: some
stalls of tens of milliseconds hit both bots at once, see the faults). The
Elo is the arena's, from pairs, with its 95% interval.

**The bot as built** (soft limit 60% of the budget, gains kept across climbs):

| Opponent | Limits | Pairs | Score | Elo | Faults | Answers after the first: median, 99.9%, slowest |
| --- | --- | ---: | --- | --- | --- | --- |
| v001 | time scale 0.2, tolerance 5 ms | 200 | 263 wins, 137 losses (65.8%) | +113.3 [+79.8, +149.0] | none | 1.5, 8.2, 10.5 ms (limit 10 ms) |
| v001 | time scale 0.2, tolerance 5 ms, another run | 300 | 376 wins, 224 losses (62.7%) | +90.0 [+63.8, +117.2] | 1 timeout, in a stall that took 72 ms from both bots | 1.6, 8.4, 72.3 ms |
| v001 | CodinGame's limits (scale 1, tolerance 0) | 100 | 135 wins, 65 losses (67.5%) | +127.0 [+81.8, +176.7] | none | 1.5, 26.1, 26.2 ms (limit 50 ms) |
| random | time scale 0.2, tolerance 5 ms | 100 | 200 wins (100%) | n/a | none | 0.8, 5.7, 6.9 ms |

The 99.9th percentile stays below 0.9 times the scaled limit (9 ms at scale
0.2, 45 ms at scale 1). The median is low because most turns end on a local
optimum before the time is up; the long ones are mid-game turns with many
units. v001 itself shows 99.9th percentiles of 2 to 3 ms here, so a few
milliseconds of any answer are the machine's.

**What was tried on the way** (all against v001 at time scale 0.2, tolerance
5 ms; each differs from the one before in one thing):

| Version | Pairs | Score | Elo | 99.9% answer |
| --- | ---: | --- | --- | --- |
| First: soft limit 85% of the budget, gains of a climb not kept | 200 | 60.2% | +72.2 [+40.1, +105.7] | 9.3 ms; one run had 3 timeouts, in a stall of 72 ms that hit both bots |
| Soft limit 75% | 200 | 57.5% | +52.5 [+19.2, +86.8] | 10.1 ms: above 0.9 times the limit, so not kept |
| Soft limit 60% | 300 | 56.0% | +41.9 [+15.7, +68.6] | 8.5 ms: safe, but the weakest of the three |
| Soft limit 60%, gains kept across climbs (the bot as built) | 200 | 65.8% | +113.3 [+79.8, +149.0] | 8.2 ms |

The first version also beat v001 at CodinGame's limits (+107.5 [+60.2,
+159.2], 100 pairs) and won all games against random. The three changes of the
table are the only variants tried. What they say: at 10 ms the search is short
of time, and keeping the gains across climbs, which makes the later climbs
cheap, bought more than 25% more time did.

## Not done, and ideas

- The evaluation is `evaluate` as it is. Its terms are placeholders (README of
  the tactics); the research's "time to win" and a weight on gold and income
  that grows late in the game are the first things to try.
- The search ends on a local optimum on most turns. More actions (two-unit
  moves, towers on the cells a reply would train on), more replies in the
  memory or a better one than bronze's greedy turn would use the time left.
- The cuts are the cut finder's, searched again at each step: a search is
  cheap on average (about 24 microseconds) but its worst case is a few
  milliseconds.
- The first turn has 1,000 ms on CodinGame and the bot uses 40 ms there.

**SPRT** (head 41aaf0d, CodinGame's limits: time scale 1, no tolerance,
against `ice-and-fire-v001`): accepted after 109 pairs, 66.5% of 218 games,
+126.4 Elo [+83.8, +173.0], LLR 2.95. Smoke test: 200 of 200 against
`random`. No timeout, crash or invalid answer; answers after the first:
median 1.9 ms, 99.9% 25.9 ms, slowest 26.5 ms. The whole SPRT took under a
minute on the runner and the league about 2 minutes: most turns end their
climbs early, far below the ADR 0042 estimates made for a bot that always
uses its 40 ms.

**League** of v002 (800 pairs each, CodinGame's limits): 66.9% against v001,
69.7% against `cutter`, 79.6% against `guard`, 81.3% against `chain`, 81.0%
against `bronze`, 100% against `random`.

**Decision:** promoted, `ice-and-fire-v002`.
