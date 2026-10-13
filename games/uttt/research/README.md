# Ultimate Tic-Tac-Toe: research

The worked example of stage 8 of the [new game playbook](../../../docs/NEW_GAME.md), written after the fact: this game's bots were built without a research stage, so this note sets what was learned in the journal beside a short scan of outside sources, done on 2026-10-13. It is short on purpose; a game going through the lifecycle does this before its first bot.

## Sources

The CodinGame forum and the conference pages below could not be opened from Claude's session (codingame.com and those sites are not reachable from it); their lines say only what a web search reported about them, to be read in full before relying on them.

| Source | Kind | What was taken from it |
| --- | --- | --- |
| This repository's [journal](../journal/README.md), E001 to E016 | our experiments | Which ideas gained strength here and which did not (below) |
| [ADR 0017](../../../docs/adr/0017-value-network.md) to [ADR 0019](../../../docs/adr/0019-value-network-gate-at-time.md) | our decisions | How the value network was judged, and why it lost at the bot's time budget |
| [uttt.ai](https://github.com/ar-nowaczynski/utttai) (Apache 2.0), read on 2026-10-13 | player's code and write-up | An AlphaZero-like bot for the browser: a policy-value network of about 5 million parameters guiding MCTS, trained in two rounds of self-play of about 8 million positions each; its author reports the second network beating plain MCTS that runs about 10,000 times more simulations. Such a network does not fit in a 100,000-byte bot |
| [Forum thread of the game](https://forum.codingame.com/t/ultimate-tic-tac-toe-puzzle-discussion/22616) | forum, not read | Reported by the search: Legend bots use MCTS with a solver, tree reuse, a lowered exploration constant and a one-move win check in playouts; speed tricks matter |
| [Ultimate Tic-Tac-Toe Bot Techniques](https://journals.flvc.org/FLAIRS/article/view/130698), FLAIRS 2022 | paper, not read | Reported by the search: compares random play, a heat map, MCTS and an AlphaZero-like method, the last one strongest |

## What strong bots do

- **MCTS is the base.** The forum reports it for Legend, and here it took the first release from greedy play to Legend ([E001](../journal/E001-mcts.md)).
- **Speed is strength.** Every faster search was accepted ([E004](../journal/E004-faster-search.md), [E006](../journal/E006-faster-search-2.md)); a larger policy that cost 20% of the iterations lost ([E016](../journal/E016-context-policy.md)).
- **Exact knowledge at the end:** proving wins and losses in the tree helped ([E003](../journal/E003-solver.md)); proving draws did not ([E009](../journal/E009-draw-proofs.md)).
- **Better playouts:** taking a game-winning move when there is one ([E005](../journal/E005-decisive-playouts.md)), then a small learned policy for the first moves ([E011](../journal/E011-playout-policy.md), [E015](../journal/E015-pattern-policy.md)) and to order the tree's children ([E014](../journal/E014-tree-priors.md)).
- **Networks:** a large network is the strongest approach outside CodinGame's limits (uttt.ai), but a value network small enough for the file lost to playouts at the bot's time budget ([E012](../journal/E012-value-network.md)).

## Candidate approaches

As they would have been ranked before the first bot, knowing the engine's speed and the 100 ms turn:

| Rank | Approach | Expected strength | Cost to build | Why |
| --- | --- | --- | --- | --- |
| 1 | MCTS with fast playouts, tree reuse and a solver | High | Low | Fits the class (perfect information, no chance) and the engine's speed; what the forum reports for Legend |
| 2 | MCTS guided by small learned weights (policy for playouts and priors) | Higher, once the first is fast | Medium: self-play and training ([ADR 0016](../../../docs/adr/0016-self-play-training.md)) | Weights of a few kilobytes fit the file |
| 3 | Alpha-beta with a hand-written evaluation | Medium | Medium | Moderate branching, but a good evaluation is hard to write for this game |
| 4 | A policy-value network | Highest outside CodinGame | High | Too large for the file; even a small value network lost at the bot's time budget (E012) |

## Proposal for the bake-off

Had stage 9 existed: MCTS with random playouts against alpha-beta with a simple evaluation (small boards won, two-in-a-lines), at 20 ms per move in the arena, then the winner as `uttt-v001`. What happened instead was MCTS directly, and it was accepted 60–0 against greedy ([E001](../journal/E001-mcts.md)).
