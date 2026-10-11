/* The browser engine, wired to the page: the wasm files next to the document, bots in module
   Workers, saved games in IndexedDB, speeds in localStorage. Loaded only when the front end uses
   this engine. */

import { botsAvailable, workerBotFactory } from './bot';
import type { Reply } from './http';
import { createEngine, type Engine } from './routes';
import { createRules } from './rules';
import { Speed, localStorageSpeedCache } from './speed';
import { openGameStore } from './storage';
import { fetchedWasm } from './wasm';

/** A short random id in lowercase hex, like the server's. */
function randomId(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(4));
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, '0')).join('');
}

let engine: Engine | null = null;

function theEngine(): Engine {
  if (engine === null) {
    // Relative to the document, so that the site can live under a subpath.
    const wasm = fetchedWasm(new URL('wasm/', document.baseURI).href);
    engine = createEngine({
      rules: createRules(() => wasm.rules()),
      wasm,
      speed: new Speed(localStorageSpeedCache()),
      makeBot: botsAvailable() ? workerBotFactory : null,
      now: () => Date.now(),
      newId: randomId,
      store: () => openGameStore(),
    });
  }
  return engine;
}

export const browserEngine = {
  handle(method: string, path: string, bodyText: string | undefined): Promise<Reply> {
    return theEngine().handle(method, path, bodyText);
  },
};
