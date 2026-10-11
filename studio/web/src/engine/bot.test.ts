/* The page's side of a bot in a worker (bot.ts), over a fake Worker that reads the shared buffer
   the way botWorker.ts does and answers as a script says. The real worker and real releases run in
   the browser tests. */

import { afterEach, describe, expect, it, vi } from 'vitest';
import { BotError, workerBotFactory } from './bot';
import { HEADER_BYTES, type BotMessage, type BotStart } from './botProtocol';
import { EMPTY_MODULE } from './testSupport';

/** What the fake bot does with each piece of input it reads: lines to print, or an exit. */
type Behaviour = (input: string, say: (message: BotMessage) => void) => void;

let behaviour: Behaviour = () => {};
let failToStart: string | null = null;
let workers: FakeWorker[] = [];

class FakeWorker {
  onmessage: ((event: MessageEvent<BotMessage>) => void) | null = null;
  onerror: ((event: { message: string }) => void) | null = null;
  terminated = false;
  started: BotStart | null = null;
  private timer: ReturnType<typeof setInterval> | null = null;

  constructor() {
    workers.push(this);
  }

  say = (message: BotMessage): void => {
    if (!this.terminated) this.onmessage?.({ data: message } as MessageEvent<BotMessage>);
  };

  postMessage(start: BotStart): void {
    this.started = start;
    queueMicrotask(() => {
      if (failToStart !== null) {
        this.onerror?.({ message: failToStart });
        return;
      }
      this.say({ ready: true });
      const control = new Int32Array(start.sab, 0, 2);
      const data = new Uint8Array(start.sab, HEADER_BYTES);
      let input = '';
      const decoder = new TextDecoder();
      // The blocking read of the real worker, as a poll.
      this.timer = setInterval(() => {
        const size = Atomics.load(control, 1);
        if (size === 0) return;
        input += decoder.decode(data.slice(0, size), { stream: true });
        Atomics.store(control, 1, 0);
        Atomics.notify(control, 1);
        if (input.endsWith('\n')) {
          const whole = input;
          input = '';
          behaviour(whole, this.say);
        }
      }, 1);
    });
  }

  terminate(): void {
    this.terminated = true;
    if (this.timer !== null) clearInterval(this.timer);
  }
}

afterEach(() => {
  vi.unstubAllGlobals();
  for (const worker of workers) worker.terminate();
  workers = [];
  behaviour = () => {};
  failToStart = null;
});

function stubWorker(): void {
  vi.stubGlobal('Worker', FakeWorker);
}

const module = () => new WebAssembly.Module(EMPTY_MODULE);

describe('a bot in a worker', () => {
  it('is started with the module, the shared buffer and the environment, once ready', async () => {
    stubWorker();
    await workerBotFactory(module(), { CG_SEED: '7', CG_FIXED_ITERS: '300' });
    expect(workers).toHaveLength(1);
    expect(workers[0]?.started?.env).toEqual({ CG_SEED: '7', CG_FIXED_ITERS: '300' });
    expect(workers[0]?.started?.sab).toBeInstanceOf(SharedArrayBuffer);
    expect(workers[0]?.started?.module).toBeInstanceOf(WebAssembly.Module);
  });

  it('gets its input and answers with the lines it prints, and how long that took', async () => {
    stubWorker();
    behaviour = (input, say) => say({ line: `echo ${input.trim()}` });
    const bot = await workerBotFactory(module(), {});
    const answer = await bot.ask('hello\n', 1, 5000);
    expect(answer.lines).toEqual(['echo hello']);
    expect(answer.ms).toBeGreaterThanOrEqual(0);
    // And again: the same bot answers several times.
    expect((await bot.ask('again\n', 1, 5000)).lines).toEqual(['echo again']);
  });

  it('collects as many lines as are asked for, and keeps the extra for the next answer', async () => {
    stubWorker();
    behaviour = (_input, say) => {
      say({ line: 'a' });
      say({ line: 'b' });
      say({ line: 'c' });
    };
    const bot = await workerBotFactory(module(), {});
    expect((await bot.ask('x\n', 2, 5000)).lines).toEqual(['a', 'b']);
    // 'c' was already there, and one more line comes with the next input.
    expect((await bot.ask('y\n', 2, 5000)).lines).toEqual(['c', 'a']);
  });

  it('writes an input longer than the buffer in pieces, whole and in order', async () => {
    stubWorker();
    let received = '';
    behaviour = (input, say) => {
      received = input;
      say({ line: 'done' });
    };
    const bot = await workerBotFactory(module(), {});
    const long = `${'0123456789'.repeat(20_000)}\n`;
    expect((await bot.ask(long, 1, 20_000)).lines).toEqual(['done']);
    expect(received).toBe(long);
  });

  it('is terminated when it does not answer in time, and says so', async () => {
    stubWorker();
    behaviour = () => {};
    const bot = await workerBotFactory(module(), {});
    const failure = await bot.ask('x\n', 1, 30).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(BotError);
    expect((failure as BotError).kind).toBe('timeout');
    expect(workers[0]?.terminated).toBe(true);
    // Asking a bot that is gone fails at once, with the same reason.
    await expect(bot.ask('y\n', 1, 1000)).rejects.toMatchObject({ kind: 'timeout' });
  });

  it('crashes when it exits before it answers, even with no output at all', async () => {
    stubWorker();
    behaviour = (_input, say) => say({ exit: 0 });
    const bot = await workerBotFactory(module(), {});
    const failure = await bot.ask('x\n', 1, 5000).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(BotError);
    expect(failure).toMatchObject({ kind: 'crash', message: 'exited with code 0' });
    expect(workers[0]?.terminated).toBe(true);
  });

  it('crashes with the message of a trap', async () => {
    stubWorker();
    behaviour = (_input, say) => say({ error: 'unreachable' });
    const bot = await workerBotFactory(module(), {});
    await expect(bot.ask('x\n', 1, 5000)).rejects.toMatchObject({
      kind: 'crash',
      message: 'unreachable',
    });
  });

  it('answers if it prints its lines and then exits', async () => {
    stubWorker();
    behaviour = (_input, say) => {
      say({ line: 'last' });
      say({ exit: 0 });
    };
    const bot = await workerBotFactory(module(), {});
    expect((await bot.ask('x\n', 1, 5000)).lines).toEqual(['last']);
    await expect(bot.ask('y\n', 1, 5000)).rejects.toMatchObject({ kind: 'crash' });
  });

  it('is cancelled when it is terminated while it is asked', async () => {
    stubWorker();
    const bot = await workerBotFactory(module(), {});
    const asked = bot.ask('x\n', 1, 5000);
    bot.terminate();
    await expect(asked).rejects.toMatchObject({ kind: 'cancelled' });
    expect(workers[0]?.terminated).toBe(true);
    bot.terminate();
  });

  it('cannot be asked twice at once', async () => {
    stubWorker();
    const bot = await workerBotFactory(module(), {});
    const first = bot.ask('x\n', 1, 5000);
    await expect(bot.ask('y\n', 1, 5000)).rejects.toThrow('already being asked');
    bot.terminate();
    await first.catch(() => {});
  });

  it('is not started when its worker fails to load', async () => {
    stubWorker();
    failToStart = 'Failed to load module script';
    await expect(workerBotFactory(module(), {})).rejects.toThrow(
      'cannot start the bot: Failed to load module script',
    );
    expect(workers[0]?.terminated).toBe(true);
  });
});
