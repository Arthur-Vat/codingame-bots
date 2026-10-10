# studio

A local app in the owner's browser, in the style of Lichess, to choose a game, play it against a friend or a release, watch two releases play each other, and review games with a bot's evaluation and best moves. It is for the owner only and is never hosted.

The decisions behind it are [ADR 0025](../docs/adr/0025-studio.md) (the studio), [ADR 0026](../docs/adr/0026-node-for-the-studio.md) (Node and npm for the front end) and [ADR 0027](../docs/adr/0027-game-records.md) (game records). The plan is phase 6 of the [roadmap](../docs/ROADMAP.md).

## Status

Only the front end's scaffold exists, in [`web/`](web/): a placeholder home screen with the checks and the CI job. There is no server, no game and no board yet.

## Front end

TypeScript, React and Vite, in `studio/web/`. It needs Node 22 (the line is written in the `engines` field of `web/package.json`, which CI follows) and npm. Run everything from `studio/web/`:

```sh
npm ci                  # install the locked dependencies
npm run dev             # the app, with reload, at http://localhost:5173
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

The first `npm run e2e` on a computer needs Chromium: `npx playwright install chromium`.

## Dependencies

Few, each one a choice (ADR 0026). Versions are exact, and every version in the lockfile was published at least two weeks before it was added (`npm install --before=<date>` finds such versions). Only permissive licenses are allowed, listed in `web/licenses.json`; `web/scripts/check-licenses.mjs` refuses a package without a license, or with one that is not on the list.

The lockfile is made with npm 11, because npm 10 (the one shipped with Node 22) fails on the peer dependencies of Vitest and rewrites the lockfile on `npm install`; `npm ci` works with either. To change a dependency, run from `studio/web/`:

```sh
npx npm@11 install --save-exact <package>@<exact version> --before=<date two weeks ago, as YYYY-MM-DD>
```

Then commit `package.json` and `package-lock.json`, and run the checks above.
