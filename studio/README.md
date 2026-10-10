# Studio

A local app to play and review games in the browser ([ADR 0025](../docs/adr/0025-studio.md)).

## How a game plugs in

A game adds an adapter crate, `games/<game>/studio`, that implements `studio_game::StudioGame` ([game/](game/)) and holds no rules of its own: positions come from replaying the game's referee, and a human's legal moves from the referee's turn input.

- `info`: the game's id (its folder name) and display name.
- `new_referee`: a fresh referee for a setup (seed and opening plies), the one the arena uses.
- `frames`: the JSON the board renderer draws, frame 0 for the start and then one per step, from the turns played.
- `human_moves`: the moves a human in a seat may play, each with the answer lines to send the referee.

`studio_game::live_game` replays turns with a fresh referee into a `cg_arena::live::LiveGame`. The adapter's frame format is documented in its crate; `games/uttt/studio` is the first.
