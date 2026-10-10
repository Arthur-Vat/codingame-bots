import { describe, expect, it } from 'vitest';
import { HOME_HEADING, homeHint } from './home';

describe('homeHint', () => {
  it('tells that games need the server when there are none', () => {
    expect(homeHint(0)).toBe('Games appear here when the server is running.');
  });

  it('counts the games, in the singular and the plural', () => {
    expect(homeHint(1)).toBe('1 game is available.');
    expect(homeHint(3)).toBe('3 games are available.');
  });
});

describe('HOME_HEADING', () => {
  it('is the heading the browser test looks for', () => {
    expect(HOME_HEADING).toBe('Choose a game');
  });
});
