import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { SetupDialog } from './SetupDialog';
import type { Mode } from './setup';

const render = (
  initialMode: Mode,
  releases: string[],
  humanPlay?: boolean,
  names?: { seatNames: readonly [string, string]; game: string },
): string =>
  renderToStaticMarkup(
    createElement(SetupDialog, {
      initialMode,
      releases,
      humanPlay,
      ...names,
      onStart: () => Promise.resolve(),
      onClose: () => undefined,
    }),
  );

const modeButtons = (html: string): string[] => {
  const group = /aria-label="Mode">(.*?)<\/div>/.exec(html)?.[1] ?? '';
  return [...group.matchAll(/<strong>([^<]*)<\/strong>/g)].map((m) => m[1] ?? '');
};

describe('the setup menu', () => {
  it('offers the three modes for a game people play', () => {
    expect(modeButtons(render('computer', ['uttt-v001']))).toEqual([
      'Human vs human',
      'Human vs bot',
      'Bot vs bot',
    ]);
    expect(modeButtons(render('computer', ['uttt-v001'], true))).toHaveLength(3);
  });

  it('opens a game people play on the mode it was given', () => {
    const html = render('friend', ['uttt-v001']);
    expect(html).toMatch(/aria-pressed="true"[^>]*><strong>Human vs human/);
  });

  it('offers only the bot game for a game without human play, whatever mode was remembered', () => {
    for (const remembered of ['friend', 'computer', 'bots'] as const) {
      const html = render(remembered, ['a-v001', 'a-v002'], false);
      expect(modeButtons(html), remembered).toEqual(['Bot vs bot']);
      expect(html).toMatch(/aria-pressed="true"[^>]*><strong>Bot vs bot/);
      expect(html).not.toContain('Your side');
      expect(html).not.toContain('Minutes per side');
    }
  });

  it('says there is no bot, and does not offer a Start, when the game has no release', () => {
    const html = render('bots', [], false);
    expect(html).toContain('This game has no bot yet');
    expect(html).toMatch(/<button[^>]*disabled=""[^>]*>Start<\/button>/);
    expect(html).not.toContain('<select');
  });

  it('says the same for a human-vs-bot game without a release, but not for two humans', () => {
    expect(render('computer', [])).toContain('This game has no bot yet');
    const friends = render('friend', []);
    expect(friends).not.toContain('no bot yet');
    expect(friends).not.toMatch(/disabled=""[^>]*>Start/);
  });

  it('starts a game with releases', () => {
    const html = render('bots', ['a-v001'], false);
    expect(html).not.toContain('no bot yet');
    expect(html).not.toMatch(/disabled=""[^>]*>Start/);
  });

  it('writes the seats as X and O, or with the names a game gives them', () => {
    const plain = render('bots', ['a-v001']);
    expect(plain).toContain('X (moves first)');
    expect(plain).toContain('>O</label>');
    const named = render('bots', ['a-v001'], false, {
      seatNames: ['Red', 'Blue'],
      game: 'ice-and-fire',
    });
    expect(named).toContain('Red (moves first)');
    expect(named).toContain('>Blue</label>');
    expect(named).not.toContain('X (moves first)');
    const withSides = render('computer', ['a-v001'], true, {
      seatNames: ['Red', 'Blue'],
      game: 'ice-and-fire',
    });
    expect(withSides).toContain('aria-label="Play Red"');
    expect(withSides).toContain('You play Red and move first');
    expect(render('computer', ['a-v001'])).toContain('aria-label="Play X"');
  });
});
