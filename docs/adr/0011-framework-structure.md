# 0011. Framework structure: text-protocol referees, reference rules, textual bundler

- Status: accepted; the consequence that the bundle keeps every comment superseded by decision 4 of [0016](0016-self-play-training.md)
- Date: 2026-10-07

## Context

Phase 1 builds the generic part of the framework. Three choices shape every later game:

1. How a game plugs into the arena. The roadmap first named a single `Game` trait for both the referee and the bots' search, in `cg-core`.
2. Where the rules of a game live, and how a rules bug is caught when the bots and the referee are written by the same hands.
3. How a bot and the crates it uses become the single file CodinGame accepts.

## Decision

1. **The arena talks to games through a `Referee` trait in `cg-arena`, in CodinGame's text protocol.** A referee says which seats act this turn, produces each seat's input as text, declares its time limits, and judges the answered lines. Several seats may act in one turn, so simultaneous-move games fit without changing the arena. The search interface for bots (typed moves, fast state) is a separate concern, decided with `cg-search` in Phase 4.
2. **Each game has a readable reference referee and, from Phase 2, a separate fast engine for bots.** The referee implements `RULES.md`, which cites the game's official source for every rule. The fast engine is checked against the referee by parity tests, and the referee against real CodinGame games.
3. **The bundler works on text.** It inlines `mod name;` files, drops test modules declared on their own line after `#[cfg(test)]`, wraps each workspace library in a root module, rewrites `crate::` and crate-name paths to match, and formats the result with rustfmt. It refuses crates.io dependencies on the bot side and `#[path]` attributes.

Supporting choices: each game has a tiny arena binary that hands its referee to the shared command line; the arena gives bots reproducible seeds in `CG_SEED`, derived from the game seed, the seat and the bot's name; tools may use crates.io (`clap`, `serde`, `serde_json`), bot-side crates may not.

## Consequences

- A new game adds `RULES.md`, a referee, an arena binary and bots; the arena, statistics and CI are reused unchanged.
- The bundle keeps every comment and reads like hand-written code. In exchange, workspace crates used by bots must follow the conventions listed in `cg-bundler`'s documentation; violations show up as a bundle that fails to compile in CI.
- Running bots as processes costs a few milliseconds per game but measures time and failures the way CodinGame does: 1,000 random games take about 3 seconds on two cores.
- The roadmap's single `Game` trait is replaced by this split; `docs/ARCHITECTURE.md` describes it.
