---
name: review-games
description: Runs a batch of game reviews - plays two bots against each other, turns the records into game files (or excerpts around each game's decisive moment) and metrics, has Haiku agents review them and another summarize the batch, and lands the batch in the repository. Use when the owner or a session asks for reviews of a game's bots.
---

# Review a batch of games

The design is in [ADR 0043](../../../docs/adr/0043-game-reviews-by-haiku.md) (full games) and [ADR 0044](../../../docs/adr/0044-excerpt-reviews.md) (excerpt batches: many games, each read only around its decisive moment; see "Excerpt batches" below); the tool and the files are described in the game's README ("Game reviews"). A game needs a review binary first (`<game>-review`, over the crate `cg-review`); A Code of Ice and Fire has `ice-and-fire-review`.

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

## Excerpt batches

When the question is where a bot loses its games rather than how one game went, play many games and read each only around its decisive moment (ADR 0044). The steps above hold, with these changes.

- **Games:** 200 by default, 100 seat-swapped pairs.
- **Files:** the records go to `SCRATCH/records`, not to the batch: they are many, and the seed rebuilds them.

  ```sh
  target/release/<game>-arena match --bot NAME_A=SCRATCH/bin/A --bot NAME_B=SCRATCH/bin/B --pairs 100 --seed SEED --records SCRATCH/records
  target/release/<game>-review excerpts --out BATCH --text SCRATCH/excerpts SCRATCH/records/*.json
  ```

  `excerpts` writes `BATCH/games.csv` and, into `SCRATCH/excerpts`, one `NAME.excerpt.md` per game and `primer.md`.
- **Review:** split the games into groups of 20 in `games.csv`'s order, and start one `excerpt-reviewer` agent per group, in the background, up to 10 at a time. Brief each with `SCRATCH/excerpts/primer.md`, its 20 excerpt paths, the categories table of the game's README ("Game reviews") pasted in full, and what to look for. Save each reply, unchanged, as `BATCH/notes-KK.md` (`notes-01.md`, ...). A reply without a readable JSON block gets one more try.
- **Statistics:** `target/release/<game>-review excerpt-summary BATCH > BATCH/summary-data.md`. Its "Problems" section lists unreadable replies, unknown games or sides, and games without a judgment: retry those groups, then compute again.
- **Summary:** brief `review-summarizer` that this is an excerpt batch: it reads `summary-data.md` and the `notes-*.md` files instead of `NAME.review.md`, and organizes its findings by category rather than by phase. Save its reply as `BATCH/summary.md`.
- **Land:** `BATCH/README.md` names the seed and pairs and the command that rebuilds the records and excerpts; the batch keeps only `README.md`, `games.csv`, `notes-*.md`, `summary-data.md` and `summary.md`.

