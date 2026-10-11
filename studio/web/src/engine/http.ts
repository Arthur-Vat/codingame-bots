/* The shape of an answer of the browser engine: what a fetch to the server would give. */

/** A response: the status and the body as text (`''` for a 204). */
export interface Reply {
  status: number;
  bodyText: string;
}

/** A failure that is the caller's answer: becomes `{"error": message}` with this status. */
export class HttpError extends Error {
  readonly status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = 'HttpError';
    this.status = status;
  }
}

export function json(status: number, value: unknown): Reply {
  return { status, bodyText: JSON.stringify(value) };
}

export function errorReply(status: number, message: string): Reply {
  return json(status, { error: message });
}

/** The text of a thrown value. */
export function messageOf(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
