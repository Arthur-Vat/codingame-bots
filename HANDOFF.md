# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Waiting for the owner

- Allow the job-log host in the environment's network settings (owner agreed, 2026-10-11: "For detailed logs I am ok with option 1 and 2"): GitHub serves job logs from `productionresultssa<N>.blob.core.windows.net` (seen: `productionresultssa18`), refused today; prefer the narrowest entry the setting accepts, not `*.blob.core.windows.net`.
- Try the studio's first version on Windows (phase 6 gate), following "Running the studio on Windows" in `studio/README.md`.
- Optional, asked 2026-10-10: make "Studio front end" and "Studio on Windows" required checks in the `main` ruleset.

## Open questions

None. (Phase 5's step D comes after phase 6, as in the plan approved 2026-10-10; leagues to be decided later.)

## Follow-ups

- **Next session's plan, in this order (owner, 2026-10-11: "all those topics are a priority"; answered in the conversation: OK with the retrospective's remarks; #77 merged; #78 for the next session).** Start a fresh session; keep each topic small, ask approval per batch as soon as it is green, and merge before stacking more.
  1. **CI failures readable without logs** (one PR, deep review): every job reports failures as annotations, as the browser tests already do (Playwright `github` reporter, #72): Rust tests and clippy (e.g. a wrapper emitting `::error` with the failing test's output tail), the scripts' checks, the Windows smoke. Then update skill `github-api`: once the owner has allowed the log host, read logs with `gh api repos/$R/actions/jobs/<id>/logs`; until then, annotations.
  2. **Dependabot** (one PR, deep review): close #78 with a comment (TypeScript 7 unsupported by typescript-eslint, Vite 8 pulls MPL `lightningcss` refused by ADR 0026, `@types/node` 26 for Node 22, `npm ci` fails); in `.github/dependabot.yml` ignore major versions for npm (`update-types: ["version-update:semver-major"]`) and keep `@types/node` on 22; majors are done by hand in their own PR.
  3. **Process fixes from the 2026-10-11 retrospective** (one PR on `.claude/skills`, deep review; owner approved the remarks): `land-work`: linear stacks only (never merge sibling branches into each other; merge main into each branch before asking approval), small approval batches merged as they come, settle shared interfaces and formats in a written spec before parallel implementers, a known-risks checklist in every spec (input validation incl. imported files, edge cases, timing races, Windows); `handoff`/session hygiene: suggest a fresh session at ~150k context and actually do it between steps; scripts that post to GitHub are dry-run first.
  4. **Phase 6 step D, analysis** (see Notes), in its own session(s), with the improved process.
- ADR 0023 still says the SPRT is not a required check, which is no longer true. The owner keeps the current merge policy (2026-10-09) and wants no new record; the line stays as is unless he asks for a fix.
- Record the CodinGame ranks of `uttt-v009` and `uttt-v010` when the owner reports them (skill `deliver-release`).

## Notes

- Phase 6 state (2026-10-11): steps A, B, C and E merged (#62 to #76; on the owner's approvals in the chat). Left: step D, analysis: (1) cg-search read-only reporting (moves' visits, win/draw/loss, PV) and keep-thinking, proving bots' moves identical at fixed seed and iterations and speed at the baseline (`cargo run --release -p uttt-engine --example speed`, games/uttt/README.md); (2) an analysis engine tool built from current code with the latest release's settings, streaming JSON; (3) server routes and the analysis panel (W/D/L bar, top moves with lines and hover highlight, heatmap on legal cells, keep thinking, evaluation chart filled in the background at about 100 ms per position; the review page has a reserved slot). Lesson: stacked branches that merge siblings by merge commits make GitHub report conflicts after earlier merges; keep stacks linear, or merge main into each branch before asking for approval. Mockup: https://claude.ai/artifact/PDeeJjWC7gHZ9eFCCZvavp.
- Phase 5's one-week trial is dropped (owner, 2026-10-09); the daily report at 06:45 Paris time, on Sonnet 5.5, keeps running.
- Ultimate Tic-Tac-Toe strength work stays paused; no training or league run by hand.

## Usage

- 2026-10-11 09:10 (Paris) | merges of #62 to #76 (three approval rounds after GitHub conflicts), flaky e2e fix, #77, retrospective and next plan | since 2026-10-10 21:00 Paris (overlapping the line below by about two hours): main session 123 steps (73.3M read; context 526k to 689k, every step re-reading a very long conversation), 7 agents (16.3M read)
- 2026-10-10 21:00 (Paris) | phase 6 steps A, B, C, E: #62 to #76 | main session 260 steps (88.6M read, context up to 588k: a very long conversation), 28 agents (54.6M read; the largest: play modes 13.0M and review/history pages 10.2M); agents hit the plan's session limit once
- 2026-10-09 22:05 (Paris) | step E light items (#58, #59) | main session 22 steps (1.93M read), 1 agent (0.22M read)
- 2026-10-09 21:50 (Paris) | agent guard hook (#52), roadmap line (#53), session setup (#54), handoff (#55), land-work and two-turn approvals (#57, #56 closed into it), all merged | main session 85 steps (10.79M read), 4 agents (0.49M read); the main session's context reached 185k over a long conversation
