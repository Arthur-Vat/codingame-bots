import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import {
  bundledTools,
  collectLicenseTexts,
  findLicenseFiles,
  isLicenseFileName,
  nameFromLockPath,
  productionClosure,
  renderNotices,
  resolveLockDependency,
  rustClosure,
  rustToolchainEntries,
  WASI_LIBC_COMMIT,
  WASI_LIBC_FILES,
} from './notices.mjs';

describe('isLicenseFileName', () => {
  it('accepts license files in any case and with any suffix', () => {
    for (const name of [
      'LICENSE',
      'license',
      'License.md',
      'LICENSE-MIT',
      'LICENSE-APACHE',
      'LICENCE.txt',
      'LICENSE_MIT',
      'COPYING',
      'COPYING.LESSER',
      'NOTICE',
      'notice.txt',
    ]) {
      expect(isLicenseFileName(name), name).toBe(true);
    }
  });

  it('refuses other files', () => {
    for (const name of ['README.md', 'package.json', 'licenser.js', 'index.js', 'LICENSES2']) {
      expect(isLicenseFileName(name), name).toBe(false);
    }
  });
});

describe('findLicenseFiles and collectLicenseTexts', () => {
  const folder = mkdtempSync(join(tmpdir(), 'notices-'));
  writeFileSync(join(folder, 'LICENSE-MIT'), 'mit text\r\n\r\n');
  writeFileSync(join(folder, 'LICENSE-APACHE'), 'apache text\n');
  writeFileSync(join(folder, 'README.md'), 'readme');
  mkdirSync(join(folder, 'LICENSES'));
  const empty = mkdtempSync(join(tmpdir(), 'notices-'));

  it('lists every license file, sorted, and no folder', () => {
    expect(findLicenseFiles(folder)).toEqual(['LICENSE-APACHE', 'LICENSE-MIT']);
  });

  it('reads all of them with LF endings', () => {
    expect(collectLicenseTexts(folder, 'x')).toEqual([
      { name: 'LICENSE-APACHE', text: 'apache text' },
      { name: 'LICENSE-MIT', text: 'mit text' },
    ]);
  });

  it('fails naming the package when there is none', () => {
    expect(() => collectLicenseTexts(empty, 'npm package left-pad@1.0.0')).toThrow(
      /npm package left-pad@1\.0\.0/,
    );
  });

  it('adds a declared license-file that the pattern does not match', () => {
    writeFileSync(join(empty, 'terms.txt'), 'terms');
    expect(collectLicenseTexts(empty, 'x', 'terms.txt')).toEqual([
      { name: 'terms.txt', text: 'terms' },
    ]);
  });
});

const lock = {
  packages: {
    '': {
      name: 'app',
      dependencies: { react: '1.0.0', '@scope/ui': '2.0.0' },
      devDependencies: { vitest: '1.0.0' },
    },
    'node_modules/react': { version: '1.0.0', license: 'MIT', dependencies: { loose: '^1' } },
    'node_modules/loose': { version: '1.5.0', license: 'MIT' },
    'node_modules/@scope/ui': {
      version: '2.0.0',
      license: 'ISC',
      dependencies: { loose: '^2' },
      peerDependencies: { react: '*', absent: '*' },
      optionalDependencies: { native: '*' },
    },
    'node_modules/@scope/ui/node_modules/loose': { version: '2.1.0', license: 'MIT' },
    'node_modules/vitest': {
      version: '1.0.0',
      license: 'MIT',
      dev: true,
      dependencies: { tiny: '1' },
    },
    'node_modules/tiny': { version: '1.0.0', license: 'MIT', dev: true },
  },
};

describe('nameFromLockPath and resolveLockDependency', () => {
  it('names scoped and nested packages', () => {
    expect(nameFromLockPath('node_modules/react')).toBe('react');
    expect(nameFromLockPath('node_modules/@scope/ui')).toBe('@scope/ui');
    expect(nameFromLockPath('node_modules/a/node_modules/@s/b')).toBe('@s/b');
  });

  it('resolves to the closest installed copy', () => {
    const { packages } = lock;
    expect(resolveLockDependency(packages, 'node_modules/@scope/ui', 'loose')).toBe(
      'node_modules/@scope/ui/node_modules/loose',
    );
    expect(resolveLockDependency(packages, 'node_modules/react', 'loose')).toBe(
      'node_modules/loose',
    );
    expect(resolveLockDependency(packages, '', 'nothing')).toBeNull();
  });
});

