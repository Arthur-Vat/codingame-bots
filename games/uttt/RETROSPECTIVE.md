# Ultimate Tic-Tac-Toe: retrospective

The worked example of stage 11 of the [new game playbook](../../docs/NEW_GAME.md), written on 2026-10-13 while iteration is paused. Its sources are the [roadmap](../../docs/ROADMAP.md), the [journal](journal/README.md) and the decision records.

## Result

- Legend on 2026-10-07 with the first real bot, `uttt-v001` (MCTS), 182nd of 443; 59th with `uttt-v008` on 2026-10-08, the last rank recorded.
- Ten releases from `uttt-v001` to `uttt-v010`; 15 experiments, 10 accepted and 5 dropped ([journal](journal/README.md)).
- Phases 0 to 4 (rules, referee, engine, evaluation, first bot) took one day, 2026-10-07; the studio (phase 6) took 2026-10-10 to 2026-10-13.

## What worked

- **A readable referee, a fast engine and parity tests** between them, then the `rules-check` bot on CodinGame, which found no difference in Bronze league (2026-10-07).
- **The SPRT as the only judge of strength** ([ADR 0012](../../docs/adr/0012-evaluation.md)): every accepted experiment held at CodinGame's time limits, and ideas that looked good locally were dropped on evidence (E012, E013, E016).
- **The journal:** dropped ideas are on record with their numbers, so they are not retried blindly.
- **Speed first:** the largest gains came from a faster engine and better playouts (E004 to E006, E011).

## What cost more than planned

- **Learning:** the value network (E012, [ADR 0017](../../docs/adr/0017-value-network.md) to [ADR 0019](../../docs/adr/0019-value-network-gate-at-time.md)) took training runs and three decision records, and lost at the bot's time budget; small learned weights for playouts and priors (E011, E014, E015) paid instead.
- **The studio:** most of the pull requests from #62 to #112, with the interface reworked after the owner tried it.
- **Stacked pull requests** that merged siblings into each other made GitHub report conflicts after the first merges (#62 to #76); stacks are now linear (skill `land-work`).
- **Very long sessions** re-read their whole context on every step; sessions now hand over through `HANDOFF.md` ([ADR 0024](../../docs/adr/0024-handoff-branch.md)) and a fresh session is suggested (skill `handoff`).

## What belongs in the framework

What this game needed and the next one will reuse is already shared: the arena, the SPRT and league, the bundler, `cg-search`'s MCTS, the release and journal machinery, and the studio's shell. What is still shaped by this game:

- `cg-search`'s `Game` trait assumes two players taking turns with full information.
- The arena assumes two seats.
- The studio names the seats X and O, and lists a human's legal moves as complete answers, one click each.

## Proposed changes

Each is for the owner to choose; each chosen one becomes its own pull request.

1. When a game needs it: seat names and colours per game in the studio, instead of X and O (framework gap, stage 4 of that game).
2. When a game needs it: a turn built command by command in the studio, for games whose answer holds several commands.
3. A shared skeleton for a game's `rules-check` bot, if the second game's turns out close to this one's.
4. A `speed` example in every engine from the start, as this game's, so that MODEL.md's numbers are measured early.
