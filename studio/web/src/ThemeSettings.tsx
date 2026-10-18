import { setGameTheme, useGameTheme, type ThemeKind } from './gameThemes';
import { themesFor } from './games/themeChoices';

const KINDS: { kind: ThemeKind; label: string }[] = [
  { kind: 'pieces', label: 'Pieces' },
  { kind: 'board', label: 'Board' },
];

/**
 * The theme choices of a game, two selects in Board settings (pieces, board); nothing for a game
 * that offers none. The choice is remembered in the browser and changes every board of the page.
 */
export default function ThemeSettings({ gameId }: { gameId: string }) {
  const chosen = useGameTheme(gameId);
  const offered = themesFor(gameId);
  if (offered === undefined) return null;
  return (
    <>
      {KINDS.map(({ kind, label }) => (
        <label key={kind} className="switch theme-pick">
          {label}
          <select
            aria-label={label}
            value={chosen[kind]}
            onChange={(event) => setGameTheme(gameId, kind, event.target.value)}
          >
            {offered[kind].options.map((option) => (
              <option key={option.id} value={option.id}>
                {option.label}
              </option>
            ))}
          </select>
        </label>
      ))}
    </>
  );
}
