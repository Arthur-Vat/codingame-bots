import { describe, expect, it } from 'vitest';
import { currentTab, memoryOf, NO_MEMORY, remember, tabHashes } from './gameTabs';
import { parseRoute } from './route';

const seen = (memory: ReturnType<typeof remember>, hash: string) =>
  remember(memory, parseRoute(hash));

describe('tabHashes', () => {
  it('leads to the plain tabs for a game not seen yet', () => {
    expect(tabHashes(NO_MEMORY, 'uttt')).toEqual({
      play: '#/game/uttt',
      history: '#/game/uttt/history',
    });
  });

  it('leads back to what each tab showed last', () => {
    let memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    memory = seen(memory, '#/game/uttt/analysis/saved/uttt-1-abc');
    memory = seen(memory, '#/game/uttt/history/arena');
    expect(tabHashes(memory, 'uttt')).toEqual({
      play: '#/game/uttt/ab12',
      history: '#/game/uttt/history/arena',
    });
  });

  it('keeps each game apart', () => {
    const memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    expect(tabHashes(memory, 'other').play).toBe('#/game/other');
    expect(memoryOf(memory, 'toString')).toEqual({ session: null, set: 'mine' });
  });
});

describe('remember', () => {
  it('follows the Play tab, including a page without a session', () => {
    let memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    expect(tabHashes(memory, 'uttt').play).toBe('#/game/uttt/ab12');
    memory = seen(memory, '#/game/uttt/cd34');
    expect(tabHashes(memory, 'uttt').play).toBe('#/game/uttt/cd34');
    memory = seen(memory, '#/game/uttt');
    expect(tabHashes(memory, 'uttt').play).toBe('#/game/uttt');
  });

  it('remembers nothing of the analysis board', () => {
    const memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    expect(seen(memory, '#/game/uttt/analysis/session/ab12')).toBe(memory);
    expect(seen(memory, '#/game/uttt/analysis/saved/uttt-1-abc')).toBe(memory);
    expect(seen(NO_MEMORY, '#/game/uttt/analysis/file/f1')).toBe(NO_MEMORY);
  });

  it('does not change what the other tabs remember', () => {
    let memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    memory = seen(memory, '#/game/uttt/history/arena');
    memory = seen(memory, '#/game/uttt/history');
    expect(tabHashes(memory, 'uttt')).toEqual({
      play: '#/game/uttt/ab12',
      history: '#/game/uttt/history',
    });
  });

  it('returns the same object when nothing changes, and ignores other pages', () => {
    const memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    expect(seen(memory, '#/game/uttt/ab12')).toBe(memory);
    expect(seen(memory, '#/')).toBe(memory);
    expect(seen(memory, '#/history')).toBe(memory);
    expect(seen(memory, '#/game/uttt/review/saved/x')).toBe(memory);
    expect(seen(NO_MEMORY, '#/game/uttt')).toBe(NO_MEMORY);
  });
});

describe('currentTab', () => {
  const current = (hash: string) => {
    const route = parseRoute(hash);
    if (route.page !== 'game') throw new Error(hash);
    return currentTab(route);
  };

  it('marks the tab of a tab', () => {
    expect(current('#/game/uttt')).toBe('play');
    expect(current('#/game/uttt/ab12')).toBe('play');
    expect(current('#/game/uttt/history')).toBe('history');
    expect(current('#/game/uttt/history/arena')).toBe('history');
  });

  it('marks, on the analysis board, the tab the game came from', () => {
    expect(current('#/game/uttt/analysis/session/ab12')).toBe('play');
    for (const kind of ['saved', 'file', 'arena']) {
      expect(current(`#/game/uttt/analysis/${kind}/x`), kind).toBe('history');
    }
  });
});
