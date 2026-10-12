/* The browser engine's API: the routes, JSON shapes, status codes and error bodies of the local
   server (studio/README.md, "API"), answered in the browser. `handle` takes what a `fetch` would
   send and gives what it would receive.

   Records travel as text: a record is stored and given back exactly as it came, and only the
   rules program reads inside it. */

import { HttpError, errorReply, json, messageOf, type Reply } from './http';
import { RulesError, type HistoryRow } from './rules';
import { STORE_UNAVAILABLE, type GameStore } from './storage';
import { Sessions, type SessionDeps } from './sessions';
import { MISSING_FILES } from './wasm';

export interface EngineDeps extends SessionDeps {
  /** Opens the saved games' store; rejects where the browser has none. */
  store: () => Promise<GameStore>;
}

export interface Engine {
  handle: (method: string, path: string, bodyText: string | undefined) => Promise<Reply>;
  sessions: Sessions;
}

/** Whether `id` can be a saved game's id: `a-z`, `0-9` and `-`, as on the server. */
export function validHistoryId(id: string): boolean {
  return /^[a-z0-9-]+$/.test(id);
}

/** A body that is JSON, or the 400 saying it is not. */
export function parseBody(bodyText: string | undefined): unknown {
  try {
    return JSON.parse(bodyText ?? '');
  } catch (error) {
    throw new HttpError(400, `invalid request body: ${messageOf(error)}`);
  }
}

function field(body: unknown, name: string): unknown {
  return typeof body === 'object' && body !== null
    ? (body as Record<string, unknown>)[name]
    : undefined;
}

function index(body: unknown, name: string): number {
  const value = field(body, name);
  if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) {
    throw new HttpError(400, `invalid request body: ${name} must be a non-negative integer`);
  }
  return value;
}

