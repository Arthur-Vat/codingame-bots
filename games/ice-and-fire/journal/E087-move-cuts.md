---
id: E087
date: 2026-11-26
pull_request: "#388"
parent: ice-and-fire-v032
hypothesis: A search that models the opponent's cuts that start with its units' steps, charges a plan for its second-worst reply as well as its worst, and is offered pairs of purchases near the front, stops opening weaknesses the opponent punishes and spends better, and beats ice-and-fire-v032.
change: Replies with steps before a cut, a charge on the second-worst reply, and spending bundles in the search; each setting at its v032 value is v032.
release: none
sprt: none
elo: "+15.6 [-4.0, +35.4] (steps3, screening)"
pairs: 400
full_time: none
decision: dropped
cg_rank: not reported
---

## Why

E086's entry ("A unit that left the line"): at game turn 10 of race2's game
on map 925707781, v032's search moved unit 11 out of our line, and v032
answered with a step of its unit next to our line followed by a cut (a
level-2 unit killing ours, the cell left empty taken, unit 11 cut off and
killed). The search's replies did not hold that kind: its cut finder chains
trainings only, and E037's threat lets only units of level 2 or more step
first. The owner (2026-11-26): all those kinds of cuts must be considered
in the opponent's replies; and with K gold the search should weigh many
ways to spend: several units, several towers, both, or saving.

With the replies below, the search models the real cut on that position
(it leaves us at 97 to 106), but rates a cut in the north worse (73), the
same worst reply whether unit 11 moves or stays (64 if it stays): the move
still wins by 9. The worst reply alone does not charge a plan for opening a
second weakness; hence the second-worst charge.

## The change

In `bots/plan/src/`:

- `search.rs`, `REPLY_STEPS`: bit 1, all of the opponent's movable units
  step toward our land (onto a cell of ours they may take, else a free cell
  next to our land), then its 2 best cuts (chains of up to
  `STEP_CUT_LENGTH`, 4, trainings); bit 2, each such step alone (the first
  `STEP_REPLIES`, 8, captures first), then its best cut.
- `search.rs`, `SECOND_QUARTERS`: a plan's score, the worst evaluation
  after its replies, less this many quarters of its second-worst reply's
  damage (the evaluation of the plan's end with no reply less the
  evaluation after that reply).
- `actions.rs`, `BUNDLE_CANDIDATES`: the first this many single trainings
  and towers on cells next to the enemy's land or to a cell it can take at
  its next turn, paired, each pair the gold pays for offered as one action;
  saving stays the climb's default (nothing added).

## Games and screening

Variants: steps1 (`REPLY_STEPS` 1, 58298d300a2b), steps3 (3, d42ffc643e34),
bundles6 (`BUNDLE_CANDIDATES` 6, fd1ee360beef), second2 (`SECOND_QUARTERS`
2, 7941e16e1f94), steps3-second2 (008d31e697d4), steps3-bundles6
(6b2b89ab3116), steps3-second2-bundles6 (1a211d7ec786).

A local check of the heaviest (steps3-second2-bundles6) in full games
against v032, 8 pairs at CodinGame's limits (seed 87): no fault, answers
after the first 41.7 ms median, 45.5 at 99%, 47.3 at 99.9% (v032 30.8, 43.7,
45.4): close to the 50 ms limit.

Games against v032 on E086's 20 maps, both seats, cut at game turn 12, 40
each, measured as E086's (v032 against itself the baseline):

| Bot | Middle cells lost, turns 6 to 10 | All cells lost, turns 6 to 10 | Holds its centre at turns 8 / 10 | Cells at turn 12, its / v032's |
| --- | --- | --- | --- | --- |
| v032 | 0.47 | 0.85 | 64% / 86% | 35.5 / 36 |
| steps1 | 0.75 | 1.23 | 64% / 89% | 35.5 / 39 |
| steps3 | 0.72 | 1.15 | 64% / 86% | 34.5 / 41 |
| bundles6 | 0.70 | 1.10 | 61% / 86% | 36 / 37 |
| second2 | 0.50 | 0.75 | 58% / 81% | 34 / 38.5 |
| steps3-second2 | 0.50 | 0.75 | 61% / 81% | 35.5 / 37.5 |
| steps3-bundles6 | 0.68 | 1.10 | 67% / 86% | 36 / 39.5 |
| steps3-second2-bundles6 | 0.80 | 1.40 | 56% / 78% | 34 / 38 |

The second-worst charge is the only change that loses no more cells than
v032 (0.75 against 0.85 in all, 0.50 against 0.47 in the middle), at a
cost in land (34 to 35.5 cells at turn 12, v032 37.5 to 38.5); modelling
more replies (steps) alone makes the search lose more and gain less land.
40 games are few: the screening is needed.

`screen.yml` run 45860907261 on the owner's approval (2026-11-26), full
games against v032, 400 pairs per variant over 2 shards, no fault on either
side:

