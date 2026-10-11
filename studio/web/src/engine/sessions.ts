/* The games in progress of the browser engine, in the memory of the page. It does what the local
   server's api.rs, runner.rs and session.rs do, with the rules from the rules program (ops `new`,
   `settle`, `play`, `replay` and `record`) and the bots as workers.

   A session is a `RulesState` (setup, seats, answers so far, result) with what is needed to run
   bots: a status, a progress, the bots, and a generation. Everything else (frames, legal moves,
   whose turn it is) comes from the rules, which replay the game each time.

   Generation: a takeback, an ended game and a deleted session raise the generation, which
   invalidates the driver that is waiting for a bot. A driver checks that it still holds the
   generation it started with before it stores anything after waiting.

   Consistency: the rules run synchronously, and a driver changes a session only in steps with no
   wait in them, so a request, which also runs without waiting, sees a session between two steps. */

import { BOTS_UNAVAILABLE, BotError, type Bot, type BotFactory } from './bot';
import { HttpError, messageOf } from './http';
import {
  RulesError,
  botSeeds,
  type GameResult,
  type Next,
  type ReplayStep,
  type Rules,
  type RulesState,
  type Settled,
  type TurnAnswer,
} from './rules';
import { iterations, type Speed } from './speed';
import type { LoadedRelease, WasmSource } from './wasm';

export type Status =
  'compiling' | 'measuring' | 'rewinding' | 'bot_thinking' | 'waiting_human' | 'over' | 'failed';

interface Progress {
  done: number;
  total: number | null;
}

/** The time a bot has for an answer while it is brought back to a position (realtime mode). */
const REPLAY_LIMIT_MS = 60_000;

/** The time a "fixed" bot has for an answer: generous, the work is a number of iterations. */
function fixedLimitMs(thinkMs: number): number {
  return 10_000 + 5 * thinkMs;
}

/** Sessions untouched for this long are dropped. */
export const IDLE_MS = 2 * 60 * 60 * 1000;

interface Session {
  id: string;
  state: RulesState;
  status: Status;
  error: string | null;
  progress: Progress | null;
  generation: number;
  bots: [Bot | null, Bot | null];
  touched: number;
  /** Raised by every change of the state: what `settled` was computed for. */
  version: number;
  settled: { version: number; value: Settled } | null;
}

export interface SessionDeps {
  rules: Rules;
  wasm: WasmSource;
  speed: Speed;
  /** Null where bots cannot run (the page is not cross-origin isolated). */
  makeBot: BotFactory | null;
  /** Milliseconds since the Unix epoch. */
  now: () => number;
  /** A fresh id. */
  newId: () => string;
}

function sameLines(a: readonly string[], b: readonly string[]): boolean {
  return a.length === b.length && a.every((line, index) => line === b[index]);
}

export class Sessions {
  private readonly sessions = new Map<string, Session>();
  private readonly deps: SessionDeps;

  constructor(deps: SessionDeps) {
    this.deps = deps;
  }

  // ---- the routes ----

  async create(request: unknown): Promise<{ id: string }> {
    const { rules, wasm } = this.deps;
    const game = (request as { game?: unknown } | null)?.game;
    const manifest = await wasm.manifest();
    const releases =
      typeof game === 'string' ? (manifest.games.find((g) => g.id === game)?.releases ?? []) : [];
    let state: RulesState;
    try {
      state = rules.call<RulesState>({ op: 'new', request, releases });
    } catch (error) {
      if (error instanceof RulesError) throw new HttpError(400, error.message);
      throw error;
    }
    const anyBot = state.seats.some((seat) => seat.kind === 'bot');
    if (anyBot && this.deps.makeBot === null) throw new HttpError(409, BOTS_UNAVAILABLE);
    this.sweep();
    const session: Session = {
      id: this.freshId(),
      state,
      status: 'waiting_human',
      error: null,
      progress: null,
      generation: 0,
      bots: [null, null],
      touched: this.deps.now(),
      version: 0,
      settled: null,
    };
    this.sessions.set(session.id, session);
    if (anyBot) {
      session.status = 'compiling';
      void this.launch(session, session.generation);
    } else {
      this.settle(session);
    }
    return { id: session.id };
  }

  /** The JSON the front end polls. */
  view(id: string): unknown {
    return this.viewOf(this.get(id));
  }

  remove(id: string): void {
    const session = this.get(id);
    this.stop(session);
    this.sessions.delete(id);
  }

