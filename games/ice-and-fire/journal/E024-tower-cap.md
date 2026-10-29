---
id: E024
date: 2026-10-29
pull_request: "none yet"
parent: ice-and-fire-v009
hypothesis: A tower that raises the enemy's chain to our HQ can add +30 or more to v009's evaluation (+10 static, the rest through the pressure term), more than a level-3 training's extra cost of 20 that it usually forces; capping what a tower adds at 20 stops towers built for their score alone and beats v009.
change: None released. Local screening only: the plan bot takes off what a side's towers add (their static value plus their gain in the pressure term) beyond 20 a tower.
release: none
sprt: not run: weaker in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Chosen by the owner on 2026-10-29 after E021 ("E024, cap at 20
(Recommended)"), the next item of the study of towers and threats
([research/towers-and-threats.md](../research/towers-and-threats.md)), and
dropped after its screening ("Drop it (Recommended)").

## The change

The study found that a tower raises the enemy's cheapest chain to the
builder's HQ by a median 20 gold (quartile 3: 20), a level-3 training's
extra cost over level 1, and proposed to cap what a tower adds, its static
value plus its pressure gain, at 20.

The study measured one tower's gain at its building; the evaluation sees
all of a side's towers at once, so the cap applies to them together: for
each side with towers, the enemy's cheapest chain to its HQ is costed
again with none of the side's towers (their cells, and the active cells
next to an active tower, enterable at levels 1 and 2 when their unit
allows it; the tower cells as an empty cell of the side). The towers'
gain is the enemy's pressure term with the towers less without them. What
`10 × active towers + gain` exceeds `20 × towers` is taken off the side's
score. A side without towers costs nothing more; one with towers costs one
more chain search.

The code is commit fb469fc on branch
`claude/ice-and-fire-e024-tower-cap` (reverted by the next commit, so
the branch carries the entry alone): the shared tactics gain
`chain::cost_to_with` (the cheapest chain on other entry costs) and
`eval::pressure_for` (the pressure term of a given cost), and the plan
bot's `eval::over_cap` the term, with a test on a corridor where a tower
next to the HQ raises the enemy's chain by 60 (over the cap by 50 when the
enemy is 10 gold short of the old chain, by 0 when it is far). The
minified bundle is 88,529 characters.

## Screening

Against `ice-and-fire-v009` at CodinGame's limits (50 ms a turn, 1,000 ms
the first, no tolerance), 400 seat-swapped pairs on seed 24024, two games
at a time on a 4-core machine; bundles built with `cg-bundler` (minified)
and rustc 1.90.0; read with `ice-and-fire-review screening`. v009's
numbers in the same games in parentheses.

| Variant | Elo against v009 | Score as red | Score as blue | Towers | Mines | Level 1 / 2 / 3 | Gold kept | Timeouts | Median, max ms |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | ---: | --- |
| cap 20 | **−23.5 [−42.6, −4.6]** | 38.2% | 55.0% | 4.06 (3.83) | 2.12 (2.13) | 25.5 / 4.2 / 1.7 (25.9 / 4.4 / 1.8) | 25.6 (29.9) | 3 (5) | 15.3, 126.7 (12.0, 70.0) |

Both bots' answers were at most 31.9 ms in 99% of turns; the slowest
first answers were 661.4 and 698.9 ms.

## What was learned

- **The cap does not cut towers: it adds some.** The variant built 4.06
  towers a game against v009's 3.83. Not measured, but two likely
  reasons: the cap also applies to the opponent's towers, so v009's walls
  count less as obstacles in the variant's evaluation; and a tower whose
  gain is capped still counts its static 10 and the cap, so the term does
  not make one cost anything.
- **The extra chain search costs time:** the median answer rose from 12.0
  to 15.3 ms, a quarter, fewer plans scored in the same 40 ms.
- **Changing what a tower is worth keeps losing.** Towers +15 (E020),
  no static value (E022's control, in E021), a discounted pressure term
  (E021) and this cap all lost or tied. v009's static +10 and its pressure
  term look well placed; the study's next items are about cuts and
  territory (E023) rather than towers.
- Not tried: other caps. A looser one trims fewer towers and would land
  between this loss and v009.