export function createEngine(deps: EngineDeps): Engine {
  const { rules } = deps;
  const sessions = new Sessions(deps);
  let opened: Promise<GameStore> | null = null;

  const store = async (): Promise<GameStore> => {
    opened ??= deps.store();
    try {
      return await opened;
    } catch {
      opened = null;
      throw new HttpError(409, STORE_UNAVAILABLE);
    }
  };

  /** The rules' answer, with their refusal as a 400. */
  const ask = <T>(request: { op: string } & Record<string, unknown>): T => {
    try {
      return rules.call<T>(request);
    } catch (error) {
      if (error instanceof RulesError) throw new HttpError(400, error.message);
      throw error;
    }
  };

  const listGames = async (): Promise<Reply> => {
    const manifest = await deps.wasm.manifest();
    const games = rules.call<{ id: string; name: string }[]>({ op: 'games' });
    return json(
      200,
      games.map((game) => ({
        ...game,
        releases: manifest.games.find((entry) => entry.id === game.id)?.releases ?? [],
      })),
    );
  };

  /** Saves a record given as text. */
  const save = async (recordText: string): Promise<Reply> => {
    const { id, row } = ask<{ id: string; row: HistoryRow }>({
      op: 'check',
      record_text: recordText,
    });
    const games = await store();
    if ((await games.get(id)) !== null) return json(200, { id, duplicate: true });
    await games.put({ id, record_text: recordText, row });
    return json(200, { id, duplicate: false });
  };

  const stored = async (id: string): Promise<string> => {
    if (!validHistoryId(id)) throw new HttpError(400, 'invalid game id');
    const found = await (await store()).get(id);
    if (found === null) throw new HttpError(404, 'unknown game');
    return found.record_text;
  };

  const route = async (
    method: string,
    path: string,
    query: string,
    bodyText: string | undefined,
  ): Promise<Reply> => {
    const segments = path.split('/');
    const key = `${method} ${segments.length}`;
    const [first, id, action] = segments;
    switch (key) {
      case 'GET 1':
        if (first === 'games') return listGames();
        if (first === 'history') {
          const rows = (await (await store()).all()).map((game) => game.row);
          return json(200, ask({ op: 'filter', query, rows }));
        }
        break;
      case 'POST 1':
        if (first === 'sessions') return json(200, await sessions.create(parseBody(bodyText)));
        if (first === 'history') return save(bodyText ?? '');
        if (first === 'view') return json(200, ask({ op: 'view', record_text: bodyText ?? '' }));
        if (first === 'filter') {
          // The rows come in the body (they are not the saved games): the same filter, so the
          // two engines list the arena games alike.
          return json(200, ask({ op: 'filter', query, rows: parseBody(bodyText) }));
        }
        break;
      case 'GET 2':
        if (first === 'sessions' && id !== undefined) return json(200, sessions.view(id));
        if (first === 'history' && id !== undefined)
          return { status: 200, bodyText: await stored(id) };
        break;
      case 'DELETE 2':
        if (first === 'sessions' && id !== undefined) {
          sessions.remove(id);
          return json(200, { deleted: true });
        }
        if (first === 'history' && id !== undefined) {
          if (!validHistoryId(id)) throw new HttpError(400, 'invalid game id');
          if (!(await (await store()).delete(id))) throw new HttpError(404, 'unknown game');
          return { status: 204, bodyText: '' };
        }
        break;
      case 'POST 3':
        if (first === 'sessions' && id !== undefined) {
          if (action === 'move') {
            const body = parseBody(bodyText);
            return json(200, sessions.move(id, index(body, 'seat'), index(body, 'index')));
          }
          if (action === 'takeback') {
            return json(200, sessions.takeback(id, index(parseBody(bodyText), 'turns')));
          }
          if (action === 'end') {
            const body = parseBody(bodyText);
            const reason = field(body, 'reason');
            return json(
              200,
              sessions.end(id, index(body, 'seat'), typeof reason === 'string' ? reason : ''),
            );
          }
          if (action === 'save') return save(sessions.record(id));
        }
        break;
      case 'GET 3':
        if (first === 'sessions' && id !== undefined && action === 'record') {
          return { status: 200, bodyText: sessions.record(id) };
        }
        if (first === 'history' && id !== undefined && action === 'view') {
          return json(200, ask({ op: 'view', record_text: await stored(id) }));
        }
        break;
    }
    return known(segments)
      ? errorReply(405, 'method not allowed')
      : errorReply(404, 'unknown route');
  };

  return {
    sessions,
    async handle(method, fullPath, bodyText) {
      try {
        const [path = '', query = ''] = splitOnce(fullPath, '?');
        if (!path.startsWith('/api/')) return errorReply(404, 'unknown route');
        try {
          await rules.ready();
        } catch (error) {
          throw new HttpError(
            500,
            `cannot load the rules program: ${messageOf(error)} (${MISSING_FILES})`,
          );
        }
        return await route(
          method === 'HEAD' ? 'GET' : method,
          path.slice('/api/'.length),
          query,
          bodyText,
        );
      } catch (error) {
        if (error instanceof HttpError) return errorReply(error.status, error.message);
        return errorReply(500, messageOf(error));
      }
    },
  };
}

function splitOnce(text: string, separator: string): [string, string] {
  const at = text.indexOf(separator);
  return at < 0 ? [text, ''] : [text.slice(0, at), text.slice(at + separator.length)];
}

/** Whether the path is a route of the API, whatever the method. */
function known(segments: string[]): boolean {
  const [first, , action] = segments;
  switch (segments.length) {
    case 1:
      return (
        first === 'games' ||
        first === 'history' ||
        first === 'view' ||
        first === 'filter' ||
        first === 'sessions'
      );
    case 2:
      return first === 'sessions' || first === 'history';
    case 3:
      return (
        (first === 'sessions' &&
          ['move', 'takeback', 'end', 'record', 'save'].includes(action ?? '')) ||
        (first === 'history' && action === 'view')
      );
    default:
      return false;
  }
}
