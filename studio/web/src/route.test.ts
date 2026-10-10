import { describe, expect, it } from 'vitest';
import { parseRoute, routeHash } from './route';

describe('parseRoute', () => {
  it('reads the home page from an empty hash and from #/', () => {
    expect(parseRoute('')).toEqual({ page: 'home' });
    expect(parseRoute('#')).toEqual({ page: 'home' });
    expect(parseRoute('#/')).toEqual({ page: 'home' });
  });

  it('reads a game page without a session', () => {
    expect(parseRoute('#/game/uttt')).toEqual({ page: 'game', gameId: 'uttt', sessionId: null });
    expect(parseRoute('#/game/uttt/')).toEqual({ page: 'game', gameId: 'uttt', sessionId: null });
  });

  it('reads a game page with a session', () => {
    expect(parseRoute('#/game/uttt/3fa9c1')).toEqual({
      page: 'game',
      gameId: 'uttt',
      sessionId: '3fa9c1',
    });
  });

  it('reads the history page', () => {
    expect(parseRoute('#/history')).toEqual({ page: 'history' });
  });

  it('sends anything unknown to the home page', () => {
    expect(parseRoute('#/game')).toEqual({ page: 'home' });
    expect(parseRoute('#/game/uttt/a/b')).toEqual({ page: 'home' });
    expect(parseRoute('#/history/extra')).toEqual({ page: 'home' });
    expect(parseRoute('#/nowhere')).toEqual({ page: 'home' });
  });

  it('decodes percent escapes and survives broken ones', () => {
    expect(parseRoute('#/game/a%20b')).toEqual({ page: 'game', gameId: 'a b', sessionId: null });
    expect(parseRoute('#/game/%E0%A4%A')).toEqual({
      page: 'game',
      gameId: '%E0%A4%A',
      sessionId: null,
    });
  });
});

describe('routeHash', () => {
  it('is the inverse of parseRoute', () => {
    for (const hash of ['#/', '#/history', '#/game/uttt', '#/game/uttt/ab12']) {
      expect(routeHash(parseRoute(hash))).toBe(hash);
    }
  });
});
