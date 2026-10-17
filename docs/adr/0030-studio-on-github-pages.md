# 0030. The studio also runs in the browser, published on GitHub Pages

- Status: accepted (owner, 2026-10-11); decision 6 completed by [0031](0031-arena-games-archive.md); decision 2 completed by [0032](0032-analysis-engine.md); decision 4 completed by [0040](0040-baseline-bots-in-the-studio.md)
- Date: 2026-10-11
- Supersedes: [0025](0025-studio.md), decision 1 (for the owner only, never hosted)
- Completes: [0025](0025-studio.md), decisions 2, 3, 4 and 5; [0026](0026-node-for-the-studio.md), decision 3; [0027](0027-game-records.md), decision 3
- Scope: framework

## Context

The owner tried the studio on Windows on 2026-10-11 and was satisfied with it, but installing Rust and Node and typing commands to start it were the obstacle. A one-click download was planned first; in the conversation of 2026-10-11 the owner preferred a site on GitHub Pages, with the bots running in the browser, and wanted to keep the local server, with the same site.

[ADR 0025](0025-studio.md) made the studio a local app for the owner only, never hosted, whose Rust server judges the rules and runs each release as a native process. GitHub Pages serves static files only: no server code runs there, and it cannot set response headers. [ADR 0026](0026-node-for-the-studio.md) chose the front end's license policy for a tool that is never published. The repository is public ([ADR 0002](0002-public-repo.md)), so a Pages site of it is public too.

A feasibility test on 2026-10-11, in Chromium from a static server that sets no headers, found:

- Every release up to `uttt-v010` compiles unchanged with `rustc --target wasm32-wasip1` into a WebAssembly file of about 200 KB that calls six WASI functions (environment, clock, read, write, exit). About a hundred lines of JavaScript provide them.
- The referee and Ultimate Tic-Tac-Toe's studio adapter compile to the same target unchanged; a small WASI program around them answered each request in 1 to 2 ms.
- A bot reads its input with a blocking read. In a Web Worker, that read can wait on a `SharedArrayBuffer`, which browsers allow only on cross-origin isolated pages; a service worker of about fifteen lines adds the two headers isolation needs, at the cost of one reload on the first visit.
- With a fixed seed and iterations, the WebAssembly releases played the same moves as their native builds in every game compared (`uttt-v010` against `uttt-v001` at 3,000 and 20,000 iterations, `uttt-v010` against itself at 50,000), and the same game twice in a row. They ran about 1.3 to 1.45 times slower than native. With their own time budgets, `uttt-v010` and `uttt-v009` answered in about 918 ms on the first turn and 91 ms after (limits 1,000 and 100 ms).
- Firefox and Safari were not tested.

The owner chose, on 2026-10-11, to keep both engines rather than move the local server to the browser engine or drop it.

## Decision

1. **The studio is also a public static site on GitHub Pages, built from `main` by a workflow.** Anyone with the link can use it. It has no accounts, sends nothing anywhere, and keeps what it stores in the visitor's browser. It still draws its own boards and copies no artwork, text or code from CodinGame. The local server stays, listening on 127.0.0.1 only.
2. **One front end, two engines, chosen by where the site is served from.** Served by the local server, the front end uses its API: native bots and saved games on disk, as before. Served as static files, it uses a browser engine that offers the same features with the same records. The two keep the same behaviour; a change to one is made to the other in the same pull request.
3. **In the browser, the rules still come only from the Rust referees.** The referees and the games' adapters, with the studio's logic that needs the rules (positions, legal moves, results, checking imported records), are built into a WebAssembly program that the browser engine calls. The browser engine itself holds no rules, as the front end holds none; the Rust code that both engines use is shared, not copied, in crates of their own under `studio/` beside the server, the game interface and the front end.
4. **In the browser, each release runs as WebAssembly in a Web Worker, built by CI from its frozen file.** It is compiled with the same Rust version as CI, for the target `wasm32-wasip1`, and fed its input through a small WASI layer of our own; it is never committed. A blocking read waits on a shared buffer, and a service worker of our own makes the page cross-origin isolated when the host cannot. A release that does not compile to WebAssembly, or does not replay identically there, is not offered in the browser.
5. **Fixed-iteration play in the browser is measured in the browser.** The think time is converted to iterations from a measurement of that release in the visitor's browser, kept in the browser, so a takeback replays exactly as with the local server.
6. **In the browser, saved games live in the browser's storage on that computer.** They use the record format of [ADR 0027](0027-game-records.md) and are exported and imported as files, as with the local server; nothing is sent to the repository or elsewhere.
7. **The published site carries the license notices of everything it bundles.** That covers the npm packages in the built front end and the crates in the WebAssembly files, all under the permissive licenses already allowed; the notices are gathered when the site is built.

## Consequences

- The owner opens a link instead of installing Rust and Node; the setup guide remains for the local server.
- The site is public; strangers can play the releases, which are public already.
- Two engines must be kept in step. Each feature of the studio, step D's analysis included, needs both, and the browser tests cover the browser engine with the real WebAssembly files, not a mocked API.
- Games saved in one browser are not seen by another, nor by the local server, until exported and imported. Clearing the browser's data deletes them.
- CI gains the WebAssembly target and a deployment workflow; the repository's Pages source must be set to GitHub Actions, once, by the owner.
- Bots in the browser take 30 to 45% more time than native for the same search, so a release plays a little weaker there at real time limits; fixed-iteration play is unaffected.
- A future release that uses something WebAssembly lacks (threads, for example) must still run natively; it is simply not offered in the browser.
