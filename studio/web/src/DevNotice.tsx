/** Development only: no studio server answered, so the page uses the browser engine. */
export function DevNotice() {
  return (
    <p className="dev-notice" role="status">
      No studio server answered on port 8411, so this page uses the browser engine. Start the server
      with <code>cargo run --release -p studio</code> and reload, or set{' '}
      <code>localStorage[&apos;studio.engine&apos;]</code> to <code>&apos;browser&apos;</code> to
      use the browser engine on purpose.
    </p>
  );
}
