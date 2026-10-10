/* Game files loaded in the browser to be reviewed. They live in the memory of the page only, under
   a generated id: a reload forgets them, and loading the file again makes a new one. */

export interface LoadedFile {
  name: string;
  /** The parsed JSON of the file, which the server checks when it is shown or saved. */
  record: unknown;
}

const files = new Map<string, LoadedFile>();

/** Keeps a file and returns the id its review is opened with. */
export function rememberFile(file: LoadedFile): string {
  // Time and chance, so that an id of an earlier page load is never reused.
  const id = `f${Date.now().toString(36)}${Math.random().toString(36).slice(2, 8)}`;
  files.set(id, file);
  return id;
}

/** The file kept under `id`, or null. */
export function recallFile(id: string): LoadedFile | null {
  return files.get(id) ?? null;
}
