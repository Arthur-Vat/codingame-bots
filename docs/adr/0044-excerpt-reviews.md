# 0044. Excerpt reviews: many games, each read only around its decisive moment

- Status: accepted (owner, 2026-10-23)
- Date: 2026-10-23
- Completes: [0043](0043-game-reviews-by-haiku.md), decisions 2 and 5 (another kind of batch beside the reviews of full games)
- Scope: framework

## Context

ADR 0043 set up reviews of full games by Haiku agents, one agent a game, 20 games a batch. After the first batch, the owner found the reviews costly and asked, on 2026-10-23, to "sample many more games, but instead of analyzing the full games analyzing only the last 3 moves of each player". The first batch had shown that A Code of Ice and Fire's games are decided by a large cut early in the middle game, often long before the game ends, so the owner chose, in a form the same day, to read the moves around the decisive moment rather than the game's last moves ("Around the decisive moment"), and 200 games a batch, 20 excerpts an agent, with one summary ("200 games, 20 per agent").

## Decision

1. **An excerpt shows each side's last three turns up to the decisive moment.** The game's review tool chooses the moment by rules: the first large cut, else the first lead change; without either, the excerpt shows the game's last plies. It shows the plies up to and including the play that caused the moment, then the position after it. The rules in short and the metrics' meanings are written once for the batch, in a primer that every excerpt shares.
2. **A Haiku agent reviews twenty excerpts.** The project agent `excerpt-reviewer` (model Haiku, a tool to read files only) reads the primer and its excerpts and returns, for each game, a few sentences and its judgments: the plays of the side the moment went against that let it happen, and the play that made it, each with a category from a fixed list that the game's README gives. The main session saves each reply unchanged.
3. **Code counts; a Haiku agent summarizes.** The review tool counts the batch from a row per game and the saved replies: results by bot and seat, the kinds of decisive moment and how often their side won, and the judgments by bot, verdict and category. The project agent `review-summarizer` turns the counts and the replies into the batch's summary, as for a batch of full games.
4. **An excerpt batch keeps only what is small.** The repository keeps the batch's README, the row per game, the saved replies, the counts and the summary; neither the records nor the excerpts, which the arena's seed and the tool rebuild.
5. **Full reviews stay available.** A batch of full games (ADR 0043) still answers questions about a whole game; the owner or a session picks the kind of batch.

## Consequences

- A batch of 200 games costs about 10 Haiku agents of about 40,000 tokens each and one summary, against 20 agents for 20 full games.
- The categories make the judgments countable across games and batches, but they can force a play into a box: the reviewer uses `other` when none fits, and the counts show how often.
- What happens after the decisive moment, and anything long before it, is not read; a full batch is the tool for that.
- The agent guard (`scripts/agent-guard.sh`) allows the new agent.
- Another game gets excerpt batches by having its review binary write excerpts with the shared crate `cg-review`, and by listing its categories in its README.
