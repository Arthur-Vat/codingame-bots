import { useCallback, useEffect, useId, useRef, useState } from 'react';
import { SeatMark } from './Marks';
import {
  DEFAULT_INCREMENT_INDEX,
  DEFAULT_MINUTES_INDEX,
  DEFAULT_THINK_INDEX,
  INCREMENTS,
  MINUTES,
  MODE_TITLES,
  THINK_MS,
  clockSeed,
  defaultRelease,
  defaultSecondRelease,
  minutesLabel,
  orderReleases,
  releaseLabel,
  resolveSeed,
  thinkLabel,
  type Mode,
  type SetupForm,
  type Side,
} from './setup';

const SIDE_CAPTIONS: Record<Side, string> = {
  x: 'You play X and move first',
  random: 'Random side',
  o: 'You play O; the bot moves first',
};

/** What the dialog hands back when Start is pressed. */
export interface SetupResult {
  form: SetupForm;
  seed: number;
}

interface Props {
  mode: Mode;
  /** The release names of the game. */
  releases: readonly string[];
  /** Starts the game; a rejected promise shows its message in the dialog. */
  onStart: (result: SetupResult) => Promise<void>;
  onClose: () => void;
}

function ReleaseSelect({
  id,
  value,
  releases,
  onChange,
}: {
  id: string;
  value: string;
  releases: readonly string[];
  onChange: (value: string) => void;
}) {
  const latest = defaultRelease(releases);
  return (
    <select id={id} value={value} onChange={(event) => onChange(event.target.value)}>
      {orderReleases(releases).map((name) => (
        <option key={name} value={name}>
          {releaseLabel(name, latest)}
        </option>
      ))}
    </select>
  );
}

function ThinkSlider({
  id,
  label,
  index,
  onChange,
}: {
  id: string;
  label: string;
  index: number;
  onChange: (index: number) => void;
}) {
  return (
    <>
      <label htmlFor={id}>
        {label} <output htmlFor={id}>{thinkLabel(THINK_MS[index] ?? 100)}</output>
      </label>
      <input
        type="range"
        id={id}
        min={0}
        max={THINK_MS.length - 1}
        value={index}
        onChange={(event) => onChange(Number(event.target.value))}
      />
    </>
  );
}

