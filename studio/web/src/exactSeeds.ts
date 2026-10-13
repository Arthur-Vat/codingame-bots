/* A seed of a game record is an unsigned 64-bit number, and `JSON.parse` rounds a number beyond
   2^53 (9007199254740993 becomes 9007199254740992). A record that goes to a file, to the history
   or to the rules is therefore kept and sent as text, never parsed and written again. This module
   is for the one place that does read the seeds, to show them. */

/** The names of a record's seeds: the game's, and each player's bot seed. */
const SEED_KEYS: ReadonlySet<string> = new Set(['seed', 'bot_seed']);

/** What a `JSON.parse` reviver is given as its third argument, where the engine supports it. */
interface ReviverContext {
  source?: string;
}

/**
 * Parses JSON, except that a seed (`seed` or `bot_seed`) that is a whole number beyond what a
 * JavaScript number holds exactly is kept as the string of its digits. The result is for showing,
 * never for sending back: a seed that became a string is not a number to the server. Where the
 * engine does not give the source text of a number (`context`), the seed stays a rounded number.
 */
export function parseKeepingSeeds(text: string): unknown {
  return JSON.parse(text, (key: string, value: unknown, context?: ReviverContext) => {
    if (
      SEED_KEYS.has(key) &&
      typeof value === 'number' &&
      !Number.isSafeInteger(value) &&
      context?.source !== undefined &&
      /^-?\d+$/.test(context.source)
    ) {
      return context.source;
    }
    return value;
  });
}
