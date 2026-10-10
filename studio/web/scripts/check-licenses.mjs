// Fails when a package of package-lock.json has no license, or one that
// licenses.json does not allow (ADR 0026). Run from studio/web: npm run licenses.
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { isLicenseAllowed } from './licenses.mjs';

const root = fileURLToPath(new URL('..', import.meta.url));
const read = (name) => JSON.parse(readFileSync(root + name, 'utf8'));

const { allowed } = read('licenses.json');
const { packages } = read('package-lock.json');

const problems = [];
let checked = 0;
for (const [path, entry] of Object.entries(packages)) {
  if (path === '') continue;
  checked += 1;
  if (!isLicenseAllowed(entry.license, allowed)) {
    // The key is node_modules/a/node_modules/@scope/b: the name is after the last one.
    const name =
      entry.name ?? path.slice(path.lastIndexOf('node_modules/') + 'node_modules/'.length);
    const license =
      entry.license === undefined
        ? 'no license'
        : JSON.stringify(entry.license).replace(/^"|"$/g, '');
    problems.push(`${name}@${entry.version}: ${license}`);
  }
}

if (problems.length > 0) {
  console.error(`${problems.length} of ${checked} packages have a missing or disallowed license:`);
  for (const problem of problems) console.error(`  ${problem}`);
  console.error(`Allowed: ${allowed.join(', ')} (studio/web/licenses.json)`);
  process.exit(1);
}
console.log(`licenses: ok, ${checked} packages`);
