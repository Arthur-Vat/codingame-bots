import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  FORCE_KEY,
  chooseEngine,
  devNoServerNotice,
  engineChoice,
  engineKind,
  serverIsThere,
  setEngineKind,
} from './detect';

afterEach(() => {
  vi.unstubAllGlobals();
  setEngineKind('server');
});

const answering = (status: number, body: string): typeof fetch =>
  (async () => new Response(body, { status })) as typeof fetch;

describe('the server is there', () => {
  it('when /api/games answers 200 with a JSON array', async () => {
    expect(await serverIsThere(answering(200, '[]'))).toBe(true);
    expect(await serverIsThere(answering(200, '[{"id":"uttt"}]'))).toBe(true);
  });

  it('is not, on a 404, an error status, HTML, JSON that is not an array, or no answer', async () => {
    expect(await serverIsThere(answering(404, '[]'))).toBe(false);
    expect(await serverIsThere(answering(500, ''))).toBe(false);
    expect(await serverIsThere(answering(200, '<!doctype html><html></html>'))).toBe(false);
    expect(await serverIsThere(answering(200, '{"error":"x"}'))).toBe(false);
    expect(await serverIsThere(answering(200, ''))).toBe(false);
    const offline = (async () => {
      throw new TypeError('Failed to fetch');
    }) as typeof fetch;
    expect(await serverIsThere(offline)).toBe(false);
  });

  it('asks for /api/games', async () => {
    const seen: string[] = [];
    await serverIsThere((async (input: RequestInfo | URL) => {
      seen.push(String(input));
      return new Response('[]');
    }) as typeof fetch);
    expect(seen).toEqual(['/api/games']);
  });
});

describe('the choice of engine', () => {
  it('starts as the server', () => {
    expect(engineKind()).toBe('server');
  });

  it('is the server when it is there, the browser otherwise, and is remembered', async () => {
    expect(await chooseEngine(answering(200, '[]'))).toBe('server');
    expect(engineKind()).toBe('server');
    expect(await chooseEngine(answering(404, '<html>'))).toBe('browser');
    expect(engineKind()).toBe('browser');
  });

  it('can be forced from localStorage, for tests and diagnosis', async () => {
    const stored = new Map<string, string>();
    vi.stubGlobal('localStorage', { getItem: (key: string) => stored.get(key) ?? null });
    stored.set(FORCE_KEY, 'server');
    expect(await chooseEngine(answering(404, ''))).toBe('server');
    stored.set(FORCE_KEY, 'browser');
    expect(await chooseEngine(answering(200, '[]'))).toBe('browser');
    stored.set(FORCE_KEY, 'something else');
    expect(await chooseEngine(answering(200, '[]'))).toBe('server');
  });

  it('is not troubled by blocked storage', async () => {
    vi.stubGlobal('localStorage', {
      getItem: () => {
        throw new Error('blocked');
      },
    });
    expect(await chooseEngine(answering(404, ''))).toBe('browser');
  });
});

describe('the notice about a missing server', () => {
  it('is for development, a probe that found no server, and nothing else', () => {
    expect(devNoServerNotice({ dev: true, forced: false, serverFound: false })).toBe(true);
    expect(devNoServerNotice({ dev: true, forced: false, serverFound: true })).toBe(false);
    expect(devNoServerNotice({ dev: true, forced: true, serverFound: false })).toBe(false);
    expect(devNoServerNotice({ dev: true, forced: true, serverFound: true })).toBe(false);
    expect(devNoServerNotice({ dev: false, forced: false, serverFound: false })).toBe(false);
    expect(devNoServerNotice({ dev: false, forced: false, serverFound: true })).toBe(false);
    expect(devNoServerNotice({ dev: false, forced: true, serverFound: false })).toBe(false);
  });

  it('is recorded by chooseEngine, with whether the choice was forced or probed', async () => {
    await chooseEngine(answering(404, '<html>'), true);
    expect(engineChoice()).toEqual({ forced: false, serverFound: false, notice: true });
    await chooseEngine(answering(404, '<html>'), false);
    expect(engineChoice()).toEqual({ forced: false, serverFound: false, notice: false });
    await chooseEngine(answering(200, '[]'), true);
    expect(engineChoice()).toEqual({ forced: false, serverFound: true, notice: false });
  });

  it('is not shown when an engine is forced, whatever the probe would find', async () => {
    const stored = new Map<string, string>();
    vi.stubGlobal('localStorage', { getItem: (key: string) => stored.get(key) ?? null });
    stored.set(FORCE_KEY, 'browser');
    await chooseEngine(answering(404, ''), true);
    expect(engineChoice()).toMatchObject({ forced: true, notice: false });
    stored.set(FORCE_KEY, 'server');
    await chooseEngine(answering(404, ''), true);
    expect(engineChoice()).toMatchObject({ forced: true, notice: false });
  });

  it('is shown when storage is blocked, which counts as not forced', async () => {
    vi.stubGlobal('localStorage', {
      getItem: () => {
        throw new Error('blocked');
      },
    });
    await chooseEngine(answering(404, ''), true);
    expect(engineChoice().notice).toBe(true);
  });
});
