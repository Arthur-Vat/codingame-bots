# Roadmap

Phases 0 to 3 built the framework and phase 4 the first real bot; phase 5 makes Claude's work more autonomous, and phase 6 adds a second game. Reaching Legend is the ongoing result of the experiment loop, not a phase. Each phase ends at a gate that CI or CodinGame can check. Game counts in the gates are proposed values.

| Phase | Scope | Gate | Status |
| --- | --- | --- | --- |
| 0. Foundations | Repository, licenses, ADRs, `CLAUDE.md`, CI skeleton, compiler target | CI green; a hello-world bot runs on CodinGame | Done (2026-10-07) |
| 1. Framework core | `Referee` trait, arena, bundler, `cg-core`; UTTT `RULES.md`, reference referee, random bot | 1,000 random-vs-random games in CI; a bundled bot plays on CodinGame | Done (2026-10-07) |
| 2. UTTT engine | Fast engine for bots, property tests, parity with the referee, rules checked against real CodinGame games | Parity on 10,000 random games; speed baseline recorded | Done (2026-10-07) |
| 3. Evaluation | Openings, smoke tests, SPRT, ratings, league, journal, releases | A/A test passes; a weakened bot is rejected by the SPRT | Done (2026-10-07) |
| 4. First real bot | MCTS `uttt-v001`, then the weekly experiment loop | v001 pasted and ranked; 3 experiments run end to end | Done (2026-10-07) |
| 5. Autonomy and workflow | Conventions, CI checks, project agents and skills, merge policy, a daily scheduled session within the owner's usage limits, arena and ratings | One week of daily scheduled sessions with the owner answering only decisions | In progress (2026-10-09) |
| 6. Second game | A game with an official referee; `new-game` skill | Phase 3 gates met, changes mostly inside the game folder | Not started |
| Ongoing | Iterate every game toward Legend, then up the Legend ranking | | Ultimate Tic-Tac-Toe: Legend since 2026-10-07 (`uttt-v001`, 182 of 443); 59th with `uttt-v008` (2026-10-08); current release `uttt-v010`; strength work paused (2026-10-09) |

## Phase 0: foundations

- [x] Public repository `Arthur-Vat/codingame-bots` ([ADR 0002](adr/0002-public-repo.md))
- [x] Licenses `LICENSE-MIT` and `LICENSE-APACHE` ([ADR 0001](adr/0001-license.md))
- [x] Decisions recorded as ADRs 0001 to 0010
- [x] Plan moved into `docs/` (this file, [ARCHITECTURE.md](ARCHITECTURE.md), [WORKFLOW.md](WORKFLOW.md), [CODINGAME.md](CODINGAME.md))
- [x] `CLAUDE.md` with commands and guardrails
- [x] Cargo workspace targeting Rust 1.90.0 and the 2021 edition ([ADR 0010](adr/0010-codingame-rust-toolchain.md))
- [x] Hello-world bot `games/uttt/bots/first-valid` with tests
- [x] CI: format, lint, test, CodinGame compatibility, dependency policy
- [x] CI green on the Phase 0 pull request
- [x] Ruleset on `main`: require the CI checks, block direct pushes (owner, 2026-10-07)
- [x] `first-valid` pasted into CodinGame and plays a full game (owner, 2026-10-07)

## Phase 1: framework core

- [x] `games/uttt/RULES.md` from the game's statement and official source ([ADR 0005](adr/0005-first-game-uttt.md))
- [x] `cg-core`: input reader and seeded random generator for bots
- [x] `cg-arena`: `Referee` trait, match runner (timeouts, crashes, invalid answers), seat-swapped tournaments, JSON lines, summary with an Elo estimate ([ADR 0011](adr/0011-framework-structure.md))
- [x] `uttt-referee` (reference rules) and `uttt-arena`
- [x] `random` bot built on `cg-core`
- [x] `cg-bundler` and `scripts/bundle-bots.sh`; CI bundles every bot, compiles the bundles on their own and plays 1,000 games between them
- [x] CI green on the Phase 1 pull request
- [x] The bundled `random` bot pasted into CodinGame and plays a full game (owner, 2026-10-07)

