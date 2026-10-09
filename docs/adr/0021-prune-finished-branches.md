# 0021. Delete finished branches, keeping their content

- Status: accepted (owner, 2026-10-09)
- Date: 2026-10-09
- Completes: decision 2 of [ADR 0016](0016-self-play-training.md), on where training results end up
- Scope: framework

## Context

On 2026-10-09 the owner asked that leftover branches be deleted, in a way that also handles the next ones.

The repository already deletes a pull request's branch when it is merged (GitHub's "Automatically delete head branches" setting). Two kinds of branches stay behind:

- **Training results.** The Train workflow commits each run's weights and report to a new branch `claude/train/<run>` ([ADR 0016](0016-self-play-training.md)); no pull request comes from it. After 2026-10-09, four were left, holding the only copies of their reports.
- **Branches of pull requests closed without merging**, such as the archive of E016's implementation (#38).

Claude cannot delete branches: its sessions may push only to `claude/` branches and not delete them. A branch that is deleted takes with it any commit that nothing else refers to, so deleting must not lose anything: GitHub keeps the commits of every pull request, closed or merged, under `refs/pull/<number>/head`, and the reports are small text files that `main` can hold.

## Decision

1. **Reports go to `main`.** When Claude reads a training run, its `report.md` is copied to `games/<game>/training/runs/<run>/report.md` in the next pull request that uses the run, or in a documentation one. The weights stay on the branch: the ones a bot uses are in the bot, and any run can be repeated from the commit and seeds its report records.
2. **A workflow deletes finished branches**, weekly and by hand (`.github/workflows/prune-branches.yml`, `scripts/prune-branches.sh`):
   - `claude/train/<run>` once its report is on `main`;
   - any other `claude/` branch once all its pull requests are closed and it still points at the last one's head, so that the pull request keeps its commits.
3. **Every other branch stays:** `main`, branches outside `claude/`, branches with an open pull request or none yet, and branches with commits pushed after their pull request closed. A run by hand only lists what it would delete unless asked to delete.

## Consequences

- The branch list holds only work in progress.
- A dropped experiment's code stays reachable through its closed pull request, as E016's is through #38.
- Training reports are reviewed with the pull request that copies them, and a run whose report was never copied keeps its branch: the workflow's summary lists such branches.
- The workflow holds write access to the repository's contents, used only to delete the branches above.
