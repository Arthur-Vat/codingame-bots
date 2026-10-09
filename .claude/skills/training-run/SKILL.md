---
name: training-run
description: Starts a game's Train workflow on GitHub Actions with chosen inputs, follows it, and brings its report back into the repository. Use only when the owner has asked for a training run.
---

# Run a training run

Training runs on GitHub Actions (ADR 0016) and writes its weights and report to a branch `claude/train/<run id>`; it never writes to `main`. Background compute is off until the owner turns it on, so start a run only when he asked for it.

## Start

1. Choose the inputs of `.github/workflows/train.yml` (stage `policy`, `value` or `patterns`; jobs, games, iterations, and the stage's own inputs). `data_from_run` refits on an earlier run's self-play data, kept 7 days, without playing again.
2. Say the inputs and the expected duration to the owner before starting if they differ from what he approved.
3. Dispatch it (skill `github-api`, "Workflow runs"), then find the run's id.

## Follow

- Poll the run every few minutes; self-play jobs and the fit take from minutes to hours.
- Logs cannot be read from the session: read the jobs' annotations, where the workflow reports results and push errors.

## Bring the results back

1. Fetch `claude/train/<run id>` and read its `report.md`: the commit, seeds, settings and measurements.
2. In the next pull request that uses the run, copy the report to `games/<game>/training/runs/<run id>/report.md` and add its row to `games/<game>/training/README.md` (what it trained, which experiment or decision used it). `scripts/check-docs.sh` checks the index.
3. Weights reach a bot only through an experiment and its SPRT (skill `run-experiment`). The run's branch is deleted by the Prune branches workflow once its report is on `main` (ADR 0021).