describe('productionClosure', () => {
  const closure = productionClosure(lock);

  it('keeps the production packages, nested copies included, sorted', () => {
    expect(closure.map((pkg) => `${pkg.name}@${pkg.version}`)).toEqual([
      '@scope/ui@2.0.0',
      'loose@1.5.0',
      'loose@2.1.0',
      'react@1.0.0',
    ]);
  });

  it('leaves out the dev dependencies and what only they need', () => {
    const names = closure.map((pkg) => pkg.name);
    expect(names).not.toContain('vitest');
    expect(names).not.toContain('tiny');
  });

  it('carries the license of the lockfile', () => {
    expect(closure.find((pkg) => pkg.name === '@scope/ui').license).toBe('ISC');
  });

  it('fails on a missing required dependency', () => {
    const broken = { packages: { '': { dependencies: { gone: '1' } } } };
    expect(() => productionClosure(broken)).toThrow(/gone/);
  });
});

function registryCrate(name, version, extra = {}) {
  return {
    id: `registry ${name} ${version}`,
    name,
    version,
    license: 'MIT',
    license_file: null,
    manifest_path: `/reg/${name}-${version}/Cargo.toml`,
    ...extra,
  };
}

function workspaceCrate(name) {
  return { ...registryCrate(name, '0.0.0'), id: `path ${name}`, license: null };
}

function edge(pkg, kind) {
  return { pkg, dep_kinds: [{ kind, target: null }] };
}

describe('rustClosure', () => {
  const metadata = {
    workspace_members: ['path studio-rules', 'path studio-core', 'path other'],
    packages: [
      workspaceCrate('studio-rules'),
      workspaceCrate('studio-core'),
      workspaceCrate('other'),
      registryCrate('serde', '1.0.0'),
      registryCrate('serde_derive', '1.0.1', { license: null, license_file: 'COPY' }),
      registryCrate('proc-macro2', '1.0.2'),
      registryCrate('tempfile', '3.0.0'),
      registryCrate('cc', '1.0.0'),
      registryCrate('only-other', '1.0.0'),
    ],
    resolve: {
      nodes: [
        { id: 'path studio-rules', deps: [edge('path studio-core', null)] },
        {
          id: 'path studio-core',
          deps: [
            edge('registry serde 1.0.0', null),
            edge('registry tempfile 3.0.0', 'dev'),
            edge('registry cc 1.0.0', 'build'),
          ],
        },
        { id: 'registry serde 1.0.0', deps: [edge('registry serde_derive 1.0.1', null)] },
        { id: 'registry serde_derive 1.0.1', deps: [edge('registry proc-macro2 1.0.2', null)] },
        { id: 'registry proc-macro2 1.0.2', deps: [] },
        { id: 'path other', deps: [edge('registry only-other 1.0.0', null)] },
      ],
    },
  };

  it('walks normal dependencies from the root, through workspace crates, without them', () => {
    expect(rustClosure(metadata, 'studio-rules').map((c) => `${c.name}@${c.version}`)).toEqual([
      'proc-macro2@1.0.2',
      'serde@1.0.0',
      'serde_derive@1.0.1',
    ]);
  });

  it('reports license and license-file', () => {
    const closure = rustClosure(metadata, 'studio-rules');
    expect(closure.find((c) => c.name === 'serde_derive')).toMatchObject({
      license: null,
      licenseFile: 'COPY',
    });
  });

  it('fails when the root is not a workspace package', () => {
    expect(() => rustClosure(metadata, 'serde')).toThrow(/serde/);
  });
});

