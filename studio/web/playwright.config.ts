import { defineConfig, devices } from '@playwright/test';

// A fixed port, away from Vite's dev server (5173).
const PORT = 4173;
const ORIGIN = `http://127.0.0.1:${PORT}`;

export default defineConfig({
  testDir: 'e2e',
  // No retries: a flaky test must fail, not hide.
  retries: 0,
  forbidOnly: !!process.env.CI,
  // In CI, the github reporter turns each failure into an annotation, readable
  // without the job's log.
  reporter: process.env.CI ? [['list'], ['github']] : [['html', { open: 'never' }]],
  use: {
    baseURL: ORIGIN,
    trace: 'retain-on-failure',
  },
  projects: [
    {
      // The pages against a mocked API: the front end must use the server's engine, which these
      // tests mock. `preview` has no /api, so left alone the page would choose the browser engine
      // (src/engine/detect.ts); the key forces the server's.
      name: 'chromium',
      testIgnore: 'browser-engine.spec.ts',
      use: {
        ...devices['Desktop Chrome'],
        storageState: {
          cookies: [],
          origins: [{ origin: ORIGIN, localStorage: [{ name: 'studio.engine', value: 'server' }] }],
        },
      },
    },
    {
      // The browser engine, as GitHub Pages serves it: static files, no API, no mocks.
      name: 'browser-engine',
      testMatch: 'browser-engine.spec.ts',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    // Serves the production build, as `npm run build` makes it, with the WebAssembly files the
    // browser engine loads (`npm run wasm`; `npm run build` itself does not need Rust).
    command: `npm run wasm && npm run build && npm run preview -- --host 127.0.0.1 --port ${PORT} --strictPort`,
    url: ORIGIN,
    reuseExistingServer: false,
    // Compiling the rules and every release to WebAssembly comes first.
    timeout: 300_000,
  },
});
