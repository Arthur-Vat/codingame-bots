// Writes THIRD-PARTY-NOTICES.txt: the license notices of everything the published site bundles
// (ADR 0030, decision 7).
//
//   node scripts/notices.mjs [folder]      (default: dist)
//
// It gathers, from local files only:
// - the npm packages whose code is in the production bundle: the production dependencies of
//   package.json and, from package-lock.json, their transitive ones (no devDependencies);
// - the Rust crates in the WebAssembly files: the dependency graph of the package studio-rules for
//   the target wasm32-wasip1, from `cargo metadata`, without this workspace's own crates;
// - the Rust standard library and wasi-libc (which the target links in), compiled into every
//   WebAssembly file, from the toolchain's share/doc/rust: its COPYRIGHT-library.html is copied
//   unchanged beside the notices as rust-library-copyright.html.
//
// It fails, naming the package, rather than writing an incomplete file. Run it from studio/web
// after `npm ci` and `npm run build` (the folder must be the built site), with Rust installed.
import { execFileSync } from 'node:child_process';
import {
  copyFileSync,
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
  writeFileSync,
} from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

export const OUTPUT_NAME = 'THIRD-PARTY-NOTICES.txt';
export const REPOSITORY_URL = 'https://github.com/Arthur-Vat/codingame-bots';
export const WASM_TARGET = 'wasm32-wasip1';
export const RULES_PACKAGE = 'studio-rules';
/** The Rust project's notice for the standard library, copied unchanged beside the notices. */
export const LIBRARY_COPYRIGHT_NAME = 'rust-library-copyright.html';

/** Whether a file name is a license file: LICENSE*, LICENCE*, COPYING*, NOTICE*, any case. */
export function isLicenseFileName(name) {
  return /^(licen[sc]e|copying|notice)(?![a-z0-9])/i.test(name);
}

/** The license files directly inside a folder, sorted by name. Directories are not files. */
export function findLicenseFiles(folder) {
  return readdirSync(folder)
    .filter((name) => isLicenseFileName(name) && statSync(join(folder, name)).isFile())
    .sort(compareText);
}

/** Plain code-unit order, so that the output does not depend on the machine's locale. */
export function compareText(a, b) {
  return a < b ? -1 : a > b ? 1 : 0;
}

function compareEntries(a, b) {
  return compareText(a.name, b.name) || compareText(a.version, b.version);
}

/** The name of the package at a lockfile path such as `node_modules/a/node_modules/@s/b`. */
export function nameFromLockPath(path) {
  const marker = 'node_modules/';
  return path.slice(path.lastIndexOf(marker) + marker.length);
}

/**
 * The lockfile path a dependency resolves to, as Node does: the closest `node_modules/<name>`
 * going up from the package that asks, or null if none is installed.
 */
export function resolveLockDependency(packages, fromPath, name) {
  let base = fromPath;
  for (;;) {
    const candidate = base === '' ? `node_modules/${name}` : `${base}/node_modules/${name}`;
    if (candidate in packages) return candidate;
    if (base === '') return null;
    const cut = base.lastIndexOf('/node_modules/');
    base = cut === -1 ? '' : base.slice(0, cut);
  }
}

/**
 * The production closure of a lockfile (version 2 or 3): every package reachable from the root's
 * `dependencies`, through `dependencies`, `optionalDependencies` and `peerDependencies`. An
 * optional or peer dependency that is not installed is skipped; a missing required one is an
 * error. Returns `[{ name, version, path, license }]`, sorted by name then version.
 */
/**
 * Build tools whose own code the production bundle contains although they are devDependencies:
 * Vite adds its module preload helpers to the page's JavaScript.
 */
export const BUNDLED_TOOLS = ['vite'];

/** The lockfile entries of `BUNDLED_TOOLS`, as `productionClosure` gives packages. */
export function bundledTools(lock) {
  return BUNDLED_TOOLS.map((name) => {
    const path = `node_modules/${name}`;
    const entry = lock.packages?.[path];
    if (entry === undefined) throw new Error(`${name} is not in package-lock.json`);
    return { name, version: entry.version, path, license: entry.license ?? null };
  });
}

