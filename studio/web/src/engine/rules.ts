/* The rules, from `wasm/rules.wasm`: the Rust program of `studio/rules` (see studio/README.md,
   "The rules in WebAssembly"). Each call instantiates the compiled module afresh, gives the
   request as the whole standard input, runs it, and reads the one line it printed. The program
   keeps nothing between calls: a game in progress travels in each request as a `State`.

   Records travel as text (`record_text`) and a bot seed as a decimal string, because a 64-bit
   number does not survive JSON.parse. Nothing here parses a record to write it again; an answer
   that holds a record's seeds (a view) is taken as text with `callText`, never through `call`. */

import type { EndReason, SeatInfo } from '../api';
import { runOnce } from './wasi';

/** The rules refused a request: the message is theirs. */
export class RulesError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'RulesError';
  }
}

/** One answer of one seat in a turn. */
export interface TurnAnswer {
  seat: number;
  lines: string[];
  ms: number;
}

/** The result of a finished game. */
export interface GameResult {
  winner: number | null;
  end: EndReason;
}

/** A game in progress, as the rules take it. */
export interface RulesState {
  game: string;
  seed: number;
  opening_plies: number;
  seats: [SeatInfo, SeatInfo];
  turns: TurnAnswer[][];
  result: GameResult | null;
}

/** What is asked of a bot, or of a human, next. */
export type Next =
  | { kind: 'over' }
  | { kind: 'human'; seat: number }
  | {
      kind: 'bot';
      seat: number;
      input: string;
      answer_lines: number;
      limit_ms: number;
      bot_seed: string;
    }
  | { kind: 'unsupported'; error: string };

export interface HumanMove {
  action: unknown;
  lines: string[];
}

export interface Settled {
  opening_turns: number;
  valid_turns: number;
  to_act: number[];
  frames: unknown[];
  human_moves: HumanMove[];
  result: GameResult | null;
  next: Next;
  /** Each seat's bot seed as decimal text, `null` for a human. */
  bot_seeds: [string | null, string | null];
}

/** A line of the history, as `GET /api/history` lists it. */
export interface HistoryRow {
  id: string;
  unix_time: number;
  [key: string]: unknown;
}

export interface ReplayStep {
  input: string;
  answer_lines: number;
  lines: string[];
}

/** How the rules program starts a successful answer. */
const OK_PREFIX = '{"ok":';

export type Request = { op: string } & Record<string, unknown>;

export interface Rules {
  /** Compiles the program (once); `call` needs it done. */
  ready: () => Promise<void>;
  /** Runs one request. Throws a `RulesError` with the rules' own message when they refuse it. */
  call: <T>(request: Request) => T;
  /**
   * `call`, giving the JSON text of the answer as the program wrote it, not parsed: for an answer
   * that holds a number beyond 2^53 (a seed), which parsing and writing again would round.
   */
  callText: (request: Request) => string;
}

/** The rules over a module that `load` provides, compiled on the first `ready`. */
export function createRules(load: () => Promise<WebAssembly.Module>): Rules {
  let compiled: Promise<WebAssembly.Module> | null = null;
  let module: WebAssembly.Module | null = null;
  const callText = (request: Request): string => {
    if (module === null) throw new Error('the rules are not loaded yet');
    const { code, stdout } = runOnce(module, JSON.stringify(request));
    // The program writes `{"ok":<answer>}` in one line, so the answer is what lies between.
    const line = stdout.trim();
    if (line.startsWith(OK_PREFIX) && line.endsWith('}')) return line.slice(OK_PREFIX.length, -1);
    let response: unknown;
    try {
      response = JSON.parse(stdout);
    } catch {
      throw new Error(`the rules program answered something that is not JSON (exit ${code})`);
    }
    if (typeof response === 'object' && response !== null) {
      if ('ok' in response) return JSON.stringify(response.ok);
      if ('error' in response && typeof response.error === 'string') {
        throw new RulesError(response.error);
      }
    }
    throw new Error('the rules program answered neither "ok" nor "error"');
  };
  return {
    async ready() {
      compiled ??= load();
      try {
        module = await compiled;
      } catch (error) {
        compiled = null;
        throw error;
      }
    },
    call<T>(request: Request): T {
      return JSON.parse(callText(request)) as T;
    },
    callText,
  };
}

/** The `CG_SEED` of each seat of a game (`null` for a human), as decimal text, never parsed as a number. */
export function botSeeds(rules: Rules, state: RulesState): [string | null, string | null] {
  return rules.call<Settled>({ op: 'settle', state }).bot_seeds;
}
