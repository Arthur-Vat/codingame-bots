#!/usr/bin/env python3
"""Token use of the current Claude Code session and of its agents.

Reads the session's own transcripts (~/.claude/projects/*/<session>.jsonl and
<session>/subagents/agent-*.jsonl), so it only sees the session it runs in.

Usage: python3 .claude/skills/session-usage/usage.py [--since 2026-10-09T17:00]
  --since   a UTC time (ISO 8601); defaults to 6 hours ago
"""
import argparse
import datetime as dt
import glob
import json
import os


def usage(path, since):
    steps = read = written = output = 0
    first = last = None
    model = prompt = ""
    seen = set()
    with open(path, encoding="utf-8") as lines:
        for line in lines:
            try:
                entry = json.loads(line)
            except ValueError:
                continue
            if entry.get("timestamp", "") < since:
                continue
            message = entry.get("message") or {}
            if entry.get("type") == "user" and not prompt and isinstance(message.get("content"), str):
                prompt = message["content"]
            tokens = message.get("usage")
            if entry.get("type") != "assistant" or not tokens or message.get("id") in seen:
                continue
            seen.add(message.get("id"))
            model = model or message.get("model", "")
            context = sum(tokens.get(k, 0) for k in ("input_tokens", "cache_read_input_tokens", "cache_creation_input_tokens"))
            first = context if first is None else first
            last = context
            steps += 1
            read += tokens.get("cache_read_input_tokens", 0)
            written += tokens.get("cache_creation_input_tokens", 0) + tokens.get("input_tokens", 0)
            output += tokens.get("output_tokens", 0)
    return dict(steps=steps, read=read, written=written, output=output, first=first or 0, last=last or 0, model=model, prompt=prompt)


def short(n):
    return f"{n / 1e6:.2f}M" if n >= 1e6 else f"{n / 1e3:.0f}k"


def main():
    parser = argparse.ArgumentParser()
    default = (dt.datetime.now(dt.timezone.utc) - dt.timedelta(hours=6)).strftime("%Y-%m-%dT%H:%M")
    parser.add_argument("--since", default=default)
    since = parser.parse_args().since
    sessions = [p for p in glob.glob(os.path.expanduser("~/.claude/projects/*/*.jsonl"))]
    if not sessions:
        raise SystemExit("no session transcript under ~/.claude/projects")
    session = max(sessions, key=os.path.getmtime)
    main_use = usage(session, since)
    print(f"since {since} UTC")
    print(f"main session: {main_use['steps']} steps, context {short(main_use['first'])} to {short(main_use['last'])}, "
          f"read {short(main_use['read'])}, written {short(main_use['written'])}, output {short(main_use['output'])}")
    total = dict(main_use)
    agents = sorted(glob.glob(os.path.join(session[:-len(".jsonl")], "subagents", "agent-*.jsonl")), key=os.path.getmtime)
    for path in agents:
        agent = usage(path, since)
        if not agent["steps"]:
            continue
        for key in ("read", "written", "output"):
            total[key] += agent[key]
        what = " ".join(agent["prompt"].split())[:50]
        print(f"  agent {agent['model'].replace('claude-', '')}: {agent['steps']} steps, starts at {short(agent['first'])}, "
              f"read {short(agent['read'])}, written {short(agent['written'])}: {what}")
    print(f"total: read {short(total['read'])}, written {short(total['written'])}, output {short(total['output'])}")


if __name__ == "__main__":
    main()
