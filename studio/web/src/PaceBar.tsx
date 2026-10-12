import { PACES, paceLabel, type Pace } from './pace';

interface Props {
  pace: Pace;
  onPace: (pace: Pace) => void;
}

/** Under the board while two bots play: how fast their moves are shown. */
export function PaceBar({ pace, onPace }: Props) {
  return (
    <div className="playback" role="group" aria-label="Pace" data-testid="pace">
      <span className="pace-label">Pace</span>
      <div className="speed">
        {PACES.map((value) => (
          <button
            key={value ?? 'max'}
            type="button"
            aria-pressed={pace === value}
            onClick={() => onPace(value)}
          >
            {paceLabel(value)}
          </button>
        ))}
      </div>
    </div>
  );
}
