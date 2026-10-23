# 0043. Game reviews written by Haiku agents from game files

- Status: accepted (owner, 2026-10-23)
- Date: 2026-10-23
- Scope: framework

## Context

Reading games to see why a bot wins or loses helped before: the qualitative study of 200 games between `ice-and-fire-v004` and `ice-and-fire-v003` (A Code of Ice and Fire's experiment E005, 2026-10-22) found what decides those games, a cut through a neck of the territory. That study was read by the main session, on the most capable and most costly model.

On 2026-10-23 the owner asked for reviews of full games by a light model (Haiku) instead, "since it is costly to do it with the orchestrator": a commentary of the good and bad plays in the early game, the middle game and the endgame; the games turned into ASCII maps on which red and blue are easy to tell apart; core metrics tracked turn by turn and correlated with the analysis; the decisive moments; and this for many bots, the new ones included. He answered the details in forms the same day: A Code of Ice and Fire first, built so other games can plug in; two characters a cell; the reviews kept as Markdown in the repository; run on demand; 20 games a batch; the endgame starting when the game is decided; as decisive moments the large cuts, the lead changes, the threats to an HQ and debt, plus the reviewer's own picks; the reviewer reads every turn in full.

## Decision

1. **Code turns a game into text; a model only reads and writes.** A game's review tool (for A Code of Ice and Fire, `ice-and-fire-review`, on the shared crate `cg-review`) replays a record and writes the game file: every ply's board, both sides' metrics and commands, the phases and the decisive moments that rules find. It computes the batch's statistics too. No model computes a number.
2. **A Haiku agent reviews each game.** The project agent `game-reviewer` (model Haiku, tools to read and write files only) reads one game file and writes one review: why the game was won, each phase's good and bad plays of each side with their plies and metrics, the decisive moments, lessons, and a JSON list of the plays it judged.
3. **A Haiku agent summarizes the batch.** The project agent `review-summarizer` (model Haiku, tools to read files only) reads the reviews and the computed summary and returns the batch's summary as its reply, which the main session saves unchanged: what decides the games, each bot's recurring plays by phase, how far the reviews agree with the metrics, and ideas for the next experiment. Agents may not write summary files in Claude's sessions, which the first batch found.
4. **The reviews are checked against the metrics.** The computed summary compares each judged play with the change of the side's lead over its next turn, so a batch says how often its reviewer agreed with the numbers.
5. **Batches run on demand and are kept in the repository.** The skill `review-games` runs a batch when the owner or a session asks: 20 games by default, as 10 seat-swapped pairs. It keeps the records, the metrics, the reviews and the summaries in `games/<game>/reviews/`, through a documentation pull request. The game files are not kept: the records and the tool rebuild them.
6. **The main session orchestrates and does not read the games.** It runs the tools, starts the agents, saves the summary, and reads only the reviewers' short replies and the summary.

## Consequences

- A batch costs about 21 Haiku agents of a few tens of thousands of tokens each, instead of the main session reading the games.
- The agent guard (`scripts/agent-guard.sh`) allows the two new agents.
- A Haiku review can be wrong; decision 4 measures how often, and a summary must say how far to trust it.
- The reviewer may write only its one file by its instructions, not by its tools: Write reaches the whole repository. Their input is the output of the project's own bots and tools, so the risk is low; a batch's pull request shows any stray file.
- Another game gets reviews by writing its own review binary over `cg-review`: a replay into a trace, its metrics, phases and moments.
- Changing a metric or a phase changes what a rebuilt game file shows; a batch's README names the tool's commit it was made with.
