# A Code of Ice and Fire: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The owner decides at the end of every stage; each gate records his answer and its date. Skill `new-game` reads this file to name the next stage.

**Stage in progress:** 10. Iteration
**Owner's stopping rule:** none yet

| # | Stage | Status | Gate: the owner's answer |
| --- | --- | --- | --- |
| 1 | Intake | Done (2026-10-17) | Facts in [README.md](README.md), #157 merged; "Merge 157 and go on" (2026-10-17) |
| 2 | Rules | Done (2026-10-17) | [RULES.md](RULES.md), #158 merged; "Merge #158, go on" (form, 2026-10-17) |
| 3 | Model | Done (2026-10-17) | [MODEL.md](MODEL.md), #159 merged; "Merge #159" (form, 2026-10-17) |
| 4 | Framework gaps | Done (2026-10-17) | [ADR 0039](../../docs/adr/0039-ice-and-fire-maps-and-studio.md) merged (#160); "Merge #160" (form, 2026-10-17) |
| 5 | Referee and engine | Done (2026-10-19) | `rules-check` on CodinGame: no difference in 4 Bronze games, both seats ([RULES.md](RULES.md)); "Merge #175, then record (Recommended)" (form, 2026-10-19) |
| 6 | Arena and baseline bots | Done (2026-10-17) | Arena, baselines and the game's numbers, #166 merged; "Merge #166", then "Fresh session, then stage 7" (forms, 2026-10-17) |
| 7 | Studio | Done (2026-10-17) | Adapter, renderer and baseline bots, #167-#169 merged; "The studio works perfectly", with the card's picture changed (2026-10-17) |
| 8 | Research | Done (2026-10-19) | [research/README.md](research/README.md), #177 merged; prototypes "plan + phases (Recommended)", no anneal (form, 2026-10-19) |
| 9 | Approach and first bot | Done (2026-10-22) | `plan` is the approach: [E002](journal/E002-plan.md), `ice-and-fire-v002`, #182 merged; "Merge #182 (Recommended)" (form, 2026-10-22) |
| 10 | Iteration | In progress | |
| 11 | Retrospective | Not started | |

Status is `Not started`, `In progress`, `Waiting for the owner` or `Done (YYYY-MM-DD)`.

## Current stage

Iteration: one experiment per pull request ([journal](journal/README.md)).

- [x] E003, the early game: research ([research/early-game.md](research/early-game.md)), an opening benchmark, the openings inside the `plan` bot, the best as `ice-and-fire-v003` through the SPRT against v002: accepted, +299.8 Elo ([E003](journal/E003-opening.md), #183)
- [x] E004, the early game planned on the first turn: a crate for the opening, a heuristic score of the middle game and an in-process harness (#184); tuned weights, a portfolio of weights chosen per map and five turn-1 planners compared at equal time by a heuristic score (the economy, then the Voronoi share); the portfolio, the best against v003 in the arena, inside the `plan` bot as `ice-and-fire-v004` through the SPRT against v003: accepted, +51.9 Elo ([E004](journal/E004-planned-opening.md), #184, #185)
- [x] E007, a gold reserve in the middle game: the plan search kept 20 gold after its turn except for attacks, as `ice-and-fire-v005`; SPRT rejected against v004, −15.9 Elo, dropped ([E007](journal/E007-reserve.md), #186)

Approach and first bot: the list in the [playbook](../../docs/NEW_GAME.md).

- [x] Shared tactics: cheapest chain with towers, cut finder, evaluation (`tactics/`)
- [x] The ladder of reference bots, each adding one step to the one before ([E001](journal/E001-ladder.md), `ladder/`): `chain`, `guard`, `cutter`, `banker`, all published as baseline bots
- [x] ice-and-fire-v001 from `banker`: SPRT accepted against `bronze`, +196.5 Elo, +207.0 at CodinGame's limits (#179)
- [x] `plan`: the plan search with a memory of opponent replies, compared with v001: `ice-and-fire-v002`, SPRT accepted against v001, +126.4 Elo at CodinGame's limits ([E002](journal/E002-plan.md), #182)
- [ ] The owner pastes v001 on CodinGame

Research: the list in the [playbook](../../docs/NEW_GAME.md).

- [x] The contest's post-mortems (the forum, read in full on 2026-10-19) and papers on turns of many commands: 4 forum threads with 15 post-mortems, and 7 papers (one could not be opened) ([research/README.md](research/README.md))
- [x] The approaches ranked by expected strength and cost; the generic method in [docs/research/multi-action-turns.md](../../docs/research/multi-action-turns.md)
- [x] The owner's gate: the prototypes of stage 9 chosen: plan and phases (2026-10-19)

Referee and engine: the list in the [playbook](../../docs/NEW_GAME.md).

- [x] Readable referee and map generator (#161)
- [x] Map generator reproducing the source's maps ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md)), checked against maps printed by the source
- [x] A fast, standard-library engine; parity tests against the referee on many random games; property tests (#162)
- [x] A speed baseline in the game's README (`cargo run --release -p ice-and-fire-engine --example speed`)
- [x] One bot per league (`wood3`, `wood2`, `wood1`, `bronze`) to climb to Bronze, where the `rules-check` bot can run (owner: never played the game, and one bot per league, 2026-10-17)
- [x] A `rules-check` bot (skill `rules-check`), written (`bots/rules-check/`)
- [x] Each league's bot against the source's boss of that league, run outside the repository: 200 of 200 games won in each league ([README.md](README.md#bots), 2026-10-19)
- [x] The `rules-check` bot pasted by the owner on CodinGame in Bronze, where the full rules apply: 4 games as red and blue, 0 differences, the longest 95 turns and 4,045 checks; the maps CodinGame sent hold every map fact of RULES.md (2026-10-19)

Arena and baseline bots: the list in the [playbook](../../docs/NEW_GAME.md).

- [x] The game's arena binary (`arena/`) and `evaluation.env`
- [x] Baseline bots: `random` (the anchor of the ratings) and the greedy `bronze`; a first-valid bot is not needed, since `wait` is always valid (the checklist's "first valid, random, greedy")
- [x] The game's numbers from the arena: seat advantage, game length, draw rate, and how the SPRT's seeds are drawn ([README.md](README.md#numbers))
- [x] CI: `ci.sh` plays the arena's games and the evaluation controls (`scripts/ci-scopes.sh` picks up the game's folder by itself)
- [x] The owner's gate: a pull request for stage 6 merged (#166)

Studio: the list in the [playbook](../../docs/NEW_GAME.md).

- [x] The adapter crate `studio/` (`ice-and-fire-studio`) and the board renderer in `studio/web/src/games/ice-and-fire/`; bot seats only, no human play (ADR 0039, decision 4)
- [x] The game's baseline bots in the studio, so that it has bots before its first release ([ADR 0040](../../docs/adr/0040-baseline-bots-in-the-studio.md))
- [x] The owner's gate: the game tried in the studio (2026-10-17), and the home card's picture of the game

## Open questions

- Leagues above Bronze today, and how many players the leaderboard and Legend hold (asked 2026-10-17). They inform the stopping rule; the owner may leave them unknown.
- Whether equal military power after 100 turns is a draw on CodinGame (asked 2026-10-17): not seen in the `rules-check` games. The seats (either) and CodinGame's referee (no difference from the source's) were settled on 2026-10-19 ([RULES.md](RULES.md#platform-facts-to-check)).

## Decisions

- 2026-10-22: the early game before anything else: "Before implementing anything, I would like you to think about strategies for the early game ... The goal is to maximize the expansion to generate the most income, while also being safe when the middle game begins. You should consider several approaches from a theoretical point of view, then benchmark them. Once best candidates are selected you can use them as part of a full bot, and then you can pick the best" (E003).
- 2026-10-22: stage 9 passed: #182 merged ("Merge #182 (Recommended)", form); `plan`, as `ice-and-fire-v002`, is the approach.

- 2026-10-21: official time limits in all games and 800 pairs per league pairing: "I would like to have the official time control in all games"; "What about 800 pairs? Is it within the job limit? If yes go on with this" ([ADR 0042](../../docs/adr/0042-official-time-limits-and-800-pairs.md)), superseding the 0.2 scale of 2026-10-17.
- 2026-10-19: the first bot is a ladder of reference bots rather than one bot called `phases`: `chain`, `guard`, `cutter`, `banker`, each adding one step to the one before and all published as baseline bots in one pull request; only `banker` is released, as v001 (the owner disliked the name `phases`).
- 2026-10-19: stage 9 releases early (form: "Release early (Recommended)"): shared tactics first, then `phases` as v001 once it beats `bronze`, then `plan` as v002 if it beats v001; the owner wants several modest versions in several pull requests rather than one strong bot at once.
- 2026-10-19: stage 8 passed: #177 merged; prototypes `plan` and `phases`, no `anneal` (form: "plan + phases (Recommended)").
- 2026-10-19: the arena's maps become exactly the source's, rebuilt from a written spec ([MAP_GENERATOR.md](MAP_GENERATOR.md)) by an implementer that never reads the source, and checked against maps printed by the source run unchanged ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md); forms: "Clean-room spec (Recommended)", then "Go, in this session (Recommended)").
- 2026-10-19: the owner pasted CodinGame's Wood 3 page, recorded as a source of RULES.md (it agrees with it), and asked for the source's league bosses to be played locally, unchanged and outside the repository (form: "Play bosses locally (Recommended)", "Record the pasted page").
- 2026-10-17: stage 7 passed: "The studio works perfectly"; the owner asked for a picture of the game on its home card, in the style of the others. Stage 8 in a fresh session (form: "Stage 8, fresh session (Recommended)").
- 2026-10-17: the studio also offers each game's baseline bots, so that this game can be watched before its first release (form: "Baseline bots (Recommended)", [ADR 0040](../../docs/adr/0040-baseline-bots-in-the-studio.md)).
- 2026-10-17: stage 6 passed: "Merge #166", then stage 7 in a fresh session (form: "Fresh session, then stage 7").
- 2026-10-17: evaluation time scale 0.2 (10 ms of 50, plus 5 ms tolerance), as ADR 0013 (form: "0.2: 10 ms, as ADR 0013").
- 2026-10-17: stage 6 started while the owner climbs the Wood leagues (form: "Stage 6: arena").
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
