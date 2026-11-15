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

## The release's check (A Code of Ice and Fire)

Once the collection holds the release's games (its ranking games and a first day of test games: about 100 games or more; skill `collect-games`), run `codingame-surprise` on them, judged by the release's own search, and add its row to `games/ice-and-fire/research/codingame/surprise-by-release.md`, in the pull request that records the rank or one of its own (light review):

```sh
W="$(mktemp -d)"; DATA="$W/data"; SRC="$W/src"; OUT="$W/out"
git fetch --depth=1 origin claude/codingame-games && git worktree add "$DATA" FETCH_HEAD
git worktree add "$SRC" ice-and-fire-vNNN   # the release's tag: its own search judges
(cd "$SRC" && cargo build --release -p ice-and-fire-bot-plan --features analysis)
cargo build --release -p ice-and-fire-review
target/release/ice-and-fire-review codingame-surprise --data "$DATA/ice-and-fire" --me Tarleya \
  --judge "$SRC/target/release/ice-and-fire-bot-plan" --release ice-and-fire-vNNN --jobs 4 --out "$OUT"
git worktree remove --force "$DATA"; git worktree remove --force "$SRC"
```

The bot's mode `surprise` came after `ice-and-fire-v026`, so the tags up to v026 cannot judge: the table's first rows were judged by main's build while its search was v026's.

The row is the table of the section "The release's check" in `$OUT/README.md`; add the data branch's commit and the judge. It takes about a minute. Tell the owner the row beside the previous release's (large misses, foreseen large removals, HQ threats not foreseen): a rise is a finding to report, not a reason to withdraw a release that passed its SPRT.
