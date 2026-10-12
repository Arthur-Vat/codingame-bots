// A fixture archive of arena games for the browser tests: the published files
// (`arena-games/index.json` and `arena-games/records/<id>.json`, ADR 0031), served by intercepting
// the requests. The browser engine reads them from its own site, the server's engine from the
// published site on github.io: both are matched.

import type { BrowserContext, Route } from '@playwright/test';

/** A row of the index, the shape of `GET /api/history`. */
export interface ArenaRow {
  id: string;
  game: string;
  unix_time: number;
  players: [string, string];
  winner: number | null;
  end: string;
  turns: number;
  source: string;
}

export const arenaRow = (patch: Partial<ArenaRow> = {}): ArenaRow => ({
  id: 'uttt-1760000000-aaaaaaaaaaaa',
  game: 'uttt',
  unix_time: 1_760_000_000,
  players: ['uttt-v010', 'uttt-v009'],
  winner: 0,
  end: 'finished',
  turns: 41,
  source: 'arena match',
  ...patch,
});

/** Both places the arena games can be read from. */
const ARENA_FILES =
  /^(https:\/\/arthur-vat\.github\.io\/codingame-bots|http:\/\/127\.0\.0\.1:4173)\/arena-games\/(index\.json|records\/[^/?#]+\.json)(\?.*)?$/;

export interface ArenaServer {
  /** The URLs asked for, in order. */
  requests: string[];
  /** What `index.json` answers: an object, raw text, or null for a 404. */
  index: unknown;
  /** The record files by id, as text; a missing one is a 404. */
  records: Record<string, string>;
  /** When set, the index waits for it. */
  gate: Promise<void> | null;
}

/** Serves the fixture archive to every page of the context. GitHub Pages sends the CORS header. */
export async function serveArena(
  context: BrowserContext,
  files: { index: unknown; records?: Record<string, string> },
): Promise<ArenaServer> {
  const server: ArenaServer = {
    requests: [],
    index: files.index,
    records: files.records ?? {},
    gate: null,
  };
  await context.route(ARENA_FILES, async (route: Route) => {
    const url = route.request().url();
    server.requests.push(url);
    const cors = { 'Access-Control-Allow-Origin': '*' };
    const match = ARENA_FILES.exec(url);
    const file = match?.[2] ?? '';
    if (file === 'index.json') {
      await server.gate;
      if (server.index === null) {
        await route.fulfill({ status: 404, headers: cors, body: 'Not Found' });
      } else {
        await route.fulfill({
          headers: cors,
          contentType: 'application/json',
          body: typeof server.index === 'string' ? server.index : JSON.stringify(server.index),
        });
      }
      return;
    }
    const id = file.replace(/^records\//, '').replace(/\.json$/, '');
    const text = server.records[id];
    if (text === undefined) {
      await route.fulfill({ status: 404, headers: cors, body: 'Not Found' });
    } else {
      await route.fulfill({ headers: cors, contentType: 'application/json', body: text });
    }
  });
  return server;
}
