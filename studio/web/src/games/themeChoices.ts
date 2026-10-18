import { BOARD_THEMES, DEFAULT_BOARD, DEFAULT_PIECES, PIECE_THEMES } from './ice-and-fire/themes';

/* The theme choices a game offers for its pieces and its board, shown in Board settings and
   remembered in the browser (`gameThemes.ts`). They live apart from the registry, which
   re-exports them, so that a renderer can read its choice without importing the registry that
   imports the renderer. */

/** The options of one choice (their ids and labels) and the id used until the user picks one. */
export interface ThemeChoice {
  options: { id: string; label: string }[];
  default: string;
}

/** What a game offers: its pieces and its board. */
export interface GameThemes {
  pieces: ThemeChoice;
  board: ThemeChoice;
}

/** A game's choices, by its id (a map without prototype, as the registry's). */
export const themes: Record<string, GameThemes> = Object.assign(
  Object.create(null) as Record<string, GameThemes>,
  {
    'ice-and-fire': {
      pieces: {
        options: PIECE_THEMES.map(({ id, label }) => ({ id, label })),
        default: DEFAULT_PIECES,
      },
      board: {
        options: BOARD_THEMES.map(({ id, label }) => ({ id, label })),
        default: DEFAULT_BOARD,
      },
    },
  } satisfies Record<string, GameThemes>,
);

/** The theme choices of a game, or undefined for a game without any. */
export function themesFor(gameId: string): GameThemes | undefined {
  return Object.hasOwn(themes, gameId) ? themes[gameId] : undefined;
}
