---
name: deliver-release
description: Finishes a release after its pull request is merged - checks the GitHub release, hands the paste-ready file to the owner, and records the CodinGame rank he reports. Use right after an accepted candidate is merged, and when the owner reports a rank.
---

# Deliver a release

## After the merge

1. The Release workflow tags the version (`<game>-vNNN`) and publishes a GitHub release with the paste-ready file. Check its run succeeded (skill `github-api`); the League workflow then rates every release.
2. Tell the owner, in one message: the version, its SPRT result, the link to its GitHub release, and that it is ready to paste (`docs/CODINGAME.md` and the game's README say how). Submitting is his (ADR 0009).

## When the owner reports a rank

In one documentation pull request (light review, skill `review-pr`):
- the entry's `cg_rank` in `games/<game>/journal/` (rank, out of how many, league, date);
- the status line of `games/<game>/README.md` when it is the best or latest recorded rank;
- the roadmap's "Ongoing" row when the league or a milestone changes.

Never guess a rank or a league: if the owner's report is unclear, ask.
