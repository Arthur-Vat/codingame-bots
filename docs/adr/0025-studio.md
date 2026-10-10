# 0025. A local studio to play and review games in the browser

- Status: accepted (owner, 2026-10-10)
- Date: 2026-10-10
- Completes: [0022](0022-scopes-and-names.md), decision 2 (a new scope)
- Scope: framework

## Context

On 2026-10-10 the owner asked for a user interface in the style of Lichess: choose a game, play it against a friend on the same screen or against any release, watch two releases play each other, review games step by step, and see a bot's evaluation and best moves. It is for the owner only, runs on a Windows computer, and is used from a desktop browser. The owner asked whether it belongs in this repository or a private one; the only worry was whether drawing our own boards for CodinGame's games is allowed, and the bots are already public ([ADR 0002](0002-public-repo.md)).

What exists shapes the design:

- The arena runs bots as separate processes that read CodinGame's text protocol, with the game's time limits, through one `Referee` trait per game ([ADR 0011](0011-framework-structure.md)). A referee cannot undo a move: any position is reached by replaying the game's answers from its seed.
- Every Ultimate Tic-Tac-Toe release reads `CG_SEED` (its random seed) and `CG_FIXED_ITERS` (a fixed number of search iterations instead of a time budget). With both set, a release should answer the same moves every time, which makes it possible to restart a bot and replay a game into it.
- A bot keeps its own state between turns and cannot be copied from outside its process on every platform; a takeback against a bot therefore means restarting it and replaying the game.
- Releases never change, and they print no evaluation; code that ends up in a bot uses the standard library only.

The owner chose, in the conversation of 2026-10-10: this repository, a local app, TypeScript with React for the front end, a home screen of games, the Lichess layout, takebacks with fixed iterations, analysis from an engine built from the current code, and our own artwork.

## Decision

1. **The studio is a local app in this repository, for the owner only.** Its server listens on 127.0.0.1 only, has no accounts, and is never hosted. It draws its own boards and pieces and copies no artwork, text or code from CodinGame.
2. **It has three parts and one plug-in point per game.** `studio/server` is the Rust server, a tool crate that may use the crates.io dependencies `deny.toml` allows; `studio/game` is a small crate with the interface a game implements for the studio; `studio/web` is the front end in TypeScript, React and Vite ([ADR 0026](0026-node-for-the-studio.md)). Each game adds an adapter crate `games/<game>/studio` and a board renderer in `studio/web/src/games/<game>/`. Work on `studio/` uses the commit scope `studio`; work on a game's adapter or renderer uses that game's scope.
3. **The server is the only judge of the rules.** A game is its seed and the answers of each turn, and any position is rebuilt by replaying the game's referee. A human's legal moves come from the referee's own turn input. The front end draws positions and turns clicks into answers; it holds no rules of its own.
4. **Bots run as processes, as in the arena.** A release is compiled with `rustc` from its frozen file and the binary cached outside the repository; its file never changes.
5. **Against a human, a bot plays with a fixed seed and a fixed number of iterations, so a takeback replays it exactly.** The think time the owner picks is converted to iterations from a measurement of that release on the owner's computer. A takeback restarts the bot and replays the game up to the chosen move. A release that does not replay identically offers no takeback rather than a wrong one. Bots that play each other use real time limits, as in the arena.
6. **Analysis comes from an engine built from the current code, not from releases.** It uses the latest release's settings, takes any position, and reports the best moves, win, draw and loss chances and the expected lines as it thinks. Code that ends up in bots may gain read-only reporting for it only if bots play identical moves for a fixed seed and iterations, and their speed stays at the baseline.

## Consequences

- One repository keeps the studio next to the referees, the arena and the releases it uses, under one CI and one set of rules for Claude's sessions.
- Every new game must provide a studio adapter and a renderer, as well as its referee.
- The repository gains a second toolchain, Node, for the front end ([ADR 0026](0026-node-for-the-studio.md)), and CI gains jobs for it, including one on Windows.
- Game records need a format and a place to live ([ADR 0027](0027-game-records.md)).
- `scripts/pr-hygiene.sh` must accept the scope `studio` before the first pull request that uses it.
- Takebacks depend on the releases being deterministic, which must be tested for each release before it is offered.
