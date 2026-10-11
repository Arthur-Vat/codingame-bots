/* What happens before the app renders: the engine is chosen and, for the browser engine, the page
   is made cross-origin isolated if it can be (which may reload it once). */

import { chooseEngine, type EngineKind } from './detect';
import { ensureIsolation, pageEnvironment } from './isolation';

export async function startEngine(): Promise<EngineKind> {
  const kind = await chooseEngine();
  if (kind === 'browser') {
    // On false the app still runs; what needs a bot fails with a clear message.
    await ensureIsolation(pageEnvironment());
  }
  return kind;
}
