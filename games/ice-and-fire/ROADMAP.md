# A Code of Ice and Fire: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The owner decides at the end of every stage; each gate records his answer and its date. Skill `new-game` reads this file to name the next stage.

**Stage in progress:** 2. Rules
**Owner's stopping rule:** none yet

| # | Stage | Status | Gate: the owner's answer |
| --- | --- | --- | --- |
| 1 | Intake | Done (2026-10-17) | Facts in [README.md](README.md), #157 merged; "Merge 157 and go on" (2026-10-17) |
| 2 | Rules | In progress | |
| 3 | Model | Not started | |
| 4 | Framework gaps | Not started | |
| 5 | Referee and engine | Not started | |
| 6 | Arena and baseline bots | Not started | |
| 7 | Studio | Not started | |
| 8 | Research | Not started | |
| 9 | Approach and first bot | Not started | |
| 10 | Iteration | Not started | |
| 11 | Retrospective | Not started | |

Status is `Not started`, `In progress`, `Waiting for the owner` or `Done (YYYY-MM-DD)`.

## Current stage

Rules: [RULES.md](RULES.md), in our own words, a source for every rule.

- [ ] `RULES.md` in our own words, a source for every rule
- [ ] The protocol: inputs, outputs, what makes an answer invalid
- [ ] End of game, scoring and tiebreaks, with their edge cases
- [ ] Every point the statement and the source disagree on, and which one applies
- [ ] Reviewed by `pr-reviewer`, briefed to check each rule against the statement and the source

## Open questions

- Leagues above Bronze today, and how many players the leaderboard and Legend hold (asked 2026-10-17). They inform the stopping rule; the owner may leave them unknown.
- Platform facts for stage 5's `rules-check` bot, not guessed here (2026-10-17): whether equal military power after 100 turns is a draw on CodinGame; whether a pairing is played from both seats; whether CodinGame's referee still matches the source's `develop` branch.

## Decisions

- 2026-10-17: stage 1 passed: "Merge 157 and go on".
- 2026-10-17: the forum thread is read later, for stage 8 (research), not at intake.
- 2026-10-17: the rules come from the source's statement template (`config/statement_en.html.tpl`) rather than a pasted CodinGame page; stage 5's `rules-check` bot checks them against CodinGame's real games.
- 2026-10-17: folder name `ice-and-fire`, the scope of the game's pull requests and decision records.
- 2026-10-17: the owner named A Code of Ice and Fire as the second game.
