import { describe, expect, it } from 'vitest';
import { emptyFrameFor, moveCellFor, ownEntry, renderers, resultDetailFor } from './registry';

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

  it('reads a move as its cell, and nothing else as one', () => {
    expect(moveCellFor('uttt', '4 5')).toEqual({ key: '4,5', at: [4, 5] });
    expect(moveCellFor('uttt', ' 0 8 ')).toEqual({ key: '0,8', at: [0, 8] });
    for (const text of ['', '9 0', '4', '4 5 6', '4,5', 'a b', '-1 0']) {
      expect(moveCellFor('uttt', text), text).toBeNull();
    }
    expect(moveCellFor('nope', '4 5')).toBeNull();
    expect(moveCellFor('constructor', '4 5')).toBeNull();
  });

  it('own entries only, for any map', () => {
    const map = { a: 1 };
    expect(ownEntry(map, 'a')).toBe(1);
    expect(ownEntry(map, 'constructor')).toBeUndefined();
    expect(ownEntry(map, '__proto__')).toBeUndefined();
  });
});
