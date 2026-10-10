import { describe, expect, it } from 'vitest';
import { HOME_HEADING, releaseCount } from './home';

describe('releaseCount', () => {
  it('counts the releases, in the singular and the plural', () => {
    expect(releaseCount(0)).toBe('0 releases');
    expect(releaseCount(1)).toBe('1 release');
    expect(releaseCount(10)).toBe('10 releases');
  });
});

describe('HOME_HEADING', () => {
  it('is the heading the browser test looks for', () => {
    expect(HOME_HEADING).toBe('Choose a game');
  });
});
