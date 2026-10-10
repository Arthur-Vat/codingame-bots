# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Waiting for the owner

- Phase 6 first version, asked 2026-10-10: merge in order #62, #63, #64, #65, #66, #67, #68, #69, #71, #70, #72, #73, #76, #74, #75 (stacked; some include lower ones by merge commits; the stack was checked to merge cleanly in this order). The owner asked on 2026-10-10 to implement the whole phase 6 plan; merges still need approval per pull request. Also the owner's call: make "Studio on Windows" (and "Studio front end") required checks in the ruleset.

## Open questions

None. (Phase 5's step D comes after phase 6, as in the plan approved 2026-10-10; leagues to be decided later.)

## Follow-ups

- ADR 0023 still says the SPRT is not a required check, which is no longer true. The owner keeps the current merge policy (2026-10-09) and wants no new record; the line stays as is unless he asks for a fix.
- Record the CodinGame ranks of `uttt-v009` and `uttt-v010` when the owner reports them (skill `deliver-release`).

## Notes

- Phase 6 state: steps A, B, C and E are done in the open PRs above (first version: play friend / release with takebacks / bot vs bot, review with playback, history with filters and import, Windows CI and guide, SPRT/league record samples). Left: step D, analysis (cg-search read-only reporting with identical moves and speed baseline, an analysis engine from current code streaming top moves / W-D-L / lines, the analysis panel with eval bar, top moves, heatmap, keep thinking, chart filled in the background at about 100 ms per position). Then the owner tests on Windows (gate). Mockup: https://claude.ai/artifact/PDeeJjWC7gHZ9eFCCZvavp.
- Phase 5's one-week trial is dropped (owner, 2026-10-09); the daily report at 06:45 Paris time, on Sonnet 5.5, keeps running.
- Ultimate Tic-Tac-Toe strength work stays paused; no training or league run by hand.

## Usage

- 2026-10-10 21:00 (Paris) | phase 6 steps A, B, C, E: #62 to #76 | main session 260 steps (88.6M read, context up to 588k: a very long conversation), 28 agents (54.6M read; the largest: play modes 13.0M and review/history pages 10.2M); agents hit the plan's session limit once
- 2026-10-09 22:05 (Paris) | step E light items (#58, #59) | main session 22 steps (1.93M read), 1 agent (0.22M read)
- 2026-10-09 21:50 (Paris) | agent guard hook (#52), roadmap line (#53), session setup (#54), handoff (#55), land-work and two-turn approvals (#57, #56 closed into it), all merged | main session 85 steps (10.79M read), 4 agents (0.49M read); the main session's context reached 185k over a long conversation
