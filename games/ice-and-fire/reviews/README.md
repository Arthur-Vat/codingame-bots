# Game reviews

Batches of recorded games reviewed by Haiku agents ([ADR 0043](../../../docs/adr/0043-game-reviews-by-haiku.md)), or measured by code and interpreted by one Haiku agent ([ADR 0044](../../../docs/adr/0044-metrics-reports.md)); the skill `review-games` makes either, and the game's README ("Game reviews") describes the tool.

Each batch's folder holds its README (bots, seed, results, the tool's commit), the records, each game's metrics (`NAME.csv`) and facts (`NAME.summary.json`), each game's review (`NAME.review.md`), the computed statistics (`summary-data.md`) and the batch's summary (`summary.md`). The game files the reviewers read are not kept: the records rebuild them. A metrics batch holds only its README, the report (`report.md`) and its interpretation (`summary.md`): its seed rebuilds the records.

| Batch | Bots | Games | Result | Summary |
| --- | --- | ---: | --- | --- |
| [2026-10-23-v004-v003](2026-10-23-v004-v003/) | v004 against v003 | 20 | 10 wins each | [summary](2026-10-23-v004-v003/summary.md) |
| [2026-10-23-v005-v004-metrics](2026-10-23-v005-v004-metrics/) | v005 against v004 (metrics report) | 1,000 | v005 567, v004 433 | [summary](2026-10-23-v005-v004-metrics/summary.md) |
