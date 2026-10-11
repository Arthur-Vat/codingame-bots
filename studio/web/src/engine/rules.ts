/* The rules, from `wasm/rules.wasm`: the Rust program of `studio/rules` (see studio/README.md,
   "The rules in WebAssembly"). Each call instantiates the compiled module afresh, gives the
   request as the whole standard input, runs it, and reads the one line it printed. The program
   keeps nothing between calls: a game in progress travels in each request as a `State`.

   Records travel as text (`record_text`) and a bot seed as a decimal string, because a 64-bit
   number does not survive JSON.parse. Nothing here parses a record to write it again. */

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

export type Request = { op: string } & Record<string, unknown>;

export interface Rules {
  /** Compiles the program (once); `call` needs it done. */
  ready: () => Promise<void>;
  /** Runs one request. Throws a `RulesError` with the rules' own message when they refuse it. */
  call: <T>(request: Request) => T;
}

/** The rules over a module that `load` provides, compiled on the first `ready`. */
export function createRules(load: () => Promise<WebAssembly.Module>): Rules {
  let compiled: Promise<WebAssembly.Module> | null = null;
  let module: WebAssembly.Module | null = null;
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
      if (module === null) throw new Error('the rules are not loaded yet');
      const { code, stdout } = runOnce(module, JSON.stringify(request));
      let response: unknown;
      try {
        response = JSON.parse(stdout);
      } catch {
        throw new Error(`the rules program answered something that is not JSON (exit ${code})`);
      }
      if (typeof response === 'object' && response !== null) {
        if ('ok' in response) return response.ok as T;
        if ('error' in response && typeof response.error === 'string') {
          throw new RulesError(response.error);
        }
      }
      throw new Error('the rules program answered neither "ok" nor "error"');
    },
  };
}

/**
 * The `CG_SEED` of each seat of a game (`null` for a human), as decimal text. The rules give a bot's
 * seed with the turn it is asked for; a restarted bot needs it before, so it is read from the
 * game's record, whose `bot_seed` keys are its players' in seat order and appear nowhere else
 * (every string in the record escapes its quotes). The digits are taken from the text, never
 * parsed as a number.
 */
export function botSeeds(rules: Rules, state: RulesState): [string | null, string | null] {
  const { record_text } = rules.call<{ record_text: string }>({
    op: 'record',
    state,
    unix_time: 0,
  });
  const found = [...record_text.matchAll(/"bot_seed":(null|\d+)/g)].map((match) => match[1]);
  const seed = (index: number): string | null => {
    const digits = found[index];
    return digits === undefined || digits === 'null' ? null : digits;
  };
  if (found.length !== 2) throw new Error('the record does not hold the two bot seeds');
  return [seed(0), seed(1)];
}
