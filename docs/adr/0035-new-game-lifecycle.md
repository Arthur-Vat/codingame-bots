# 0035. A lifecycle for each new game

- Status: accepted (owner, 2026-10-13)
- Date: 2026-10-13
- Scope: framework

## Context

Ultimate Tic-Tac-Toe went from rules to Legend in phases 1 to 4, then gained a studio in phase 6, but nothing written says how the next game should go. The roadmap's phase 7 planned a second game and a `new-game` skill. On 2026-10-13 the owner asked for a checklist and a roadmap that every new game follows: identify the rules, model the problem precisely, build an engine, a visualizer and bots, research approaches and iterate, with documentation and a project structure that keep it clear. He answered Claude's framing questions the same day in the conversation; this record writes down his answers.

The game the owner considered then, A Code of Ice and Fire, has its source under GPL v3, while this repository is under MIT or Apache 2.0 ([ADR 0001](0001-license.md)). The source of Ultimate Tic-Tac-Toe's game has no license and was read for facts only ([games/uttt/RULES.md](../../games/uttt/RULES.md)). CodinGame's contest rules, pasted by the owner on 2026-10-13, count as cheating "being assisted by a person or a machine" and the "public release of a solution" (article 8), which a public repository of bots written by Claude is.

## Decision

1. **Every new game goes through eleven stages, in this order:**
   1. intake: the statement, the source and its license, time limits, protocol and the rules of each league, gathered with their sources;
   2. rules: `games/<game>/RULES.md` in our own words, with a source for every rule;
   3. model: `games/<game>/MODEL.md`, the problem in plain English, then a short formal part (sets, transition, payoff, the class of game);
   4. framework gaps: what the framework cannot do yet for this game, each gap decided in a decision record before the game's code needs it;
   5. referee and engine: a readable referee, a fast engine, parity tests, a speed baseline, and a `rules-check` bot the owner pastes to compare our engine with CodinGame's;
   6. arena and baseline bots: the game's arena, simple bots, and its numbers (seat advantage, game length, draw rate);
   7. studio: the game's adapter and renderer, replay and analysis first, human play when the turn format allows it;
   8. research: CodinGame write-ups and academic papers, ranked candidate approaches with sources;
   9. approach and first bot: two or three quick prototypes compared in the arena (a bake-off), then the first release;
   10. iteration: experiments judged by the SPRT ([ADR 0012](0012-evaluation.md)); machine learning only once search stops improving;
   11. retrospective: what was reusable goes into the framework and the skills, and the lessons into the playbook.
2. **The owner names each game and decides at the end of every stage.** No game is compared or chosen by Claude; the goal is Legend first, after which the owner pauses or continues, and he sets the stopping rule of each game. One game is worked on at a time.
3. **Only CodinGame's multiplayer bot games, outside contests.** A contest's rules forbid what this repository does (help from a machine, public solutions). Games for two players come first; games for three or four players and games with hidden information are wanted later, and simultaneous moves are undecided. Each needs a framework decision record at stage 4 before the game's work goes on.
4. **A source under the GPL, or with no license, is read for facts only.** Rules are written in our own words and the code is written for this repository; no code is translated or copied from it. Other players' published bots may be read for ideas, never copied.
5. **The owner pastes CodinGame's pages** (statements, league rules, forum write-ups) into the conversation; the intake lists what to paste.
6. **Research is per game, in `games/<game>/research/`, and generic methods are shared in `docs/research/`.** Each research note lists its sources and what was taken from each. A focused scan of about 10 to 15 sources is the default; a multi-agent deep research runs only when the owner asks.
7. **The lifecycle is written once and followed per game.** A playbook, `docs/NEW_GAME.md`, gives each stage's outputs, checklist and gate; each game's state lives in `games/<game>/ROADMAP.md`, and the main roadmap has one line per game. Small skills cover the stages that repeat, and a skill `new-game` reads the game's roadmap and names the next stage. `docs/WORKFLOW.md` describes the lifecycle as part of the way of working.
8. **Ultimate Tic-Tac-Toe is brought to the same standard as the worked example,** with its model, a research note, a roadmap and a retrospective; documents only.

## Consequences

- A new game starts from a checklist instead of a conversation, and its state survives between sessions in its own roadmap.
- The owner decides at eleven gates per game, which is slower than deciding at a few, but no stage runs ahead of him.
- Writing every game's rules and code ourselves costs more than translating a GPL referee, and the referee can differ from CodinGame's; the `rules-check` bot and the parity tests catch that.
- Research comes after the studio, so the game can be watched before approaches are chosen.
- Games with three or four players, hidden information or simultaneous moves wait for framework work and their own records.
- The playbook, the skills, the template of a game's roadmap and `docs/research/` must now be written, and Ultimate Tic-Tac-Toe documented to match.
