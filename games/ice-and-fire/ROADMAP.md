# A Code of Ice and Fire: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The owner decides at the end of every stage; each gate records his answer and its date. Skill `new-game` reads this file to name the next stage.

**Stage in progress:** 1. Intake
**Owner's stopping rule:** none yet

| # | Stage | Status | Gate: the owner's answer |
| --- | --- | --- | --- |
| 1 | Intake | In progress | |
| 2 | Rules | Not started | |
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

Intake: the facts, with their sources, are in [README.md](README.md).

- [x] The pages: the statement template of the source stands in for CodinGame's pages (owner, 2026-10-17); each league's rules are in it; the forum thread waits for stage 8
- [x] The game's source, its license (GPL 3: facts only) and where the statement links to it
- [x] Players, turn format, what a turn's answer is, hidden information, randomness
- [x] Time limits for the first turn and the others; output format and what makes an answer lose
- [ ] The leagues and when the full rules apply (done: Bronze); the leagues above Bronze and the number of players on the leaderboard (open questions below)
- [x] Notes on the owner's criteria of 2026-10-13
- [x] Unknowns listed as questions for the owner

## Open questions

- Leagues above Bronze today, and how many players the leaderboard and Legend hold (asked 2026-10-17). They inform the stopping rule; the owner may leave them unknown.
- Platform facts for stage 5's `rules-check` bot, not guessed here (2026-10-17): whether equal military power after 100 turns is a draw on CodinGame; whether a pairing is played from both seats; whether CodinGame's referee still matches the source's `develop` branch.

## Decisions

- 2026-10-17: the forum thread is read later, for stage 8 (research), not at intake.
- 2026-10-17: the rules come from the source's statement template (`config/statement_en.html.tpl`) rather than a pasted CodinGame page; stage 5's `rules-check` bot checks them against CodinGame's real games.
- 2026-10-17: folder name `ice-and-fire`, the scope of the game's pull requests and decision records.
- 2026-10-17: the owner named A Code of Ice and Fire as the second game.
