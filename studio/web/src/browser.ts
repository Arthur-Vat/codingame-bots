/* Small helpers of the page that the game and review screens share. */

import { isTextEntry } from './play';

/** Whether a key press belongs to a field that takes text or a choice, not to the board. */
export function typesText(target: EventTarget | null): boolean {
  return (
    target instanceof HTMLElement &&
    isTextEntry({
      tagName: target.tagName,
      type: target instanceof HTMLInputElement ? target.type : undefined,
      isContentEditable: target.isContentEditable,
    })
  );
}

export function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

/** Downloads `data` as a JSON file. */
export function download(name: string, data: unknown): void {
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = name;
  document.body.appendChild(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
