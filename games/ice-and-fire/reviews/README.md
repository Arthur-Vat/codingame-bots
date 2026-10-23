# Game reviews

Batches of recorded games reviewed by Haiku agents ([ADR 0043](../../../docs/adr/0043-game-reviews-by-haiku.md)); the skill `review-games` makes one, and the game's README ("Game reviews") describes the tool.

Each batch's folder holds its README (bots, seed, results, the tool's commit), the records, each game's metrics (`NAME.csv`) and facts (`NAME.summary.json`), each game's review (`NAME.review.md`), the computed statistics (`summary-data.md`) and the batch's summary (`summary.md`). The game files the reviewers read are not kept: the records rebuild them.

| Batch | Bots | Games | Result | Summary |
| --- | --- | ---: | --- | --- |
| [2026-10-23-v004-v003](2026-10-23-v004-v003/) | v004 against v003 | 20 | 10 wins each | [summary](2026-10-23-v004-v003/summary.md) |
