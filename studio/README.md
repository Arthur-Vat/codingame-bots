# studio

A local app in the owner's browser, in the style of Lichess, to choose a game, play it against a friend or a release, watch two releases play each other, and analyse games with a bot's evaluation and best moves. It runs locally, served by its own server, and also as a public static site on GitHub Pages, with the rules and the bots in the browser: see "On GitHub Pages" and "Two engines" below ([ADR 0030](../docs/adr/0030-studio-on-github-pages.md); phase 6, step F).

The decisions behind it are [ADR 0025](../docs/adr/0025-studio.md) (the studio), [ADR 0026](../docs/adr/0026-node-for-the-studio.md) (Node and npm for the front end) and [ADR 0027](../docs/adr/0027-game-records.md) (game records). The plan is phase 6 of the [roadmap](../docs/ROADMAP.md).

## On GitHub Pages

The studio is published at <https://arthur-vat.github.io/codingame-bots/>. Anyone with the link can use it. It has no accounts, sends nothing anywhere (it only downloads the public files of the site itself, such as the arena games) and stores nothing outside the visitor's browser; it draws its own boards and copies nothing from CodinGame.

- **First visit.** The page reloads once by itself: it registers a small service worker (`web/public/coi-sw.js`) that makes the page cross-origin isolated, which the bots need (see "Two engines"). Later visits are isolated from the start.
- **Browsers.** Tested in Chromium (Chrome, Edge). Others should work if they support cross-origin isolation and module Web Workers; Firefox and Safari have not been tested. Where the page cannot be isolated (a private window without service workers, for example), games between people, the history and the analysis board work, and a game with a bot is refused with a message.
- **Saved games** live in the browser's storage (IndexedDB) on that computer, for that site only: another browser, another computer and the local server do not see them, and clearing the browser's data for the site deletes them. To keep a game, press Export file (on the game's page, or for a saved game in the game's History tab) and keep the `.json` file; "Load game files" in the History tab imports it again, here or with the local server.
- **Licenses.** A "Licenses" link at the foot of the home page opens `THIRD-PARTY-NOTICES.txt`, with the licenses of the npm packages, the Rust crates, the Rust standard library and wasi-libc (which Rust's WebAssembly target links in) that the site bundles ([ADR 0030](../docs/adr/0030-studio-on-github-pages.md), decision 7). Beside it, `rust-library-copyright.html` is the Rust project's own notice for the standard library and the crates it contains, copied unchanged from the toolchain. wasi-libc's license files are not in the toolchain: they are kept in `web/scripts/licenses/wasi-libc/`, from a named commit of its repository. Vite is listed too, since it adds its module preload helpers to the bundle. `npm run notices` (in `web/`, after `npm run build`, with Rust installed) writes both into `dist/`; it fails if a package or a crate has no license file, or if the toolchain lacks the notice or a license text. `npm run build` does not need Rust, so with the local server the link leads to a missing file (a 404) unless `npm run notices` was run too.
- **How it is built.** The [Pages workflow](../.github/workflows/pages.yml) rebuilds and redeploys the site from `main` whenever something it contains changes (the studio, the games' releases, referees and adapters, the shared crates, the lockfile), and by hand from the Actions tab. Pull requests never deploy: CI builds the same site and runs `npm run notices`.
- **The owner's one-time setting**, which Claude's sessions cannot make: in the repository's **Settings → Pages → Build and deployment → Source**, choose **GitHub Actions**. Until then the workflow's deployment fails.

## Status

The front end's scaffold exists, in [`web/`](web/), and so does the app shell: the top bar, the home screen (one card per game from the server), the game page's layout and the setup menu with its advanced mode. Games are playable: Ultimate Tic-Tac-Toe's board is drawn from the server's frames, and the three modes work against the server: a friend on the same screen, a release with takebacks, and two releases playing each other. Human clocks (minutes plus increment) run in the front end, which ends the game on a timeout; a bot shows the time it has used. The move list and the keys ← → Home End look back at earlier positions, and input is accepted on the latest one only. A game's page and its analysis board fit the window without scrolling (wide layout): the board is as large as the column and the window's height allow, up to 900 px, and the table beside it is as tall as the squares (below 1181 px wide the panels stack and the page may scroll for them, never for the board). Before a game, the board is drawn empty. Resign asks a second click ("Confirm resign"), and the setup menu warns that Start ends a game in progress; a game won with three small boards in a row shows a dashed line through them. Each game has one page with two tabs in the top bar, **Play** and **History**: `#/game/<game>[/<session>]` and `#/game/<game>/history[/arena]`. Each tab's link leads back to what that tab showed last (the session in Play, the set in History), kept in memory until the page is reloaded. The Play tab stays in the page, hidden, while History or the analysis board is shown, so a game goes on (the bots, the human clocks, a timeout is still reported) and its state is as it was when the tab is shown again; History starts afresh each time it is shown. A game between two bots is not shown while it is played: the board stays empty, with a spinning circle and "Playing…" over it (the info card keeps its status and the number of the move being played), and no move, result or time used appears; the viewer sees the game once it is played.

**The setup menu.** One modal, **Game setup**, starts every game (Escape or × closes it). At the top, three buttons choose the mode, one at a time: **A friend** (two people, one screen), **A release** (you against a release) and **Bot against bot**. Under them come the choices of the mode: for a friend, the clock (minutes per side and increment); for a release, the release (the latest by default), your side (X, random or O), the bot's think time and your clock; for two bots, the X release, the O release and one think time for both. At the bottom, **Advanced mode** adds the seed (the clock's if left off) and, for two bots, a think time for each bot (starting from the shared one; with advanced mode off both bots use the shared one). What was chosen in a mode is kept while the menu stays open and another mode is tried. **Start** sends the request. The Play tab opens the menu by itself, on the mode last chosen in the page (a release at first), whenever it is shown with no game: from the home page's card, from the History tab or the analysis board, or by address. It does not open over a game, running or finished, nor again after it was closed while Play stays shown; it waits for the server's list of games and never shows on another tab. The left column's **New game** button opens it with the mode last chosen, and **New game** after a game (in the end block, or on the analysis board of a game just played) with the mode of the game just played.

**After a game.** A game that ends stays in the Play tab, at the same address: the playback bar appears under the board (first, previous, play or pause, next, last, and the speeds 0.25×, 0.5×, 1×, 2×, 4×: the whole game lasts 120 s at 1×) on the frame that was on the board (a game between two bots opens on its first move, paused, also when it was over when opened), the move list is clickable, the keys ← → Home End and Space work, and Save and Export file are above the end block. The end block has three buttons, in this order, for every mode: **Rematch** (the same settings, a new seed), **New game** (the setup menu, on the mode just played) and **Analysis board**. A game between two bots no longer changes the address when it ends.

**The analysis board** is a separate view of a game, not a tab: `#/game/<game>/analysis/<kind>/<id>`, with the kind `session` (a game played in this page, by its session id; shown only once it is over, so a hand-typed address of a game still being played says "This game is still being played." and shows no move), `saved` (a saved game), `file` (a record loaded in the browser) or `arena` (an arena game, see "Arena games"). Anything else, such as `#/game/<game>/analysis` alone or the addresses of the old Review tab (`#/game/<game>/review...`), is the home page. It shows the game from its first position, read-only, with the playback bar, the move list, the players' strips, Save to history and Export file, the board settings and the keys; the place for the engine's analysis (the reserved bar beside the board and a note under the move list saying "The engine's analysis comes in a later version.") is kept for step D. A link at the top of the left column leads back: "Back to the game" for a game played here, "Back to history" for the others (to the arena games for an arena game, else to your games). The tab marked current is the one the game came from: Play for a game played here, History for the others. A game just played also offers **Rematch** and **New game** there, as long as the Play tab still holds that session (not after a reload, nor once another game was started in the Play tab); they act through the Play tab, which starts the new game or opens its setup menu. Games from the History never offer them. A game picked in the History (**Analysis board** in each row, or **Open a file**) opens directly on the analysis board.

The History tab lists the game's saved games ("My games"; `#/game/<game>/history/arena` lists its arena games, see "Arena games") with filters (release, result, source, dates, sent to `GET /api/history` with `game=<game>`) and, for each one, Analysis board, Export and Delete (asked twice in the row). "Load game files" adds several `.json` files to the history and says how many were imported, were there already, or were refused, with the server's reasons (a workflow artifact comes zipped: unzip it first); "Open a file" opens one file on the analysis board without saving it, on the page of the game the file names. The same front end runs on the browser engine, in the page, when no server answers: the rules in WebAssembly, the bots as WebAssembly in Web Workers, saved games in IndexedDB. The site is published on GitHub Pages by a workflow from `main`, with the license notices of what it bundles (see "On GitHub Pages"); the first deployment needs the repository's Pages source set to GitHub Actions. The engine's analysis is not there yet.

## Front end

TypeScript, React and Vite, in `studio/web/`. It needs Node 22 (the line is written in the `engines` field of `web/package.json`, which CI follows) and npm. Run everything from `studio/web/`:

```sh
npm ci                  # install the locked dependencies
npm run dev             # the app, with reload, at http://localhost:5173 (calls to /api go to the server on port 8411)
npm run build           # type check, then the production build in dist/
npm run preview         # serve the production build, as static files: the browser engine
npm run wasm            # build the rules and the releases to WebAssembly (the browser engine's files)
```

The checks, all run by the `Studio: front end` job of CI:

```sh
npm run licenses        # every package of the lockfile has an allowed license (licenses.json)
npm run format:check    # Prettier (npm run format fixes)
npm run lint            # ESLint
npm run typecheck       # TypeScript
npm test                # unit tests (Vitest)
npm run e2e             # browser tests (Playwright, Chromium); builds the WebAssembly files and the app, and serves it
```

To work on the app against the real server, start the server in one terminal and the dev server in another:

```sh
cargo run --release -p studio     # the API, at http://127.0.0.1:8411
npm run dev                       # in studio/web: the app, proxying /api to the server
```

The browser tests do not need the server. Most mock the API (`e2e/*.spec.ts`; their pages are told to use the server's engine, see below). `e2e/browser-engine.spec.ts` runs the browser engine for real, with real bots, on the production build served by `npm run preview`, which has no `/api`. So `npm run e2e` and `npm test` need the WebAssembly files, and therefore Rust with the `wasm32-wasip1` target (`rustup target add wasm32-wasip1`): `npm run e2e` builds them first, and `npm test` fails with a message saying to run `npm run wasm` when they are missing. `npm run build` does not need Rust. `npm run wasm` runs a bash script (`scripts/build-studio-wasm.sh`): on Windows use Git Bash or WSL for it; running the local server needs none of this.

The first `npm run e2e` on a computer needs Chromium: `npx playwright install chromium`.

## Two engines

The front end calls one API (the routes below) and has two engines behind it ([ADR 0030](../docs/adr/0030-studio-on-github-pages.md)). They keep the same behaviour: a change to one is made to the other in the same pull request.

- **The server's engine**: `fetch` to the local server (`cargo run --release -p studio`): native bots, saved games on disk.
- **The browser engine** (`web/src/engine/`): the same routes, JSON shapes, status codes and error bodies, answered in the page. `routes.ts` is the router. `rules.ts` calls `wasm/rules.wasm` (see "The rules in WebAssembly") with a fresh instance for each request. `sessions.ts` mirrors the server's sessions: statuses, progress, takebacks that restart each bot and replay it, and a generation counter that cancels a bot's turn when a takeback or a deletion makes it stale. `bot.ts` and `botWorker.ts` run a release in a Web Worker, its standard input blocking on a `SharedArrayBuffer`. `speed.ts` measures a release's speed in the browser (3000 iterations, seed 1, the release against itself) for "fixed" mode. `storage.ts` keeps the saved games. `wasi.ts` is our own small WASI layer.

**How the engine is chosen.** At start-up, before the app renders, the page asks `GET /api/games`: a 200 answer that is a JSON array means the studio server is there; anything else (404, a page of HTML, an error, no answer) means the browser engine. The choice shows in the top bar (a tooltip, and "in your browser" beside the title for the browser engine). For diagnosis, `localStorage['studio.engine'] = 'server'` or `'browser'` forces one; the browser tests of the server's engine use it, since `preview` has no server. With `npm run dev` and no server on port 8411, the page uses the browser engine, which needs the files of `npm run wasm`.

**Isolation, and the one reload.** A bot reads its input with a blocking read that waits on a `SharedArrayBuffer`, which browsers allow only on a cross-origin isolated page (the headers `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`). The dev server sets them. A static host that cannot (GitHub Pages, `npm run preview`) is helped by `public/coi-sw.js`, a service worker of ours that adds the two headers to same-origin responses: the first visit registers it and reloads the page once (a `sessionStorage` key stops a second reload), and later visits are isolated from the start. Nothing is loaded from another origin. Where the page cannot be isolated (no service worker, for example in some private windows), games between people, the history and the analysis board work, and a game with a bot is refused with "This browser does not let bots run on this page (cross-origin isolation is unavailable)".

**Where the browser engine keeps data.** In that browser, on that computer, for that site: saved games in IndexedDB (database `studio`, store `games`; each value holds the record text exactly as saved), and measured speeds in `localStorage` (key `studio.speed.v1`, by release name and the SHA-256 of its WebAssembly file). Clearing the site's data deletes the saved games: export what matters (Export, in the History tab). Nothing is sent anywhere; the only requests are for the site's own files, among them the arena games. The local server's saved games and the browser's are separate, until exported and loaded. Where IndexedDB is unavailable, saving and the history answer 409 with a message, and playing still works.

**What differs from the server.** Bots run about 1.3 to 1.45 times slower in WebAssembly than natively, so a release plays a little weaker at real time limits; fixed-iteration play is unaffected. A release that does not compile to WebAssembly is not in `wasm/manifest.json`, so the browser does not offer it. A release's speed is measured in the browser, not taken from the server's `speed.json`.

## Dependencies

Few, each one a choice (ADR 0026). Versions are exact, and every version in the lockfile was published at least two weeks before it was added (`npm install --before=<date>` finds such versions). Only permissive licenses are allowed, listed in `web/licenses.json`; `web/scripts/check-licenses.mjs` refuses a package without a license, or with one that is not on the list.

The lockfile is made with npm 11, because npm 10 (the one shipped with Node 22) fails on the peer dependencies of Vitest and rewrites the lockfile on `npm install`; `npm ci` works with either. To change a dependency, run from `studio/web/`:

```sh
npx npm@11 install --save-exact <package>@<exact version> --before=<date two weeks ago, as YYYY-MM-DD>
```

Then commit `package.json` and `package-lock.json`, and run the checks above.

Dependabot proposes minor and patch updates once a month, in one pull request. Major versions are left to a hand-made pull request each, which checks the toolchain still works with them (for example typescript-eslint's supported TypeScript versions, the `@types/node` major matching Node 22) and the licenses of any new package. The ignore rule may also hold back a security fix that only exists in a new major: that one is done by hand too, when GitHub reports the advisory.

## How a game plugs in

A game adds an adapter crate, `games/<game>/studio`, that implements `studio_game::StudioGame` ([game/](game/)) and holds no rules of its own: positions come from replaying the game's referee, and a human's legal moves from the referee's turn input.

- `info`: the game's id (its folder name) and display name.
- `new_referee`: a fresh referee for a setup (seed and opening plies), the one the arena uses.
- `opening_turns`: the leading turns the referee imposes (its opening plies); the server plays them itself.
- `frames`: the JSON the board renderer draws: frame 0 is the position after the opening, then one frame per step; frame `i` of a turn-based game shows the turns up to `opening_turns + i`.
- `human_moves`: the moves a human in a seat may play, each with the answer lines to send the referee.

`studio_game::live_game` replays turns with a fresh referee into a `cg_arena::live::LiveGame`. The adapter's frame format is documented in its crate; `games/uttt/studio` is the first.

A game's board renderer lives in `web/src/games/<game>/` and is registered in `web/src/games/registry.ts`, which maps the game's id to a React component that draws one frame and reports a chosen action (`onAction`, the `action` of one of the session's `human_moves`), and may add a function that words how a finished game ended (`resultDetails`) and an empty frame, the board drawn before a game starts (`emptyFrames`).

## Running the server

```sh
cargo run --release -p studio
```

Then open the URL it prints, `http://127.0.0.1:8411/`. The server listens on 127.0.0.1 only and refuses requests whose `Host` or `Origin` is not its own. Without the front end built (`npm run build` in `studio/web`) it still serves the API, and `/` says how to build it.

| Flag | Default | Meaning |
| --- | --- | --- |
| `--port` | `8411` | The port to listen on. |
| `--repo DIR` | the nearest folder above the current one holding `games/` and `Cargo.toml` | The repository root: the games' releases are read from `games/<game>/releases/`. |
| `--web DIR` | `<repo>/studio/web/dist` | The built front end. Any path that is not a file in it gives its `index.html`. |
| `--data DIR` | `<repo>/studio/data` (never committed) | Compiled bots in `bin/`, named `<release>-<hash of the file>`, the speed measurements in `speed.json`, and the saved games in `history/`. |

A release is compiled with `rustc` (from `PATH`) the first time a game uses it, and the binary is reused. Release files are never modified. A computer seat in "fixed" mode needs the release's speed in iterations per millisecond: the first game against a release measures it by playing the release against itself (3000 iterations per answer, seed 1) and keeps the result in `speed.json`; the session's status is `measuring` meanwhile.

### API

JSON in and out; errors are `{"error": "..."}`. Ids are short random hex strings.

| Route | What it does |
| --- | --- |
| `GET /api/games` | The games, each with its releases, newest first. |
| `POST /api/sessions` | Starts a game: `{"game", "seed", "opening_plies": 0, "seats": [S0, S1]}`. A seat is `{"kind": "human", "name"}` or `{"kind": "bot", "release", "think_ms": 10..=10000, "mode": "fixed" \| "realtime"}`. Returns `{"id"}`. Openings are not supported in live games yet, so `opening_plies` must be 0. |
| `GET /api/sessions/{id}` | The session to poll: `status` (`waiting_human`, `bot_thinking`, `measuring`, `compiling`, `rewinding`, `over`, `failed`), `error`, `to_act`, `opening_turns`, `turns`, `frames`, `human_moves` (for the human to act), `result` and `progress` (`{done, total}` while rewinding; `total` is null while measuring). |
| `POST /api/sessions/{id}/move` | `{"seat", "index"}`: plays `human_moves[index]`. 409 if it is not that human's turn or the index is wrong. |
| `POST /api/sessions/{id}/takeback` | `{"turns": k}`: keeps the first `k` turns by replaying the game, and restarts each bot by replaying its answers. 409 against a "realtime" bot, for `k` out of range, or while the session is busy. A release that does not replay identically fails the session. |
| `POST /api/sessions/{id}/end` | `{"seat", "reason": "resign" \| "timeout"}`: the other seat wins. A timeout ends with `Timeout {seat, limit_ms: 0}` (the front end runs the human clocks); a resignation with `Resigned {seat}`. Only human seats can end a game this way (409 for a bot). |
| `GET /api/sessions/{id}/record` | The game so far as a game record, format 1, source `studio` ([ADR 0027](../docs/adr/0027-game-records.md)). A bot's `command` is the release's name (the path of the compiled binary is specific to the computer). A game not over ends with `Aborted {reason: "unfinished"}`. |
| `POST /api/sessions/{id}/save` | Saves the session's record in the history (same as `GET .../record` then `POST /api/history`). Returns `{"id", "duplicate"}`. |
| `POST /api/history` | Saves one game record (format 1), atomically (a temporary file renamed into place; a file of the same name that is not a record is replaced) in the history. It must be format 1, of a known game, and replay with the game's referee, or the answer is 400 naming the turn that breaks the rules; the one exception is a game that ended with an invalid answer, whose last turn may be the invalid one. The end and winner must also agree with the replay: `finished` is the referee's own result (the game is over, with the same winner), `resigned`, `timeout`, `crash` and `invalid` are won by the other seat, `aborted` has no winner; otherwise 400 with the reason. Returns `{"id", "duplicate"}`: the same record saved twice is kept once (`duplicate` is true the second time). |
| `GET /api/history` | The saved games, newest first, as `{id, game, unix_time, players: [name, name], winner: 0 \| 1 \| null, end, turns, source}` (`end` is `finished`, `timeout`, `crash`, `invalid`, `aborted` or `resigned`). Optional filters: `game`, `release` (either player's name; a league record names its bots without the game, `v009`, so the release `uttt-v009` also matches a player named `v009` in a `uttt` game), `result` (`x`: seat 0 won, `o`: seat 1 won, `draw`: a finished game without a winner, `fault`: a timeout, crash or invalid answer, `unfinished`: an aborted game), `source` (`studio` or `arena`), `from` and `to` (`YYYY-MM-DD`, UTC days, both included). Values are percent-decoded (`%XX`, and `+` for a space). Files that cannot be read are skipped and named on the server's stderr. |
| `GET /api/history/{id}` | The saved record, as the front end offers it for export. Ids are file names without `.json`, made of `a-z`, `0-9` and `-`. |
| `GET /api/history/{id}/view` | `{"record", "frames", "opening_turns", "shown_turns"}`: the record with the game's frames. `shown_turns` is the number of turns that replay: all, or all but a final invalid one. |
| `POST /api/view` | The same view for a record in the body that is not saved, to show a loaded file on the analysis board before saving it. |
| `POST /api/filter?...` | The rows in the body, a JSON array of rows as `GET /api/history` lists them, that pass the same filters (the same query as `GET /api/history`), newest first. It reads no saved game: the front end uses it for the arena games, which it fetched from the site. 400 for a bad query (read first), or a body that is not an array of rows (`invalid request body: ...`, with serde's message, the same in both engines). The local server reads up to 64 MiB for this route, 1 MiB (413 above) for every other: the archive has no size limit. |
| `DELETE /api/history/{id}` | Deletes a saved game: 204, or 404. |
| `DELETE /api/sessions/{id}` | Stops the bots and forgets the session. Sessions untouched for two hours are dropped. |

The two modes of a computer seat:

- `fixed`: a fixed number of iterations, `think_ms` times the release's measured speed, and the bot seed `cg_arena::runner::bot_seed`; this is what takebacks replay.
- `realtime`: the release's own time budget scaled by `think_ms / 100` (`CG_TIME_SCALE`), with the game's limits scaled likewise plus 50 ms; for bots playing each other. A bot that times out, crashes or answers invalidly loses.

### Saved games

The history is one JSON file per game in `studio/data/history/` (or `<data>/history/`), named `<game>-<unix time>-<12 hex digits of a hash of the record>.json`. The folder is never committed ([ADR 0027](../docs/adr/0027-game-records.md)) and GitHub does not back it up: keep what matters by exporting it (`GET /api/history/{id}` gives the file). Loading a file adds it to the history through `POST /api/history`. The listing reads every file each time, which is fine for a few thousand games.

### Arena games

The history's second set, "Arena games" (`#/game/<game>/history/arena`), lists the games the releases played on GitHub, read-only, beside the visitor's own games ([ADR 0031](../docs/adr/0031-arena-games-archive.md)). They are the SPRT and league samples that the [archive workflow](../.github/workflows/archive.yml) keeps, one archive per game, checked by replaying each record with the game's referee. The [Pages workflow](../.github/workflows/pages.yml) gathers the archives while it builds (`scripts/arena-archive.sh site`, which downloads each game's current archive and merges them with `studio-archive`) and publishes them in the site's `arena-games/` folder: `index.json`, `{"format":1,"rows":[...]}` with the rows of `GET /api/history`, and `records/<id>.json`. The site is rebuilt when an archive changes. Without an archive the build only warns, and the page says "No arena games published yet."

- **Where the front end reads them** (`web/src/arena.ts`): the browser engine, from its own site, relative to the page (`arena-games/`); the local server's front end, from the published site, <https://arthur-vat.github.io/codingame-bots/arena-games/>, so with the local server they need a network connection (GitHub Pages allows cross-origin reads; no credentials are sent). Your own games never do.
- **What the page does.** It fetches `index.json` once for as long as the page lives and checks it (format 1, a list of rows; a row without its fields, or with an id that is not `a-z`, `0-9` and `-`, is left out and counted on the page), then filters the rows with `POST /api/filter`, the same filters as for your own games except the source. A list of hundreds of games costs one request: a record is fetched only when it is opened on the analysis board, saved or exported. The analysis board (`#/game/<game>/analysis/arena/<id>`) fetches the record's text and has the rules check it (`POST /api/view`), so a damaged file cannot show a game that breaks the rules; Export downloads that text as `<id>.json`; Save to history copies it to your games, where it is a game like any other (source filter "Arena samples").
- **Never changed from the studio.** There is no import and no delete in this set.
- **A missing `arena-games/index.json`** (404, or a host's fallback page) means no arena games are published yet, not an error; an unreachable site or a damaged index is an error that names the URL.
- **With `npm run dev` or `npm run preview`** there are no arena files, so the list is empty; to try the page, put an archive built by `studio-archive` in `web/public/arena-games/` (not committed).

### Games from GitHub runs

The SPRT and league workflows keep a sample of the games they play (about 10 per step or per pair of bots, [ADR 0027](../docs/adr/0027-game-records.md)). In the run's page on GitHub, download the artifact `game-records-<game>` (`league-game-records-<game>` for the league), unzip it, then in the studio open History, then Load game files, and choose the unzipped files. Artifacts expire after 30 days; the games of the releases are also kept as arena games (see above).

### Building an archive of arena games

```sh
cargo run --release -p studio --bin studio-archive -- --out DIR [--game GAME] INPUT...
```

The tool that builds the archive of a game's arena games ([ADR 0031](../docs/adr/0031-arena-games-archive.md)) from folders of records, such as unzipped workflow artifacts or an earlier archive. It reads every `.json` file in the `INPUT` folders and their subfolders (not `index.json`, names starting with a dot or symbolic links), replays each record with the game's referee as `POST /api/history` does, and keeps each game once, by its id (the first input wins). If any file fails, nothing is written, the exit status is 1, and stderr names every failing file and why. Otherwise it writes `DIR/records/<id>.json`, each as the history saves it, and `DIR/index.json`, `{"format":1,"rows":[...]}` with the rows of `GET /api/history`, newest first. `DIR` must not exist or must be empty. With `--game`, only records of that game are accepted. Run it with `--help` for the details.

## The rules in WebAssembly

The browser engine of the static site ([ADR 0030](../docs/adr/0030-studio-on-github-pages.md)) holds no rules: it calls a WebAssembly program built from the same Rust code as the server.

- [`core/`](core/) (`studio-core`): the logic of the server that needs the rules and no process, file or clock: the seats and the state of a game with its settling, answers, time limits and record, the checking of a session request, the checking of a game record, the rows and filters of the history, and the conversion of a think time to iterations. The server calls it, so the two cannot disagree; it also registers the games (`studio_core::games::registered`), the one place a new game is added. Time is passed in, not read.
- [`rules/`](rules/) (`studio-rules`): `studio_rules::handle(request) -> response`, and a binary that reads one request on stdin and prints one response and a newline on stdout, so that it runs as a WASI command for `wasm32-wasip1`. It keeps nothing between requests: a game in progress travels in each request as a `State`.

A request is a JSON object with a string field `op`; the response is `{"ok": value}` or `{"error": "message"}` (an unknown op, malformed JSON and bad fields give an error, never a crash). A `Seat` is `{"kind": "human", "name"}` or `{"kind": "bot", "release", "think_ms", "mode", "fixed_iters"}` (a bot seat in an output also has `name`, its release), an `Answer` is `{"seat", "lines", "ms"}`, a `Result` is `{"winner", "end"}` with the end as in a game record, a `State` is `{"game", "seed", "opening_plies", "seats", "turns", "result"}` and a `Row` is a line of `GET /api/history`. A record travels as text (`record_text`) and a bot seed as a decimal string, because JavaScript loses the precision of a 64-bit number. For the same reason a `State`'s `seed` must be a safe integer (at most 2^53 − 1, which the front end's seeds are), and the numbers of the `record` that `view` returns (its seed and bot seeds) may lose precision once parsed in JavaScript: they are for display, and the record text stays the reference.

| `op` | Request fields | `ok` |
| --- | --- | --- |
| `games` | none | `[{"id", "name"}]`, in the server's order |
| `new` | `request` (the body of `POST /api/sessions`), `releases` (the game's known releases) | a `State` with no turns and no result; the errors are those of `POST /api/sessions` |
| `settle` | `state` | `{"opening_turns", "valid_turns", "to_act", "frames", "human_moves", "result", "next", "bot_seeds"}`: the result if the rules say the game is over, `bot_seeds` (each seat's bot seed as a decimal string, `null` for a human; a restarted bot needs it before its turn), and `next`: `{"kind": "over"}`, `{"kind": "human", "seat"}`, `{"kind": "bot", "seat", "input", "answer_lines", "limit_ms", "bot_seed"}` or `{"kind": "unsupported", "error"}` |
| `play` | `state`, `answer` | `{"turns", "result"}`: the answer appended; one the rules reject is kept and loses the game for its seat |
| `replay` | `state`, `seat` | `[{"input", "answer_lines", "lines"}]`: what a restarted bot of that seat is asked, and must answer again, turn by turn |
| `record` | `state`, `unix_time` | `{"record_text"}`: the game record, as `GET /api/sessions/{id}/record` writes it |
| `check` | `record_text` | `{"id", "row"}`: the checks of `POST /api/history`, and the id the server would save it under |
| `view` | `record_text` | `{"record", "frames", "opening_turns", "shown_turns"}`, as `POST /api/view` |
| `filter` | `query`, `rows` | the rows `GET /api/history?query` would list, newest first |
| `filter_body` | `query`, `body` | the rows of the text `body`, a JSON array of rows, that pass the filter: the answer of `POST /api/filter?query`, with its refusals (the query first, then `invalid request body: ...`), shared with the server |

`scripts/build-studio-wasm.sh [OUT] [--strict]` builds `OUT/rules.wasm`, every release of every game as `OUT/<game>/<release>.wasm` (compiled unchanged with `rustc --target wasm32-wasip1`), and `OUT/manifest.json` listing the releases that compiled, newest first. `OUT` is `studio/web/public/wasm` by default (not committed). A release that does not compile is skipped with a warning; `--strict`, used by CI, fails instead. It needs the target: `rustup target add wasm32-wasip1`. To try the program without a browser: `echo '{"op":"games"}' | cargo run -q -p studio-rules`.

## Running the studio on Windows

The studio is meant for a Windows computer ([ADR 0025](../docs/adr/0025-studio.md)); the `Studio: Windows` job of CI builds it, tests it and plays a game through the server there. The commands are for PowerShell.

1. Install Rust with rustup from <https://rustup.rs>, accepting the default (MSVC) toolchain. It needs Microsoft's C++ build tools: accept its offer to install Visual Studio, or install the Visual Studio Build Tools with the workload "Desktop development with C++". Then open a new terminal.
2. Install Node 22 LTS from <https://nodejs.org>: the front page may offer a newer line, so pick version 22 on the downloads page.
3. Clone the repository (`git clone https://github.com/Arthur-Vat/codingame-bots.git`) or, if you already have it, run `git pull`.
4. Build the front end:

   ```powershell
   cd studio\web
   npm ci
   npm run build
   ```

5. Start the studio from the repository root, then open <http://127.0.0.1:8411/>:

   ```powershell
   cd ..\..
   cargo run --release -p studio
   ```

Where the data lives: in `studio\data` (saved games in `history\`, compiled bots in `bin\`, speed measurements in `speed.json`). It is never committed and nothing backs it up: export the games that matter from the app.

To update, run `git pull`, then steps 4 and 5 again.

Troubleshooting:

- `rustc` or `cargo` is not found: close the terminal and open a new one, which reads the `PATH` that rustup changed. The studio compiles each release with `rustc` from `PATH`.
- The port is in use: start the server on another one, `cargo run --release -p studio -- --port 8412`, and open that address.
- The first game against a release takes a few seconds: the studio compiles the release, then measures its speed once. Later games start at once.
