# A Code of Ice and Fire: model

The problem as a search or learning problem, for stages 3 and 4 of [docs/NEW_GAME.md](../../docs/NEW_GAME.md). Rules are in [RULES.md](RULES.md); this file says how to think about them. Numbers marked "estimate" come from the rules alone; stage 6 replaces them with measures.

## In plain English

Each turn a player sees everything: the map, both players' gold and income, every unit and building. It then writes a whole plan for the turn: any number of trainings, moves and buildings, carried out in the order written. Gold is the constraint. Each cell held pays 1 a turn, a mine 4 more, and units cost upkeep, a level 3 unit 20 a turn. So a player keeps choosing between growing (more cells, more mines), defending (towers, strong units on the front) and striking.

Territory only counts while it is joined to the HQ. Capturing one cell can cut off a whole region: its cells stop paying, and its units die at the start of their owner's next turn. Trainings can be placed on any cell next to the territory, and each one extends the territory. So a rich player can chain trainings across several cells in one turn, all the way to the enemy HQ, which wins at once. Most of the game's tactics come from these two facts: cuts that kill many units, and chains of trainings that reach the HQ. Towers block the chains unless the player pays for level 3 units, and level 3 units are expensive to keep.

A player wants to capture the enemy HQ. Failing that, it wants more military power after 100 turns each: gold plus what its units cost to train.

## Numbers

| Quantity | Value | How it was obtained |
| --- | --- | --- |
| Players | 2, red moving first; blue starts with one more cell | RULES.md |
| Map | 12×12, about half the cells void (estimate: the source makes 45% void before smoothing); 6 to about 12 mine spots (estimate: 15% of about 60 to 80 non-void cells) | RULES.md; estimate |
| Turns per game | At most 100 per player (200 answers); typical length unknown, estimate 20 to 100 game turns | RULES.md; estimate, measured at stage 6 |
| Commands per turn | 0 to a few dozen: one move per unit, plus trainings and buildings as gold allows | estimate |
| Distinct answers per turn | Start: under 10 (20 gold, two cells to train on). Middle: with 10 units of about 3 useful moves each and a few training cells, about 10^5 to 10^8 move sets, more with orders and trainings. Late: more. Not enumerable in general | estimate |
| State size | About 200 to 400 bytes: 144 cells of about 1 byte (owner, active, mine spot, building, unit level), or about ten 144-bit planes, plus units' ids and both golds | estimate, then the engine at stage 5 |
| Time per turn | 1,000 ms for the first answer, 50 ms later | RULES.md |
| Engine speed | Unknown until stage 5 | stage 5 |

## Formal model

- **Players:** seats 0 (red, HQ at (0, 0)) and 1 (blue, HQ at (11, 11)).
- **States:** the map (void cells and mine spots, fixed for the game), each cell's owner and activity, buildings, units (owner, id, level, cell, whether it may still move this turn), both golds, the turn number and the player to move. Both players observe all of it.
- **Actions:** in a player's turn, a finite sequence of commands. Each command's effect depends on the state left by the ones before it. So a turn is a path in a tree of single commands, not a choice from a short list. Many sequences reach the same state: independent moves commute.
- **Transition:** deterministic. The only chance is the map, drawn from the seed before the first turn and shown to both players. The players alternate whole turns; inside a turn, the start-of-turn steps of RULES.md run first.
- **Outcome:** a win, a draw or a loss (the end of the game in RULES.md). The military power margin decides only after 100 turns. Whether equal power is a draw on CodinGame is still to check.
- **Class:** a two-player zero-sum game of perfect information, deterministic once the map is drawn, with alternating macro-turns (a turn is a sequence of commands). Close to turn-based strategy games such as Antiyoy, which the source's package is named after.

## What follows for bots

- **Symmetry.** The half-turn (x, y) → (11 − x, 11 − y) maps the map to itself and red's place to blue's. Red's tie-break order for moves (up, right, down, left) maps to blue's (down, left, up, right). So a bot can always think as red, rotating the input and its answer when it plays blue. The only differences left are who moves first and blue's extra cell at the start.
- **Precomputation.** The map is fixed for the game, so the first turn's 1,000 ms can compute all distances between cells (144 × 144). It can also find the narrow passages that make cuts, and each mine spot's distance to both HQs. Each turn, a bot needs connectivity to the HQ after a capture, which changes cell by cell.
- **Tactics a search must see.** Chains of trainings toward the enemy HQ, bounded by gold and blocked by towers unless at level 3. Cuts, worth the units they kill and the income they remove. The opponent's best reply to both, since a turn left open to a lethal chain loses.
- **Size.** 100,000 characters leave room for a hand-written engine and heuristics, and for small learned weights packed into characters ([research note](../../docs/research/utf16-packing.md)).
- **Families of methods**, to rank at stage 8, not chosen here:
  - planning one turn as a sequence (greedy construction, beam search or sampling over command sequences, with an evaluation of the state the turn leaves);
  - the same with a look at the opponent's reply (two plies of whole turns, the reply limited to its threats);
  - rolling-horizon evolution of the next few turns;
  - search over whole turns (MCTS or minimax) where each node's turns are sampled or generated, not enumerated;
  - rule-based tactics (lethal chain, cut, defence) on top of any of these.

## Framework gaps

What the framework assumes that this game breaks (stage 4). Each gap goes to the owner as a decision; one that is costly to reverse also gets a decision record.

| Gap | Where | What would change | Can the game wait? | Decision |
| --- | --- | --- | --- | --- |
| 1. A turn is many commands | `crates/cg-search`: the `Game` trait lists every legal move of a turn; its MCTS backs values up as if the players strictly alternate one move at a time (`1 - best` in `mcts.rs`) | Either the game's bots search in their own crates, or `cg-search` learns turns of several plies (a node knows its mover, values back up by mover, an end-of-turn move) | Yes, until stage 9 (approach): the referee, arena and baseline bots do not use `cg-search` | To ask |
| 2. Maps for our arena | The referee must make maps like CodinGame's; the source's generator is GPL, so it is not translated | Our own generator, written from the map facts in RULES.md (symmetric, connected, HQ corners clear, the mine spots' count and places), compared with real maps that stage 5's `rules-check` bot would log on CodinGame; or a bank of such logged maps | No: stage 5's referee needs it | To ask |
| 3. Human play in the studio | `studio/game`: a human's moves are listed as complete answers (`HumanMove`); `studio/web` names the seats X and O and draws 9×9 boards | A renderer for this map; seat names per game; for human play, a way to build a turn command by command before sending it | Yes, until stage 7; replay and analysis could come before human play | To ask |

Checked and not gaps:

- **`cg-arena`.** Its referee interface already takes answers as lines, sends a game's map once (`initial_input`), and ends a game with a win or a draw. Its seat-swapped pairs play the same seed, so both bots meet each map from both seats, which cancels red's first move and blue's extra cell. Random opening plies (`OPENING_PLIES` in a game's `evaluation.env`, passed to the referee as `GameSetup::opening_plies`) are not needed to vary games, since the map does; the game's evaluation settings choose at stage 6.
- **`cg-bundler` and the 100,000-character limit.** Nothing in this game needs more than a uttt bot does.
- **Time.** 50 ms per answer is half of uttt's 100 ms. Each bot sets its own budget, and the arena's time scale (`CG_TIME_SCALE`, `crates/cg-core/src/time.rs`) applies as for uttt.
