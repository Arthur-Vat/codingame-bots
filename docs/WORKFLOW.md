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
4. Approve the merge of accepted candidates. Rejected ones are closed and keep their journal entry.
5. When a new champion is released, paste its file into CodinGame and report the rank.

## Rules for every change

- Claude works on `claude/`-prefixed branches and opens a pull request. It never merges without the owner's explicit go ([ADR 0006](adr/0006-human-approves-merges.md)).
- One topic per pull request; one experiment per candidate pull request.
- A decision that is costly to reverse is proposed to the owner first, then recorded as an ADR.
- Tests, the referee and CI checks are never weakened to make a change pass.

## Claude Code setup (Phase 5)

- **Project instructions** (when Projects is available): the goal, the repository, and the rules above.
- **Skills** in `.claude/skills/`: `new-experiment` (hypothesis to pull request with a journal draft), `new-game` (rules to engine to parity tests), `promote-version`, `weekly-report`.
- **Subagent** `rules-reviewer`: checks every engine change against the game's `RULES.md`.
- **Weekly routine:** writes the weekly report and proposes the next experiments. One run a week keeps plan usage low.
