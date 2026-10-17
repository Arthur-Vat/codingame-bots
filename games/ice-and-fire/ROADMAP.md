# A Code of Ice and Fire: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The owner decides at the end of every stage; each gate records his answer and its date. Skill `new-game` reads this file to name the next stage.

**Stage in progress:** 5. Referee and engine; next, 6. Arena and baseline bots
**Owner's stopping rule:** none yet

| # | Stage | Status | Gate: the owner's answer |
| --- | --- | --- | --- |
| 1 | Intake | Done (2026-10-17) | Facts in [README.md](README.md), #157 merged; "Merge 157 and go on" (2026-10-17) |
| 2 | Rules | Done (2026-10-17) | [RULES.md](RULES.md), #158 merged; "Merge #158, go on" (form, 2026-10-17) |
| 3 | Model | Done (2026-10-17) | [MODEL.md](MODEL.md), #159 merged; "Merge #159" (form, 2026-10-17) |
| 4 | Framework gaps | Done (2026-10-17) | [ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md) merged (#160); "Merge #160" (form, 2026-10-17) |
| 5 | Referee and engine | In progress | |
| 6 | Arena and baseline bots | Not started | |
| 7 | Studio | Not started | |
| 8 | Research | Not started | |
| 9 | Approach and first bot | Not started | |
| 10 | Iteration | Not started | |
| 11 | Retrospective | Not started | |

Status is `Not started`, `In progress`, `Waiting for the owner` or `Done (YYYY-MM-DD)`.

## Current stage

Referee and engine: the list in the [playbook](../../docs/NEW_GAME.md).

- [x] Readable referee and map generator (#161)
- [ ] A fast, standard-library engine; parity tests against the referee on many random games; property tests
- [ ] A speed baseline in the game's README (`cargo run --release -p ice-and-fire-engine --example speed`)
- [ ] A `rules-check` bot (skill `rules-check`), pasted by the owner on CodinGame in the league where the full rules apply

## Open questions

- Leagues above Bronze today, and how many players the leaderboard and Legend hold (asked 2026-10-17). They inform the stopping rule; the owner may leave them unknown.
- Platform facts for stage 5's `rules-check` bot, not guessed here (2026-10-17): whether equal military power after 100 turns is a draw on CodinGame; whether a pairing is played from both seats; whether CodinGame's referee still matches the source's `develop` branch.

## Decisions

- 2026-10-17: stage 5 go-ahead, in this session (form: "Go, in this session").
- 2026-10-17: framework gaps (form): (1) cg-search and multi-command turns: "Decide at stage 9"; (2) maps: "Own generator + check" ([ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md)); (3) studio: "No human play for this game, replay first play later" (ADR 0039).
- 2026-10-17: stage 3 passed: "Merge #159" (form).
- 2026-10-17: stages 3 and 4 in one pull request, the gaps listed, each then brought to the owner (form: "One PR, gaps listed").
- 2026-10-17: stage 2 passed: "Merge #158, go on" (form).
- 2026-10-17: stage 1 passed: "Merge 157 and go on".
- 2026-10-17: the forum thread is read later, for stage 8 (research), not at intake.
- 2026-10-17: the rules come from the source's statement template (`config/statement_en.html.tpl`) rather than a pasted CodinGame page; stage 5's `rules-check` bot checks them against CodinGame's real games.
- 2026-10-17: folder name `ice-and-fire`, the scope of the game's pull requests and decision records.
- 2026-10-17: the owner named A Code of Ice and Fire as the second game.
