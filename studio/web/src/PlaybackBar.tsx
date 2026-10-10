import { SPEEDS, speedLabel } from './review';

interface Props {
  cursor: number;
  last: number;
  playing: boolean;
  speed: number;
  onGoto: (frame: number) => void;
  onToggle: () => void;
  onSpeed: (speed: number) => void;
}

/** The buttons under the board: first, previous, play or pause, next, last, and the speeds. */
export function PlaybackBar({ cursor, last, playing, speed, onGoto, onToggle, onSpeed }: Props) {
  const atStart = cursor <= 0;
  const atEnd = cursor >= last;
  return (
    <div className="playback" data-testid="playback">
      <button type="button" aria-label="First move" disabled={atStart} onClick={() => onGoto(0)}>
        ⏮
      </button>
      <button
        type="button"
        aria-label="Previous move"
        disabled={atStart}
        onClick={() => onGoto(cursor - 1)}
      >
        ◀
      </button>
      <button
        type="button"
        className="play"
        aria-label={playing ? 'Pause' : 'Play'}
        disabled={last < 1}
        onClick={onToggle}
      >
        {playing ? '❚❚' : '▶'}
      </button>
      <button
        type="button"
        aria-label="Next move"
        disabled={atEnd}
        onClick={() => onGoto(cursor + 1)}
      >
        ▶▶
      </button>
      <button type="button" aria-label="Last move" disabled={atEnd} onClick={() => onGoto(last)}>
        ⏭
      </button>
      <div className="speed" role="group" aria-label="Speed">
        {SPEEDS.map((value) => (
          <button
            key={value}
            type="button"
            aria-pressed={speed === value}
            onClick={() => onSpeed(value)}
          >
            {speedLabel(value)}
          </button>
        ))}
      </div>
    </div>
  );
}
