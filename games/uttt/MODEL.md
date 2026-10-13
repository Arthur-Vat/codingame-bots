# Ultimate Tic-Tac-Toe: model

The problem as a search problem, written after the fact as the worked example of stage 3 of the [new game playbook](../../docs/NEW_GAME.md). The rules are in [RULES.md](RULES.md); this file says how to think about them.

## In plain English

Two players take turns marking one empty cell of a 9×9 board split into nine small 3×3 boards. The cell a player marks, read as a position inside its small board, sends the opponent to the small board at that position; if that board is closed (won or full), the opponent may play anywhere still open. Winning a small board needs three in a line inside it; winning the game needs three won small boards in a line, or, once no move is left, more small boards won. Each turn a player sees the whole board and chooses one cell.

The routing rule is what makes the game hard: a move is judged by the small board it wins or blocks, and also by the board it sends the opponent to, which may hand that player a win or a free choice.

## Numbers

| Quantity | Value | How it was obtained |
| --- | --- | --- |
| Players | 2 | [RULES.md](RULES.md) |
| Moves per game | at most 81; 58.9 on average in random play from the start | rules; `cargo run --release -p uttt-engine --example speed` ([README.md](README.md), "Engine speed") |
| Legal moves per turn | 81 on the first move; at most 9 when sent to an open board; up to 81 minus the cells played on a free choice | rules |
| State size | about 50 bytes: two marks per small board, won and closed boards, the target board, the player to move (`Board` in [engine/src/board.rs](engine/src/board.rs)) | engine |
| Time per turn | 1,000 ms for the first answer, then 100 ms | [RULES.md](RULES.md) |
| Engine speed | about 1,190,000 random playouts and 755,000 MCTS iterations per second on one thread | [README.md](README.md), "Engine speed" (2026-10-08) |
| Draws between strong bots | between a sixth and a third of the games of the SPRT samples | the journal, [E002](journal/E002-tree-reuse.md) to [E006](journal/E006-faster-search-2.md) (for example 236 draws in 721 games in E006) |

## Formal model

- **Players:** {0, 1}; player 0 moves first.
- **States:** for each player, the set of cells they marked; with them, which small boards each player won, which are closed, and the target board (one of nine, or none for a free choice). Both players observe the whole state.
- **Actions:** in state s, the empty cells of the target board if it is open, else the empty cells of every open small board; one action per turn.
- **Transition:** deterministic; marking cell (b, c), with b its small board and c its position inside it, updates small board b, then sets the target board to c. Players alternate.
- **Outcome:** a terminal state is one where a player has three won small boards in a line, or where no action is left; the payoff is +1, 0 or −1 for player 0 (win, draw on equal small boards won, loss), and its opposite for player 1.
- **Class:** a finite, two-player, zero-sum game of perfect information with alternating moves and no chance; every game ends within 81 moves.

## What follows for bots

- **Symmetries:** the eight symmetries of the square, applied at once to the positions of small boards and of cells inside them, keep the rules unchanged, including the routing rule.
- **Search fits the class:** a game of perfect information with no chance and moderate branching suits minimax and Monte Carlo tree search; a 100 ms turn and an engine of about a million playouts per second favour MCTS with fast playouts.
- **Precomputed knowledge** must fit in a 100,000-byte file: tables of small-board patterns (512 per player) and small learned weights fit; large networks do not.
- **Exact values** exist for the end of a game: a solver can prove wins and losses in the tree.

## Framework gaps

None: the framework was built for this game, two players taking turns with full information ([ADR 0011](../../docs/adr/0011-framework-structure.md)).

| Gap | Where | Decision |
| --- | --- | --- |
| None | | |
