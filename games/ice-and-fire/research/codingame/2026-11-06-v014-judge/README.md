# Judgement of the players' moves

- One unit of score is one gold in the plan bot's evaluation (income counts 3, an active cell 4, a unit 10 per level, a tower 10); the gap threshold of 20 is about two level-1 units or five active cells, a clear material difference. Forced results (a win, or the opponent taking our HQ next turn) score 500,000 or more and are decided.
- 322 games used (0 left out), players judged: zasmu, Pduhard-, karliso, _iaz, reCurse, _iahelka, Waffle3z, Zylo, NinjaDoggy, aCat.
- 1987 of 1987 plies of these players sent to the judge, 1987 judged, 600 tries a search.
- The commands judged are those our engine carried out. The opening is treated as over, as `bank` does.

Scores are our search's, for the player to act: the worst evaluation after the opponent's replies. The gap is our plan's score less the played plan's; a gap above 20 counts as a played plan that scores much lower. A ply is decided when either score is at least 250000 in absolute value (a forced win or loss, not a board evaluation); decided plies are counted apart and left out of the gap statistics and of `largest-gaps.md`.

- `judgements.csv`: one row per judged ply.
  - `game_id`, `player`, `rank` (the player's rank in the leaderboard snapshot, empty if none), `ply` (index in the game), `turn` (the game's turn), `phase` (`early_game`, `middle_game`, `endgame`).
  - `played`, `ours`: the commands played and the commands our search chooses, joined by `;`.
  - `played_score`, `ours_score`: our search's scores of the two plans; `gap` is `ours_score` less `played_score`.
  - `match`: `same` (same commands), `same_first` (same first command only) or `different`.
  - `situation`: `undecided`, `ours_wins` (our search has a forced win), `played_loses` (the played plan loses by force) or `decided_other`.
- `summary.csv`: agreement and gap by player and phase (`all` too), in long format.
  - `player`, `phase`, `metric`, `value`; the metrics are `plies`, `same`, `same_first`, `different` (all plies), `undecided`, `ours_wins`, `played_loses`, `decided_other` (the plies by situation), and, over the undecided plies only, `median_gap`, `gap_over_threshold` (a gap above the threshold) and `gap_over_threshold_share`.
- `largest-gaps.md`: the 20 undecided plies with the largest gaps, each with both plans and the board before the ply.
