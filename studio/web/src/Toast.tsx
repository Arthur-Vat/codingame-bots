import { useCallback, useEffect, useState } from 'react';

/** How long a toast stays. */
export const TOAST_MS = 2600;

/** A short confirmation at the bottom of the window. Pass the text, or null for none. */
export function Toast({ text }: { text: string | null }) {
  if (text === null) return null;
  return (
    <div className="toast" role="status">
      {text}
    </div>
  );
}

/** The text of a toast and a function that shows one (replacing the one on screen). */
export function useToast(): { text: string | null; show: (text: string) => void } {
  const [held, setHeld] = useState<{ text: string; n: number } | null>(null);

  useEffect(() => {
    if (held === null) return;
    const timer = setTimeout(() => setHeld(null), TOAST_MS);
    return () => clearTimeout(timer);
  }, [held]);

  const show = useCallback(
    (text: string) => setHeld((previous) => ({ text, n: (previous?.n ?? 0) + 1 })),
    [],
  );
  return { text: held?.text ?? null, show };
}
