/// <reference types="node" />
/* Helpers of the browser engine's unit tests: the real WebAssembly files, built by
   scripts/build-studio-wasm.sh into public/wasm/, and fakes for what the page provides. */

import { existsSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import type { Bot, BotFactory } from './bot';
import { createEngine, type Engine } from './routes';
import { createRules, type Rules } from './rules';
import { Speed, memorySpeedCache, type MeasureOptions } from './speed';
import { memoryGameStore, type GameStore } from './storage';
import type { LoadedRelease, WasmSource } from './wasm';

const WASM_DIR = fileURLToPath(new URL('../../public/wasm/', import.meta.url));

/** The bytes of a built file, or a failure that says how to build it: tests never skip. */
export function wasmBytes(path: string): Buffer {
  const file = `${WASM_DIR}${path}`;
  if (!existsSync(file)) {
    throw new Error(
      `${file} is missing: the browser engine's tests need the WebAssembly files. ` +
        'Run scripts/build-studio-wasm.sh (npm run wasm in studio/web) first.',
    );
  }
  return readFileSync(file);
}

export async function rulesModule(): Promise<WebAssembly.Module> {
  return WebAssembly.compile(new Uint8Array(wasmBytes('rules.wasm')));
}

/** The real rules program, ready to call. */
export async function loadedRules(): Promise<Rules> {
  const rules = createRules(rulesModule);
  await rules.ready();
  return rules;
}

/** The files, with a manifest of the given releases (the real release files are not read). */
export function fakeWasm(releases: string[], broken: string[] = []): WasmSource {
  const rules = rulesModule();
  return {
    manifest: async () => ({ games: [{ id: 'uttt', releases }] }),
    rules: () => rules,
    release: async (game, release): Promise<LoadedRelease> => {
      if (!releases.includes(release) || broken.includes(release)) {
        throw new Error(`cannot load wasm/${game}/${release}.wasm (HTTP 404)`);
      }
      return { module: new WebAssembly.Module(EMPTY_MODULE), hash: `hash-${release}` };
    },
  };
}

/** The smallest valid module: 8 bytes. */
export const EMPTY_MODULE = new Uint8Array([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]);

/** What a scripted bot does when asked. */
export type Script = (input: string, answerLines: number, limitMs: number) => Promise<string[]>;

/** A bot that answers as `script` says; `script` rejects to make the bot fail. */
export function scriptedBot(script: Script): Bot {
  let stopped = false;
  return {
    ask: async (input, answerLines, limitMs) => {
      const start = performance.now();
      const lines = await script(input, answerLines, limitMs);
      if (stopped) throw new Error('stopped');
      return { lines, ms: performance.now() - start };
    },
    terminate: () => {
      stopped = true;
    },
  };
}

/** The first of the legal moves Ultimate Tic-Tac-Toe's bot input lists: its opponent's last move, a count, then the moves. */
export function firstLegalMove(input: string): string {
  const lines = input.split('\n');
  const move = lines[2];
  if (move === undefined || move === '') throw new Error('the input lists no legal move');
  return move;
}

/** A factory of scripted bots; the environment each was started with is kept. */
export function scriptedFactory(scripts: Script[]): {
  factory: BotFactory;
  envs: Record<string, string>[];
  bots: Bot[];
} {
  const envs: Record<string, string>[] = [];
  const bots: Bot[] = [];
  const factory: BotFactory = async (_module, env) => {
    const script = scripts[envs.length];
    if (script === undefined) throw new Error('no script for this bot');
    envs.push(env);
    const bot = scriptedBot(script);
    bots.push(bot);
    return bot;
  };
  return { factory, envs, bots };
}

export interface TestEngine {
  engine: Engine;
  store: GameStore | null | (() => Promise<GameStore>);
  clock: { now: number };
}

/** An engine over the real rules, in-memory saved games, and the given bots (none: no isolation). */
export async function testEngine(
  options: {
    makeBot?: BotFactory | null;
    releases?: string[];
    /** Releases whose file cannot be loaded. */
    broken?: string[];
    /** The store, `null` for none, or what opens it. */
    store?: GameStore | null | (() => Promise<GameStore>);
    measure?: (options: MeasureOptions) => Promise<number>;
  } = {},
): Promise<TestEngine> {
  const rules = await loadedRules();
  const store = options.store === undefined ? memoryGameStore() : options.store;
  const clock = { now: 1_760_000_000_000 };
  let counter = 0;
  const engine = createEngine({
    rules,
    wasm: fakeWasm(options.releases ?? ['uttt-v002', 'uttt-v001'], options.broken),
    speed: new Speed(memorySpeedCache(), options.measure ?? (async () => 2)),
    makeBot: options.makeBot ?? null,
    now: () => clock.now,
    newId: () => `s${(counter++).toString(16)}`,
    store:
      store === null
        ? () => Promise.reject(new Error('no IndexedDB'))
        : typeof store === 'function'
          ? store
          : async () => store,
  });
  return { engine, store, clock };
}

/** `call` on the engine and the parsed body. */
export async function request(
  engine: Engine,
  method: string,
  path: string,
  body?: unknown,
): Promise<{ status: number; body: unknown; text: string }> {
  const reply = await engine.handle(
    method,
    path,
    body === undefined ? undefined : typeof body === 'string' ? body : JSON.stringify(body),
  );
  const text = reply.bodyText;
  return { status: reply.status, body: text === '' ? undefined : JSON.parse(text), text };
}

/** Polls `read` until `done` holds; the tests wait on a condition, never on a fixed time. */
export async function until<T>(read: () => Promise<T>, done: (value: T) => boolean): Promise<T> {
  const deadline = Date.now() + 20_000;
  for (;;) {
    const value = await read();
    if (done(value)) return value;
    if (Date.now() > deadline) throw new Error('the condition did not hold in time');
    await new Promise((resolve) => setTimeout(resolve, 5));
  }
}
