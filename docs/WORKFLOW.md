# Workflow

How the owner and Claude work together. The owner spends a few hours a week deciding; Claude writes all the code; CI judges every result.

## Where things happen

- **Hub:** one claude.ai conversation until Claude Code Projects is available on the owner's account, then a project named "CodinGame Bot Lab" ([ADR 0008](adr/0008-claude-hub.md)). The owner asks for work, reads results and approves merges there.
- **Repository:** the durable record. Decisions go into ADRs, plans into `docs/`, experiment results into the journal. Nothing important lives only in a conversation.
- **GitHub:** pull requests and CI. The owner rarely needs to open it; notifications for this repository are set to "Participating" to avoid duplicates.
- **CodinGame:** the owner pastes released bots and reports their rank ([ADR 0009](adr/0009-manual-submission.md)).

## The weekly loop

Once Phase 4 starts, a week looks like this (about one to two hours of the owner's time):

1. Read the weekly report: results since last week and two or three proposed experiments.
2. Pick experiments or describe new ideas.
3. Claude implements each experiment in its own pull request; CI runs the tests and the SPRT.
4. Approve the merge of accepted candidates. A rejected one comes back reduced to its journal entry: merge that too, so the failure stays on record.
5. When a new champion is released, paste its file (attached to its GitHub release) into CodinGame and report the rank.

## An experiment, step by step

The rules are in [ADR 0012](adr/0012-evaluation.md).

1. On a branch `claude/<game>-eNNN-<short-name>`, change the bot, then freeze it as the candidate: `scripts/new-release.sh <game> <bot>` writes `games/<game>/releases/<game>-vNNN.rs`.
2. Start the journal entry `games/<game>/journal/ENNN-<short-name>.md` from the template: hypothesis, change, and the line `release: <game>-vNNN`. Open the pull request.
3. CI checks the release, and the SPRT workflow runs the smoke test and the SPRT against the previous release, then posts the result on the pull request. Once the game's bot is in Legend, both play at CodinGame's exact time limits ([ADR 0020](adr/0020-full-time-sprt-in-legend.md)); below Legend they play at 20 ms, and an accepted candidate is then confirmed at full time if the game asks for it. Locally, `scripts/sprt.sh <game> <candidate.rs>` runs the same test.
4. Claude copies the result into the entry. Only Markdown changes, so the SPRT workflow reuses its result instead of running again. Re-running the SPRT job on GitHub does test again, for a failure caused by the machine; every test posts its result on the pull request.
5. Accepted: the owner merges; the release workflow publishes the version and the league workflow updates the ratings. Rejected or inconclusive: Claude removes the release file and the bot change, sets `release: none` and `decision: dropped`, and the owner merges the entry.

## Rules for every change

- Claude works on `claude/`-prefixed branches and opens a pull request. It never merges without the owner's explicit go ([ADR 0006](adr/0006-human-approves-merges.md)).
- One topic per pull request; one experiment per candidate pull request.
- A decision that is costly to reverse is proposed to the owner first, then recorded as an ADR.
- Tests, the referee and CI checks are never weakened to make a change pass.
- Pull request titles and decision records name their scope, a game or a framework area ([ADR 0022](adr/0022-scopes-and-names.md)); a workflow labels pull requests by the paths they change.
- Finished branches are deleted ([ADR 0021](adr/0021-prune-finished-branches.md)): merged ones by GitHub on merge, the others by the weekly Prune branches workflow once their content is safe. When Claude reads a training run, it copies the run's `report.md` to `games/<game>/training/runs/<run>/` in the next pull request, so that the run's branch can go.

## Claude Code setup (Phase 5)

- **Project instructions** (when Projects is available): the goal, the repository, and the rules above.
- **Skills** in `.claude/skills/`: `new-experiment` (hypothesis to pull request with a journal draft), `new-game` (rules to engine to parity tests), `promote-version`, `weekly-report`.
- **Subagent** `rules-reviewer`: checks every engine change against the game's `RULES.md`.
- **Weekly routine:** writes the weekly report and proposes the next experiments. One run a week keeps plan usage low.
