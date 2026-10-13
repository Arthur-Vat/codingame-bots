# <Game name>: model

The problem as a search or learning problem, for stage 3 of `docs/NEW_GAME.md`. Rules are in `RULES.md`; this file says how to think about them.

## In plain English

What a player sees, decides and wants each turn, in a few paragraphs.

## Numbers

| Quantity | Value | How it was obtained |
| --- | --- | --- |
| Players | | statement |
| Turns per game (typical, maximum) | | estimate, then measured at stage 6 |
| Legal answers per turn (start, middle, end) | | estimate, then measured |
| State size in bytes | | engine |
| Time per turn (first, later) | | statement |
| Engine speed (states or playouts per second) | | stage 5 |

## Formal model

- Players: the set of seats.
- States: what a state holds; what each player observes (all of it, or an observation function).
- Actions: the set of legal answers in a state; a turn of several commands is a sequence or a set.
- Transition: deterministic or random; players move one after the other or simultaneously.
- Outcome: terminal states and the payoff of each player (win, draw, loss, or a score).
- Class: for example, a two-player zero-sum game of perfect information with alternating moves.

## What follows for bots

- Symmetries, invariants, what can be precomputed or learned, what fits in 100,000 bytes.
- The families of methods this class suits, to be ranked at stage 8.

## Framework gaps

What the framework assumes that this game breaks (stage 4), each with the decision record that settles it.

| Gap | Where | Decision |
| --- | --- | --- |