## Phase 2: UTTT engine

- [x] `uttt-engine`: bitboard position, move generation without allocation, random playouts; std only, bundled into bots
- [x] Property tests on 2,000 random games and scenario tests for every rule
- [x] Parity with the reference referee on 10,000 random games (valid actions, small-board winners, points, results)
- [x] Speed baseline: about 440,000 random playouts per second on one thread ([games/uttt/README.md](../games/uttt/README.md)); CI reports it on every push
- [x] `wood` bot: perfect 3×3 tic-tac-toe, to leave Wood league, where Ultimate is not played yet
- [x] `rules-check` bot: compares CodinGame's valid actions with the engine's every turn
- [x] CI green on the Phase 2 pull request
- [x] Promoted out of Wood league with the `wood` bot (owner, 2026-10-07)
- [x] `rules-check` in Bronze: CodinGame's valid actions matched the engine's on every turn checked (owner, 2026-10-07)

## Phase 3: evaluation

- [x] Arena commands `match`, `sprt` and `league`; bots receive `CG_TIME_SCALE` when time limits are scaled
- [x] Seeded random openings imposed by the UTTT referee, the same for both games of a pair
- [x] Pentanomial SPRT with no verdict before 30 pairs; a simulation test keeps both error rates near 5%
- [x] League ratings: Bradley-Terry maximum likelihood with 95% intervals
- [x] `greedy` bot: one-move lookahead, a fixed baseline about 500 Elo above random
- [x] Releases: `scripts/new-release.sh`; CI keeps them frozen and true to their sources (`scripts/check-releases.sh`)
- [x] Workflows: SPRT of pull requests that add a release (smoke test, comment, result kept across Markdown-only pushes), GitHub release on merge, league after each release
- [x] Evaluation settings in `games/uttt/evaluation.env`; journal README and entry template
- [x] Gate, A/A test: 40 SPRTs of `greedy` against itself, 39 rejected, 1 inconclusive, none accepted; CI repeats one with a fixed seed on every push
- [x] Gate, weaker bot rejected: `random` against `greedy`, in CI on every push
- [x] CI green on the Phase 3 pull request
- [x] [ADR 0012](adr/0012-evaluation.md) accepted by the owner (2026-10-07); time scale and opening length to be revisited with the MCTS bot

## Phase 4: first real bot

- [x] `cg-search`: `Game` trait, UCT Monte Carlo tree search, time budgets from the scaled CodinGame limit or `CG_FIXED_ITERS`
- [x] `uttt-engine` implements the trait; the speed example also measures MCTS iterations
- [x] `mcts` bot: searches each turn from scratch with random playouts; exploration constant 0.5, measured about 160 Elo above 1.0
- [x] Time limits revisited with MCTS: 20 ms plus 5 ms of tolerance in evaluations, full-time calibration once the bot is strong ([ADR 0013](adr/0013-evaluation-time-limits.md))
- [x] Release `uttt-v001` through the SPRT (first experiment, E001): accepted against `greedy`, 60 wins in 60 games
- [x] v001 pasted into CodinGame and ranked (owner, 2026-10-07): promoted from Bronze to Legend on its own, 182 of 443 in Legend
- [x] Full-time confirmation of accepted candidates, set off by reaching Gold ([ADR 0014](adr/0014-full-time-confirmation.md))
- [x] E002, keeping the search tree between turns: accepted, +49.6 Elo at 20 ms and +73.3 at full time (`uttt-v002`)
- [x] Safer time budget, 82 ms of 100, after a timeout at full time on CI; only the candidate's faults fail an evaluation
- [x] E003, proving wins and losses in the tree: accepted, +31.6 Elo at 20 ms and +13.6 at full time (`uttt-v003`)
- [x] SPRT at CodinGame's exact limits for games in Legend, replacing the full-time confirmation ([ADR 0020](adr/0020-full-time-sprt-in-legend.md))
- [ ] Opening length checked with MCTS (ADR 0012), carried into the experiment loop

## Phase 5: autonomy and workflow

