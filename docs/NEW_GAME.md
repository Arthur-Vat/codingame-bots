# New game playbook

How a new CodinGame game goes from its statement to a bot in Legend, stage by stage ([ADR 0035](adr/0035-new-game-lifecycle.md)). The owner names the game and decides at the end of every stage; nothing moves to the next stage without his answer in the conversation.

## How it is organised

- **This playbook** says what each stage produces, its checklist and its gate. It changes through pull requests, like any doc.
- **The game's roadmap**, `games/<game>/ROADMAP.md`, holds the game's state: the stages with their check boxes, the owner's answers at each gate, and what comes next. It starts from [templates/game-roadmap.md](templates/game-roadmap.md). The main [roadmap](ROADMAP.md) has one line per game.
- **Skills** carry the stages that repeat: `new-game` reads the game's roadmap and names the next stage and its skill; `game-intake`, `game-model`, `rules-check`, `game-research` and `game-retrospective` do their stages. The other stages use the usual skills: `land-work` and `review-pr` for pull requests, `run-experiment`, `training-run` and `deliver-release` for bots.
- **`HANDOFF.md`** says which stage waits for the owner, as for any other work ([ADR 0024](adr/0024-handoff-branch.md)).

## Scope

- CodinGame multiplayer bot games, outside contests: a contest's rules forbid help from a machine and public solutions.
- One game at a time. The goal is Legend first; then the owner pauses or continues, and he sets each game's stopping rule.
- Two players today. Three or four players, hidden information and simultaneous moves each need a framework decision at stage 4.
- Sources under the GPL or with no license are read for facts only; other players' bots may be read for ideas, never copied.
- Claude does not open codingame.com: the owner pastes its pages into the conversation.

## The stages

| # | Stage | Skill | Output | Gate (the owner decides) |
| --- | --- | --- | --- | --- |
| 1 | Intake | `game-intake` | `games/<game>/README.md` facts with sources, `games/<game>/ROADMAP.md` | Facts complete; anything unknown named |
| 2 | Rules | `rules-reviewer` agent | `games/<game>/RULES.md` | Every rule sourced; ambiguities listed |
| 3 | Model | `game-model` | `games/<game>/MODEL.md` | The model and its numbers read |
| 4 | Framework gaps | `game-model`, `write-adr` | Gap list in `MODEL.md`; a decision record per gap | Each gap decided |
| 5 | Referee and engine | `land-work`, `rules-check` | Referee, engine, parity tests, speed baseline, `rules-check` bot | CI green; `rules-check` agrees with CodinGame |
| 6 | Arena and baseline bots | `land-work` | Arena binary, simple bots, evaluation settings, the game's numbers | Numbers recorded |
| 7 | Studio | `land-work` | Adapter and renderer; replay and analysis, human play when the turn format allows it | Tried in the studio |
| 8 | Research | `game-research` | `games/<game>/research/` with ranked approaches | Approaches to prototype chosen |
| 9 | Approach and first bot | `land-work`, `run-experiment` | Two or three prototypes compared in the arena, then `<game>-v001` | Approach chosen; v001 pasted on CodinGame |
| 10 | Iteration | `run-experiment`, `training-run` | Experiments in the journal, releases | Legend, then the owner's call |
| 11 | Retrospective | `game-retrospective` | `games/<game>/RETROSPECTIVE.md`, framework and playbook changes proposed | Changes approved |

Each stage's work goes through pull requests as usual. A stage ends when its outputs are merged and the owner has answered at its gate; his answer is recorded in the game's roadmap with its date.

## Checklists

### 1. Intake

- The pages the owner pasted: the statement, each league's rules (lower leagues often play simplified rules), and the forum thread of the game.
- The game's source, its license and where the statement links to it.
- Players, turn format (one after the other or simultaneous), what a turn's answer is (one command or many), hidden information, randomness.
- Time limits for the first turn and the others; output format and what CodinGame accepts.
- The leagues and when the full rules apply; the number of players on the leaderboard.
- The owner's four criteria, as notes: interest and research value, source and license, how it shows in the studio, depth of the leaderboard.
- Anything unknown is listed as a question for the owner, never guessed.

