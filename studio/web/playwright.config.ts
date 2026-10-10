import { defineConfig, devices } from '@playwright/test';

// A fixed port, away from Vite's dev server (5173).
const PORT = 4173;

export default defineConfig({
  testDir: 'e2e',
  // No retries: a flaky test must fail, not hide.
  retries: 0,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? 'list' : [['html', { open: 'never' }]],
  use: {
    baseURL: `http://127.0.0.1:${PORT}`,
    trace: 'retain-on-failure',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: {
    // Serves the production build, made by `npm run build`.
    command: `npm run build && npm run preview -- --host 127.0.0.1 --port ${PORT} --strictPort`,
    url: `http://127.0.0.1:${PORT}`,
    reuseExistingServer: false,
    timeout: 120_000,
  },
});
