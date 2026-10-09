---
name: session-usage
description: Measures how many tokens the current session and its agents used since a given time, from the session's own logs. Use at the end of every step or piece of work the owner asked for, and put the result in the summary to him.
---

# Measure a session's token use

The owner pays for every step in usage, and wants to see when work costs too much (2026-10-09). Measure each piece of work, and report it in one line.

## Run

```sh
python3 .claude/skills/session-usage/usage.py --since <UTC time the work started, such as 2026-10-09T17:00>
```

It reads only this session's logs: the main session's steps, then each agent started since that time.
- **read**: context re-read from the cache on each step. It is most of the volume, and grows with the conversation's length.
- **written**: context written to the cache, and new input.
- **output**: what the model wrote.
- **starts at**: an agent's context before it does anything. It is about 20k with a short tool list, and about 60k without one.

## Report

One line in the summary to the owner, for example: "Usage: main session 26 steps (10.1M read), 2 agents (0.8M read)." Add a cause when a figure stands out: a long conversation, an agent with many steps, or an agent that starts high.

Add the same line to the Usage section of `HANDOFF.md` (skill `handoff`), so that the daily report can sum up what each session cost. The daily scheduled session reports its own line in the report instead.

## Limits

- The figures are tokens, not the plan's percentage, which sessions cannot read (`docs/ROADMAP.md`, phase 5, step C).
- A scheduled session sees only its own logs: the daily report learns what other sessions cost from their lines in `HANDOFF.md`.
