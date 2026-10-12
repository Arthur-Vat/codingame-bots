import { describe, expect, it } from 'vitest';
import { requestIsLive, type AfterRequest } from './afterRequest';
import { parseRoute } from './route';

const request: AfterRequest = { gameId: 'uttt', sessionId: 'ab12', action: 'rematch' };
const live = (hash: string) => requestIsLive(request, parseRoute(hash));

describe('requestIsLive', () => {
  it('lives on the analysis board of its session and in the Play tab showing it', () => {
    expect(live('#/game/uttt/analysis/session/ab12')).toBe(true);
    expect(live('#/game/uttt/ab12')).toBe(true);
  });

  it('is stale in the Play tab with another session or none', () => {
    expect(live('#/game/uttt/cd34')).toBe(false);
    expect(live('#/game/uttt')).toBe(false);
  });

  it('is stale anywhere else', () => {
    expect(live('#/game/uttt/analysis/session/cd34')).toBe(false);
    expect(live('#/game/uttt/analysis/saved/ab12')).toBe(false);
    expect(live('#/game/uttt/history')).toBe(false);
    expect(live('#/game/other/ab12')).toBe(false);
    expect(live('#/')).toBe(false);
  });
});
