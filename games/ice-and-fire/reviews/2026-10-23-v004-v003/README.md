# Reviews: ice-and-fire-v004 against ice-and-fire-v003

The first batch of game reviews (owner, 2026-10-23; [ADR 0043](../../../../docs/adr/0043-game-reviews-by-haiku.md), skill `review-games`).

- **Bots:** [`ice-and-fire-v004`](../../releases/ice-and-fire-v004.rs) (the current release) against its parent [`ice-and-fire-v003`](../../releases/ice-and-fire-v003.rs), both compiled with Rust 1.90.0.
- **Games:** 10 seat-swapped pairs, seed 151, at CodinGame's time limits: 10 wins each, no timeout, crash or invalid answer.
- **Tool:** `ice-and-fire-review` at commit 1dfc16c. Its rules primer then said that any cell next to an enemy tower needs level 3; the summary noticed reviews relying on it ("How far to trust the reviews", towers). The primer now states the rule as RULES.md does (commit 64d948d): only a tower's cell, and its owner's active cells next to it while the tower is active, need level 3.
- **Reviews:** one per game, `NAME.review.md`, by the `game-reviewer` agent (Haiku); the batch's summary, [summary.md](summary.md), by the `review-summarizer` agent (Haiku), saved unchanged by the main session (agents may not write summary files); the computed statistics, [summary-data.md](summary-data.md).
- **Data:** the records in `records/`; for each game, its metrics (`NAME.csv`) and its result, phases and decisive moments (`NAME.summary.json`).

The game files the reviewers read are not kept; the records rebuild them:

```sh
cargo run --release -p ice-and-fire-review -- games --out /tmp/batch --text /tmp/batch/text games/ice-and-fire/reviews/2026-10-23-v004-v003/records/*.json
```
