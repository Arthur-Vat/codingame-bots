import { describe, expect, it } from 'vitest';
import { parseRoute, routeHash, type Route } from './route';

describe('parseRoute', () => {
  it('reads the home page from an empty hash and from #/', () => {
    expect(parseRoute('')).toEqual({ page: 'home' });
    expect(parseRoute('#')).toEqual({ page: 'home' });
    expect(parseRoute('#/')).toEqual({ page: 'home' });
  });

  it('reads the play tab without a session', () => {
    expect(parseRoute('#/game/uttt')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'play',
      sessionId: null,
    });
    expect(parseRoute('#/game/uttt/')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'play',
      sessionId: null,
    });
  });

  it('reads the play tab with a session', () => {
    expect(parseRoute('#/game/uttt/3fa9c1')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'play',
      sessionId: '3fa9c1',
    });
  });

  it('reads the review tab, with nothing open and with a game of each kind', () => {
    expect(parseRoute('#/game/uttt/review')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'review',
      review: null,
    });
    for (const [kind, id] of [
      ['saved', 'uttt-1-abc'],
      ['file', 'f1'],
      ['arena', 'uttt-1-abc'],
    ] as const) {
      expect(parseRoute(`#/game/uttt/review/${kind}/${id}`)).toEqual({
        page: 'game',
        gameId: 'uttt',
        tab: 'review',
        review: { kind, id },
      });
    }
  });

  it('reads the history tab, of my games and of the arena games', () => {
    expect(parseRoute('#/game/uttt/history')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'history',
      set: 'mine',
    });
    expect(parseRoute('#/game/uttt/history/')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'history',
      set: 'mine',
    });
    expect(parseRoute('#/game/uttt/history/arena')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'history',
      set: 'arena',
    });
    expect(parseRoute('#/game/uttt/history/arena?x=1')).toEqual({
      page: 'game',
      gameId: 'uttt',
      tab: 'history',
      set: 'arena',
    });
  });

  it('sends the addresses of the time before the tabs to the home page', () => {
    for (const hash of [
      '#/history',
      '#/history/',
      '#/history/arena',
      '#/review/saved/uttt-1-abc',
      '#/review/file/f1',
      '#/review/arena/uttt-1-abc',
      '#/review/session/ab12',
    ]) {
      expect(parseRoute(hash), hash).toEqual({ page: 'home' });
    }
  });

  it('sends the review of a session to the home page: a session is reviewed in the Play tab', () => {
    expect(parseRoute('#/game/uttt/review/session/ab12')).toEqual({ page: 'home' });
  });

  it('sends anything unknown or malformed to the home page', () => {
    for (const hash of [
      '#/game',
      '#/game/',
      '#/game/uttt/a/b',
      '#/game/uttt/review/bogus/x',
      '#/game/uttt/review/session',
      '#/game/uttt/review/session/a',
      '#/game/uttt/review/saved/x/extra',
      '#/game/uttt/history/other',
      '#/game/uttt/history/arena/extra',
      '#/game/uttt/history/mine',
      '#/game/uttt/abcd/review',
      '#/review',
      '#/review/other/x',
      '#/nowhere',
    ]) {
      expect(parseRoute(hash), hash).toEqual({ page: 'home' });
    }
  });

  it('decodes percent escapes and survives broken ones', () => {
    expect(parseRoute('#/game/a%20b')).toEqual({
      page: 'game',
      gameId: 'a b',
      tab: 'play',
      sessionId: null,
    });
    expect(parseRoute('#/game/%E0%A4%A')).toEqual({
      page: 'game',
      gameId: '%E0%A4%A',
      tab: 'play',
      sessionId: null,
    });
    expect(parseRoute('#/game/a%20b/review/file/x%2Fy')).toEqual({
      page: 'game',
      gameId: 'a b',
      tab: 'review',
      review: { kind: 'file', id: 'x/y' },
    });
  });
});

describe('routeHash', () => {
  it('is the inverse of parseRoute', () => {
    for (const hash of [
      '#/',
      '#/game/uttt',
      '#/game/uttt/ab12',
      '#/game/uttt/review',
      '#/game/uttt/review/saved/uttt-1-abc',
      '#/game/uttt/review/file/f1',
      '#/game/uttt/review/arena/uttt-1-abc',
      '#/game/uttt/history',
      '#/game/uttt/history/arena',
    ]) {
      expect(routeHash(parseRoute(hash)), hash).toBe(hash);
    }
  });

  it('percent-encodes what is not a plain segment, and parses it back', () => {
    const routes: Route[] = [
      { page: 'game', gameId: 'a b', tab: 'play', sessionId: 'x/y' },
      { page: 'game', gameId: '../x', tab: 'review', review: { kind: 'file', id: '<b>#?' } },
      { page: 'game', gameId: 'é', tab: 'history', set: 'arena' },
    ];
    for (const route of routes) {
      const hash = routeHash(route);
      expect(hash.slice(1)).not.toMatch(/[<> ?#]/);
      expect(parseRoute(hash)).toEqual(route);
    }
  });
});
