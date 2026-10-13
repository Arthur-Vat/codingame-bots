# 0032. Analysis: one program per game, run as WebAssembly in the page or natively by the local server

- Status: accepted (owner, 2026-10-13)
- Date: 2026-10-13
- Completes: [0025](0025-studio.md), decision 6; [0030](0030-studio-on-github-pages.md), decision 2
- Scope: framework

## Context

[ADR 0025](0025-studio.md), decision 6, says that analysis comes from an engine built from the current code with the latest release's settings, which reports the best moves, the win, draw and loss chances and the expected lines as it thinks. It lets code that ends up in bots gain read-only reporting only if bots play identical moves for a fixed seed and iterations and keep their speed. [ADR 0030](0030-studio-on-github-pages.md) added a second engine, in the browser, for the site on GitHub Pages, and asks each feature, analysis included, to work with both engines.

The search the bots use (`cg-search`'s Monte Carlo tree search) keeps each node's average score, not its share of draws, so it cannot tell a draw from an even game. The local server answers each request in one buffered response from a few worker threads, and the front end already polls the server for the state of a game. The browser engine runs each release in a Web Worker as a WASI program, and the rules as a WASI program with a fresh instance for each request.

On 2026-10-13 the owner approved the plan of phase 6's step D: the analysis engine is built as WebAssembly and runs in the page, the local server's route comes second, the engine uses the current code with the latest release's settings rather than the bot that played the game, and the evaluation chart is filled in the background at about 100 ms per position.

## Decision

1. **Each game has one analysis program, `games/<game>/analysis`.** It is built from the current code with the settings of the game's latest release, and it is a library with a small command around it: one JSON request (the moves played from the start, a number of iterations, how often to report) gives JSON lines, one report per step, each with the moves the search tried, their visits, win, draw and loss chances for each seat and their lines, until it has run the iterations, proved the position or filled its tree. The protocol is written in `studio/README.md`, and is the same for every game.
2. **The search counts draws only when asked.** `cg-search` gains a report of the root, a way to go on thinking on the same position, and an optional count of draws per node whose default costs nothing; bots keep the default, and play the same moves at the same speed (ADR 0025, decision 6).
3. **In the browser engine, the program runs as WebAssembly in a Web Worker.** It is compiled for `wasm32-wasip1` with the rules and the releases, and each request runs in a fresh instance in its own worker; stopping, or moving to another position, ends the worker.
4. **The local server runs the same library natively.** It links each game's analysis library and runs one request per thread, so that `cargo run -p studio` needs no WebAssembly. Both engines answer the same routes (`POST /api/analysis` starts a request, `GET /api/analysis/<id>` gives its latest report, `DELETE /api/analysis/<id>` stops it), and a request nobody reads for a while stops by itself.
5. **The evaluation chart searches each position for a fixed number of iterations, with a fixed seed.** The same game gives the same chart every time on one engine, and the chart costs about 100 ms per position, filled in the background while the analysis board is shown.
6. **Analysis runs on the analysis board only.** It never runs while a game is being played, so that it cannot help a human against a bot or slow the bots down.

## Consequences

- A new game gets analysis by adding its analysis crate and registering it with the studio; the front end does not change.
- The analysis follows the code, not a release: when a new release changes the settings, the analysis program must change with it. A test fails when its settings differ from the latest release's.
- Bots pay nothing for the reporting, and a test keeps the size of their search nodes unchanged.
- Analysis in the browser is about 1.3 to 1.45 times slower than natively, like the bots; the reports say how many iterations they come from.
- Each engine runs its own copy of the program, so the server and the browser can give slightly different numbers for the same position.