| Variant | Elo over v032 [95%] |
| --- | --- |
| steps1 | -2.6 [-23.3, +18.0] |
| steps3 | +15.6 [-4.0, +35.4] |
| bundles6 | -14.8 [-35.5, +5.8] |
| second2 | -85.0 [-107.8, -63.0] |
| steps3-second2 | -58.7 [-80.3, -37.6] |
| steps3-bundles6 | -20.0 [-40.1, -0.0] |
| steps3-second2-bundles6 | -70.4 [-92.8, -48.7] |

The opponent's cuts after its units' steps, each step alone as well
(steps3), lean positive, the best result of E083 to E087; the second-worst
charge, which lost the fewest cells in the local games cut at turn 12, is
clearly weaker in full games (it makes the search passive: less land), and
the spending bundles a little weaker.

### Longer and more cuts: a benchmark

The owner (2026-11-26): couldn't the opponent's cuts be longer, and more of
them considered? Benchmark first. Three more settings in `search.rs`:
`STEP_CUTS` (cuts kept after each single step, 1), `REPLY_CUT_LENGTH` (the
opponent's plain cuts' longest chain, v032's `CUT_LENGTH` 5), and
`PAIR_CUTS` (pairs of its plain cuts on distinct cells its gold pays for,
drawn from its best `PAIR_POOL` 6, as replies: several separate captures in
one turn). Two measures for each setting, with its analysis build:

- the time of the search alone (`speed`, 300 tries, 120 middle-game
  positions drawn by `ice-and-fire-review bank` from 400 of v032's
  CodinGame games);
- what the replies miss (`codingame-surprise` as the judge, 300 tries,
  3,000 of v032's CodinGame plies): among the plies of game turns 7 to 14
  whose opponent's reply removed 3 or more of our cells, the share where
  that reply scored 20 or more below every modelled one, or 50 or more.

| Setting | Steps | Cut after a step | Cuts a step | Steps tried | Plain cuts (length, count) | Pairs | ms a position (300 tries) | Large removals | Missed (<= -20) | Badly missed (<= -50) | Modelled replies (median) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| v032 | 0 | - | - | - | 5, 3 | no | 13.97 | 256 | 20% | 12% | 12 |
| s3 (steps3) | 3 | 4 | 1 | 8 | 5, 3 | no | 14.22 | 248 | 17% | 9% | 15 |
| s3-L5 | 3 | 5 | 1 | 8 | 5, 3 | no | 14.62 | 248 | 17% | 9% | 15 |
| s3-L6 | 3 | 6 | 1 | 8 | 5, 3 | no | 13.59 | 249 | 18% | 9% | 15 |
| s3-k2 | 3 | 4 | 2 | 8 | 5, 3 | no | 13.97 | 248 | 17% | 9% | 15 |
| s3-k3 | 3 | 4 | 3 | 8 | 5, 3 | no | 13.69 | 248 | 17% | 8% | 16 |
| s3-n16 | 3 | 4 | 1 | 16 | 5, 3 | no | 13.86 | 248 | 17% | 9% | 15 |
| s3-L5k2n12 | 3 | 5 | 2 | 12 | 5, 3 | no | 15.10 | 248 | 17% | 9% | 15 |
| plain7x6 | 0 | - | - | - | 7, 6 | no | 13.84 | 254 | 19% | 11% | 15 |
| s3-plain7x6 | 3 | 4 | 1 | 8 | 7, 6 | no | 14.16 | 249 | 17% | 8% | 17 |
| pairs | 0 | - | - | - | 5, 3 | yes | 15.33 | 255 | 20% | 12% | 14 |
| s3-pairs | 3 | 4 | 1 | 8 | 5, 3 | yes | 14.47 | 248 | 17% | 8% | 17 |

The replies cost little time (13.6 to 15.3 ms a position against v032's
14.0). The steps cut the large removals badly missed from 12% to 9%; longer
cuts, more cuts, more steps, longer and more plain cuts, and pairs of cuts
add at most a point. With the widest setting (s3-plain7x6), of the large
surprises left, 42% are trainings alone that neither the search's cut
finder nor a wider one (chains of 8 and 20) matches, 23% remove none of our
cells (units killed, or an HQ threat), 18% still need moves.

### What the unmatched replies are

The owner (2026-11-26): what are the replies no cut finder matches? And
his guess: making an inactive region active again by training on it. Of the
60 large surprises (-50 or less) of the widest setting above (s3-plain7x6),
read on the board before and after the opponent's reply (the plies of
`arena/examples/plies.rs` replayed from the CodinGame records):

| Cause (`codingame-surprise`) | Cases | Replies that reactivate the opponent's inactive cells | Its trainings next to its active land / next to an earlier training / next to an inactive pocket only | Replies of 2+ trainings forming one connected chain |
| --- | --- | --- | --- | --- |
| neither finder matches it | 25 | 20 (median 3 cells) | 26 / 45 / 23 | 3 of 24 |
| moves needed | 11 | 8 | 2 / 13 / 8 (5 else) | 6 of 10 |
| only the wider finder | 5 | 1 | 7 / 19 / 0 | 5 of 5 |
| the search's finder | 5 | 4 | 5 / 5 / 3 | 1 of 4 |
| no cell of ours removed | 14 | 3 | 2 / 1 / 1 | 1 of 1 |

The replies no finder matches reconnect the opponent's own inactive land,
and train from several places, the reconnected pocket included: a cut is a
set of trainings each legal when it is made, not a chain each next to the
one before. Explained in all: 52 of the 60 (moves 11, sets of trainings or
reconnections 21 of the 25, longer chains 5, cuts the finder finds but
ranks below its best three for the units they kill 5, and 14 replies that
remove none of our cells: six `WAIT`s, mostly at game turns 21 to 29, where
every reply modelled spends the opponent's gold and the evaluation rates
that better for us than its keeping it, and replies that build towers or
move units, changing the evaluation's terms; the owner will look at those
afterwards). Not fully explained: 4 long strikes (6 to 8 commands, level-3
trainings in a near-straight line with moves; two the same game twice),
and 4 replies removing no cell whose evaluation term is not checked.

### Scattered cuts and pairs of steps

On the owner's go (2026-11-26), steps 1 and 2 of the plan:

- `tactics/src/cut.rs`, `scatter_cuts`: sets of up to `max_len` trainings
  each on any cell next to the side's active land as it stands after the
  ones before (reconnected pockets included), on the opponent's cells, the
  cells next to them or next to the side's inactive pockets; a beam over
  the number of trainings, `width` sets kept at each, by the cut finder's
  own score (which counts the side's income gained: reconnections count).
- `search.rs`: `SCATTER_CUTS` (scattered cuts from the position after our
  turn), `SCATTER_LENGTH` (4) and `SCATTER_WIDTH` (30),
  `SCATTER_AFTER_STEPS` (the best scattered cut after each single step),
  `REPLY_STEPS`' bit 4 (pairs of single steps by two units, the first
  `STEP_PAIRS`, 10, then the best cut).

The same benchmark:

| Setting | ms a position (300 tries) | Large removals, turns 7 to 14 | Missed (<= -20) | Badly missed (<= -50) | Badly missed, all turns |
| --- | --- | --- | --- | --- | --- |
| v032 | 13.97 | 256 | 20% | 12% | 12% |
| s3 (steps3) | 14.22 | 248 | 17% | 9% | 9% |
| s3-scatter3 | 17.91 | 247 | 15% | 6% | 6% |
| s3-scatter3-after | 15.68 | 247 | 13% | 6% | 6% |
| s3-scatter3-w60L5 | 13.84 | 247 | 15% | 6% | 5% |
| s3-pairs (`REPLY_STEPS` 7) | 14.64 | 248 | 17% | 8% | 9% |
| s3-pairs-scatter3-after | 16.68 | 247 | 13% | 5% | 6% |

The scattered cuts halve the badly missed removals from v032 (12% to 5 or
6%); pairs of steps add little alone.

### Round 2: screening

Variants (from 65d86725800b): s3 (steps3 again), s3-scatter (`SCATTER_CUTS`
3, `SCATTER_AFTER_STEPS`, 4cd721d3cbae), s3-scatter-pairs (and
`REPLY_STEPS` 7, 3deeab4f567a), s3-scatter-w60L5 (`SCATTER_CUTS` 3, width
60, length 5, e00fc93a1192), s3-scatter-own (s3-scatter and `MACRO_SCATTER`
2 in `actions.rs`: scattered cuts offered to our own search, 9dab590fe4f2),
s3-own (steps3 and `MACRO_SCATTER` 2, 3e6d234bdcac). A local check of
s3-scatter-pairs against v032, 8 pairs (seed 871): no fault, answers after
the first 41.6 ms median, 48.0 ms at 99.9%.

`screen.yml` run 45869403769 on the owner's approval (2026-11-26; a first
dispatch, run 45869280130, failed on a garbled list of variants), full
games against v032, 400 pairs per variant over 2 shards (the list cut the
name of s3-scatter-w60L5 to `5`):

| Variant | Elo over v032 [95%] | Timeouts of the variant (of 800 games) |
| --- | --- | --- |
| s3 | -4.3 [-24.0, +15.3] | 0 |
| s3-scatter | -27.9 [-48.1, -7.8] | 9 |
| s3-scatter-pairs | -23.5 [-43.6, -3.5] | 9 |
| s3-scatter-w60L5 | -16.5 [-36.2, +3.1] | 0 |
| s3-scatter-own | -14.8 [-34.3, +4.7] | 8 |
| s3-own | -16.5 [-35.3, +2.2] | 0 |

steps3 is level this time (+15.6 in round 1): together, about +5, within
the noise of one screening. The scattered cuts, which halve the replies
badly missed, do not make the search stronger: with them after each step it
times out in about 1% of its games on CodinGame-speed runners, and without
those timeouts (w60L5) it is still weaker; offered to our own search, they
are a little weaker too.

## Result

Dropped: weaker or level in screening (+15.6 [-4.0, +35.4] at best, steps3; -11.9 in E088). What E083 to E088 taught is in
`research/2026-11-26-early-fights-lessons.md`.
