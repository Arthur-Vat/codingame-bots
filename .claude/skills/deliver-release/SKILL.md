---
name: deliver-release
description: Finishes a release after its pull request is merged - checks the GitHub release, submits it to CodinGame through the API on the owner's approval (or hands him the file), and records its CodinGame rank. Use right after an accepted candidate is merged, and when a submission's ranking ends or the owner reports a rank.
---

# Deliver a release

## After the merge

1. The Release workflow tags the version (`<game>-vNNN`) and publishes a GitHub release with the paste-ready file. Check its run succeeded (skill `github-api`); the nightly league (or, for a game without one, the league of the release's pull request) rates it; the new release's rating is on the branch `league-results` the morning after, and in `main` with the weekly pull request.
2. Tell the owner, in one message: the version, its SPRT result, the link to its GitHub release, and ask (skill `ask-decision`) whether to submit it to CodinGame through the API ([ADR 0047](../../../docs/adr/0047-codingame-api.md)). Submit only on his approval of that release in the conversation; if he would rather paste it himself, the delivery is done once he has the file.

## Submitting

With the CodinGame Agent Skill (`docs/CODINGAME.md`, section "API"; the owner approves the device flow if the session has no token):

1. A few test games against the league's boss first, to check that the release compiles and answers in time on CodinGame.
2. Submit the release file unchanged (`POST /puzzles/<pretty_id>/submissions`, language `Rust`), and note its `submission_id` and the file's SHA-256.
3. Poll `/progress` every few minutes in the background until `percentage` is 100 and `in_progress` is false. Never ask the owner for a rank.

## Recording a rank

When a submission's ranking ends, or when the owner reports a rank of his own accord. In one documentation pull request (light review, skill `review-pr`):
- the entry's `cg_rank` in `games/<game>/journal/` (rank, out of how many, league, date);
- the status line of `games/<game>/README.md` when it is the best or latest recorded rank;
- the roadmap's "Ongoing" row when the league or a milestone changes.

Never guess a rank or a league: record what `/progress` says, and if the owner's report is unclear, ask.
