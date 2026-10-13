import { describe, expect, it } from 'vitest';
import {
  KEY_ENGINE_ON,
  KEY_HEAT,
  KEY_LINES,
  readEngineOn,
  readHeat,
  readLines,
  writeEngineOn,
  writeHeat,
  writeLines,
  type SettingsStorage,
} from './analysisSettings';

function memory(
  initial: Record<string, string> = {},
): SettingsStorage & { data: Map<string, string> } {
  const data = new Map(Object.entries(initial));
  return {
    data,
    getItem: (key) => data.get(key) ?? null,
    setItem: (key, value) => void data.set(key, value),
  };
}

const blocked: SettingsStorage = {
  getItem: () => {
    throw new Error('blocked');
  },
  setItem: () => {
    throw new Error('blocked');
  },
};

describe('the remembered settings', () => {
  it('have the owner’s defaults: engine on, heat map off, one line', () => {
    const storage = memory();
    expect(readEngineOn(storage)).toBe(true);
    expect(readHeat(storage)).toBe(false);
    expect(readLines(storage)).toBe(1);
  });

  it('are written under their keys and read back', () => {
    const storage = memory();
    writeEngineOn(false, storage);
    writeHeat(true, storage);
    writeLines(3, storage);
    expect([...storage.data]).toEqual([
      ['studio.analysis.on', '0'],
      ['studio.analysis.heat', '1'],
      ['studio.analysis.lines', '3'],
    ]);
    expect(KEY_ENGINE_ON).toBe('studio.analysis.on');
    expect(KEY_HEAT).toBe('studio.analysis.heat');
    expect(KEY_LINES).toBe('studio.analysis.lines');
    expect(readEngineOn(storage)).toBe(false);
    expect(readHeat(storage)).toBe(true);
    expect(readLines(storage)).toBe(3);
  });

  it('fall back to the defaults for anything else', () => {
    const storage = memory({
      'studio.analysis.on': 'yes',
      'studio.analysis.heat': '',
      'studio.analysis.lines': '2',
    });
    expect(readEngineOn(storage)).toBe(true);
    expect(readHeat(storage)).toBe(false);
    expect(readLines(storage)).toBe(1);
  });

  it('survive blocked storage: defaults when reading, nothing thrown when writing', () => {
    expect(readEngineOn(blocked)).toBe(true);
    expect(readHeat(blocked)).toBe(false);
    expect(readLines(blocked)).toBe(1);
    expect(() => writeEngineOn(false, blocked)).not.toThrow();
    expect(() => writeHeat(true, blocked)).not.toThrow();
    expect(() => writeLines(3, blocked)).not.toThrow();
    expect(readEngineOn(null)).toBe(true);
    expect(() => writeLines(3, null)).not.toThrow();
  });
});
