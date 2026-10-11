import { describe, expect, it } from 'vitest';
import { EMPTY_MODULE, wasmBytes } from './testSupport';
import { fetchedWasm, parseManifest } from './wasm';

describe('the manifest', () => {
  it('lists the games and their releases', () => {
    expect(
      parseManifest(
        '{"games":[{"id":"uttt","releases":["uttt-v002","uttt-v001"]},{"id":"x","releases":[]}]}',
      ),
    ).toEqual({
      games: [
        { id: 'uttt', releases: ['uttt-v002', 'uttt-v001'] },
        { id: 'x', releases: [] },
      ],
    });
  });

  it('refuses what is not one, saying how to build the files', () => {
    expect(() => parseManifest('<!doctype html>')).toThrow('build-studio-wasm.sh');
    expect(() => parseManifest('{}')).toThrow('no list of games');
    expect(() => parseManifest('{"games":[{"id":3}]}')).toThrow('lists a game wrongly');
  });

  it('is the real one when the files are built', () => {
    const manifest = parseManifest(wasmBytes('manifest.json').toString('utf8'));
    expect(manifest.games.find((game) => game.id === 'uttt')?.releases).toContain('uttt-v001');
  });
});

describe('fetching the files', () => {
  const answers = new Map<string, Response>();
  const asked: string[] = [];
  const fetcher = (async (input: RequestInfo | URL) => {
    const url = String(input);
    asked.push(url);
    return answers.get(url)?.clone() ?? new Response('', { status: 404 });
  }) as typeof fetch;
  const base = 'https://example.org/codingame-bots/wasm/';

  it('reads them relative to the folder of the page, once each', async () => {
    answers.clear();
    asked.length = 0;
    answers.set(`${base}manifest.json`, new Response('{"games":[]}'));
    answers.set(`${base}uttt/uttt-v001.wasm`, new Response(EMPTY_MODULE));
    const source = fetchedWasm(base, fetcher);
    expect(await source.manifest()).toEqual({ games: [] });
    await source.manifest();
    const release = await source.release('uttt', 'uttt-v001');
    await source.release('uttt', 'uttt-v001');
    expect(release.module).toBeInstanceOf(WebAssembly.Module);
    expect(release.hash).toMatch(/^[0-9a-f]{64}$/);
    expect(asked).toEqual([`${base}manifest.json`, `${base}uttt/uttt-v001.wasm`]);
  });

  it('fails clearly when a file is missing, and tries again next time', async () => {
    answers.clear();
    asked.length = 0;
    const source = fetchedWasm(base, fetcher);
    await expect(source.release('uttt', 'uttt-v001')).rejects.toThrow(
      'cannot load wasm/uttt/uttt-v001.wasm (HTTP 404)',
    );
    await expect(source.manifest()).rejects.toThrow('cannot load wasm/manifest.json (HTTP 404)');
    answers.set(`${base}uttt/uttt-v001.wasm`, new Response(EMPTY_MODULE));
    expect((await source.release('uttt', 'uttt-v001')).module).toBeInstanceOf(WebAssembly.Module);
  });

  it('fails when the file is not WebAssembly', async () => {
    answers.clear();
    answers.set(`${base}rules.wasm`, new Response('<!doctype html>'));
    await expect(fetchedWasm(base, fetcher).rules()).rejects.toThrow();
  });
});