export function productionClosure(lock) {
  const packages = lock.packages ?? {};
  const root = packages[''];
  if (!root) throw new Error('package-lock.json has no root package ("packages[\\"\\"]")');
  const seen = new Set();
  const queue = [];
  const visit = (fromPath, name, required) => {
    const path = resolveLockDependency(packages, fromPath, name);
    if (path === null) {
      if (required) {
        throw new Error(
          `package-lock.json: ${name} (needed by ${fromPath || 'the root'}) is missing`,
        );
      }
      return;
    }
    if (!seen.has(path)) {
      seen.add(path);
      queue.push(path);
    }
  };
  for (const name of Object.keys(root.dependencies ?? {})) visit('', name, true);
  while (queue.length > 0) {
    const path = queue.pop();
    const entry = packages[path];
    for (const name of Object.keys(entry.dependencies ?? {})) visit(path, name, true);
    for (const name of Object.keys(entry.optionalDependencies ?? {})) visit(path, name, false);
    for (const name of Object.keys(entry.peerDependencies ?? {})) visit(path, name, false);
  }
  return [...seen]
    .map((path) => ({
      name: packages[path].name ?? nameFromLockPath(path),
      version: packages[path].version,
      path,
      license: packages[path].license,
    }))
    .sort(compareEntries);
}

/**
 * The crates of the dependency graph of the package named `rootName` in `cargo metadata` output,
 * following normal dependencies only (no dev or build dependencies), without the workspace's own
 * crates (which are still walked through). Returns `[{ name, version, license, licenseFile,
 * manifestPath }]`, sorted by name then version.
 */
export function rustClosure(metadata, rootName) {
  const byId = new Map(metadata.packages.map((pkg) => [pkg.id, pkg]));
  const nodes = new Map(metadata.resolve.nodes.map((node) => [node.id, node]));
  const members = new Set(metadata.workspace_members);
  const root = metadata.packages.find((pkg) => pkg.name === rootName && members.has(pkg.id));
  if (!root) throw new Error(`cargo metadata: no workspace package named ${rootName}`);
  const seen = new Set([root.id]);
  const queue = [root.id];
  while (queue.length > 0) {
    const node = nodes.get(queue.pop());
    if (!node) continue;
    for (const dep of node.deps) {
      if (!dep.dep_kinds.some((kind) => kind.kind === null)) continue;
      if (seen.has(dep.pkg)) continue;
      seen.add(dep.pkg);
      queue.push(dep.pkg);
    }
  }
  return [...seen]
    .filter((id) => !members.has(id))
    .map((id) => byId.get(id))
    .map((pkg) => ({
      name: pkg.name,
      version: pkg.version,
      license: pkg.license ?? null,
      licenseFile: pkg.license_file ?? null,
      manifestPath: pkg.manifest_path,
    }))
    .sort(compareEntries);
}

/** The text of a file with LF line endings and no trailing blank lines. */
export function readText(path) {
  return readFileSync(path, 'utf8').replace(/\r\n?/g, '\n').trimEnd();
}

/** The license files of a folder as `[{ name, text }]`; an error naming `label` if none. */
export function collectLicenseTexts(folder, label, extraFile = null) {
  const names = findLicenseFiles(folder);
  if (extraFile !== null && !names.includes(extraFile) && existsSync(join(folder, extraFile))) {
    names.push(extraFile);
  }
  if (names.length === 0) {
    throw new Error(
      `no license file (LICENSE*, LICENCE*, COPYING*, NOTICE*) found for ${label} in ${folder}`,
    );
  }
  return names.map((name) => ({ name, text: readText(join(folder, name)) }));
}

const RULE = '='.repeat(78);
const THIN = '-'.repeat(78);

/** One entry: a title line, the license, and the full text of each license file. */
export function renderEntry({ title, license, texts, description = null }) {
  const lines = [THIN, title, `License: ${license}`];
  if (description !== null) lines.push(description);
  for (const { name, text } of texts) {
    lines.push('', `[${name}]`, '', text);
  }
  return lines.join('\n');
}

