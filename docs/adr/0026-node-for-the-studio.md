# 0026. Node and npm build the studio's front end

- Status: accepted (owner, 2026-10-10)
- Date: 2026-10-10
- Scope: framework

## Context

The studio's front end is written in TypeScript with React ([ADR 0025](0025-studio.md)), which the owner chose on 2026-10-10 as efficient and modern; the owner reads the code but does not change it. Building it needs Node and npm, a second toolchain beside Rust. Claude's cloud sessions have Node 22 and reach the npm registry (checked 2026-10-10); the owner uses Windows.

The repository's Rust dependencies follow `deny.toml`: allowed licenses and sources, checked in CI. npm packages are many and small, and their licenses are more varied (ISC and BSD licenses are common), so the same list cannot be applied unchanged. Nothing from npm ends up in a bot; the front end is a tool that runs on the owner's computer and is never published.

## Decision

1. **Node's long-term-support line is pinned, and npm is the only package manager.** The version is written once in the front end's `package.json` and followed by CI and the setup guide. The lockfile is committed and installs use `npm ci`.
2. **Dependencies stay few, and each one is a choice.** A new package is added only for work it does substantially, and its pull request says why.
3. **Only permissive licenses are allowed, listed in one file and checked in CI.** The list starts from the licenses `deny.toml` allows and adds other permissive ones only when a package needs them; copyleft licenses (GPL, LGPL, AGPL, MPL, EPL and the like) are refused.
4. **CI checks the front end on every change to it:** type check, lint, unit tests, production build, browser tests with Chromium and the license check.

## Consequences

- Claude's sessions, CI and the owner's computer need Node as well as Rust.
- npm brings a supply-chain risk that the lockfile, `npm ci`, the small dependency list and the license check reduce but do not remove.
- CI takes longer when the front end changes.
- Dependency updates of the front end follow the same review as other changes.
