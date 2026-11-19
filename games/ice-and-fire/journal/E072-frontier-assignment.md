---
id: E072
date: 2026-11-19
pull_request: "#369"
parent: ice-and-fire-v029
hypothesis: Matching our units and the trainings we can pay to the middle chokes and to cells spread along the Voronoi frontier, all at once each turn (step 1 of phase 2 of the early-game plan), takes the middle earlier than v029's opening and wins the duels cut at game turn 12.
change: A joint frontier assignment (ice_and_fire_opening::assign, the Hungarian method on arrival turns) played as the opening (A), or offered to the solo opening's beam as extra steps beside a weight on the middle chokes (B), each behind a switch that is off in the bot.
release: none
sprt: not run: weaker in the cut duels
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Phase 2, step 1 of the early-game rework
([research/codingame/2026-11-19-early-game.md](../research/codingame/2026-11-19-early-game.md)),
started on 2026-11-19 ("Read the handoff and start phase 2"). After the
first variants the owner asked to step back and compare the approaches
first; he chose to put the assignment inside the existing searches (form,
"Guide in the searches"), then to try it with the opening playing until
game turns 6, 7 and 8, and to study the optimal plans of the early game
before going further ([research/codingame/2026-11-19-optimal-early-plans.md](../research/codingame/2026-11-19-optimal-early-plans.md)).

## The assignment

Each turn of the opening: the targets are the middle chokes we do not hold,
then neutral cells along the Voronoi frontier (at most one step nearer one
side's land than the other's), at least 3 steps apart; the agents are our
units that may move and the trainings the gold allows (the payback rule's
count, or all it can pay); the assignment minimises the total arrival turns
(a unit arrives after its distance less one, a training after the distance
from the trainable cell nearest its target), less 4 for a choke, plus 2 for
each turn the enemy's land could reach the target first, solved exactly by
the Hungarian method. A unit beside a cell it can take always takes one; the
cell is the first step of the path to its target, at most 2 steps longer
than the shortest, that takes the most cells, the units with the fewest
cells to take next to them choosing first and each path's cells kept from
the later units. Played by one side alone (the other passing), its land
grew as fast as the solo opening's only after these three rules: 16.7 cells
at the start of turn 7 against 16.5 (200 maps, both sides playing the same
opening); without them, 15.2, the units walking through their own land
behind each other and into dead ends.

## Judge

Duels cut at game turn 12 (`ice-and-fire-arena-early`, won by the higher
zasmu-likeness, [2026-11-19-zasmu-likeness.md](../research/codingame/2026-11-19-zasmu-likeness.md))
against `ice-and-fire-v029`, seed 72, both seats, time scale 0.586 and 8 ms
of tolerance from `scripts/calibrate.sh` on the session's 4 CPUs; bots
compiled with `cargo build --release`, v029 from its release file with
`rustc -C opt-level=3`.

## A: the assignment as the opening (turns 1 to 6)

| Variant | Change | Pairs | Elo over v029 [95%] |
| --- | --- | ---: | --- |
| a1 | first version | 200 | −88.7 [−122.4, −56.6] |
| a2 | a1 with the three rules above | 200 | −26.1 [−58.9, +6.2] |
| a3 | a2, every affordable training | 200 | −20.9 [−53.8, +11.7] |
| a4 | a2, the opening until turn 8 | 200 | −36.6 [−70.9, −3.0] |
| a5 | a3, the opening until turn 8 | 200 | −121.1 [−159.5, −85.3] |

From zasmu's starts in the opening suite (`early-openings`, suite v1, the
player's mean / the bot's mean), a1 held fewer active cells than v029 at
every turn (turn 10: red 26.7 against v029's 28.2, blue 29.7 against 31.7;
zasmu 30.0 and 33.8) and no more middle chokes (0.20 / 0.33 at turn 10
against 0.26 / 0.33).

## B: the assignment as steps of the solo opening's beam

`Objective::assign` adds the assignment's turn (with the payback rule, and
spending all the gold) to the steps the beam tries; `Objective::chokes`
weighs each middle choke held at the end of a plan.

| Variant | Change | Opening until turn 6 (200 pairs) | Until turn 7 (400) | Until turn 8 (400) |
| --- | --- | --- | --- | --- |
| b0 | none (v029's opening) | +13.9 [−4.0, +31.9] | −18.3 [−40.3, +3.7] | −24.4 [−46.8, −2.1] |
| b1 | the assignment's steps | −52.5 [−82.0, −23.8] | −92.5 [−115.5, −70.2] | −90.6 [−114.1, −67.9] |
| b2 | b1 and 6 a choke | −41.9 [−71.0, −13.4] | | |
| b3 | 6 a choke | −6.9 [−30.6, +16.6] | −41.0 [−64.4, −18.0] | −19.1 [−43.5, +5.0] |
| b4 | b2 without the shape terms | −41.9 [−72.6, −11.8] | | |

b0 at turn 6 is v029 itself: its +13.9 is the noise of 200 pairs.

## What it says

- The beam picks the assignment's steps, since they score better by its
  objective (9 turns, the opponent passing), and they lose 50 to 90 Elo:
  the objective, not the search, is what misleads.
- Playing the opening longer loses (b0), as E067 found: the middle-game
  search plays turns 7 and 8 better.
- A weight on the middle chokes alone is level at turn 6 and does not
  gain later.

The assignment and the solo opening's two options stay in the code behind
their switches, off (`ASSIGN` and `OBJECTIVE` in the plan bot), for the
exact solver (`opening/examples/oracle.rs`) that compares the openings in
the study of optimal plans.
