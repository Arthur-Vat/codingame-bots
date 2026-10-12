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

  it('reads the history page, of my games and of the arena games', () => {
    expect(parseRoute('#/history')).toEqual({ page: 'history', set: 'mine' });
    expect(parseRoute('#/history/')).toEqual({ page: 'history', set: 'mine' });
    expect(parseRoute('#/history/arena')).toEqual({ page: 'history', set: 'arena' });
    expect(parseRoute('#/history/arena?x=1')).toEqual({ page: 'history', set: 'arena' });
  });

  it('reads the review page of each kind', () => {
    expect(parseRoute('#/review/session/ab12')).toEqual({
      page: 'review',
      kind: 'session',
      id: 'ab12',
    });
    expect(parseRoute('#/review/saved/uttt-1-abc')).toEqual({
      page: 'review',
      kind: 'saved',
      id: 'uttt-1-abc',
    });
    expect(parseRoute('#/review/file/f1')).toEqual({ page: 'review', kind: 'file', id: 'f1' });
    expect(parseRoute('#/review/arena/uttt-1-abc')).toEqual({
      page: 'review',
      kind: 'arena',
      id: 'uttt-1-abc',
    });
  });

  it('sends a review without a known kind or an id to the home page', () => {
    expect(parseRoute('#/review')).toEqual({ page: 'home' });
    expect(parseRoute('#/review/session')).toEqual({ page: 'home' });
    expect(parseRoute('#/review/other/x')).toEqual({ page: 'home' });
    expect(parseRoute('#/review/session/a/b')).toEqual({ page: 'home' });
  });

  it('sends anything unknown to the home page', () => {
    expect(parseRoute('#/game')).toEqual({ page: 'home' });
    expect(parseRoute('#/game/uttt/a/b')).toEqual({ page: 'home' });
    expect(parseRoute('#/history/extra')).toEqual({ page: 'home' });
    expect(parseRoute('#/history/arena/extra')).toEqual({ page: 'home' });
    expect(parseRoute('#/history/mine')).toEqual({ page: 'home' });
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
    for (const hash of [
      '#/',
      '#/history',
      '#/history/arena',
      '#/game/uttt',
      '#/game/uttt/ab12',
      '#/review/session/ab12',
      '#/review/saved/uttt-1-abc',
      '#/review/arena/uttt-1-abc',
    ]) {
      expect(routeHash(parseRoute(hash))).toBe(hash);
    }
  });
});