describe('renderNotices', () => {
  const entry = (title) => ({
    title,
    license: 'MIT',
    texts: [{ name: 'LICENSE', text: `text of ${title}` }],
  });

  it('is deterministic and holds every entry in full', () => {
    const input = { npm: [entry('a 1.0.0')], crates: [entry('b 2.0.0')], std: [entry('std')] };
    const out = renderNotices(input);
    expect(renderNotices(input)).toBe(out);
    expect(out).toContain('MIT OR Apache-2.0');
    expect(out).toContain('https://github.com/Arthur-Vat/codingame-bots');
    for (const text of ['text of a 1.0.0', 'text of b 2.0.0', 'text of std']) {
      expect(out).toContain(text);
    }
    expect(out.endsWith('\n')).toBe(true);
  });
});

describe('rustToolchainEntries', () => {
  function toolchainDocs(files) {
    const folder = mkdtempSync(join(tmpdir(), 'rustdoc-'));
    mkdirSync(join(folder, 'licenses'));
    writeFileSync(join(folder, 'COPYRIGHT-library.html'), '<html>notice</html>');
    for (const name of files) writeFileSync(join(folder, 'licenses', name), `text of ${name}\n`);
    return folder;
  }
  function wasiLicenses(skip = null) {
    const folder = mkdtempSync(join(tmpdir(), 'wasi-'));
    for (const [file] of WASI_LIBC_FILES) {
      if (file !== skip) writeFileSync(join(folder, file), `text of ${file}\n`);
    }
    return folder;
  }
  const all = ['MIT.txt', 'Apache-2.0.txt'];

  it('gives the standard library and wasi-libc their texts, and the notice file to copy', () => {
    const folder = toolchainDocs(all);
    const { entries, copyrightFile } = rustToolchainEntries(folder, 'rustc 1.90.0', wasiLicenses());
    expect(copyrightFile).toBe(join(folder, 'COPYRIGHT-library.html'));
    expect(entries.map((e) => e.title)).toEqual([
      'The Rust standard library (rustc 1.90.0)',
      'wasi-libc (linked by the Rust target wasm32-wasip1)',
    ]);
    expect(entries[0].description).toContain('rust-library-copyright.html');
    expect(entries[0].texts.map((t) => t.text)).toEqual([
      'text of MIT.txt',
      'text of Apache-2.0.txt',
    ]);
    expect(entries[1].description).toContain('https://github.com/WebAssembly/wasi-libc');
    expect(entries[1].description).toContain(WASI_LIBC_COMMIT);
    expect(entries[1].texts.map((t) => t.text)).toEqual(
      WASI_LIBC_FILES.map(([file]) => `text of ${file}`),
    );
  });

  it('fails naming the missing file', () => {
    expect(() => rustToolchainEntries(toolchainDocs(['MIT.txt']), 'v', wasiLicenses())).toThrow(
      /Apache-2\.0\.txt/,
    );
    expect(() =>
      rustToolchainEntries(toolchainDocs(all), 'v', wasiLicenses('musl-COPYRIGHT')),
    ).toThrow(/musl-COPYRIGHT/);
    const noNotice = toolchainDocs(all);
    rmSync(join(noNotice, 'COPYRIGHT-library.html'));
    expect(() => rustToolchainEntries(noNotice, 'v', wasiLicenses())).toThrow(
      /COPYRIGHT-library\.html/,
    );
  });

  it('keeps every wasi-libc license file in the repository', () => {
    const kept = join(dirname(fileURLToPath(import.meta.url)), 'licenses', 'wasi-libc');
    for (const [file] of WASI_LIBC_FILES) expect(existsSync(join(kept, file))).toBe(true);
  });
});

describe('bundledTools', () => {
  it('adds Vite, whose preload helpers are in the bundle, from the lockfile', () => {
    const lock = { packages: { 'node_modules/vite': { version: '7.1.0', license: 'MIT' } } };
    expect(bundledTools(lock)).toEqual([
      { name: 'vite', version: '7.1.0', path: 'node_modules/vite', license: 'MIT' },
    ]);
    expect(() => bundledTools({ packages: {} })).toThrow(/vite/);
  });
});
