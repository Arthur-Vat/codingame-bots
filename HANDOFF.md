# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Waiting for the owner

- Try the studio's first version on Windows (phase 6 gate), following "Running the studio on Windows" in `studio/README.md`; all of it is on `main` since 2026-10-11.
- Optional, asked 2026-10-10: make "Studio front end" and "Studio on Windows" required checks in the `main` ruleset (sessions cannot edit rulesets).

## Open questions

None. (Phase 5's step D comes after phase 6, as in the plan approved 2026-10-10; leagues to be decided later.)

## Follow-ups

- ADR 0023 still says the SPRT is not a required check, which is no longer true. The owner keeps the current merge policy (2026-10-09) and wants no new record; the line stays as is unless he asks for a fix.
- Record the CodinGame ranks of `uttt-v009` and `uttt-v010` when the owner reports them (skill `deliver-release`).

## Notes

- Phase 6 state (2026-10-11): steps A, B, C and E merged (#62 to #76; on the owner's approvals in the chat). Left: step D, analysis: (1) cg-search read-only reporting (moves' visits, win/draw/loss, PV) and keep-thinking, proving bots' moves identical at fixed seed and iterations and speed at the baseline (`cargo run --release -p uttt-engine --example speed`, games/uttt/README.md); (2) an analysis engine tool built from current code with the latest release's settings, streaming JSON; (3) server routes and the analysis panel (W/D/L bar, top moves with lines and hover highlight, heatmap on legal cells, keep thinking, evaluation chart filled in the background at about 100 ms per position; the review page has a reserved slot). Lesson: stacked branches that merge siblings by merge commits make GitHub report conflicts after earlier merges; keep stacks linear, or merge main into each branch before asking for approval. Mockup: https://claude.ai/artifact/PDeeJjWC7gHZ9eFCCZvavp.
- Phase 5's one-week trial is dropped (owner, 2026-10-09); the daily report at 06:45 Paris time, on Sonnet 5.5, keeps running.
- Ultimate Tic-Tac-Toe strength work stays paused; no training or league run by hand.

## Usage

- 2026-10-10 21:00 (Paris) | phase 6 steps A, B, C, E: #62 to #76 | main session 260 steps (88.6M read, context up to 588k: a very long conversation), 28 agents (54.6M read; the largest: play modes 13.0M and review/history pages 10.2M); agents hit the plan's session limit once
- 2026-10-09 22:05 (Paris) | step E light items (#58, #59) | main session 22 steps (1.93M read), 1 agent (0.22M read)
- 2026-10-09 21:50 (Paris) | agent guard hook (#52), roadmap line (#53), session setup (#54), handoff (#55), land-work and two-turn approvals (#57, #56 closed into it), all merged | main session 85 steps (10.79M read), 4 agents (0.49M read); the main session's context reached 185k over a long conversation
