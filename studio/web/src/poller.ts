/** What a poller needs to know. */
export interface PollerOptions<T> {
  /** One request. */
  fetch: () => Promise<T>;
  /** Milliseconds to wait after a value before the next request; null stops polling. */
  delayAfter: (value: T) => number | null;
  /** Milliseconds to wait after a failure before the next request; null stops polling. */
  delayAfterError: (error: unknown) => number | null;
  onValue: (value: T) => void;
  onError: (error: unknown) => void;
}

/**
 * Calls `fetch` over and over: the next request starts only after the previous one ended and
 * its delay passed, so requests never overlap. After the returned function is called, nothing
 * more is requested and a response still on its way is dropped (neither callback runs).
 */
export function startPoller<T>(options: PollerOptions<T>): () => void {
  let stopped = false;
  let timer: ReturnType<typeof setTimeout> | undefined;

  const next = (delay: number | null): void => {
    if (delay !== null && !stopped) {
      timer = setTimeout(() => void poll(), delay);
    }
  };

  const poll = async (): Promise<void> => {
    let delay: number | null;
    try {
      const value = await options.fetch();
      if (stopped) return;
      options.onValue(value);
      delay = options.delayAfter(value);
    } catch (error) {
      if (stopped) return;
      options.onError(error);
      delay = options.delayAfterError(error);
    }
    next(delay);
  };

  void poll();
  return () => {
    stopped = true;
    clearTimeout(timer);
  };
}
