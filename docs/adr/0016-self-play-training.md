# 0016. Train from self-play on GitHub Actions, with a pure-Rust tool

- Status: accepted; decision 2 completed by [0021](0021-prune-finished-branches.md) (reports copied to `main`, branches deleted)
- Date: 2026-10-08
- Scope: framework

## Context

The experiments so far tuned a Monte Carlo tree search with uniformly random playouts. By E010's profile, it is close to its speed floor: about 0.75 µs per iteration, 60% of it in playouts, with at most 5 to 10% left to gain without changing the moves played. Exploration and draw proofs (E008, E009) gave nothing measurable. What did pay was knowledge in the playouts: taking a game-winning move (E005) was worth +78 Elo at full time.

More knowledge has to be learned, not guessed. E005 tried two rules picked by hand, avoiding the opponent's game-winning boards and grabbing small boards, and both made the bot weaker. The owner agreed on 2026-10-08 to learn it in stages:

1. **A playout policy:** a few cheap features of each move, with weights fitted to what a long search chooses. Its weights are a few dozen numbers.
2. **A value network** replacing playouts with an evaluation, if the pipeline and stage 1 show it can pay off.
3. **A policy and value network searched together, AlphaZero-style,** if stage 2 pays off.

Each stage needs positions from self-play, searched longer than a turn allows, and a training step. That takes far more computing than this sandbox offers: 2 cores, and reclaimed when idle, which killed background runs on 2026-10-08. GitHub Actions is free for standard runners in public repositories. Each job has 4 cores and may run for 6 hours, and the Free plan runs 20 jobs at once and up to 256 jobs per run ([Limits in GitHub Actions](https://docs.github.com/en/actions/reference/limits), [About billing for GitHub Actions](https://docs.github.com/en/actions/reference/usage-limits-billing-and-administration), checked 2026-10-08). There are no GPUs, and none are needed for networks small enough to fit the 100 kB file: a few tens of thousands of weights.

From this sandbox, Claude cannot download GitHub's job logs or artifacts; it can fetch git branches and read pull request comments.

## Decision

1. **Training runs on GitHub Actions, started by hand.** A `train` workflow, run only on manual dispatch, plays self-play games in a matrix of parallel jobs (each with its own seed), then trains in one job that collects their data. Its inputs (games, jobs, search length) are set when it is started. It never runs on pushes or pull requests.
2. **Results come back as a commit on a `claude/` branch.** The training job commits the trained weights and a report to a new branch `claude/train/<run>`, with write access given to that job only; it never writes to `main`. The report records the commit trained from, the seeds, the parameters and the training measures, so any run can be repeated. Claude fetches the branch, and the weights reach a bot only through an experiment pull request and its SPRT, as for any other change.
3. **The tool is a Rust crate of the workspace, `uttt-trainer`, without a machine-learning library.** It reuses the engine and the search, so the positions and moves it learns from are exactly those the bot sees. The models of the three stages are small enough to train with hand-written gradient descent, which avoids large dependencies and their review under `deny.toml`. A tool may still take dependencies that `deny.toml` allows, if a stage needs one.
4. **Weights live in the bot's source as constants.** The bot stays one standard-library file and loads nothing at run time; the weights count toward the 100 kB. When a stage needs the room, removing comments and indentation in the bundler frees about 37 kB; that change gets its own pull request.
5. **Stage 1 starts now.** Stages 2 and 3 each start with a proposal to the owner, saying what network, how fast it evaluates and how many positions per turn it leaves; an ADR records the decision if it is costly to reverse.

## Consequences

- Hours of self-play cost nothing, and do not depend on this sandbox staying awake.
- A training run is slower to start and to inspect than a local one: the report is how Claude and the owner see what happened. Short checks still run locally.
- A workflow can write to the repository. It is limited to new `claude/` branches, on runs the owner or Claude starts.
- Learned knowledge is judged like any other change: by an SPRT against the current release and a confirmation at CodinGame's limits.
