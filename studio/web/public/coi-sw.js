// A service worker that makes the page cross-origin isolated on a host that cannot set headers
// (GitHub Pages): it answers each same-origin request with the two headers that isolation needs.
// The browser engine needs isolation for SharedArrayBuffer, which lets a bot wait for its input
// (studio/README.md, "Isolation"; ADR 0030). Nothing is cached and nothing leaves the page.

self.addEventListener('install', () => self.skipWaiting());
self.addEventListener('activate', (event) => event.waitUntil(self.clients.claim()));

self.addEventListener('fetch', (event) => {
  const request = event.request;
  // Chrome fails a request that is only-if-cached and not same-origin; leave it alone.
  if (request.cache === 'only-if-cached' && request.mode !== 'same-origin') return;
  // Only same-origin requests get the headers: the page loads nothing from elsewhere.
  if (new URL(request.url).origin !== self.location.origin) return;
  event.respondWith(
    fetch(request).then((response) => {
      if (response.status === 0) return response;
      const headers = new Headers(response.headers);
      headers.set('Cross-Origin-Opener-Policy', 'same-origin');
      headers.set('Cross-Origin-Embedder-Policy', 'require-corp');
      return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers,
      });
    }),
  );
});
