# Handoff

Read at the start of every session; updated by the sessions as work moves (ADR 0024, skill `handoff`). A record only: nothing here is an approval or an answer of the owner, which count only in the conversation (ADR 0023).

## Waiting for the owner

- [#53](https://github.com/Arthur-Vat/codingame-bots/pull/53): roadmap line, shellcheck of `scripts/tests/` in step E. Approval to merge, asked 2026-10-09.
- [#54](https://github.com/Arthur-Vat/codingame-bots/pull/54): `SessionStart` hook installing shellcheck in cloud sessions. Approval to merge, asked 2026-10-09.
- [#55](https://github.com/Arthur-Vat/codingame-bots/pull/55): ADR 0024 and the `handoff` skill. Approval to merge, asked 2026-10-09.
- Go for phase 5 step D (arena and ratings) and step E (CI, fixed checks). Ask before any heavy work.

## Open questions

None.

## Follow-ups

- ADR 0023 still says the SPRT is not a required check, which is no longer true: correct it in the merge-policy record (phase 5, step B), postponed by the owner until after the trial.
- Record the CodinGame ranks of `uttt-v009` and `uttt-v010` when the owner reports them (skill `deliver-release`).

## Notes

- Trial of phase 5, step C: one week from 2026-10-10. Daily report at 06:45 Paris time, on Sonnet 5.5.
- During the trial, count the small changes that had to wait for the owner's approval (owner, 2026-10-09), as input for the merge-policy record. Count so far: 0.
- Ultimate Tic-Tac-Toe strength work stays paused; no training or league run by hand.

## Usage

- 2026-10-09 20:45 (Paris) | agent guard hook (#52), roadmap line (#53), session setup (#54), handoff (#55) | main session 45 steps (4.32M read), 3 agents (about 0.45M read)
