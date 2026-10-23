---
name: review-games
description: Runs a batch of game reviews - plays two bots against each other, turns the records into game files and metrics, has a Haiku agent review each game and another summarize the batch, and lands the batch in the repository. Use when the owner or a session asks for reviews of a game's bots.
---

# Review a batch of games

The design is in [ADR 0043](../../../docs/adr/0043-game-reviews-by-haiku.md); the tool and the files are described in the game's README ("Game reviews"). A game needs a review binary first (`<game>-review`, over the crate `cg-review`); A Code of Ice and Fire has `ice-and-fire-review`.

The main session runs the tools and starts the agents. It does not read the game files or the reviews: only the agents' short replies and the batch's summary.

## 1. Settle the batch

- **The bots:** two releases, or a candidate and its parent. By default, the current release against the one before it.
- **Games:** 20 by default, 10 seat-swapped pairs.
- **Seed:** one not used for another batch (see the batch READMEs).
- **What to look for:** one sentence for the reviewers, for example "the candidate v005 against its parent v004: does it keep its gold?".

Ask the owner only what he has not said.

## 2. Play and turn the games into files

`BATCH` is `games/<game>/reviews/YYYY-MM-DD-<bot>-<bot>` (for example `2026-10-23-v004-v003`); `SCRATCH` is the session's scratch directory.

```sh
cargo build --release -p <game>-arena -p <game>-review
mkdir -p SCRATCH/bin
rustc +1.90.0 --edition 2021 -O -o SCRATCH/bin/A games/<game>/releases/<release A>.rs   # and B
target/release/<game>-arena match --bot NAME_A=SCRATCH/bin/A --bot NAME_B=SCRATCH/bin/B --pairs 10 --seed SEED --records BATCH/records
target/release/<game>-review games --out BATCH --text SCRATCH/text BATCH/records/*.json
```

`NAME_A` and `NAME_B` are the bots' names in the game files and reviews: the releases' short names (`v004`). The match plays at CodinGame's limits (the arena's default). `games` writes `NAME.csv` and `NAME.summary.json` into `BATCH` and the game files, `NAME.md`, into `SCRATCH/text`: they are large, and the records rebuild them.

## 3. Review each game

Start one `game-reviewer` agent per game, in the background, up to 10 at a time. Brief each with:

- the game file, `SCRATCH/text/NAME.md`;
- the review file to write, `BATCH/NAME.review.md`;
- what to look for (step 1).

Wait for every agent's reply. An agent that fails, or a review that the next step cannot read, gets one more try.

## 4. Compute the statistics

```sh
target/release/<game>-review summary BATCH > BATCH/summary-data.md
```

It ends with any review whose JSON block could not be read: retry those (step 3), then compute again.

## 5. Summarize the batch

Start one `review-summarizer` agent with the batch's directory and what to look for. Its reply is the summary (agents may not write summary files): save it, unchanged, as `BATCH/summary.md`. This is the one long text the main session reads.

## 6. Land the batch

- `BATCH/README.md`: the bots (release files), the seed, the pairs, the results, the commit of the tool (`git rev-parse --short HEAD`), how to rebuild the game files (the `games` command above), and links to `summary.md` and `summary-data.md`.
- A row for the batch in `games/<game>/reviews/README.md` (create it if missing: what a batch holds, how to make one, and a table of batches).
- A documentation pull request (`docs(<game>): reviews of <bot> against <bot>`) on a `claude/` branch, light review (skill `review-pr`), merged on the owner's approval.
- Tell the owner the summary's main findings and its ideas for the next experiment; reading `BATCH/summary.md` is enough.
