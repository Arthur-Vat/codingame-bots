import { describe, expect, it } from 'vitest';
import { HOME_HEADING, botSummary, releaseCount } from './home';

describe('releaseCount', () => {
  it('counts the releases, in the singular and the plural', () => {
    expect(releaseCount(0)).toBe('0 releases');
    expect(releaseCount(1)).toBe('1 release');
    expect(releaseCount(10)).toBe('10 releases');
  });
});

describe('botSummary', () => {
  it('is about the releases, and says baseline bots only for a game without a release', () => {
    expect(botSummary(10, 2)).toBe('10 releases');
    expect(botSummary(1, 5)).toBe('1 release');
    expect(botSummary(0, 5)).toBe('No releases · 5 baseline bots');
    expect(botSummary(0, 1)).toBe('No releases · 1 baseline bot');
    expect(botSummary(0, 0)).toBe('0 releases');
  });
});

describe('HOME_HEADING', () => {
  it('is the heading the browser test looks for', () => {
    expect(HOME_HEADING).toBe('Choose a game');
  });
});