### 2. Rules

- `RULES.md` in our own words, a source for every rule ([games/uttt/RULES.md](../games/uttt/RULES.md) is the example).
- The protocol: inputs, outputs, what makes an answer invalid.
- End of game, scoring and tiebreaks, with their edge cases.
- Every point the statement and the source disagree on, and which one applies.
- Reviewed by the `rules-reviewer` agent.

### 3. Model

From [templates/game-model.md](templates/game-model.md): the problem in plain English, then a short formal part.

- State: what it holds, its size in bytes, what is hidden from whom.
- Actions: one per turn or a set of commands, the branching factor at the start, middle and end of games (measured once the engine exists, estimated before).
- Transitions: deterministic or random, alternating or simultaneous.
- Outcome and score; game length in turns.
- Time per turn, and what fits in a 100,000-byte bot (tables, weights).
- Symmetries and what can be precomputed.
- The class of game and the families of methods that fit it (to be ranked at stage 8, not chosen here).

### 4. Framework gaps

Check the game against what the framework assumes today, and list each gap:

- `cg-arena`: two seats (`[BotSpec; 2]`, a seat's opponent is `1 - seat`), seat-swapped pairs for the SPRT.
- `cg-search`: a two-player game of perfect information where players take turns.
- The studio: two seats named X and O, and a human's legal moves listed as complete answers.
- `cg-bundler` and the 100,000-byte limit.

Each gap is a decision for the owner (skill `ask-decision`), recorded with skill `write-adr` before the game's code needs it.

### 5. Referee and engine

- A readable referee implementing `Referee`, written from `RULES.md`, never from copied code.
- A fast, standard-library engine; parity tests against the referee on many random games; property tests.
- A speed baseline in the game's README (`cargo run --release -p <game>-engine --example speed`).
- A `rules-check` bot (skill `rules-check`), pasted by the owner on CodinGame in the league where the full rules apply.

### 6. Arena and baseline bots

- The game's arena binary and `evaluation.env`.
- Bots: first valid, random, a greedy one.
- The game's numbers from the arena: seat advantage, game length, draw rate, and how the SPRT's openings or seeds are drawn.
- CI: the game's `ci.sh` and its scope in `scripts/ci-scopes.sh`.

### 7. Studio

- The adapter crate `games/<game>/studio` and the renderer in `studio/web/src/games/<game>/` ([studio/README.md](../studio/README.md), "How a game plugs in").
- Replay and bot against bot first; human play when the turn format allows it.
- Any gap found at stage 4 for the studio is handled here.

### 8. Research

- CodinGame write-ups (post-mortems on the forum and players' blogs) and academic papers; other players' published code may be read for ideas.
- A focused scan of about 10 to 15 sources; a multi-agent deep research only when the owner asks.
- Output in `games/<game>/research/`, from [templates/research-note.md](templates/research-note.md): sources and what was taken from each, then candidate approaches ranked by expected strength and cost. Generic methods go into [research/](research/README.md) and are linked.

### 9. Approach and first bot

- Two or three quick prototypes of the top approaches, played against each other and the baseline bots in the arena.
- The owner chooses; the first real bot becomes `<game>-v001` (`scripts/new-release.sh`) and the owner pastes it.

### 10. Iteration

- Experiments one at a time, each judged by the SPRT (`docs/WORKFLOW.md`, "An experiment, step by step").
- Machine learning only once search stops improving.
- After Legend, the owner decides whether to continue.

### 11. Retrospective

- What was game-specific and what could serve the next game.
- Changes proposed to the framework, the skills and this playbook, each as its own pull request.

## A game's documents

```text
games/<game>/
├─ README.md          facts, layout, bots, how to play matches; current release
├─ ROADMAP.md         the game's stages, gates and owner's answers
├─ RULES.md           the rules in our own words, with sources
├─ MODEL.md           the problem modelled, and the framework gaps
├─ research/          README.md with the sources and ranked approaches
├─ RETROSPECTIVE.md   written at stage 11
├─ journal/           one entry per experiment
└─ ...                referee/, engine/, arena/, bots/, releases/, studio/
```
