export const HOME_HEADING = 'Choose a game';

/** What to tell when the server does not answer. */
export const SERVER_HINT = 'Start the server: cargo run --release -p studio';

/** "10 releases", "1 release". */
export function releaseCount(count: number): string {
  return count === 1 ? '1 release' : `${count} releases`;
}

/**
 * What a game's card says of its bots: its releases; for a game with baseline bots but no
 * release, that, so that the card does not read "0 releases" of a game that can be played.
 */
export function botSummary(releases: number, baselines: number): string {
  if (releases > 0 || baselines === 0) return releaseCount(releases);
  const bots = baselines === 1 ? '1 baseline bot' : `${baselines} baseline bots`;
  return `No releases · ${bots}`;
}
