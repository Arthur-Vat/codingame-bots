# Decisive moments of our losses: data

2176 games used, 1617 of them our losses. Made by `codingame-moments --out`; the report that links this folder reads these numbers.

## Files

- `moments.csv`: one row per loss (group `loss`), and one row per win of the control (group `win_control`).
- `summary.csv`: the summary tables in long format.
- `examples.md`: for the top players, the first losses with the boards before and after the decisive moment, in code blocks.

## Columns of moments.csv

- `group`: `loss`, or `win_control` (one of our wins used as a control: for each loss with a decisive moment of the row's kind at game turn t, each win that reached turn t is read at the first opponent ply of turn t, so a win can appear once per matched moment; wins that ended earlier are left out).
- `game_id`, `opponent`, `rank` (in the leaderboard snapshot, empty if unranked), `focus` (whether the opponent is a focus player, the top of the snapshot that we met), `our_seat` (side name).
- `kind`: the decisive kind (large cut, HQ threat, debt), or `none` for a loss without one; for a win, the kind of the matched moment.
- `ply`, `turn`: the ply of the moment (the control ply for a win) and its game turn (for a win, the turn matched).
- `theirs_<metric>`, `ours_<metric>` for `gold`, `income`, `cells`, `army`, `mines`, `towers`, `exposed30`, `hq_gap`, `material`, `voronoi`: each side's value right after its own last ply before the moment; empty if the side had no ply yet or the value means none. Empty for wins.
- `exposure_1` to `exposure_3`: our `exposed30` right after our last, second to last, ... own ply before the moment (before the control ply for a win); empty if there was no such ply.

## Columns of summary.csv

- `table`: `losses_by_kind` (by `group` focus or others, and `kind`, with `none` and `all`) or `exposure` (by `group` losses_focus, losses_others, wins_matched_focus or wins_matched_others, `kind`, `own_plies_back` 1 to 3).
- `threshold`: the exposure threshold, for `at_or_above` and `share_pct`; empty otherwise.
- `measure`: `count`, `median_turn`, `n` (cases with a value at that own ply back), `median` (of the exposure), `at_or_above` (cases at or above the threshold), `share_pct` (that share of `n`).
- `value`: the number.
