export const HOME_HEADING = 'Choose a game';

export const HOME_INTRO =
  'Each game the framework supports gets a board here. Pick one to play a friend, play a release, or watch two releases fight.';

/** What to tell when the server does not answer. */
export const SERVER_HINT = 'Start the server: cargo run --release -p studio';

/** "10 releases", "1 release". */
export function releaseCount(count: number): string {
  return count === 1 ? '1 release' : `${count} releases`;
}