/** The modal that sets a game up. It is mounted for one opening, so its fields start afresh. */
export function SetupDialog({ mode, releases, onStart, onClose }: Props) {
  const ids = useId();
  const id = (name: string) => `${ids}-${name}`;
  const startRef = useRef<HTMLButtonElement>(null);

  const [release, setRelease] = useState(() => defaultRelease(releases));
  const [thinkIndex, setThinkIndex] = useState(DEFAULT_THINK_INDEX);
  const [xRelease, setXRelease] = useState(() => defaultRelease(releases));
  const [oRelease, setORelease] = useState(() => defaultSecondRelease(releases));
  const [xThinkIndex, setXThinkIndex] = useState(DEFAULT_THINK_INDEX);
  const [oThinkIndex, setOThinkIndex] = useState(DEFAULT_THINK_INDEX);
  const [minutesIndex, setMinutesIndex] = useState(DEFAULT_MINUTES_INDEX);
  const [incrementIndex, setIncrementIndex] = useState(DEFAULT_INCREMENT_INDEX);
  const [side, setSide] = useState<Side>('x');
  const [advanced, setAdvanced] = useState(false);
  const [seedText, setSeedText] = useState(() => String(clockSeed(Date.now())));
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    startRef.current?.focus();
  }, []);

  // While the game is being created the dialog stays: closing it would hide an answer.
  const close = useCallback(() => {
    if (!busy) onClose();
  }, [busy, onClose]);

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === 'Escape') close();
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [close]);

  const minutes = MINUTES[minutesIndex] ?? 10;
  const incrementSeconds = INCREMENTS[incrementIndex] ?? 0;
  const think = (index: number) => THINK_MS[index] ?? 100;

  const start = async () => {
    let form: SetupForm;
    if (mode === 'friend') {
      form = { mode, minutes, incrementSeconds };
    } else if (mode === 'computer') {
      form = { mode, release, thinkMs: think(thinkIndex), side, minutes, incrementSeconds };
    } else {
      form = {
        mode,
        xRelease,
        oRelease,
        xThinkMs: think(xThinkIndex),
        oThinkMs: think(oThinkIndex),
      };
    }
    setBusy(true);
    setError(null);
    try {
      await onStart({ form, seed: resolveSeed(advanced, seedText, Date.now()) });
    } catch (failure) {
      setError(failure instanceof Error ? failure.message : String(failure));
      setBusy(false);
    }
  };

  const title = MODE_TITLES[mode];
  const timed = mode === 'friend' || mode === 'computer';

  return (
    <div
      className="modal"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) close();
      }}
    >
      <div className="dialog" role="dialog" aria-modal="true" aria-labelledby={id('title')}>
        <header>
          <h2 id={id('title')}>{title}</h2>
          <button className="x-btn" type="button" aria-label="Close" onClick={close}>
            ×
          </button>
        </header>
        <div className="body">
          {mode === 'computer' && (
            <>
              <div className="field">
                <label htmlFor={id('release')}>Release</label>
                <ReleaseSelect
                  id={id('release')}
                  value={release}
                  releases={releases}
                  onChange={setRelease}
                />
              </div>
              <div className="field">
                <ThinkSlider
                  id={id('think')}
                  label="Think time"
                  index={thinkIndex}
                  onChange={setThinkIndex}
                />
                <p className="hint">Fixed iterations, so takebacks replay exactly.</p>
              </div>
            </>
          )}
          {mode === 'bots' && (
            <div className="bots-grid">
              <div className="field">
                <label htmlFor={id('x-release')}>X (moves first)</label>
                <ReleaseSelect
                  id={id('x-release')}
                  value={xRelease}
                  releases={releases}
                  onChange={setXRelease}
                />
              </div>
              <div className="field">
                <label htmlFor={id('o-release')}>O</label>
                <ReleaseSelect
                  id={id('o-release')}
                  value={oRelease}
                  releases={releases}
                  onChange={setORelease}
                />
              </div>
              <div className="field">
                <ThinkSlider
                  id={id('x-think')}
                  label="X think"
                  index={xThinkIndex}
                  onChange={setXThinkIndex}
                />
              </div>
              <div className="field">
                <ThinkSlider
                  id={id('o-think')}
                  label="O think"
                  index={oThinkIndex}
                  onChange={setOThinkIndex}
                />
              </div>
            </div>
          )}
          {timed && (
            <>
              <div className="field">
                <label htmlFor={id('minutes')}>
                  {mode === 'computer' ? 'Your minutes' : 'Minutes per side'}{' '}
                  <output htmlFor={id('minutes')}>{minutesLabel(minutes)}</output>
                </label>
                <input
                  type="range"
                  id={id('minutes')}
                  min={0}
                  max={MINUTES.length - 1}
                  value={minutesIndex}
                  onChange={(event) => setMinutesIndex(Number(event.target.value))}
                />
              </div>
              <div className="field">
                <label htmlFor={id('increment')}>
                  Increment in seconds <output htmlFor={id('increment')}>{incrementSeconds}</output>
                </label>
                <input
                  type="range"
                  id={id('increment')}
                  min={0}
                  max={INCREMENTS.length - 1}
                  value={incrementIndex}
                  onChange={(event) => setIncrementIndex(Number(event.target.value))}
                />
              </div>
            </>
          )}
          {mode === 'computer' && (
            <div className="field">
              <span className="flabel">Your side</span>
              <div className="sides" role="group" aria-label="Your side">
                <button
                  type="button"
                  aria-pressed={side === 'x'}
                  aria-label="Play X"
                  onClick={() => setSide('x')}
                >
                  <SeatMark seat={0} />
                </button>
                <button
                  type="button"
                  className="rnd"
                  aria-pressed={side === 'random'}
                  aria-label="Random side"
                  onClick={() => setSide('random')}
                >
                  ?
                </button>
                <button
                  type="button"
                  aria-pressed={side === 'o'}
                  aria-label="Play O"
                  onClick={() => setSide('o')}
                >
                  <SeatMark seat={1} />
                </button>
              </div>
              <div className="side-cap">{SIDE_CAPTIONS[side]}</div>
            </div>
          )}
          <label className="switch" htmlFor={id('advanced')}>
            Advanced mode
            <input
              type="checkbox"
              id={id('advanced')}
              checked={advanced}
              onChange={(event) => setAdvanced(event.target.checked)}
            />
          </label>
          {advanced && (
            <div className="adv">
              <div className="field">
                <label htmlFor={id('seed')}>Seed</label>
                <input
                  type="text"
                  id={id('seed')}
                  inputMode="numeric"
                  value={seedText}
                  onChange={(event) => setSeedText(event.target.value)}
                />
              </div>
              <p className="hint">Otherwise taken from the clock.</p>
            </div>
          )}
          {error !== null && (
            <p className="form-error" role="alert">
              {error}
            </p>
          )}
        </div>
        <footer>
          <button
            ref={startRef}
            className="btn primary"
            type="button"
            disabled={busy}
            onClick={() => void start()}
          >
            {busy ? 'Starting…' : 'Start'}
          </button>
        </footer>
      </div>
    </div>
  );
}
