# 0005. Ultimate Tic-Tac-Toe as the first game

- Status: accepted
- Date: 2026-10-06

## Context

The first game shakes down the whole pipeline: engine, referee, arena, statistics, releases. A game with complex rules would mix pipeline bugs with rule bugs.

## Decision

Start with CodinGame's Ultimate Tic-Tac-Toe (UTTT).

## Consequences

- Small rules, perfect information, fast simulation; Monte Carlo tree search is the well-known strong approach.
- UTTT has no random map, so two near-deterministic bots would replay the same game. Matches must start from varied openings and be played in pairs with seats swapped.
- Strong bots draw often, so the statistics must handle draws (scored on game pairs).
- CodinGame's tiebreak (more small boards won) differs from common UTTT rules and must be tested explicitly. See `games/uttt/README.md`.
