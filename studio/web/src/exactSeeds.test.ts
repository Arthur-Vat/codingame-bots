import { describe, expect, it } from 'vitest';
import { parseKeepingSeeds } from './exactSeeds';

describe('parseKeepingSeeds', () => {
  it('keeps a seed beyond 2^53 as its digits, whatever its place', () => {
    const parsed = parseKeepingSeeds(
      '{"seed":18446744073709551615,"players":[{"bot_seed":9007199254740993},{"bot_seed":9007199254740992}]}',
    );
    expect(parsed).toEqual({
      seed: '18446744073709551615',
      players: [{ bot_seed: '9007199254740993' }, { bot_seed: '9007199254740992' }],
    });
  });

  it('leaves a seed that a number holds exactly as a number', () => {
    expect(
      parseKeepingSeeds('{"seed":0,"x":[{"bot_seed":null},{"bot_seed":1760000000000}]}'),
    ).toEqual({ seed: 0, x: [{ bot_seed: null }, { bot_seed: 1760000000000 }] });
    expect(parseKeepingSeeds('{"seed":9007199254740991}')).toEqual({ seed: 9007199254740991 });
  });

  it('leaves every other number as JSON.parse reads it', () => {
    const text = '{"a":9007199254740993,"ms":-1.5,"turns":[[12345678901234567890]],"seed":-2.5e3}';
    expect(parseKeepingSeeds(text)).toEqual(JSON.parse(text));
  });

  it('refuses what is not JSON, as JSON.parse does', () => {
    expect(() => parseKeepingSeeds('{')).toThrow(SyntaxError);
  });
});
