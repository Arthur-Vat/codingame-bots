# Roadmap

Seven phases build the framework before the first real bot. Reaching Legend is the ongoing result of the experiment loop, not a phase. Each phase ends at a gate that CI or CodinGame can check. Game counts in the gates are proposed values.

| Phase | Scope | Gate | Status |
| --- | --- | --- | --- |
| 0. Foundations | Repository, licenses, ADRs, `CLAUDE.md`, CI skeleton, compiler target | CI green; a hello-world bot runs on CodinGame | Done (2026-10-07) |
| 1. Framework core | `Referee` trait, arena, bundler, `cg-core`; UTTT `RULES.md`, reference referee, random bot | 1,000 random-vs-random games in CI; a bundled bot plays on CodinGame | Done (2026-10-07) |
| 2. UTTT engine | Fast engine for bots, property tests, parity with the referee, rules checked against real CodinGame games | Parity on 10,000 random games; speed baseline recorded | Done (2026-10-07) |
| 3. Evaluation | Openings, smoke tests, SPRT, ratings, league, journal, releases | A/A test passes; a weakened bot is rejected by the SPRT | Done (2026-10-07) |
| 4. First real bot | MCTS `uttt-v001`, then the weekly experiment loop | v001 pasted and ranked; 3 experiments run end to end | In progress |
| 5. Autonomy | Hub, project skills, weekly routine, notifications | One full loop without opening GitHub | Not started |
| 6. Second game | A game with an official referee; `new-game` skill | Phase 3 gates met, changes mostly inside the game folder | Not started |
| Ongoing | Iterate every game toward Legend | | |

Phases 4 and 5 can overlap.

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
- [ ] Release `uttt-v001` through the SPRT (first experiment, E001)
- [ ] v001 pasted into CodinGame and ranked (owner)
- [ ] Two more experiments run end to end
- [ ] Opening length checked with MCTS (ADR 0012)

## Risks

| Risk | Impact | Mitigation |
| --- | --- | --- |
| CodinGame changes its Rust version | Code that builds locally fails on CodinGame | `rust-version` plus CI on 1.90.0 and the standalone compile; re-run the probe in [CODINGAME.md](CODINGAME.md) after language updates |
| Our engine differs from CodinGame's | The bot wins locally and loses on CodinGame | Parity tests, checks against real CodinGame games, dedicated tiebreak tests |
| Timing noise on shared runners | Timeouts and wrong SPRT verdicts | One game per core, A/A test in CI, time scale revisited with the first real bot ([ADR 0012](adr/0012-evaluation.md)) |
| Claude plan usage limits | Slower iteration | Actions does all the compute; one weekly routine; smaller models for routine work |
| CodinGame rules on outside help | Problems in prize contests | Multiplayer leaderboards only; read a contest's rules before entering it ([ADR 0009](adr/0009-manual-submission.md)) |
| Projects beta not available | No single hub with an Overview | One long claude.ai conversation meanwhile ([ADR 0008](adr/0008-claude-hub.md)) |
| Claude workspace cannot download Rust toolchains | Local checks run on a newer Rust than CodinGame | `rust-version` lints locally; CI on exactly 1.90.0 is the judge ([ADR 0010](adr/0010-codingame-rust-toolchain.md)) |
| Public code gets copied | Others use these bots | Accepted ([ADR 0002](adr/0002-public-repo.md)) |
