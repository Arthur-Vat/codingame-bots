---
name: docs-refresh
description: Brings the repository's docs back in line with what the repository holds - READMEs, architecture, workflow, roadmap and platform facts. Use when the docs check fails, after a roadmap step, or when a daily report finds drift.
---

# Refresh the docs

`scripts/check-docs.sh` catches drift in indexes, current-release lines, decision records' scopes, relative links, agents' headers, and the architecture's and README's lists of workflows and scripts. Everything else is checked by reading.

## Check

1. Run `scripts/check-docs.sh` on an up-to-date `main`.
2. Compare each list in the docs with the repository:
   - workflows in `docs/ARCHITECTURE.md` (tree and CI table) and `README.md` against `.github/workflows/`, with their triggers;
   - scripts in the architecture's tree and `README.md` against `scripts/`;
   - crates, game folders and bots against the tree;
   - releases, ranks and dates in `README.md`, `games/<game>/README.md` and the roadmap against the journal;
   - roadmap check boxes against what is merged; required checks against the "Protect main" ruleset (`gh api repos/<repo>/rulesets`).
3. Look for statements that were true once: phases, counts, "not yet", "planned".

## Fix

- One documentation pull request, `docs(docs[,<game>]): ...`, with a light review (skill `review-pr`).
- Facts come from the repository, the journal or a source; anything uncertain about CodinGame is asked, not guessed (`CLAUDE.md`).
- Do not rewrite accepted decision records: only their status lines change.
