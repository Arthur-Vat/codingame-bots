# Planning a turn of many commands

How to choose a whole turn when a turn is a sequence of commands (moves of several units, trainings, buildings), each changing the state the next one sees. The branching factor of a turn is the product of its commands' choices, often 10^6 or more, so a search that lists every turn, as `cg-search` does today, no longer fits.

**Status:** researched for [A Code of Ice and Fire](../../games/ice-and-fire/research/README.md) on 2026-10-19; no bot of this repository uses it yet.

## When it fits

Turn-based games where a player gives many commands per turn, with perfect information or close to it: strategy games (Hero Academy, Tribes, A Code of Ice and Fire), and games with many units moved at once. With a few dozen choices per turn or fewer, plain search over turns stays better (Justesen et al., at 1 to 5 actions per turn).

## Methods

- **Greedy construction with an evaluation.** Add the single command that most improves an evaluation of the state, until none does. Commands add rather than multiply, so a turn costs a few thousand evaluations. Composite commands (a chain of trainings found by a path search) can be offered as single choices. It won A Code of Ice and Fire's contest, with the opponent handled as below (reCurse).
- **The opponent's reply, cheaply.** Plan as if the opponent passed; find the opponent's best reply to that plan; store it; plan again with every stored reply played on each evaluated state, keeping the worst score. Replies cut at their first invalid command reward blocking them. A handful of stored replies is usually enough (reCurse). Alternatives: the opponent's best tactical reply inside the evaluation (NinjaDoggy), or two plies over a short list of whole turns (Magus).
- **Evolution over the turn (online evolutionary planning, OEP).** A genome is one turn's commands; crossover takes each command from either parent when it is still legal, else repairs; fitness is a static evaluation. Better than MCTS from about 10 actions per turn in Hero Academy, but it stalls in local optima and ignores the opponent (Justesen et al.).
- **Evolutionary MCTS (EMCTS, FH-EMCTS).** A tree whose nodes are whole turns and whose edges are mutations; the flexible-horizon form appends the opponent's next actions from a fast greedy model. Ahead of OEP and plain MCTS in Hero Academy; at long turns the best horizon was the current turn alone (Baier and Cowling).
- **Constrained MCTS.** Over single actions, MCTS needs hard pruning, progressive widening, domain pruning (Hsu and Perez-Liebana), a portfolio of scripts in place of raw actions (Perez-Liebana et al.; Churchill and Buro's Portfolio Greedy Search), or naive sampling, which treats units as independent (Ontañón). Unconstrained, it played like a one-action greedy player (Justesen et al.).

## Pitfalls

- Commands interact: units block each other's paths, and the order of moves matters. Methods that treat units as independent (naive sampling) or plan them one by one miss this.
- A static evaluation of the end of one's own turn misses the opponent's tactical reply, which is where these games are lost; some model of the reply pays more than a deeper own search (reCurse).
- Variable turn lengths break methods that assume a fixed number of actions per turn (OEP, EMCTS; Hsu and Perez-Liebana).

## Sources

| Source | What was taken from it |
| --- | --- |
| [reCurse's post-mortem](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/30), with his answers in [#38](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/38), [#40](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/40) and [#48](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/48) | Greedy construction and the memory of opponent replies |
| [NinjaDoggy's](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/45) and [Magus's](https://forum.codingame.com/t/a-code-of-ice-fire-feedbacks-and-strategies/105722/3) post-mortems | The opponent's reply in the evaluation; two plies of whole turns |
| [Justesen, Mahlmann, Risi and Togelius 2017](https://researcher.itu.dk/ws/files/82456662/justesen_tciaig17.pdf) | OEP, and MCTS variants that limit exploration, in Hero Academy |
| [Baier and Cowling 2018](https://ojs.aaai.org/index.php/AIIDE/article/view/13023) | EMCTS and its flexible horizon |
| [Hsu and Perez-Liebana 2020](https://ceur-ws.org/Vol-2862/paper27.pdf) | Pruning MCTS in Tribes, where turn lengths vary |
| [Perez-Liebana et al. 2021](https://arxiv.org/abs/2104.08641) | Portfolio MCTS with progressive unpruning |
| [Ontañón 2017](https://arxiv.org/abs/1710.04805) | Naive sampling for combinatorial branching |
| [Villabrille and Montoliu 2022](https://ceur-ws.org/Vol-3305/paper3.pdf) | NTBOE: online evolution with a bandit model cheaper than the forward model |
| Churchill and Buro 2013, Portfolio Greedy Search (CIG), not read: its PDF answered 404 on 2026-10-19 | Only its abstract, from a search engine |

## Games that use it

None yet. [A Code of Ice and Fire](../../games/ice-and-fire/research/README.md) proposes it for stage 9.
