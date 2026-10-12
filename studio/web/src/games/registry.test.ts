import { describe, expect, it } from 'vitest';
import { emptyFrameFor, ownEntry, renderers, resultDetailFor } from './registry';

describe('the registry lookups', () => {
  it('find the entries of a registered game', () => {
    expect(renderers['uttt']).toBeTypeOf('function');
    expect(emptyFrameFor('uttt')).toBeTypeOf('object');
    expect(resultDetailFor('uttt', { nope: true })).toBeNull();
  });

  it('find nothing for an unknown game, and nothing a plain object inherits', () => {
    for (const id of ['nope', '', 'constructor', '__proto__', 'toString', 'hasOwnProperty']) {
      expect(renderers[id], id).toBeUndefined();
      expect(emptyFrameFor(id), id).toBeUndefined();
      expect(resultDetailFor(id, {}), id).toBeNull();
    }
  });

  it('own entries only, for any map', () => {
    const map = { a: 1 };
    expect(ownEntry(map, 'a')).toBe(1);
    expect(ownEntry(map, 'constructor')).toBeUndefined();
    expect(ownEntry(map, '__proto__')).toBeUndefined();
  });
});
