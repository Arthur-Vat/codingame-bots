/* The analysis board's remembered settings, in localStorage. Every access may fail (blocked
   storage, a full disk): a failed read gives the default, a failed write is forgotten. */

export const KEY_ENGINE_ON = 'studio.analysis.on';
export const KEY_HEAT = 'studio.analysis.heat';
export const KEY_LINES = 'studio.analysis.lines';

/** The part of `Storage` the settings use. */
export interface SettingsStorage {
  getItem: (key: string) => string | null;
  setItem: (key: string, value: string) => void;
}

function browserStorage(): SettingsStorage | null {
  try {
    return localStorage;
  } catch {
    return null;
  }
}

/** The engine is on unless the user turned it off. */
export const DEFAULT_ENGINE_ON = true;
/** The heat map is off unless the user turned it on. */
export const DEFAULT_HEAT = false;
/** One line, extendable to three. */
export const LINE_COUNTS = [1, 3] as const;
export type LineCount = (typeof LINE_COUNTS)[number];
export const DEFAULT_LINES: LineCount = 1;

function read(key: string, storage: SettingsStorage | null): string | null {
  try {
    return storage?.getItem(key) ?? null;
  } catch {
    return null;
  }
}

function write(key: string, value: string, storage: SettingsStorage | null): void {
  try {
    storage?.setItem(key, value);
  } catch {
    // Not remembered; the setting holds until the page is left.
  }
}

function readFlag(
  key: string,
  fallback: boolean,
  storage: SettingsStorage | null = browserStorage(),
): boolean {
  const value = read(key, storage);
  return value === '1' ? true : value === '0' ? false : fallback;
}

export const readEngineOn = (storage?: SettingsStorage | null): boolean =>
  readFlag(KEY_ENGINE_ON, DEFAULT_ENGINE_ON, storage);
export const readHeat = (storage?: SettingsStorage | null): boolean =>
  readFlag(KEY_HEAT, DEFAULT_HEAT, storage);

export function readLines(storage: SettingsStorage | null = browserStorage()): LineCount {
  const value = read(KEY_LINES, storage);
  return value === '3' ? 3 : value === '1' ? 1 : DEFAULT_LINES;
}

export function writeEngineOn(on: boolean, storage: SettingsStorage | null = browserStorage()) {
  write(KEY_ENGINE_ON, on ? '1' : '0', storage);
}

export function writeHeat(on: boolean, storage: SettingsStorage | null = browserStorage()) {
  write(KEY_HEAT, on ? '1' : '0', storage);
}

export function writeLines(count: LineCount, storage: SettingsStorage | null = browserStorage()) {
  write(KEY_LINES, String(count), storage);
}
