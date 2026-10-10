export const HOME_HEADING = 'Choose a game';

/** The line under the heading of the home screen, for a given number of games. */
export function homeHint(gameCount: number): string {
  if (gameCount === 0) {
    return 'Games appear here when the server is running.';
  }
  return gameCount === 1 ? '1 game is available.' : `${gameCount} games are available.`;
}
