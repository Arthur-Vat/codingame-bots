import { useCallback, useEffect, useEffectEvent, useState } from 'react';
import { takesSpace, typesText } from './browser';
import {
  NO_PLAYBACK,
  playbackFor,
  playbackInterval,
  playbackReducer,
  retime,
  analysisKey,
  shownCursor,
  type PlaybackAction,
} from './analysis';

export interface Playback {
  /** The frame on the board. */
  cursor: number;
  playing: boolean;
  speed: number;
  goto: (frame: number) => void;
  toggle: () => void;
  setSpeed: (speed: number) => void;
}

interface Options {
  /** Whether the page is in front: a hidden page does not play. */
  active: boolean;
  /** Whether the keys ← → Home End and Space act on the playback. */
  keys: boolean;
}

/**
 * The playback of a finished game: a cursor over its frames `0..last`, the timer that moves it,
 * and the keys. `key` names the game; a new key (including the first one, after null) starts a
 * new playback on frame `start`. With the key null nothing is played and the keys do nothing.
 */
export function usePlayback(
  last: number,
  start: number,
  key: string | null,
  { active, keys }: Options,
): Playback {
  const [held, setHeld] = useState(NO_PLAYBACK);
  // A state adjusted while rendering: it only changes when the key does.
  const current = playbackFor(held, key, start);
  if (current !== held) setHeld(current);
  const { state } = current;

  const cursor = shownCursor(state, last);
  const wanted = playbackInterval(last + 1, state.speed);
  // The delay in use changes with the speed, not with every frame of a game still going on.
  const [delay, setDelay] = useState(wanted);
  const kept = retime(delay, wanted);
  if (kept !== delay) setDelay(kept);

  const dispatch = useCallback(
    (action: PlaybackAction) =>
      setHeld((previous) => ({ ...previous, state: playbackReducer(previous.state, action) })),
    [],
  );
  const goto = (frame: number) => dispatch({ type: 'goto', frame, last });
  const toggle = () => dispatch({ type: 'toggle', last });
  const setSpeed = (speed: number) => dispatch({ type: 'speed', speed });

  const live = key !== null && active;
  const tick = useEffectEvent(() => dispatch({ type: 'tick', last }));
  // One timer for as long as the game plays at one delay: a step moves the cursor, and the one
  // that reaches the last frame stops playing. It stops when the page hides or goes away.
  useEffect(() => {
    if (!live || !state.playing || delay === null) return;
    const timer = setInterval(tick, delay);
    return () => clearInterval(timer);
  }, [live, state.playing, delay]);

  const onKey = useEffectEvent((event: KeyboardEvent) => {
    if (!keys || key === null || typesText(event.target)) return;
    if (event.altKey || event.ctrlKey || event.metaKey) return;
    const action = analysisKey(event.key, cursor, last);
    if (action === undefined) return;
    // Space on a button, a link, a checkbox or a radio button is theirs.
    if (action === 'toggle' && takesSpace(event.target)) return;
    event.preventDefault();
    if (action === 'toggle') {
      if (!event.repeat) toggle();
    } else {
      goto(action);
    }
  });
  useEffect(() => {
    const down = (event: KeyboardEvent) => onKey(event);
    window.addEventListener('keydown', down);
    return () => window.removeEventListener('keydown', down);
  }, []);

  return { cursor, playing: state.playing, speed: state.speed, goto, toggle, setSpeed };
}
