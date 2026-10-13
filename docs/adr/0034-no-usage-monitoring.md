# 0034. No usage monitoring

- Status: accepted (owner, 2026-10-13)
- Date: 2026-10-13
- Supersedes: [0024](0024-handoff-branch.md) decision 3 (the usage line), [0033](0033-no-daily-scheduled-session.md) decision 3 (the usage line)
- Scope: framework

## Context

On 2026-10-09 the owner asked to see what each piece of work cost. The skill `session-usage` measured a session's tokens from its own logs, each piece of work ended with that line in the summary to the owner, `HANDOFF.md` kept one usage line per piece of work ([ADR 0024](0024-handoff-branch.md)), and the summary at the end of a piece of work carried it ([ADR 0033](0033-no-daily-scheduled-session.md)). The skill `handoff` also used the measured context to suggest a fresh session. On 2026-10-13 the owner said: "I don't want to monitor usage anymore. Remove it." He decides when to stop work on a game himself.

## Decision

1. **Claude no longer measures or reports token use.** The skill `session-usage` is removed; summaries to the owner, `HANDOFF.md` and skills carry no usage line.
2. **A fresh session is suggested without measuring.** Claude suggests one when the conversation has grown long and more work is waiting, when the next work is unrelated to the conversation, or when the owner will be away for over an hour, judged from the conversation itself.
3. **What went wrong in a piece of work is still said to the owner at its end** ([ADR 0033](0033-no-daily-scheduled-session.md) decision 3, without its usage line).

## Consequences

- Ending a piece of work takes fewer steps, and `HANDOFF.md` is shorter.
- Nothing records what work cost; the owner sees his plan's usage in the Claude app.
- Asking before heavy work still holds (`CLAUDE.md`): the owner wants to know before a large change starts, measured or not.
- Bringing usage reports back is a new decision.
