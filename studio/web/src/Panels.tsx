import type { ReactNode } from 'react';
import { useReplaceHash } from './hooks';

/** A message in the page body: a game loading, or why it cannot be shown. */
export function StatusPanel({
  text,
  alert = false,
  children,
}: {
  text: string;
  alert?: boolean;
  /** More under the text, such as a link back. */
  children?: ReactNode;
}) {
  return (
    <main className="wrap">
      <section className="panel empty-page">
        <p role={alert ? 'alert' : 'status'}>{text}</p>
        {children}
      </section>
    </main>
  );
}

/** Goes to another hash, replacing this one in the history, and says so meanwhile. */
export function Redirect({ hash, text = 'Loading…' }: { hash: string; text?: string }) {
  useReplaceHash(hash);
  return <StatusPanel text={text} />;
}