export function renderSection(heading, entries) {
  return [RULE, `${heading} (${entries.length})`, RULE, ...entries.map(renderEntry)].join('\n\n');
}

export const HEADER = `THIRD-PARTY NOTICES

This is the Studio of the CodinGame bots repository, a static web page that lets you play
Ultimate Tic-Tac-Toe against a friend or against the bots of the repository, and review games.
It runs in your browser and sends nothing anywhere.

The repository's own code is licensed under MIT OR Apache-2.0 (see LICENSE-MIT and
LICENSE-APACHE in the repository):
${REPOSITORY_URL}

The page also bundles the software below, under its own licenses, whose notices follow:
- the npm packages in the page's JavaScript;
- the Rust crates compiled into the rules' WebAssembly file;
- the Rust standard library and wasi-libc, compiled and linked into every WebAssembly file (the
  rules and each bot); the Rust project's own notice for the library and the crates it contains
  is the file ${LIBRARY_COPYRIGHT_NAME}, beside this one.`;

export function renderNotices({ npm, crates, std }) {
  return (
    [
      HEADER,
      renderSection('NPM PACKAGES', npm),
      renderSection('RUST CRATES', crates),
      renderSection('THE RUST STANDARD LIBRARY AND WASI-LIBC', std),
    ].join('\n\n') + '\n'
  );
}

/** The repository's root, two levels above studio/web/scripts. */
function repositoryRoot() {
  return resolve(dirname(fileURLToPath(import.meta.url)), '..', '..', '..');
}

function webRoot() {
  return resolve(dirname(fileURLToPath(import.meta.url)), '..');
}

function npmEntries(web) {
  const lock = JSON.parse(readFileSync(join(web, 'package-lock.json'), 'utf8'));
  const packages = [...productionClosure(lock), ...bundledTools(lock)].sort(compareEntries);
  return packages.map((pkg) => {
    const folder = join(web, ...pkg.path.split('/'));
    const label = `npm package ${pkg.name}@${pkg.version}`;
    if (!existsSync(folder)) throw new Error(`${label} is not installed (${folder}): run npm ci`);
    const license =
      pkg.license ?? JSON.parse(readFileSync(join(folder, 'package.json'), 'utf8')).license;
    if (typeof license !== 'string' || license === '')
      throw new Error(`${label} declares no license`);
    return {
      title: `${pkg.name} ${pkg.version}`,
      license,
      texts: collectLicenseTexts(folder, label),
    };
  });
}

function cargoMetadata(root) {
  const output = execFileSync(
    'cargo',
    ['metadata', '--format-version', '1', '--locked', '--filter-platform', WASM_TARGET],
    {
      cwd: root,
      encoding: 'utf8',
      maxBuffer: 256 * 1024 * 1024,
      stdio: ['ignore', 'pipe', 'inherit'],
    },
  );
  return JSON.parse(output);
}

function crateEntries(root) {
  return rustClosure(cargoMetadata(root), RULES_PACKAGE).map((crate) => {
    const label = `Rust crate ${crate.name}@${crate.version}`;
    const license = crate.license ?? (crate.licenseFile ? `see ${crate.licenseFile}` : null);
    if (license === null) throw new Error(`${label} declares no license`);
    const licenseFile = crate.licenseFile === null ? null : crate.licenseFile.split(/[\\/]/).pop();
    return {
      title: `${crate.name} ${crate.version}`,
      license,
      texts: collectLicenseTexts(dirname(crate.manifestPath), label, licenseFile),
    };
  });
}

function requireFile(path, what, where = 'in the Rust toolchain', hint = 'is Rust installed?') {
  if (!existsSync(path) || !statSync(path).isFile()) {
    throw new Error(`${what} not found ${where} (${path}): ${hint}`);
  }
  return path;
}

/** The commit of https://github.com/WebAssembly/wasi-libc that `licenses/wasi-libc/` comes from. */
export const WASI_LIBC_COMMIT = '165235bc467d5fa52d424f5d82587dfb76ed9d54';

