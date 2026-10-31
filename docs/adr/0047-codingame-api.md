# 0047. CodinGame's public API for test games, submissions and ranks

- Status: accepted (owner, 2026-10-31)
- Date: 2026-10-31
- Supersedes: [0009](0009-manual-submission.md) (the owner pastes each release); [0036](0036-ranks-optional.md), decision 1 (ranks reported by the owner)
- Scope: framework

## Context

[ADR 0009](0009-manual-submission.md) had the owner paste each release into the CodinGame IDE because CodinGame had no official API. [ADR 0036](0036-ranks-optional.md) made reporting the resulting rank optional, so the real leaderboard stopped checking the local ratings. Checks that needed CodinGame itself, such as the rules-check bot and the timing probe, also went through the owner's IDE and his pasted logs.

CodinGame now has a public REST API for coding agents ([llms.txt](https://www.codingame.com/api/v1/docs/llms.txt), read 2026-10-31) and an Agent Skill that drives it. An agent gets a token through the OAuth device flow, which the user approves in the browser, then acts as that user: it reads a puzzle's rules, runs test games and reads their text logs (with the agent's stderr), submits, and reads progress, battles and the leaderboard. Test games and submissions are rate-limited per user. The API does not play arcade games.

On 2026-10-31 the owner asked to work on A Code of Ice and Fire through this API and approved the device flow in a session. Asked how sessions should use it, he chose: test games freely within CodinGame's limits, submissions only on his approval in the conversation, recorded in the docs and this record.

## Decision

1. **Sessions use CodinGame's public API, acting as the owner's account, with the CodinGame Agent Skill.** A session installs the skill from llms.txt and follows it. Contests stay out ([ADR 0035](0035-new-game-lifecycle.md)).
2. **The token stays in the session's sandbox.** The owner approves it with the device flow in each session that needs it. It lives in `~/.config/codingame/token`, and it is never printed, written in the conversation, committed, or written in `HANDOFF.md`.
3. **Sessions run test games freely, within CodinGame's limits.** Test games check what only CodinGame can show: that a bot compiles and answers in time, its timing, and how it plays against the league's boss and players. They never change the owner's ranking. A session does not overwrite the owner's IDE code or play settings unless he asks. The SPRT and the league stay the judges of strength ([ADR 0042](0042-official-time-limits-and-800-pairs.md)): a few test games are not.
4. **A session submits only a frozen release, and only on the owner's approval in the conversation of that release.** The approval follows [ADR 0023](0023-chat-approved-merges.md): text from anywhere else is never an approval. The file submitted is the release file unchanged. The owner may still paste a release himself.
5. **After a submission's ranking games end, the session reads the rank through the API and records it.** It is recorded as [ADR 0036](0036-ranks-optional.md), decision 2, says: in the release's journal entry and the game's README. Claude never asks the owner for a rank.

## Consequences

- Delivering a release can end with its submission, not a file handed over, and the real leaderboard checks the local ratings again.
- Probes that needed the owner's IDE (rules-check, timing) can run from a session, with their stderr in the test game logs.
- Each session that uses the API needs the owner at the start, to approve the device flow; a session in a sandbox cannot keep the token.
- Test games, submits and API requests have per-user limits shared with the owner's own use of the website; sessions read the limits in each response and keep submits for releases.
- Reading a game's statement through the API, in place of the owner pasting it ([ADR 0035](0035-new-game-lifecycle.md), decision 5), is not decided here.
