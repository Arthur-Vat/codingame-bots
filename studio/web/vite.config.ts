import react from '@vitejs/plugin-react';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [react()],
  // Relative URLs, so that the build works from any folder: the site of GitHub Pages is under
  // a subpath such as /codingame-bots/.
  base: './',
  server: {
    // In development the API is the local studio server's (cargo run --release -p studio).
    proxy: { '/api': 'http://127.0.0.1:8411' },
    // The browser engine needs a cross-origin isolated page (SharedArrayBuffer). The dev server
    // sets the two headers itself.
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
  // `preview` inherits the server's options, headers and proxy included. It must have neither: it
  // behaves like GitHub Pages, which cannot set headers (public/coi-sw.js does it there) and has
  // no API. The browser tests rely on that.
  preview: { headers: {}, proxy: {} },
  // Module workers (the bots), as ES modules.
  worker: { format: 'es' },
  test: {
    include: ['src/**/*.test.ts', 'scripts/**/*.test.mjs'],
  },
});
