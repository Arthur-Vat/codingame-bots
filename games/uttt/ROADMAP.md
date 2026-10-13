# Ultimate Tic-Tac-Toe: roadmap

The state of this game in the new-game lifecycle ([playbook](../../docs/NEW_GAME.md), [ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). The game was built in phases 1 to 6 of the [main roadmap](../../docs/ROADMAP.md), before the lifecycle existed; this file maps that work onto the stages, and stages 3, 8 and 11 were written afterwards as the worked example. Before the lifecycle, the owner's answers were his approvals of each pull request.

**Stage in progress:** 10. Iteration, paused (strength work paused by the owner, 2026-10-09)
**Owner's stopping rule:** Legend reached on 2026-10-07; further work at the owner's call

| # | Stage | Status | Gate: the owner's answer |
| --- | --- | --- | --- |
| 1 | Intake | Done (2026-10-07) | Game chosen ([ADR 0005](../../docs/adr/0005-first-game-uttt.md)) |
| 2 | Rules | Done (2026-10-07) | [RULES.md](RULES.md) merged (phase 1) |
| 3 | Model | Done (2026-10-13), after the fact | [MODEL.md](MODEL.md) |
| 4 | Framework gaps | Done (2026-10-07) | None: the framework was built for this game |
| 5 | Referee and engine | Done (2026-10-07) | Parity on 10,000 random games; `rules-check` matched CodinGame in Bronze (owner, 2026-10-07; phase 2) |
| 6 | Arena and baseline bots | Done (2026-10-07) | Random, greedy and first-valid bots; SPRT and league (phases 1 and 3) |
| 7 | Studio | Done (2026-10-13) | Phase 6 merged on the owner's approvals |
| 8 | Research | Done (2026-10-13), after the fact | [research/](research/README.md) |
| 9 | Approach and first bot | Done (2026-10-07) | MCTS, `uttt-v001` pasted: Legend, 182nd of 443 ([E001](journal/E001-mcts.md)) |
| 10 | Iteration | In progress, paused | Releases up to `uttt-v010`; experiments in the [journal](journal/README.md) |
| 11 | Retrospective | Done (2026-10-13), as the worked example | [RETROSPECTIVE.md](RETROSPECTIVE.md) |

## Current stage

Iteration is paused. When the owner resumes it, each experiment follows "An experiment, step by step" in [docs/WORKFLOW.md](../../docs/WORKFLOW.md).

## Open questions

None.

## Decisions

- 2026-10-13: Ultimate Tic-Tac-Toe is documented to the lifecycle's standard as its worked example, documents only ([ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)).
- 2026-10-13: the owner no longer reports CodinGame ranks; none is asked for.
- 2026-10-09: strength work paused while phase 5 is built; no training or league run by hand.