  move(id: string, seat: number, index: number): unknown {
    const session = this.get(id);
    if (session.status !== 'waiting_human') {
      throw new HttpError(409, `not waiting for a move (status ${session.status})`);
    }
    const settled = this.settledOf(session);
    const human = session.state.seats[seat]?.kind === 'human';
    if (settled.to_act.length !== 1 || settled.to_act[0] !== seat || !human) {
      throw new HttpError(409, "it is not that human's turn");
    }
    const moves = settled.human_moves;
    const chosen = moves[index];
    if (chosen === undefined) {
      throw new HttpError(409, `index ${index} is not one of the ${moves.length} moves`);
    }
    this.apply(session, { seat, lines: chosen.lines, ms: 0 });
    if (this.settle(session) !== null) void this.drive(session, session.generation);
    return this.viewOf(session);
  }

  takeback(id: string, turns: number): unknown {
    const session = this.get(id);
    const { state } = session;
    if (!['waiting_human', 'bot_thinking', 'over'].includes(session.status)) {
      throw new HttpError(409, `no takeback now (status ${session.status})`);
    }
    const realtime = state.seats.some((seat) => seat.kind === 'bot' && seat.mode === 'realtime');
    if (realtime) {
      throw new HttpError(409, 'takebacks are unavailable against a bot playing in real time');
    }
    const settled = this.settledOf(session);
    const valid = settled.valid_turns;
    if (turns < settled.opening_turns || turns > valid) {
      throw new HttpError(
        409,
        `turns must be from ${settled.opening_turns} to ${valid}, got ${turns}`,
      );
    }
    this.stop(session);
    state.turns = state.turns.slice(0, turns);
    state.result = null;
    session.version += 1;
    session.error = null;
    session.progress = null;
    if (state.seats.some((seat) => seat.kind === 'bot')) {
      session.status = 'rewinding';
      session.progress = { done: 0, total: this.botAnswers(session) };
      void this.launch(session, session.generation);
    } else {
      this.settle(session);
    }
    return this.viewOf(session);
  }

  end(id: string, seat: number, reason: string): unknown {
    const session = this.get(id);
    if (seat !== 0 && seat !== 1) throw new HttpError(400, 'seat must be 0 or 1');
    if (reason !== 'resign' && reason !== 'timeout') {
      throw new HttpError(400, 'reason must be "resign" or "timeout"');
    }
    if (session.state.seats[seat]?.kind !== 'human') {
      throw new HttpError(409, 'only a human seat can resign or run out of time');
    }
    if (session.status === 'over' || session.status === 'failed') {
      throw new HttpError(409, 'the game is already over');
    }
    this.finish(session, {
      winner: 1 - seat,
      end:
        reason === 'resign' ? { kind: 'resigned', seat } : { kind: 'timeout', seat, limit_ms: 0 },
    });
    return this.viewOf(session);
  }

  /** The game record (format 1) of the session so far, as text. */
  record(id: string): string {
    return this.recordText(this.get(id));
  }

  /** The number of sessions held. */
  get size(): number {
    return this.sessions.size;
  }

  // ---- bookkeeping ----

  private get(id: string): Session {
    const session = this.sessions.get(id);
    if (session === undefined) throw new HttpError(404, 'unknown session');
    session.touched = this.deps.now();
    return session;
  }

  private freshId(): string {
    for (;;) {
      const id = this.deps.newId();
      if (!this.sessions.has(id)) return id;
    }
  }

  /** Drops the sessions nobody has asked about for two hours. */
  private sweep(): void {
    const now = this.deps.now();
    for (const [id, session] of this.sessions) {
      if (now - session.touched > IDLE_MS) {
        this.stop(session);
        this.sessions.delete(id);
      }
    }
  }

  private recordText(session: Session): string {
    const unixTime = Math.floor(this.deps.now() / 1000);
    return this.deps.rules.call<{ record_text: string }>({
      op: 'record',
      state: session.state,
      unix_time: unixTime,
    }).record_text;
  }

  private botAnswers(session: Session): number {
    const { seats, turns } = session.state;
    return turns.flat().filter((answer: TurnAnswer) => seats[answer.seat]?.kind === 'bot').length;
  }

  /** What the rules say of the position, kept until the state changes. */
  private settledOf(session: Session): Settled {
    if (session.settled?.version !== session.version) {
      const value = this.deps.rules.call<Settled>({ op: 'settle', state: session.state });
      session.settled = { version: session.version, value };
    }
    return session.settled.value;
  }

