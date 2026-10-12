import { describe, expect, it } from 'vitest';
import { memoryOf, NO_MEMORY, remember, tabHashes } from './gameTabs';
import { parseRoute } from './route';

const seen = (memory: ReturnType<typeof remember>, hash: string) =>
  remember(memory, parseRoute(hash));

describe('tabHashes', () => {
  it('leads to the plain tabs for a game not seen yet', () => {
    expect(tabHashes(NO_MEMORY, 'uttt')).toEqual({
      play: '#/game/uttt',
      review: '#/game/uttt/review',
      history: '#/game/uttt/history',
    });
  });

  it('leads back to what each tab showed last', () => {
    let memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    memory = seen(memory, '#/game/uttt/review/saved/uttt-1-abc');
    memory = seen(memory, '#/game/uttt/history/arena');
    expect(tabHashes(memory, 'uttt')).toEqual({
      play: '#/game/uttt/ab12',
      review: '#/game/uttt/review/saved/uttt-1-abc',
      history: '#/game/uttt/history/arena',
    });
  });

  it('keeps each game apart', () => {
    const memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    expect(tabHashes(memory, 'other').play).toBe('#/game/other');
    expect(memoryOf(memory, 'toString')).toEqual({ session: null, review: null, set: 'mine' });
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

  it('keeps the last review when the Review tab shows nothing, and never keeps a session', () => {
    let memory = seen(NO_MEMORY, '#/game/uttt/review/file/f1');
    memory = seen(memory, '#/game/uttt/review');
    expect(tabHashes(memory, 'uttt').review).toBe('#/game/uttt/review/file/f1');
    memory = seen(memory, '#/game/uttt/review/session/ab12');
    expect(tabHashes(memory, 'uttt').review).toBe('#/game/uttt/review/file/f1');
  });

  it('does not change what the other tabs remember', () => {
    let memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    memory = seen(memory, '#/game/uttt/history/arena');
    memory = seen(memory, '#/game/uttt/history');
    expect(tabHashes(memory, 'uttt')).toEqual({
      play: '#/game/uttt/ab12',
      review: '#/game/uttt/review',
      history: '#/game/uttt/history',
    });
  });

  it('returns the same object when nothing changes, and ignores other pages', () => {
    const memory = seen(NO_MEMORY, '#/game/uttt/ab12');
    expect(seen(memory, '#/game/uttt/ab12')).toBe(memory);
    expect(seen(memory, '#/')).toBe(memory);
    expect(seen(memory, '#/history')).toBe(memory);
    expect(seen(memory, '#/review/saved/x')).toBe(memory);
    expect(seen(NO_MEMORY, '#/game/uttt')).toBe(NO_MEMORY);
  });
});
