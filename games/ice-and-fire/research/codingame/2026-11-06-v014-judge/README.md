# Judgement of the players' moves

- 312 games used (0 left out), players judged: zasmu, Pduhard-, karliso, _iaz, reCurse, _iahelka, Waffle3z, Zylo, NinjaDoggy, aCat.
- 1769 of 1769 plies of these players sent to the judge, 1769 judged, 600 tries a search.
- The commands judged are those our engine carried out. The opening is treated as over, as `bank` does.

Scores are our search's, for the player to act: the worst evaluation after the opponent's replies. The gap is our plan's score less the played plan's; a gap above 20 counts as a played plan that scores much lower.

- `judgements.csv`: one row per judged ply.
  - `game_id`, `player`, `rank` (the player's rank in the leaderboard snapshot, empty if none), `ply` (index in the game), `turn` (the game's turn), `phase` (`early_game`, `middle_game`, `endgame`).
  - `played`, `ours`: the commands played and the commands our search chooses, joined by `;`.
  - `played_score`, `ours_score`: our search's scores of the two plans; `gap` is `ours_score` less `played_score`.
  - `match`: `same` (same commands), `same_first` (same first command only) or `different`.
- `summary.csv`: agreement and gap by player and phase (`all` too), in long format.
  - `player`, `phase`, `metric`, `value`; the metrics are `plies`, `same`, `same_first`, `different`, `median_gap`, `gap_over_threshold` (plies with a gap above the threshold) and `gap_over_threshold_share`.
- `largest-gaps.md`: the 20 plies with the largest gaps, each with both plans and the board before the ply.