  private viewOf(session: Session): unknown {
    const { state } = session;
    const settled = this.settledOf(session);
    const over = state.result !== null || session.status === 'failed';
    const toAct = over ? [] : settled.to_act;
    const waiting = session.status === 'waiting_human' && toAct.length === 1;
    return {
      id: session.id,
      game: state.game,
      seed: state.seed,
      opening_plies: state.opening_plies,
      seats: state.seats,
      status: session.status,
      error: session.error,
      to_act: toAct,
      opening_turns: settled.opening_turns,
      turns: state.turns,
      frames: settled.frames,
      human_moves: waiting ? settled.human_moves : [],
      result: state.result,
      progress: session.progress,
    };
  }

  // ---- the changes of a session; each is one step with no wait in it ----

  /** Stops the bots and the driver waiting for them. */
  private stop(session: Session): void {
    session.generation += 1;
    const bots = session.bots;
    session.bots = [null, null];
    for (const bot of bots) bot?.terminate();
  }

  private finish(session: Session, result: GameResult): void {
    this.stop(session);
    session.state.result = result;
    session.version += 1;
    session.status = 'over';
    session.progress = null;
    session.error = null;
  }

  private fail(session: Session, message: string): void {
    this.stop(session);
    session.status = 'failed';
    session.progress = null;
    session.error = message;
  }

  private lose(session: Session, seat: number, end: GameResult['end']): void {
    this.finish(session, { winner: 1 - seat, end });
  }

  /**
   * Looks at the position and sets the status: over, waiting for a human, or a bot to think.
   * Returns what a bot must be asked when one must act now.
   */
  private settle(session: Session): Extract<Next, { kind: 'bot' }> | null {
    if (session.state.result !== null) return null;
    const settled = this.settledOf(session);
    const next = settled.next;
    switch (next.kind) {
      case 'over':
        if (settled.result !== null) this.finish(session, settled.result);
        return null;
      case 'unsupported':
        this.fail(session, next.error);
        return null;
      case 'bot':
        session.error = null;
        session.status = 'bot_thinking';
        return next;
      case 'human':
        session.error = null;
        session.status = 'waiting_human';
        return null;
    }
  }

  /** Plays an answer. One the rules reject is kept and loses the game for its seat. */
  private apply(session: Session, answer: TurnAnswer): void {
    const played = this.deps.rules.call<{ turns: TurnAnswer[][]; result: GameResult | null }>({
      op: 'play',
      state: session.state,
      answer,
    });
    session.state.turns = played.turns;
    session.state.result = played.result;
    session.version += 1;
    if (played.result !== null) this.finish(session, played.result);
  }

  /** Changes the session unless a newer generation replaced the caller. */
  private update(session: Session, generation: number, change: () => void): boolean {
    if (session.generation !== generation) return false;
    change();
    return true;
  }

  // ---- the driver: starts the bots and plays their turns ----

  private async launch(session: Session, generation: number): Promise<void> {
    try {
      if (await this.prepare(session, generation)) await this.drive(session, generation);
    } catch (error) {
      this.failIfCurrent(session, generation, error);
    }
  }

  /** The driver met a failure that is not a bot's: the session cannot go on, if it is still its. */
  private failIfCurrent(session: Session, generation: number, error: unknown): void {
    if (session.generation === generation) this.fail(session, messageOf(error));
  }