/** wasi-libc's license files, kept in `licenses/wasi-libc/` (the toolchain has none). */
export const WASI_LIBC_FILES = [
  ['LICENSE', 'wasi-libc LICENSE'],
  ['LICENSE-APACHE-LLVM', 'wasi-libc LICENSE-APACHE-LLVM'],
  ['LICENSE-APACHE', 'wasi-libc LICENSE-APACHE'],
  ['LICENSE-MIT', 'wasi-libc LICENSE-MIT'],
  ['cloudlibc-LICENSE', 'wasi-libc libc-bottom-half/cloudlibc/LICENSE'],
  ['musl-COPYRIGHT', 'wasi-libc libc-top-half/musl/COPYRIGHT'],
];

/**
 * The entries for what the WebAssembly files contain besides crates, from the toolchain's
 * `share/doc/rust` folder (`docFolder`): the Rust standard library and wasi-libc, which the target
 * wasm32-wasip1 links into every file. Returns `{ entries, copyrightFile }`, the second being the
 * Rust project's own notice for the library, to be copied beside the notices. Fails if a file is
 * missing.
 */
export function rustToolchainEntries(docFolder, version, wasiFolder) {
  const copyrightFile = requireFile(
    join(docFolder, 'COPYRIGHT-library.html'),
    'COPYRIGHT-library.html',
  );
  const licenseText = (file, name) => ({
    name,
    text: readText(requireFile(join(docFolder, 'licenses', file), `licenses/${file}`)),
  });
  const mit = licenseText('MIT.txt', 'MIT license');
  const apache = licenseText('Apache-2.0.txt', 'Apache License 2.0');
  return {
    copyrightFile,
    entries: [
      {
        title: `The Rust standard library (${version})`,
        description: `The library and the crates it contains are listed with their licenses in ${LIBRARY_COPYRIGHT_NAME}, the Rust project's own notice, beside this file.`,
        license: 'MIT OR Apache-2.0',
        texts: [mit, apache],
      },
      {
        title: 'wasi-libc (linked by the Rust target wasm32-wasip1)',
        description: `Source: https://github.com/WebAssembly/wasi-libc (license files from commit ${WASI_LIBC_COMMIT}). Parts come from cloudlibc and musl, whose notices follow.`,
        license:
          'Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT; cloudlibc BSD-2-Clause; musl MIT',
        texts: WASI_LIBC_FILES.map(([file, name]) => ({
          name,
          text: readText(
            requireFile(
              join(wasiFolder, file),
              file,
              'in the kept wasi-libc licenses',
              'restore it from git',
            ),
          ),
        })),
      },
    ],
  };
}

function toolchainEntries() {
  const sysroot = execFileSync('rustc', ['--print', 'sysroot'], { encoding: 'utf8' }).trim();
  const version = execFileSync('rustc', ['--version'], { encoding: 'utf8' }).trim();
  const wasiFolder = resolve(dirname(fileURLToPath(import.meta.url)), 'licenses', 'wasi-libc');
  return rustToolchainEntries(join(sysroot, 'share', 'doc', 'rust'), version, wasiFolder);
}

export function main(argv) {
  const folder = resolve(argv[0] ?? 'dist');
  const toolchain = toolchainEntries();
  const notices = renderNotices({
    npm: npmEntries(webRoot()),
    crates: crateEntries(repositoryRoot()),
    std: toolchain.entries,
  });
  if (!existsSync(join(folder, 'index.html'))) {
    throw new Error(`${folder} holds no built site (index.html): run npm run build first`);
  }
  copyFileSync(toolchain.copyrightFile, join(folder, LIBRARY_COPYRIGHT_NAME));
  writeFileSync(join(folder, OUTPUT_NAME), notices);
  console.log(`wrote ${join(folder, OUTPUT_NAME)} (${Buffer.byteLength(notices)} bytes)`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    console.error(`notices: ${error.message}`);
    process.exit(1);
  }
}
