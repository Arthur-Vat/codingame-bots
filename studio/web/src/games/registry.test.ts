import { describe, expect, it } from 'vitest';
import {
  actionTextFor,
  emptyFrameFor,
  hasActionTexts,
  moveCellFor,
  ownEntry,
  renderers,
  resultDetailFor,
  seatNamesFor,
} from './registry';

describe('the registry lookups', () => {
  it('find the entries of a registered game', () => {
    expect(renderers['uttt']).toBeTypeOf('function');
    expect(emptyFrameFor('uttt')).toBeTypeOf('object');
    expect(resultDetailFor('uttt', { nope: true })).toBeNull();
  });

  it('find the entries of Ice and Fire, which has a board but no moves to read or write', () => {
    expect(renderers['ice-and-fire']).toBeTypeOf('function');
    expect(emptyFrameFor('ice-and-fire')).toBeTypeOf('object');
    expect(resultDetailFor('ice-and-fire', { nope: true })).toBeNull();
    expect(moveCellFor('ice-and-fire', '1 2')).toBeNull();
    expect(actionTextFor('ice-and-fire', { row: 1, col: 2 })).toBeNull();
    expect(hasActionTexts('ice-and-fire')).toBe(false);
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

  it('writes an action of the renderer as its move, the inverse of reading a move as a cell', () => {
    expect(actionTextFor('uttt', { row: 4, col: 5 })).toBe('4 5');
    for (const text of ['0 0', '4 5', '8 8']) {
      const at = moveCellFor('uttt', text)?.at;
      expect(actionTextFor('uttt', { row: at?.[0], col: at?.[1] })).toBe(text);
    }
    for (const action of [
      null,
      undefined,
      3,
      'x',
      {},
      { row: 9, col: 0 },
      { row: 0, col: -1 },
      { row: 1.5, col: 0 },
      { row: '1', col: 2 },
    ]) {
      expect(actionTextFor('uttt', action), JSON.stringify(action)).toBeNull();
    }
    expect(actionTextFor('nope', { row: 4, col: 5 })).toBeNull();
    expect(actionTextFor('constructor', { row: 4, col: 5 })).toBeNull();
    expect(hasActionTexts('uttt')).toBe(true);
    expect(hasActionTexts('nope')).toBe(false);
    expect(hasActionTexts('constructor')).toBe(false);
  });

  it('names the seats of Ice and Fire, and keeps X and O for the rest', () => {
    expect(seatNamesFor('ice-and-fire')).toEqual(['Red', 'Blue']);
    for (const id of ['uttt', 'nope', 'constructor', '__proto__']) {
      expect(seatNamesFor(id), id).toEqual(['X', 'O']);
    }
  });

  it('own entries only, for any map', () => {
    const map = { a: 1 };
    expect(ownEntry(map, 'a')).toBe(1);
    expect(ownEntry(map, 'constructor')).toBeUndefined();
    expect(ownEntry(map, '__proto__')).toBeUndefined();
  });
});