Planned with the owner on 2026-10-09. Goal: the owner only arbitrates. Claude plans and reviews with a strong model, implements with cheaper ones, runs on a schedule within the owner's usage limits, and merges what the owner has delegated. Ultimate Tic-Tac-Toe strength work is paused meanwhile, and no training or league run is started by hand, at the owner's request; the League workflow's automatic triggers stay.

Gate: one week of daily scheduled sessions in which Claude moves work forward on its own, the owner answers only decisions, and nothing is merged without the owner's approval or outside the delegated classes.

### A. Repository refresh and conventions

- [x] Docs refreshed: READMEs, architecture, workflow and this roadmap describe the repository as it is
- [x] Leftover class-policy weights moved out of the bot's sources
- [x] Decision records carry a scope (`framework` or a game), and the index groups them by scope ([ADR 0022](adr/0022-scopes-and-names.md))
- [x] Pull request titles name their scope (a game, or a framework area); a workflow labels pull requests by the paths they change and checks titles
- [x] CI checks that indexes and status lines match the repository (releases, decision records, journal, training runs) and that relative links resolve; shellcheck and actionlint run in CI

### B. Agents, skills and the review loop

- [x] Project agents in `.claude/agents/`: `implementer` (Sonnet 5.5) codes to a precise spec; `pr-reviewer` (Opus 5.5, Sonnet 5.5 for light reviews) reviews every pull request but small bookkeeping against the hard rules, decision records, tests, docs and privacy; `rules-reviewer` checks engine changes against the game's `RULES.md`. Claude's main session plans, splits the work and checks the results
- [x] Review depth matched to risk (after step A, 2026-10-09): documentation gets a quick check, or one light review per batch; workflows, permissions and scripts get a deep `pr-reviewer` review; bot changes get the SPRT and `rules-reviewer`. Reviewers get the diff and a checklist rather than exploring, and agents' reports stay under about 200 words. After step C (owner, 2026-10-09): agents get only the tools they need (their unused tools' descriptions cost about 40,000 tokens of context on every step), light reviews run on Sonnet, and small bookkeeping (at most about 20 lines recording decisions) gets no separate review and rides with the next pull request
- [x] Planning before dispatch: decision records and names are settled before implementers start; each spec gives examples of inputs and expected outputs; the merge order of related pull requests (stacks, shared roadmap lines) is planned up front (`pr-train`)
- [x] Usage made visible (owner, 2026-10-09): CI checks that every agent declares its model and tools (`scripts/check-docs.sh`), and each piece of work ends with its token use (`session-usage`)
- [x] Agents limited by a hook (owner, 2026-10-09): `.claude/settings.json` runs `scripts/agent-guard.sh` before every agent start and refuses all but `implementer`, `pr-reviewer`, `rules-reviewer` and the read-only built-in agents (`Explore`, `Plan`, `claude-code-guide`); CI tests the script
- [x] Skills carry the recipes that work in Claude's sessions: pull requests through GitHub's REST API, fetches with explicit refspecs (`github-api`)
- [x] Project skills in `.claude/skills/`: `pr-train` (plan, dispatch, stack and merge), `review-pr`, `github-api`, `ask-decision`, `write-adr`, `run-experiment`, `deliver-release`, `training-run`, `docs-refresh`, `daily-report`, `session-usage`; later `new-game`
- [x] The owner approves merges in the conversation and Claude carries them out ([ADR 0023](adr/0023-chat-approved-merges.md))
- [ ] Postponed by the owner until after step C's trial (2026-10-09). Merge policy (a decision record superseding [ADR 0006](adr/0006-human-approves-merges.md)): Claude may merge without asking docs, journal entries, dropped experiments, tooling that changes no bot, and releases accepted by the full-time SPRT, once required checks pass and `pr-reviewer` approves; the owner merges decision records, `CLAUDE.md`, workflows and permissions, and rules or referee changes
- [x] `Docs and scripts` and the SPRT are required checks, so that broken docs or a rejected release cannot be merged (set by the owner, 2026-10-09; Claude's sessions cannot edit rulesets)
- [x] Decisions reach the owner in the conversation, each with options and a recommendation (`ask-decision`); when he is away, one push notification and the daily report say one is waiting. Answers count only in the conversation: GitHub issues were dropped after review, since Claude's sessions post under the owner's account

### C. Autonomous operation

- [x] Usage guard: the 7-day usage of the owner's plan is not readable from cloud sessions. The status line carries it (`rate_limits.seven_day.used_percentage`), but it does not run in them (tested 2026-10-09). The owner's pause switch replaces the guard: he pauses the scheduled task in the Claude app when his usage runs high, and the run is kept quick and read-only
- [x] One quick scheduled session a day, reduced from four on 2026-10-09 to spare the owner's usage: every day at 06:45, Paris time, the daily report (`daily-report`) of the day's work, areas of improvement, and hot fixes and evolutions to propose (the owner's brief, 2026-10-09). Its final message reaches the owner's phone as a push notification. It changes nothing and never merges ([ADR 0023](adr/0023-chat-approved-merges.md)). Set up 2026-10-09 as the scheduled task "CodinGame bots: daily report", on Sonnet 5.5 at the owner's request
- [ ] One week of trial, from 2026-10-10, then the report's form, the frequency and the merge classes adjusted with the owner

### D. Arena and ratings

Background runs stay off until the owner turns them on.

- [ ] Decision record on the rating model: Bradley-Terry over all games for published ratings, and Glicko-2 uncertainty to choose the next matchups
- [ ] Results kept across runs (per-pair summaries on a data branch outside `claude/`), with a leaderboard in the job summary and the game's README
- [ ] Arena: leagues at CodinGame's limits, adaptive choice of pairs (close ratings, high uncertainty), optional outside or older opponents; scheduled runs, off by default

### E. CI, fixed checks

- [ ] Speed regression check (tier 5 of the evaluation pipeline): the pull request against `main` in the same job, to cancel the runner's noise
- [ ] Documentation-only pull requests skip the heavy steps while still reporting the required checks
- [ ] Agentic checks stay in Claude's sessions (owner's choice, 2026-10-09): no Claude job in GitHub Actions
- [ ] The docs check also verifies that the architecture and the README list every workflow and script
- [ ] Shellcheck also lints the scripts' tests, `scripts/tests/*.sh` (owner, 2026-10-09; they pass it today)
- [ ] Optional: commit subjects checked like pull request titles

Order: A, then B, then C and E side by side, then D.

## Risks

| Risk | Impact | Mitigation |
| --- | --- | --- |
| CodinGame changes its Rust version | Code that builds locally fails on CodinGame | `rust-version` plus CI on 1.90.0 and the standalone compile; re-run the probe in [CODINGAME.md](CODINGAME.md) after language updates |
| Our engine differs from CodinGame's | The bot wins locally and loses on CodinGame | Parity tests, checks against real CodinGame games, dedicated tiebreak tests |
| Timing noise on shared runners | Timeouts and wrong SPRT verdicts | One game per core, A/A test in CI, time limits revisited with the first real bot ([ADR 0013](adr/0013-evaluation-time-limits.md)), CodinGame's exact limits for games in Legend ([ADR 0020](adr/0020-full-time-sprt-in-legend.md)) |
| Claude plan usage limits | Slower iteration | Actions does the compute; one quick daily scheduled session, which the owner pauses when his usage runs high (phase 5); cheaper models implement |
| CodinGame rules on outside help | Problems in prize contests | Multiplayer leaderboards only; read a contest's rules before entering it ([ADR 0009](adr/0009-manual-submission.md)) |
| Projects beta not available | No single hub with an Overview | One long claude.ai conversation meanwhile ([ADR 0008](adr/0008-claude-hub.md)) |
| Claude workspace cannot download Rust toolchains | Local checks run on a newer Rust than CodinGame | `rust-version` lints locally; CI on exactly 1.90.0 is the judge ([ADR 0010](adr/0010-codingame-rust-toolchain.md)) |
| Public code gets copied | Others use these bots | Accepted ([ADR 0002](adr/0002-public-repo.md)) |
