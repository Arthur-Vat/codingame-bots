# A Code of Ice and Fire rules on CodinGame

These are the rules our referee and engine will implement, written in our own words. They come from two sources, and a third confirms part of them (see [README.md](README.md)):

- **Statement:** the statement template of the game's source, `config/statement_en.html.tpl`, which the owner chose on 2026-10-17 over pasting CodinGame's page;
- **Source:** the referee, [Azkellas/a-code-of-ice-and-fire](https://github.com/Azkellas/a-code-of-ice-and-fire), branch `develop`, commit `efe9e78`. Paths are relative to `src/main/java/com/codingame/`: `game/Referee.java` (the turn loop and the parsing of answers) and, under `antiyoy/`, `GameState.java` (map, turn start, income, actions), `Cell.java` (capture and protection), `Pathfinding.java` (where a move goes), `Unit.java` and `Constants.java`;
- **Wood 3 page:** CodinGame's own statement of the game in Wood 3, pasted by the owner on 2026-10-19. No rule it states differs from those below; it confirms the Wood 3 rules and that all the rules are unlocked in Bronze.

The source is under the GNU GPL 3, so it is read for facts only: nothing of it is translated or copied ([ADR 0035](../../docs/adr/0035-new-game-lifecycle.md)). Where the statement is silent, the rule comes from the source and says so. The points where the two differ, and the platform facts neither gives, are listed at the end.

**Checked on CodinGame:** the Wood 3 page agrees with these rules (2026-10-19); CodinGame's games not yet. Stage 5's `rules-check` bot compares these rules with CodinGame's games; CodinGame's behaviour wins.

## Board and players

- The map has 12×12 cells, `(x, y)` with x the column and y the row, both 0 to 11; (0, 0) is the top-left corner (statement, "The map").
- A cell is void (not part of the map), neutral, or owned by a player. An owned cell is active or inactive (below) (statement, "The map").
- Two cells are neighbours when they share a side; diagonals are not neighbours, and a void cell is nobody's neighbour (`GameState.java`, `computeNeighbours`; statement, "Buildings" for towers).
- Two players: red (seat 0) with its headquarters (HQ) at (0, 0), blue (seat 1) with its HQ at (11, 11) (statement, "The map"; `GameState.java`, `createHQs`). Red plays first (below).

## The map

Bots receive the map; these facts matter for an arena that makes its own maps.

- The map is random, drawn from the game's seed (statement, "The map"; `GameState.java`, `generateMap`).
- It is symmetric under a half-turn: (x, y) and (11 − x, 11 − y) are both void or both not, and both mine spots or both not (`GameState.java`, `generateMap` and `linkComponents`).
- The 3×3 block of cells at each corner of an HQ, (0..2, 0..2) and (9..11, 9..11), is never void (`GameState.java`, `generateMap`; the statement says the cells next to an HQ, diagonals included, are never void, which this implies).
- The non-void cells form one connected area (`GameState.java`, `linkComponents`).
- The source draws the void cells at random, smooths them with a cellular automaton, mirrors them, clears the HQ corners and joins separate areas with L-shaped corridors (along a row, then along a column). Our arena's generator reproduces its maps exactly, from the steps written in [MAP_GENERATOR.md](MAP_GENERATOR.md) ([ADR 0041](../../docs/adr/0041-ice-and-fire-source-maps-from-a-spec.md)).

### Mine spots

- Mine spots exist from Wood 1 on; in Wood 3 and Wood 2 there are none and their count is 0 (`GameState.java`, `generateMap`; statement, "Game Input").
- The count is even and between 6 and the larger of 6 and 15% (rounded down) of the non-void cells (statement, "Expert Rules"; `GameState.java`, `generateMap`, which counts the non-void cells before the HQs are placed).
- (1, 0), right of the red HQ, and (10, 11), left of the blue HQ, are always mine spots (statement, "Expert Rules").
- No mine spot is on an HQ cell, on (0, 1), or on (11, 10) (`GameState.java`, `generateMap`).

## Start of the game

- Each player owns its HQ cell; blue also owns (11, 10) (statement, "Expert Rules"; `GameState.java`, `createHQs`). Every other non-void cell is neutral.
- Each player starts with 20 gold (`GameState.java`, constructor: twice the cost of a level 1 unit). The statement does not give it.
- There are no units and no buildings but the two HQs.

## Turns

- Players alternate: red plays a whole turn, then blue. A game turn is red's turn then blue's; there are at most 100 game turns, so each player plays at most 100 turns (statement, "Victory Conditions"; `Referee.java`, `computeCurrentPlayer`).
- A player's turn has three parts: the start of turn, then the player's answer, then its commands, carried out one by one in the order written (statement, "Order of actions"; `Referee.java`, `gameTurn`).

### Start of a player's turn

In this order (`GameState.java`, `initTurn`; statement, "Income" and "Armies"):

1. The player's cells are marked active or inactive (below).
2. The player's units on inactive cells die.
3. Both players' incomes are computed.
4. The player's income is added to its gold.
5. If its gold is now below 0, the gold becomes 0 and all of the player's units die. The income that the turn's input shows is still the one of step 3 (the source does not recompute it).
6. The player's units may move again this turn.

### Active cells

A player's cell is active when a path of the player's own cells, from neighbour to neighbour, joins it to the player's HQ; otherwise it is inactive (statement, "Territory ownership"; `GameState.java`, `computeActiveCells`). The source recomputes both players' active cells after every move and every training, so a cut takes effect at once; units on cells that became inactive die only at the start of their owner's next turn (step 2 above).

An inactive cell stays owned, and a building on it is kept but inactive (statement, "Buildings").

### Income

A player's income is (statement, "Income"; `GameState.java`, `computeIncome`):

- +1 for each active cell it owns;
- +4 for each mine it owns on an active cell (the statement says "each mine they control");
- minus the upkeep of each of its units on an active cell: 1 for level 1, 4 for level 2, 20 for level 3. Units on inactive cells cost nothing until they die.

Both incomes are recomputed after every action that succeeds.

## Commands

A turn's answer is one line of commands separated by `;` (statement, "Output"). The commands:

| Command | Effect |
| --- | --- |
| `MOVE id x y` | Moves the player's unit `id` one cell toward (x, y) |
| `TRAIN level x y` | Trains a unit of that level on (x, y) |
| `BUILD MINE x y`, `BUILD TOWER x y` | Builds on (x, y) |
| `WAIT` | Nothing |
| `MSG text` | Shows `text` in the viewer; no effect on the game |

### Reading the answer

The whole line is read first; the commands are carried out afterwards (`Referee.java`, `readInput`, then `gameTurn`).

- The line is split on `;`, and each part then loses its leading and trailing whitespace (Java's `trim`: spaces, tabs and other characters up to U+0020). Parts that are already empty before trimming, at the very end of the line, are dropped (Java's `String.split`); any other part left empty, an empty line or a last part of only spaces included, is an unrecognised command (`Referee.java`, `readInput`).
- A part must then match exactly: `WAIT`; `MSG`, a space and any text (a bare `MSG` is unrecognised, since trimming removed its space); `MOVE` or `TRAIN` and three non-negative integers; or `BUILD`, `MINE` or `TOWER`, and two non-negative integers; in upper case, separated by single spaces (`Constants.java`, the patterns; `Referee.java`, `matchMsg`, `matchMoveTrain`, `matchBuild`). A part that matches none of them, or whose integer does not fit a 32-bit signed integer, is **unrecognised: the player loses at once** (statement, "Lose Conditions"; `Referee.java`, `readInput`). The parts before it have been read but none is carried out.
- While reading, a recognised command is dropped, with a note in the game summary and no other effect, when (`Referee.java`, `matchMoveTrain`, `matchBuild`, `createTrainAction`, `createMoveAction`):
  - (x, y) is off the map or a void cell;
  - `TRAIN` has a level below 1 or above the league's highest (in Wood 3, any level but 1);
  - `MOVE` names a unit that does not exist when the line is read (a unit trained earlier in the same line among them), or one of the opponent's.

### Carrying out the commands

Each command is checked when its turn comes, against the state left by the commands before it. A command that fails a check is skipped and the next one is tried (statement, "Order of actions"; `Referee.java`, `gameTurn`, `makeAction`).

#### Capture

A player's unit of level L may enter a cell, by training or moving, when (`Cell.java`, `isCapturable`; statement, "Armies"):

1. the cell is not an opponent's active cell protected by a tower, unless L is 3;
2. the cell does not hold an opponent's tower, active or not, unless L is 3;
3. and either the cell is empty (no unit, no building), or it is not the player's own and its unit, if any, can be beaten: a unit of level 3 beats any unit, a unit of level 2 beats level 1, a unit of level 1 beats none (in Wood 3, where only level 1 exists, level 1 beats level 1).

So a player never enters its own cell when it holds a unit or a building, its HQ included; an opponent's mine or HQ can be entered by any level, subject to rules 1 and 2 (a level 1 or 2 unit cannot enter an HQ protected by a tower).

On entering, the unit takes the cell: a unit there dies, a mine or tower there is destroyed, and the cell becomes the player's (`GameState.java`, `addUnit`, `moveUnit`, `clearCell`). An HQ is never destroyed: a player whose HQ cell an opponent's unit has entered has lost (below). The attacker always survives (statement, "Armies"). A cell changes owner only when a unit enters it: a unit that leaves a cell leaves it owned (`GameState.java`, `moveUnit`).

#### Protection

An opponent's cell is protected when it holds a tower, or when it is active and one of its neighbours is a cell of the same owner, active, that holds a tower (`Cell.java`, `isProtected`; statement, "Buildings"). The statement does not say that the tower and the protected cell must be active; the source requires both, except that a tower on an inactive cell still stops units below level 3 from entering its own cell (rule 2 above).

#### TRAIN level x y

Checks, in order (`Referee.java`, `makeAction`, `makeTrainAction`; statement, "Armies"):

1. (x, y) is the player's active cell or a neighbour of one;
2. the level is at most the league's highest;
3. the player has at least the unit's cost: 10, 20 or 30 gold for levels 1, 2 and 3;
4. a unit of that level may enter the cell (capture, above).

Then the unit appears on the cell, taking it as above, and the cost is paid. A unit trained this turn cannot move this turn (statement, "Armies"; `Unit.java`).

#### MOVE id x y

The target may be any non-void cell, at any distance. Checks, in order (`Referee.java`, `makeMoveAction`; `Pathfinding.java`, `getNearestCell`; statement, "Armies"):

1. The unit has not moved this turn and was not trained this turn.
2. If the target is a neighbour of the unit's cell and the unit may not enter it, the command fails.
3. Among the neighbours of the unit's cell that it may enter, it goes to the one nearest to the target, if that is strictly nearer than its own cell. Distance is the length of the shortest path between the two cells over non-void cells, ignoring units, buildings and owners. On equal distance, the first neighbour wins in the order up, right, down, left for red, and down, left, up, right for blue (the source orders them so to keep the game symmetric). If no neighbour is strictly nearer, the command fails and the unit stays.

A unit moves at most one cell per turn (statement, "Armies"; `Constants.java`, `MAX_MOVE_LENGTH`).

#### BUILD MINE x y and BUILD TOWER x y

Checks, in order (`Referee.java`, `makeAction`, `makeBuildAction`; statement, "Buildings"):

1. (x, y) is the player's active cell or a neighbour of one;
2. the league allows the building: none in Wood 3 and Wood 2, mines only in Wood 1, both from Bronze on;
3. the cell holds no unit and no building;
4. the cell is the player's own (with check 1, an active cell);
5. the player has at least 15 gold;
6. a mine goes on a mine spot; a tower never does.

Then the building appears and 15 gold is paid. Neither the statement nor the source of this version makes a mine cost more as a player owns more (the cost increment is 0, `Constants.java`, `MINE_INCREMENT`).

## End of the game

- **HQ captured.** After each command carried out, if a player's HQ cell belongs to its opponent, that player loses at once; the rest of the turn is not played (statement, "Victory Conditions"; `Referee.java`, `checkForHqCapture`).
- **Lost by answer.** A player that does not answer in time, or sends an unrecognised command, loses at once (statement, "Lose Conditions"; `Referee.java`, `readInput`).
- **After 100 game turns.** When both players have played 100 turns, each player's military power is its gold plus the training cost of each of its units alive (10, 20 or 30); the higher wins (statement, "Victory Conditions"; `Referee.java`, `discriminateEndGame`; `GameState.java`, `getScores`).
- **Scores.** The source gives the loser of an HQ capture, a timeout or an unrecognised command a score of −1 and the winner its military power; after 100 turns, each player gets its military power (`Referee.java`, `checkForEndGame`, `discriminateEndGame`). Equal military power: see "Platform facts to check".

## Protocol

### Initial input

Before the first turn's input, each player receives (statement, "Initialization input"; `Referee.java`, `sendInitialInput`):

1. `numberMineSpots`;
2. `numberMineSpots` lines `x y`, row by row: by y, then by x.

### Each turn's input

The player to move receives (statement, "Game turn input"; `GameState.java`, `sendState`):

1. its gold; 2. its income; 3. the opponent's gold; 4. the opponent's income;
5. 12 lines of 12 characters, one line per row y from 0, one character per x from 0: `#` void, `.` neutral, `O` own active, `o` own inactive, `X` opponent's active, `x` opponent's inactive;
6. `buildingCount`, then that many lines `owner buildingType x y`: owner 0 for the player's own and 1 for the opponent's; type 0 HQ, 1 mine, 2 tower. Own buildings come first, then by type, then by x, then by y (`GameState.java`, `sendBuildings`);
7. `unitCount`, then that many lines `owner unitId level x y`, own units first, then by id (`GameState.java`, `sendUnits`).

Unit ids are unique and grow as units are trained, by either player (`Unit.java`). In the source the counter is shared by every game the referee process plays, so bots should not expect the first unit to have id 1.

### Output

One line per turn, as in "Commands" above (statement, "Output"). The game shows at most 20 characters of a `MSG` text (`Player.java`).

## Time limits

1,000 ms for a player's first answer, then 50 ms per answer (statement, "Constraints").

## Leagues

The rules grow with the leagues, and stop growing at Bronze (statement, league notes; Wood 3 page; `Referee.java`, `init`; `Constants.java`):

| League | Units | Level 1 against level 1 | Buildings | Mine spots |
| --- | --- | --- | --- | --- |
| Wood 3 | Level 1 only | Beats it | None | None |
| Wood 2 | Levels 1 to 3 | Does not | None | None |
| Wood 1 | Levels 1 to 3 | Does not | Mines | Yes |
| Bronze and above | Levels 1 to 3 | Does not | Mines and towers | Yes |

## Where the statement and the source differ

Neither contradicts the other on any rule above; the source adds detail where the statement is silent or loose, and that detail applies until `rules-check` shows otherwise:

- Starting gold (20), the order of the start of a turn, the income shown after a wipe-out, the exact matching of commands, which commands are dropped while reading, how `MOVE` chooses its cell, and the order of buildings and units in the input: from the source only.
- The statement says a tower protects the owner's neighbouring cells; the source also requires the tower and the protected cell to be active.
- The statement says the cells next to an HQ are never void; the source keeps the whole 3×3 block at each corner.
- The statement's "+4 income for each mine they control" counts, in the source, only mines on active cells.
- The statement template declares five league levels; the source knows four and plays every level from the fourth up as Bronze.

## Platform facts to check

Neither source gives these; stage 5's `rules-check` bot checks them on CodinGame, and nothing here guesses them:

- Whether equal military power after 100 turns is a draw.
- Whether CodinGame's referee still matches commit `efe9e78`.
- Whether the time limits measure as the statement says.
- Which seat a bot gets: red, which plays first, or blue, which starts with one more cell.
