# studio

A local app in the owner's browser, in the style of Lichess, to choose a game, play it against a friend or a release, watch two releases play each other, and review games with a bot's evaluation and best moves. It is for the owner only and is never hosted.

The decisions behind it are [ADR 0025](../docs/adr/0025-studio.md) (the studio), [ADR 0026](../docs/adr/0026-node-for-the-studio.md) (Node and npm for the front end) and [ADR 0027](../docs/adr/0027-game-records.md) (game records). The plan is phase 6 of the [roadmap](../docs/ROADMAP.md).

## Status

The front end's scaffold exists, in [`web/`](web/), and so does the app shell: the top bar, the home screen (one card per game from the server), the game page's layout and the three setup dialogs with their advanced mode. Games are playable: Ultimate Tic-Tac-Toe's board is drawn from the server's frames, and the three modes work against the server: a friend on the same screen, a release with takebacks, and two releases playing each other. Human clocks (minutes plus increment) run in the front end, which ends the game on a timeout; a bot shows the time it has used. The move list and the keys ← → Home End look back at earlier positions, and input is accepted on the latest one only. A game that ends shows its result, with Rematch, New game, Save and Export file. The interface a game implements for the studio is in [`game/`](game/), with Ultimate Tic-Tac-Toe's adapter, and the local server in [`server/`](server/): see "Running the server" below. The review screen (playback speeds, analysis) and the history page's list are not there yet.

## Front end

TypeScript, React and Vite, in `studio/web/`. It needs Node 22 (the line is written in the `engines` field of `web/package.json`, which CI follows) and npm. Run everything from `studio/web/`:

```sh
npm ci                  # install the locked dependencies
npm run dev             # the app, with reload, at http://localhost:5173 (calls to /api go to the server on port 8411)
npm run build           # type check, then the production build in dist/
npm run preview         # serve the production build
```

The checks, all run by the `Studio front end` job of CI:

```sh
npm run licenses        # every package of the lockfile has an allowed license (licenses.json)
npm run format:check    # Prettier (npm run format fixes)
npm run lint            # ESLint
npm run typecheck       # TypeScript
npm test                # unit tests (Vitest)
npm run e2e             # browser tests (Playwright, Chromium); builds and serves the app itself
```

To work on the app against the real server, start the server in one terminal and the dev server in another:

```sh
cargo run --release -p studio     # the API, at http://127.0.0.1:8411
npm run dev                       # in studio/web: the app, proxying /api to the server
```

The browser tests do not need the server: they mock the API.

The first `npm run e2e` on a computer needs Chromium: `npx playwright install chromium`.

## Dependencies

Few, each one a choice (ADR 0026). Versions are exact, and every version in the lockfile was published at least two weeks before it was added (`npm install --before=<date>` finds such versions). Only permissive licenses are allowed, listed in `web/licenses.json`; `web/scripts/check-licenses.mjs` refuses a package without a license, or with one that is not on the list.

The lockfile is made with npm 11, because npm 10 (the one shipped with Node 22) fails on the peer dependencies of Vitest and rewrites the lockfile on `npm install`; `npm ci` works with either. To change a dependency, run from `studio/web/`:

```sh
npx npm@11 install --save-exact <package>@<exact version> --before=<date two weeks ago, as YYYY-MM-DD>
```

Then commit `package.json` and `package-lock.json`, and run the checks above.

## How a game plugs in

A game adds an adapter crate, `games/<game>/studio`, that implements `studio_game::StudioGame` ([game/](game/)) and holds no rules of its own: positions come from replaying the game's referee, and a human's legal moves from the referee's turn input.

- `info`: the game's id (its folder name) and display name.
- `new_referee`: a fresh referee for a setup (seed and opening plies), the one the arena uses.
- `opening_turns`: the leading turns the referee imposes (its opening plies); the server plays them itself.
- `frames`: the JSON the board renderer draws: frame 0 is the position after the opening, then one frame per step; frame `i` of a turn-based game shows the turns up to `opening_turns + i`.
- `human_moves`: the moves a human in a seat may play, each with the answer lines to send the referee.

`studio_game::live_game` replays turns with a fresh referee into a `cg_arena::live::LiveGame`. The adapter's frame format is documented in its crate; `games/uttt/studio` is the first.

A game's board renderer lives in `web/src/games/<game>/` and is registered in `web/src/games/registry.ts`, which maps the game's id to a React component that draws one frame and reports a chosen action (`onAction`, the `action` of one of the session's `human_moves`), and may add a function that words how a finished game ended (`resultDetails`).

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
| `GET /api/history` | The saved games, newest first, as `{id, game, unix_time, players: [name, name], winner: 0 \| 1 \| null, end, turns, source}` (`end` is `finished`, `timeout`, `crash`, `invalid`, `aborted` or `resigned`). Optional filters: `game`, `release` (either player's name), `result` (`x`: seat 0 won, `o`: seat 1 won, `draw`: a finished game without a winner, `fault`: a timeout, crash or invalid answer, `unfinished`: an aborted game), `source` (`studio` or `arena`), `from` and `to` (`YYYY-MM-DD`, UTC days, both included). Values are percent-decoded (`%XX`, and `+` for a space). Files that cannot be read are skipped and named on the server's stderr. |
| `GET /api/history/{id}` | The saved record, as the front end offers it for export. Ids are file names without `.json`, made of `a-z`, `0-9` and `-`. |
| `GET /api/history/{id}/view` | `{"record", "frames", "opening_turns", "shown_turns"}`: the record with the game's frames. `shown_turns` is the number of turns that replay: all, or all but a final invalid one. |
| `POST /api/view` | The same view for a record in the body that is not saved, to review a loaded file before saving it. |
| `DELETE /api/history/{id}` | Deletes a saved game: 204, or 404. |
| `DELETE /api/sessions/{id}` | Stops the bots and forgets the session. Sessions untouched for two hours are dropped. |

The two modes of a computer seat:

- `fixed`: a fixed number of iterations, `think_ms` times the release's measured speed, and the bot seed `cg_arena::runner::bot_seed`; this is what takebacks replay.
- `realtime`: the release's own time budget scaled by `think_ms / 100` (`CG_TIME_SCALE`), with the game's limits scaled likewise plus 50 ms; for bots playing each other. A bot that times out, crashes or answers invalidly loses.

### Saved games

The history is one JSON file per game in `studio/data/history/` (or `<data>/history/`), named `<game>-<unix time>-<12 hex digits of a hash of the record>.json`. The folder is never committed ([ADR 0027](../docs/adr/0027-game-records.md)) and GitHub does not back it up: keep what matters by exporting it (`GET /api/history/{id}` gives the file). Loading a file adds it to the history through `POST /api/history`. The listing reads every file each time, which is fine for a few thousand games.

### Games from GitHub runs

The SPRT and league workflows keep a sample of the games they play (about 10 per step or per pair of bots, [ADR 0027](../docs/adr/0027-game-records.md)). In the run's page on GitHub, download the artifact `game-records` (`league-game-records` for the league), unzip it, then in the studio open History, then Load game files, and choose the unzipped files. Artifacts expire after 30 days.

## Running the studio on Windows

The studio is meant for a Windows computer ([ADR 0025](../docs/adr/0025-studio.md)); the `Studio on Windows` job of CI builds it, tests it and plays a game through the server there. The commands are for PowerShell.

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