  /**
   * Loads the releases, measures the "fixed" ones, starts the bots and replays the turns into
   * them. False when the driver was replaced meanwhile.
   */
  private async prepare(session: Session, generation: number): Promise<boolean> {
    const { rules, wasm, speed, makeBot } = this.deps;
    if (makeBot === null) throw new Error(BOTS_UNAVAILABLE);
    if (session.generation !== generation) return false;
    if (session.status !== 'rewinding') session.status = 'compiling';
    const { state } = session;

    interface Plan {
      seat: number;
      release: string;
      module: WebAssembly.Module;
      env: Record<string, string>;
      limitMs: number;
    }
    const plans: Plan[] = [];
    for (const [seat, config] of state.seats.entries()) {
      if (config.kind !== 'bot') continue;
      const { release, think_ms: thinkMs } = config;
      let file: LoadedRelease;
      try {
        file = await wasm.release(state.game, release);
      } catch (error) {
        throw new Error(`cannot load ${release}: ${messageOf(error)}`, { cause: error });
      }
      if (session.generation !== generation) return false;
      const env: Record<string, string> = {};
      let limitMs: number;
      if (config.mode === 'fixed') {
        if (speed.cached(release, file.hash) === null) {
          if (!this.update(session, generation, () => (session.status = 'measuring'))) return false;
        }
        const rate = await speed.rate(release, file.hash, {
          rules,
          makeBot,
          module: file.module,
          game: state.game,
          release,
          progress: (done) =>
            void this.update(session, generation, () => {
              session.progress = { done, total: null };
            }),
        });
        const fixedIters = iterations(thinkMs, rate);
        const stored = this.update(session, generation, () => {
          const target = state.seats[seat];
          if (target?.kind === 'bot') target.fixed_iters = fixedIters;
          session.version += 1;
          session.progress = null;
        });
        if (!stored) return false;
        env.CG_FIXED_ITERS = String(fixedIters);
        limitMs = fixedLimitMs(thinkMs);
      } else {
        // The scale of the release's own time budget; left unset when it is 1, as the server does.
        const scale = thinkMs / 100;
        if (scale !== 1) env.CG_TIME_SCALE = String(scale);
        limitMs = REPLAY_LIMIT_MS;
      }
      plans.push({ seat, release, module: file.module, env, limitMs });
    }

    // Each bot answers its recorded turns again: that is the replay.
    const seeds = botSeeds(rules, state);
    const steps = new Map<number, ReplayStep[]>();
    let total = 0;
    for (const plan of plans) {
      const list = rules.call<ReplayStep[]>({ op: 'replay', state, seat: plan.seat });
      steps.set(plan.seat, list);
      total += list.length;
    }
    if (total > 0) {
      const set = this.update(session, generation, () => {
        session.status = 'rewinding';
        session.progress = { done: 0, total };
      });
      if (!set) return false;
    }

    const started: [Bot | null, Bot | null] = [null, null];
    let handedOver = false;
    try {
      let replayed = 0;
      for (const plan of plans) {
        const seed = seeds[plan.seat];
        if (seed === null || seed === undefined) throw new Error('a bot seat has no seed');
        const bot = await makeBot(plan.module, { CG_SEED: seed, ...plan.env });
        started[plan.seat] = bot;
        if (session.generation !== generation) return false;
        for (const step of steps.get(plan.seat) ?? []) {
          let lines: string[];
          try {
            lines = (await bot.ask(step.input, step.answer_lines, plan.limitMs)).lines;
          } catch (error) {
            if (session.generation !== generation) return false;
            throw new Error(`cannot restart ${plan.release}: ${messageOf(error)}`, {
              cause: error,
            });
          }
          if (session.generation !== generation) return false;
          if (!sameLines(lines, step.lines)) {
            throw new Error(
              `${plan.release} did not replay identically; takebacks are unavailable against it`,
            );
          }
          replayed += 1;
          const done = replayed;
          this.update(session, generation, () => {
            session.progress = { done, total };
          });
        }
      }
      handedOver = this.update(session, generation, () => {
        session.bots = started;
        session.progress = null;
      });
      return handedOver;
    } finally {
      if (!handedOver) for (const bot of started) bot?.terminate();
    }
  }

  /** Plays the bots' turns until a human must act or the game is over. */
  private async drive(session: Session, generation: number): Promise<void> {
    try {
      await this.play(session, generation);
    } catch (error) {
      this.failIfCurrent(session, generation, error);
    }
  }

  private async play(session: Session, generation: number): Promise<void> {
    for (;;) {
      if (session.generation !== generation) return;
      const next = this.settle(session);
      if (next === null) return;
      const bot = session.bots[next.seat];
      if (bot === null || bot === undefined) {
        this.fail(session, `the bot of seat ${next.seat} is not running`);
        return;
      }
      let answer: { lines: string[]; ms: number };
      try {
        answer = await bot.ask(next.input, next.answer_lines, next.limit_ms);
      } catch (error) {
        if (session.generation !== generation) return;
        if (!(error instanceof BotError)) throw error;
        const seat = next.seat;
        if (error.kind === 'timeout') {
          this.lose(session, seat, { kind: 'timeout', seat, limit_ms: next.limit_ms });
        } else {
          this.lose(session, seat, { kind: 'crash', seat, detail: error.message });
        }
        return;
      }
      if (session.generation !== generation) return;
      this.apply(session, { seat: next.seat, lines: answer.lines, ms: answer.ms });
    }
  }
}
