import { afterEach, describe, expect, it, vi } from 'vitest';
import { FORCE_KEY, chooseEngine, engineKind, serverIsThere, setEngineKind } from './detect';

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
