# 0040. The studio also plays each game's baseline bots

- Status: accepted (owner, 2026-10-17)
- Date: 2026-10-17
- Completes: [0025](0025-studio.md), decision 4
- Scope: framework

## Context

The studio's computer seats are built from a game's releases, the frozen files in `games/<game>/releases/` ([ADR 0025](0025-studio.md), decision 4; [ADR 0030](0030-studio-on-github-pages.md) for the site). A game gets its first release only once an approach is chosen, at stage 9 of its lifecycle ([ADR 0035](0035-new-game-lifecycle.md)), while its studio is built at stage 7. Until then the studio has no bot to play. For A Code of Ice and Fire this leaves stage 7 with nothing to watch, since the game also has no human play ([ADR 0039](0039-ice-and-fire-maps-and-studio.md), decision 4).

Each game already has simple bots in `games/<game>/bots/`, built from the current code by the bundler: a random bot, which anchors the ratings, and greedy or league bots (stage 6 of the lifecycle). They read `CG_SEED` like the releases, so with a fixed seed they answer the same moves every time.

On 2026-10-17 the owner chose, among "baseline bots in the studio", "record files loaded by hand" and "wait for the first release", to have the studio offer each game's baseline bots, for every game.

## Decision

1. **The studio offers a game's baseline bots as computer seats, beside its releases, on the local server and on the site.** They play as releases do: against a human with a fixed seed and iterations, so takebacks replay them; against each other in real time.
2. **Each game's adapter names its baseline bots, among the folders of `games/<game>/bots/`.** A bot that exists only to check something, such as a rules-check bot, is never one.
3. **A baseline bot is named `<game>-<folder>`, the name of its bundle, and records keep that name where they keep a release's.** A release is always `<game>-vNNN`, so the two cannot clash, and the record format does not change.
4. **A baseline bot is built from the current code with the bundler, then compiled as a release is.** The local server builds it on first use and caches it by the bundle's contents; the site builds it with the releases. Unlike a release, it changes when its code changes: a record names the bot, not its version.

## Consequences

- A game can be tried in the studio from stage 7, before its first release, and a release can be watched against the baselines it is rated against.
- The local server needs `rustfmt`, which the bundler runs, besides `rustc`; both come with Rust.
- A record with a baseline bot cannot say which version of it played. Records keep the answers, so the game still replays; only restarting that bot on an old record may answer differently.
- Each baseline bot must replay identically from its seed, as each release must, and is tested for it.
