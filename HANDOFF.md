# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Waiting for the owner

- CI scoping (ADR 0029): #83 merged 2026-10-11. #84 (`ci.yml`) and #85 (SPRT and league, stacked on #84) wait for the owner's approval; he chose two ruleset edits (2026-10-11): before #84 merges, swap `Format, lint, test`, `CodinGame compatibility`, `Dependency licenses and sources`, `Docs and scripts` for `CI passed` and `Pull request title and labels` (keep `SPRT of new releases`); before #85, swap `SPRT of new releases` for `SPRT passed`. Answered in the conversation 2026-10-11: "I agree with the checks" (the hygiene check becomes required).
- Try the studio's first version on Windows (phase 6 gate), following "Running the studio on Windows" in `studio/README.md`. The owner will do it later (2026-10-11).

## Open questions

None. (Phase 5's step D comes after phase 6, as in the plan approved 2026-10-10; leagues to be decided later.)

## Follow-ups

- **After #83 to #85 merge:** prove the skipping with a docs-only and a uttt-only draft PR (real one-line changes), closed after; the first push to `main` must be fully green. The SPRT legs only run with a new release: watch the first candidate.
- **Then: phase 6 step D, analysis** (see Notes), in its own session(s), following the updated `land-work` (#82). Ask the owner before starting: it is heavy work.
- ADR 0023 still says the SPRT is not a required check, which is no longer true. The owner keeps the current merge policy (2026-10-09) and wants no new record; the line stays as is unless he asks for a fix.
- Record the CodinGame ranks of `uttt-v009` and `uttt-v010` when the owner reports them (skill `deliver-release`); the owner will report them later (2026-10-11).

## Notes

- Phase 6 state (2026-10-11): steps A, B, C and E merged (#62 to #76; on the owner's approvals in the chat). Left: step D, analysis: (1) cg-search read-only reporting (moves' visits, win/draw/loss, PV) and keep-thinking, proving bots' moves identical at fixed seed and iterations and speed at the baseline (`cargo run --release -p uttt-engine --example speed`, games/uttt/README.md); (2) an analysis engine tool built from current code with the latest release's settings, streaming JSON; (3) server routes and the analysis panel (W/D/L bar, top moves with lines and hover highlight, heatmap on legal cells, keep thinking, evaluation chart filled in the background at about 100 ms per position; the review page has a reserved slot). Lesson: stacked branches that merge siblings by merge commits make GitHub report conflicts after earlier merges; keep stacks linear, or merge main into each branch before asking for approval. Mockup: https://claude.ai/artifact/PDeeJjWC7gHZ9eFCCZvavp.
- CI failures (2026-10-11): every workflow step reports its failure as an annotation through `scripts/annotate.sh` (#79); job logs are readable from sessions, annotations first (ADR 0028, #80, skill `github-api`). The exact network entry the owner allowed stays out of the repository at the owner's request; do not write it in any file, this one included (the branch is public). cargo-deny now runs from its pinned release in CI (version and SHA-256 in `ci.yml`), no longer through EmbarkStudios' action.
- In cloud sessions, `sudo dockerd` starts Docker, so actionlint can run locally as in CI.
- Phase 5's one-week trial is dropped (owner, 2026-10-09); the daily report at 06:45 Paris time, on Sonnet 5.5, keeps running.
- Ultimate Tic-Tac-Toe strength work stays paused; no training or league run by hand.

## Usage

- 2026-10-11 15:45 (Paris) | CI review and the plan to scope CI by area and game (no PR yet) | main session 13 steps (1.90M read, context 130k to 172k)
- 2026-10-11 15:15 (Paris) | process fixes in the skills (#82 merged) | main session 17 steps (1.91M read, context 101k to 130k), 1 agent (0.20M read)
- 2026-10-11 14:40 (Paris) | Dependabot: npm majors by hand (#81 merged), #78 closed | main session 28 steps (2.35M read, context 66k to 97k), 1 agent (0.15M read)
- 2026-10-11 14:30 (Paris) | step 1, CI failures as annotations (#79) and ADR 0028 (#80), both merged | main session 93 steps (16.0M read, context 66k to 261k), 4 agents (3.2M read; the implementer 2.2M over three rounds)
- 2026-10-11 09:10 (Paris) | merges of #62 to #76 (three approval rounds after GitHub conflicts), flaky e2e fix, #77, retrospective and next plan | since 2026-10-10 21:00 Paris (overlapping the line below by about two hours): main session 123 steps (73.3M read; context 526k to 689k, every step re-reading a very long conversation), 7 agents (16.3M read)
- 2026-10-10 21:00 (Paris) | phase 6 steps A, B, C, E: #62 to #76 | main session 260 steps (88.6M read, context up to 588k: a very long conversation), 28 agents (54.6M read; the largest: play modes 13.0M and review/history pages 10.2M); agents hit the plan's session limit once
- 2026-10-09 22:05 (Paris) | step E light items (#58, #59) | main session 22 steps (1.93M read), 1 agent (0.22M read)
- 2026-10-09 21:50 (Paris) | agent guard hook (#52), roadmap line (#53), session setup (#54), handoff (#55), land-work and two-turn approvals (#57, #56 closed into it), all merged | main session 85 steps (10.79M read), 4 agents (0.49M read); the main session's context reached 185k over a long conversation
