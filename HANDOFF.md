# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Waiting for the owner

- Go for phase 5 step D (arena and ratings), asked 2026-10-09. Step E's light items were merged on 2026-10-09 (#58, #59; owner approved in the conversation). Left in E: speed regression check and docs-only PRs skipping heavy steps (heavier, ask first). Ask before any heavy work.

## Open questions

- Studio (planned phase 6, discussed 2026-10-10, nothing implemented): a local, Lichess-style browser app in this repository (`studio/`, scope `studio`), Rust server plus TypeScript and React front end, for the owner only, on Windows. Home screen of games; play a friend (clock, 10+0 by default), play any release (fixed iterations so takebacks replay exactly via `CG_SEED`/`CG_FIXED_ITERS`), bot against bot then review (120 s at 1x, speeds, steps), analysis engine from current code (win/draw/loss bar, top moves with lines, heatmap on legal cells, evaluation chart), history page with a sample (about 10 games a run) and file import/export. Mockup for the owner: https://claude.ai/artifact/PDeeJjWC7gHZ9eFCCZvavp. Next: the owner's remarks on the mockup, then the plan and decision records.

## Follow-ups

- ADR 0023 still says the SPRT is not a required check, which is no longer true. The owner keeps the current merge policy (2026-10-09) and wants no new record; the line stays as is unless he asks for a fix.
- Record the CodinGame ranks of `uttt-v009` and `uttt-v010` when the owner reports them (skill `deliver-release`).

## Notes

- Phase 5's one-week trial is dropped (owner, 2026-10-09); the daily report at 06:45 Paris time, on Sonnet 5.5, keeps running.
- Ultimate Tic-Tac-Toe strength work stays paused; no training or league run by hand.

## Usage

- 2026-10-09 22:05 (Paris) | step E light items (#58, #59) | main session 22 steps (1.93M read), 1 agent (0.22M read)
- 2026-10-09 21:50 (Paris) | agent guard hook (#52), roadmap line (#53), session setup (#54), handoff (#55), land-work and two-turn approvals (#57, #56 closed into it), all merged | main session 85 steps (10.79M read), 4 agents (0.49M read); the main session's context reached 185k over a long conversation
