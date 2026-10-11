/* A bot in the browser: a release (WebAssembly) running in a Web Worker. The page writes the bot's
   input into a SharedArrayBuffer that the worker's blocking read waits on, and collects the lines
   the worker prints. A bot that is too slow is terminated; stderr is dropped. */

import { DATA_BYTES, HEADER_BYTES, type BotMessage, type BotStart } from './botProtocol';

/** Why a bot did not answer. */
export class BotError extends Error {
  readonly kind: 'timeout' | 'crash' | 'cancelled';

  constructor(kind: 'timeout' | 'crash' | 'cancelled', message: string) {
    super(message);
    this.name = 'BotError';
    this.kind = kind;
  }
}

/** A running bot. */
export interface Bot {
  /**
   * Sends `input`, then waits for `answerLines` lines of output within `limitMs`. `ms` is how long
   * that took. A bot that does not answer in time is terminated and the call fails with a
   * `BotError` of kind `timeout`; one that exits or fails, `crash`; one terminated meanwhile,
   * `cancelled`.
   */
  ask: (
    input: string,
    answerLines: number,
    limitMs: number,
  ) => Promise<{ lines: string[]; ms: number }>;
  /** Stops the bot. Safe to call twice. */
  terminate: () => void;
}

/** Starts a bot from a compiled release with the given environment. */
export type BotFactory = (module: WebAssembly.Module, env: Record<string, string>) => Promise<Bot>;

/** What the page needs for bots: shared memory, so a cross-origin isolated page. */
export function botsAvailable(): boolean {
  return (
    typeof SharedArrayBuffer !== 'undefined' &&
    typeof Worker !== 'undefined' &&
    globalThis.crossOriginIsolated === true
  );
}

export const BOTS_UNAVAILABLE =
  'This browser does not let bots run on this page (cross-origin isolation is unavailable)';

const sleep = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));

interface Pending {
  need: number;
  start: number;
  resolve: (answer: { lines: string[]; ms: number }) => void;
  reject: (error: BotError) => void;
  timer: ReturnType<typeof setTimeout>;
}

/** A bot over a worker; `worker` is already running the worker script. */
class WorkerBot implements Bot {
  private readonly control: Int32Array;
  private readonly data: Uint8Array;
  private readonly lines: string[] = [];
  private pending: Pending | null = null;
  private ended: BotError | null = null;
  private readonly worker: Worker;

  constructor(worker: Worker, sab: SharedArrayBuffer) {
    this.worker = worker;
    this.control = new Int32Array(sab, 0, 2);
    this.data = new Uint8Array(sab, HEADER_BYTES);
  }

  /** Called with each message of the worker after the first, `ready`. */
  onMessage(message: BotMessage): void {
    if ('line' in message) {
      this.lines.push(message.line);
      this.deliver();
    } else if ('exit' in message) {
      this.end(new BotError('crash', `exited with code ${message.exit}`));
    } else if ('error' in message) {
      this.end(new BotError('crash', message.error));
    }
  }

  onFailure(detail: string): void {
    this.end(new BotError('crash', detail));
  }

  /** The bot is gone: whoever waits for it learns why, and so does the next to ask. */
  private end(error: BotError): void {
    this.ended ??= error;
    this.worker.terminate();
    const pending = this.pending;
    if (pending !== null) {
      this.pending = null;
      clearTimeout(pending.timer);
      pending.reject(error);
    }
  }

  private deliver(): void {
    const pending = this.pending;
    if (pending === null || this.lines.length < pending.need) return;
    this.pending = null;
    clearTimeout(pending.timer);
    pending.resolve({
      lines: this.lines.splice(0, pending.need),
      ms: performance.now() - pending.start,
    });
  }

  async ask(
    input: string,
    answerLines: number,
    limitMs: number,
  ): Promise<{ lines: string[]; ms: number }> {
    if (this.ended !== null) throw this.ended;
    if (this.pending !== null) throw new Error('the bot is already being asked');
    const start = performance.now();
    const answer = new Promise<{ lines: string[]; ms: number }>((resolve, reject) => {
      const timer = setTimeout(() => {
        this.end(new BotError('timeout', 'the bot did not answer in time'));
      }, limitMs);
      this.pending = { need: answerLines, start, resolve, reject, timer };
    });
    // If writing fails the bot has ended, which rejected `answer` too: that is reported once, by
    // the throw below.
    answer.catch(() => {});
    await this.write(new TextEncoder().encode(input));
    this.deliver();
    return answer;
  }

  /** Writes `bytes` as the worker's input, piece by piece when it does not fit at once. */
  private async write(bytes: Uint8Array): Promise<void> {
    let offset = 0;
    while (offset < bytes.length) {
      // The worker empties the buffer as soon as it reads; wait for the previous piece.
      while (Atomics.load(this.control, 1) !== 0) {
        if (this.ended !== null) throw this.ended;
        await sleep(1);
      }
      if (this.ended !== null) throw this.ended;
      const size = Math.min(this.data.length, bytes.length - offset);
      this.data.set(bytes.subarray(offset, offset + size));
      Atomics.store(this.control, 1, size);
      Atomics.notify(this.control, 1);
      offset += size;
    }
  }

  terminate(): void {
    this.end(new BotError('cancelled', 'the bot was stopped'));
  }
}

/** Starts a bot in a module Worker and waits until the release is instantiated. */
export const workerBotFactory: BotFactory = (module, env) =>
  new Promise<Bot>((resolve, reject) => {
    const sab = new SharedArrayBuffer(HEADER_BYTES + DATA_BYTES);
    const worker = new Worker(new URL('./botWorker.ts', import.meta.url), { type: 'module' });
    const bot = new WorkerBot(worker, sab);
    let started = false;
    worker.onmessage = ({ data }: MessageEvent<BotMessage>) => {
      if (!started && 'ready' in data) {
        started = true;
        resolve(bot);
      } else {
        bot.onMessage(data);
      }
    };
    worker.onerror = (event) => {
      const detail = event.message === '' ? 'the worker failed' : event.message;
      if (started) {
        bot.onFailure(detail);
      } else {
        worker.terminate();
        reject(new Error(`cannot start the bot: ${detail}`));
      }
    };
    const message: BotStart = { module, sab, env };
    worker.postMessage(message);
  });
