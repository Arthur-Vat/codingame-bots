import { noServerParts, noServerText } from './engine/detect';

/**
 * Development only: no studio server answered, so the page uses the browser engine. One line, cut
 * with an ellipsis when the window is narrow (the whole text is its tooltip and in the console).
 */
export function DevNotice() {
  return (
    <p className="dev-notice" role="status" title={noServerText()}>
      {noServerParts().map((part, index) =>
        part.code ? <code key={index}>{part.text}</code> : part.text,
      )}
    </p>
  );
}
