import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { SettingsStorage } from './analysisSettings';
import {
  forgetChosenThemes,
  readTheme,
  setGameTheme,
  subscribe,
  themeKey,
  useGameTheme,
  writeTheme,
} from './gameThemes';

function memory(
  initial: Record<string, string> = {},
): SettingsStorage & { data: Map<string, string> } {
  const data = new Map(Object.entries(initial));
  return {
    data,
    getItem: (key) => data.get(key) ?? null,
    setItem: (key, value) => void data.set(key, value),
  };
}

const blocked: SettingsStorage = {
  getItem: () => {
    throw new Error('blocked');
  },
  setItem: () => {
    throw new Error('blocked');
  },
};

afterEach(forgetChosenThemes);

describe('the remembered themes', () => {
  it('are under the game’s id and the kind', () => {
    expect(themeKey('ice-and-fire', 'pieces')).toBe('studio.theme.ice-and-fire.pieces');
    expect(themeKey('ice-and-fire', 'board')).toBe('studio.theme.ice-and-fire.board');
  });

  it('have the owner’s defaults: lava and glacier pieces on a Scandinavian board', () => {
    const storage = memory();
    expect(readTheme('ice-and-fire', 'pieces', storage)).toBe('lava');
    expect(readTheme('ice-and-fire', 'board', storage)).toBe('scandinavian');
    expect(readTheme('ice-and-fire', 'pieces', null)).toBe('lava');
  });

  it('are written and read back, one kind apart from the other', () => {
    const storage = memory();
    expect(writeTheme('ice-and-fire', 'pieces', 'storybook', storage)).toBe(true);
    expect(writeTheme('ice-and-fire', 'board', 'japanese', storage)).toBe(true);
    expect([...storage.data]).toEqual([
      ['studio.theme.ice-and-fire.pieces', 'storybook'],
      ['studio.theme.ice-and-fire.board', 'japanese'],
    ]);
    expect(readTheme('ice-and-fire', 'pieces', storage)).toBe('storybook');
    expect(readTheme('ice-and-fire', 'board', storage)).toBe('japanese');
  });

  it('give the default for a stored id the game does not offer', () => {
    for (const value of ['nope', '', 'Lava', '../x', '__proto__', 'constructor', 'japanese ']) {
      const storage = memory({
        'studio.theme.ice-and-fire.pieces': value,
        'studio.theme.ice-and-fire.board': value,
      });
      expect(readTheme('ice-and-fire', 'pieces', storage), value).toBe('lava');
      expect(readTheme('ice-and-fire', 'board', storage), value).toBe('scandinavian');
    }
    // A board id is not a pieces id when the lists differ.
    const storage = memory({ 'studio.theme.ice-and-fire.pieces': 'watercolour' });
    expect(readTheme('ice-and-fire', 'pieces', storage)).toBe('lava');
  });

  it('refuse an id the game does not offer, and a game with no themes', () => {
    const storage = memory();
    expect(writeTheme('ice-and-fire', 'pieces', 'nope', storage)).toBe(false);
    expect(writeTheme('uttt', 'pieces', 'lava', storage)).toBe(false);
    expect(writeTheme('constructor', 'board', 'lava', storage)).toBe(false);
    expect(storage.data.size).toBe(0);
    expect(readTheme('uttt', 'board', storage)).toBe('');
    expect(readTheme('constructor', 'board', storage)).toBe('');
  });

  it('survive a storage that throws: the default is read, a write is forgotten', () => {
    expect(readTheme('ice-and-fire', 'pieces', blocked)).toBe('lava');
    expect(writeTheme('ice-and-fire', 'pieces', 'deco', blocked)).toBe(true);
  });

  it('hold for the page in memory, so that a choice works where no storage does', () => {
    // There is no browser storage under test: the hook reads what was chosen on the page.
    const Probe = () => {
      const { pieces, board } = useGameTheme('ice-and-fire');
      return createElement('i', { 'data-pieces': pieces, 'data-board': board });
    };
    const html = () => renderToStaticMarkup(createElement(Probe));
    expect(html()).toBe('<i data-pieces="lava" data-board="scandinavian"></i>');
    setGameTheme('ice-and-fire', 'board', 'deco');
    setGameTheme('ice-and-fire', 'pieces', 'nope');
    expect(html()).toBe('<i data-pieces="lava" data-board="deco"></i>');
  });
});

describe('the storage event', () => {
  afterEach(() => vi.unstubAllGlobals());

  /** Sends the event to a listener on a stand-in window, and says what it did to the choices. */
  function after(key: string | null): { heard: number; board: string } {
    const win = new EventTarget();
    vi.stubGlobal('window', win);
    let heard = 0;
    const unsubscribe = subscribe(() => heard++);
    setGameTheme('ice-and-fire', 'board', 'deco');
    heard = 0;
    win.dispatchEvent(Object.assign(new Event('storage'), { key }));
    unsubscribe();
    return { heard, board: readTheme('ice-and-fire', 'board', null) };
  }

  it('clears the choices made on the page when a theme key changed, or all storage did', () => {
    // Nothing is stored under test, so a cleared choice reads as the default again.
    expect(after('studio.theme.ice-and-fire.board')).toEqual({ heard: 1, board: 'scandinavian' });
    expect(after(null)).toEqual({ heard: 1, board: 'scandinavian' });
  });

  it('leaves the choices alone when another key changed', () => {
    const result = after('studio.analysis.settings');
    expect(result.heard).toBe(0);
  });
});
