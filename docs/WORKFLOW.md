# Workflow

How the owner and Claude work together. The owner spends a few hours a week deciding; Claude writes all the code; CI judges every result.

## Where things happen

- **Conversation:** the owner and Claude work in a claude.ai conversation, the hub, until Claude Code Projects is available on the owner's account, then in a project named "CodinGame Bot Lab" ([ADR 0008](adr/0008-claude-hub.md)). The owner asks for work, reads results and decides there.
- **Repository:** the durable record. Decisions go into ADRs, plans into `docs/`, experiment results into the journal, reports of training runs into `games/<game>/training/`. Nothing important lives only in a conversation.
- **GitHub:** Claude opens a pull request from a `claude/` branch for every change. CI and the SPRT workflow judge it; the owner approves the merge in the conversation and Claude carries it out ([ADR 0023](adr/0023-chat-approved-merges.md)). The owner rarely needs to open GitHub for anything else; notifications for this repository are set to "Participating" to avoid duplicates.
- **CodinGame:** the owner pastes released bots and reports their rank ([ADR 0009](adr/0009-manual-submission.md)). Claude records the rank in the release's journal entry.

## A round of work

The owner starts each round in the conversation; a daily scheduled session comes with phase 5 (below). For bot strength, a round looks like this (about one to two hours of the owner's time):

1. Claude reports the results since the last round and proposes two or three experiments, or the owner describes an idea.
2. Claude implements each experiment in its own pull request; CI runs the tests and the SPRT workflow runs the smoke test and the SPRT.
3. The owner approves merging accepted candidates. A rejected one comes back reduced to its journal entry, which is merged too, so the failure stays on record.
4. When a release is published, the owner pastes its file (attached to its GitHub release) into CodinGame and reports the rank.

Other work, such as tooling, documentation or training runs, takes the same path: a pull request from a `claude/` branch, CI as judge, the owner approving the merge. Bot strength work is paused from 2026-10-09 while phase 5 is built.

## An experiment, step by step

The rules are in [ADR 0012](adr/0012-evaluation.md).

1. On a branch `claude/<game>-eNNN-<short-name>`, change the bot and screen it locally against the current release with the game's arena (20 ms moves are enough to screen, [ADR 0020](adr/0020-full-time-sprt-in-legend.md)). Then freeze it as the candidate: `scripts/new-release.sh <game> <bot>` writes `games/<game>/releases/<game>-vNNN.rs`; point the game README's `**Current release:**` line and the root README's games table to it (`scripts/check-docs.sh` checks them).
2. Start the journal entry `games/<game>/journal/ENNN-<short-name>.md` from the template: hypothesis, change, and the line `release: <game>-vNNN`. Open the pull request.
3. CI checks the release, and the SPRT workflow runs the smoke test and the SPRT against the previous release, then posts the result on the pull request. Once the game's bot is in Legend, both play at CodinGame's exact time limits ([ADR 0020](adr/0020-full-time-sprt-in-legend.md)); below Legend they play at 20 ms, and an accepted candidate is then confirmed at full time if the game asks for it. Locally, `scripts/sprt.sh <game> <candidate.rs>` runs the same test.
4. Claude copies the result into the entry. Only Markdown changes, so the SPRT workflow reuses its result instead of running again. Re-running the SPRT job on GitHub does test again, for a failure caused by the machine; every test posts its result on the pull request.
5. Accepted: merged on the owner's approval; the release workflow publishes the version and the league workflow updates the ratings. Rejected or inconclusive: Claude removes the release file and the bot change, sets `release: none` and `decision: dropped`, and the entry is merged on the owner's approval.

## Rules for every change

- Claude works on `claude/`-prefixed branches and opens a pull request. It merges only what the owner approved in the conversation, and records the approval on the pull request ([ADR 0006](adr/0006-human-approves-merges.md), [ADR 0023](adr/0023-chat-approved-merges.md)).
- One topic per pull request; one experiment per candidate pull request.
- A decision that is costly to reverse is proposed to the owner first, then recorded as an ADR.
- Tests, the referee and CI checks are never weakened to make a change pass.
- Pull request titles and commit subjects name a game or a framework area as their scope; decision records name a game or `framework` ([ADR 0022](adr/0022-scopes-and-names.md)). A workflow labels pull requests by the paths they change and checks their titles.
- Finished branches are deleted ([ADR 0021](adr/0021-prune-finished-branches.md)): merged ones by GitHub on merge, the others by the weekly Prune branches workflow once their content is safe. When Claude reads a training run, it copies the run's `report.md` to `games/<game>/training/runs/<run>/` in the next pull request, so that the run's branch can go.

## Claude Code setup (Phase 5)

Phase 5 has been in progress since 2026-10-09. Its goal is that the owner only arbitrates and Claude moves work forward between the owner's visits. The plan, with its checklist and gate, is in the [roadmap](ROADMAP.md#phase-5-autonomy-and-workflow). In short:

- **Instructions:** `CLAUDE.md` holds the rules for every session.
- **Agents** in `.claude/agents/`: `implementer` (Sonnet) codes to a precise spec; `pr-reviewer` (Opus) reviews every pull request at a depth matched to its risk; `rules-reviewer` (Opus) checks engine and referee changes against the game's `RULES.md`. Claude's main session plans, splits the work, checks the results and merges what the owner approved.
- **Skills** in `.claude/skills/`: the steps that repeat. `pr-train` plans and lands work across pull requests, `review-pr` chooses the review's depth, `github-api` has the recipes that work from Claude's sessions, `ask-decision` and `write-adr` bring decisions to the owner and record them; `run-experiment`, `deliver-release`, `training-run`, `docs-refresh` and `daily-report` cover the rest.
- **Merge policy:** the owner approves every merge in the conversation and Claude carries it out ([ADR 0023](adr/0023-chat-approved-merges.md)). The plan lets Claude merge some classes of change without asking, in a later decision record, and keeps the rest with the owner.
- **Decisions:** asked in the conversation, each with options and a recommendation; when the owner is away, one push notification and the daily report say one is waiting. Only answers in the conversation count.
- **Scheduled session:** one quick session a day, within the owner's usage limits: a daily report of the day's work, areas of improvement, and hot fixes and evolutions to propose; it changes nothing and never merges ([ADR 0023](adr/0023-chat-approved-merges.md)).
