//! Judgements of a chosen player's moves by our own search (ADR 0056,
//! decision 8): at each ply the player played, what our search would play
//! there, and how the search scores both its own plan and the played one.
//! Where it scores the played plan far below its own, our evaluation
//! underrates what the player did.
//!
//! A game provides the [`Judging`] hooks (how two moves compare, what a
//! large score gap is) and the judgements themselves (its search runs them);
//! everything else is generic: [`judgements_csv`], [`summary_csv`] (the
//! agreement and the score gap by player and phase), [`largest_gaps`] (the
//! plies with the largest gaps, with boards) and [`readme`] make the files of
//! a judgement folder.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::trace::Phase;

/// The plies listed as the largest gaps.
pub const WORST_SHOWN: usize = 20;

/// How a played move compares with ours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveMatch {
    /// The same move.
    Same,
    /// Not the same, but the same first command.
    First,
    /// Neither.
    Different,
}

/// What a game provides to judge its players' moves.
pub trait Judging {
    /// How the move `played` compares with `ours`, both as the game writes
    /// a move (the commands of a ply, for example).
    fn compare(played: &str, ours: &str) -> MoveMatch;
    /// A score gap (ours less played) above which the played move counts as
    /// scored much lower than ours.
    fn gap_threshold() -> i64;
    /// A score at or beyond which (either sign) a position is decided: the
    /// score of a forced win or loss, not an evaluation of the board.
    fn decided_bound() -> i64;
}

/// What a judged ply is, by the scores of the two plans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Situation {
    /// Neither score is a forced result: the gap measures the evaluation.
    Undecided,
    /// Our search finds a forced win.
    OursWins,
    /// The played plan loses by force (the opponent wins next turn).
    PlayedLoses,
    /// Any other decided position.
    Other,
}

impl Situation {
    /// The name in the CSV files.
    pub fn name(self) -> &'static str {
        match self {
            Situation::Undecided => "undecided",
            Situation::OursWins => "ours_wins",
            Situation::PlayedLoses => "played_loses",
            Situation::Other => "decided_other",
        }
    }
}

/// One ply of a player, judged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Judgement {
    pub player: String,
    /// The player's rank in the leaderboard snapshot, if it has one.
    pub rank: Option<u32>,
    pub game_id: u64,
    pub ply: usize,
    /// The game's turn number at the ply.
    pub turn: u32,
    pub phase: Phase,
    /// What our search would play.
    pub ours: String,
    /// What the player played.
    pub played: String,
    /// Our search's scores of the two plans, higher being better for the
    /// player to act.
    pub ours_score: i64,
    pub played_score: i64,
    /// The board before the ply, one line a row.
    pub board: Vec<String>,
}

impl Judgement {
    /// Whether a score is a forced result under `bound`, and which.
    pub fn situation(&self, bound: i64) -> Situation {
        if self.ours_score >= bound {
            Situation::OursWins
        } else if self.played_score <= -bound {
            Situation::PlayedLoses
        } else if self.ours_score <= -bound || self.played_score >= bound {
            Situation::Other
        } else {
            Situation::Undecided
        }
    }

    /// Our score less the played plan's.
    pub fn gap(&self) -> i64 {
        self.ours_score.saturating_sub(self.played_score)
    }
}

/// The median of `values`: the mean of the two middle ones when even.
fn median(values: &mut [i64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_unstable();
    let n = values.len();
    Some(if n % 2 == 1 {
        values[n / 2] as f64
    } else {
        (values[n / 2 - 1] as f64 + values[n / 2] as f64) / 2.0
    })
}

/// The rows of a table by player and phase: for each player, a row for each
/// phase it has plies in, and one for all of them.
fn groups(judgements: &[Judgement]) -> Vec<(String, &'static str, Vec<&Judgement>)> {
    let mut by: BTreeMap<&str, Vec<&Judgement>> = BTreeMap::new();
    for j in judgements {
        by.entry(&j.player).or_default().push(j);
    }
    let mut out = Vec::new();
    for (player, all) in by {
        for phase in [Phase::Early, Phase::Middle, Phase::End] {
            let rows: Vec<&Judgement> = all.iter().copied().filter(|j| j.phase == phase).collect();
            if !rows.is_empty() {
                out.push((player.to_string(), phase.name(), rows));
            }
        }
        out.push((player.to_string(), "all", all));
    }
    out
}

/// A CSV field, quoted when it holds a comma, a quote or a line break.
fn field(text: &str) -> String {
    if text.contains([',', '"', '\n']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

/// The name of a match in the CSV files.
fn match_name(m: MoveMatch) -> &'static str {
    match m {
        MoveMatch::Same => "same",
        MoveMatch::First => "same_first",
        MoveMatch::Different => "different",
    }
}

/// The phase as the CSV files write it.
fn phase_name(phase: &str) -> String {
    phase.replace(' ', "_")
}

/// `judgements.csv`: one row per judged ply.
pub fn judgements_csv<J: Judging>(judgements: &[Judgement]) -> String {
    let mut out = String::from(
        "game_id,player,rank,ply,turn,phase,played,ours,played_score,ours_score,gap,match,situation\n",
    );
    for j in judgements {
        let _ = writeln!(
            out,
            "{},{},{},{},{},{},{},{},{},{},{},{},{}",
            j.game_id,
            field(&j.player),
            j.rank.map_or(String::new(), |r| r.to_string()),
            j.ply,
            j.turn,
            phase_name(j.phase.name()),
            field(&j.played),
            field(&j.ours),
            j.played_score,
            j.ours_score,
            j.gap(),
            match_name(J::compare(&j.played, &j.ours)),
            j.situation(J::decided_bound()).name()
        );
    }
    out
}

/// `summary.csv`: the agreement and the gap by player and phase, in long
/// format (`player,phase,metric,value`).
pub fn summary_csv<J: Judging>(judgements: &[Judgement]) -> String {
    let threshold = J::gap_threshold();
    let mut out = String::from("player,phase,metric,value\n");
    for (player, phase, js) in groups(judgements) {
        let phase = phase_name(phase);
        let count = |m: MoveMatch| {
            js.iter()
                .filter(|j| J::compare(&j.played, &j.ours) == m)
                .count()
        };
        let bound = J::decided_bound();
        let situations = |s: Situation| js.iter().filter(|j| j.situation(bound) == s).count();
        let mut gaps: Vec<i64> = js
            .iter()
            .filter(|j| j.situation(bound) == Situation::Undecided)
            .map(|j| j.gap())
            .collect();
        let over = gaps.iter().filter(|&&g| g > threshold).count();
        let rows = [
            ("plies", js.len().to_string()),
            ("same", count(MoveMatch::Same).to_string()),
            ("same_first", count(MoveMatch::First).to_string()),
            ("different", count(MoveMatch::Different).to_string()),
            ("undecided", gaps.len().to_string()),
            ("ours_wins", situations(Situation::OursWins).to_string()),
            (
                "played_loses",
                situations(Situation::PlayedLoses).to_string(),
            ),
            ("decided_other", situations(Situation::Other).to_string()),
            (
                "median_gap",
                median(&mut gaps).map_or(String::new(), |m| format!("{m:.1}")),
            ),
            ("gap_over_threshold", over.to_string()),
            (
                "gap_over_threshold_share",
                format!("{:.3}", over as f64 / gaps.len().max(1) as f64),
            ),
        ];
        for (metric, value) in rows {
            let _ = writeln!(out, "{},{phase},{metric},{value}", field(&player));
        }
    }
    out
}

/// `largest-gaps.md`: the plies with the largest gaps, with their boards.
pub fn largest_gaps<J: Judging>(judgements: &[Judgement]) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# The {WORST_SHOWN} plies with the largest gaps\n\nUndecided positions only (no forced win or loss in either score), where our search thinks the move played is worst relative to its own (the gap is our plan's score less the played plan's).\n"
    );
    let mut worst: Vec<&Judgement> = judgements
        .iter()
        .filter(|j| j.situation(J::decided_bound()) == Situation::Undecided)
        .collect();
    worst.sort_by(|a, b| {
        b.gap()
            .cmp(&a.gap())
            .then(a.game_id.cmp(&b.game_id))
            .then(a.ply.cmp(&b.ply))
    });
    for (rank, j) in worst.iter().take(WORST_SHOWN).enumerate() {
        let _ = writeln!(
            out,
            "## {}. {} in game {}, turn {} ({}), gap {}\n",
            rank + 1,
            j.player,
            j.game_id,
            j.turn,
            j.phase.name(),
            j.gap()
        );
        let _ = writeln!(out, "- played ({}): `{}`", j.played_score, j.played);
        let _ = writeln!(out, "- ours ({}): `{}`\n", j.ours_score, j.ours);
        let _ = writeln!(out, "```text");
        for row in &j.board {
            let _ = writeln!(out, "{row}");
        }
        let _ = writeln!(out, "```\n");
    }
    out
}

/// `README.md` of the folder: how it was made (`notes`, one bullet each),
/// then one line per file and column.
pub fn readme<J: Judging>(notes: &[String]) -> String {
    let mut out = String::from("# Judgement of the players' moves\n\n");
    for note in notes {
        let _ = writeln!(out, "- {note}");
    }
    let _ = writeln!(
        out,
        "\nScores are our search's, for the player to act: the worst evaluation after the opponent's replies. The gap is our plan's score less the played plan's; a gap above {} counts as a played plan that scores much lower. A ply is decided when either score is at least {} in absolute value (a forced win or loss, not a board evaluation); decided plies are counted apart and left out of the gap statistics and of `largest-gaps.md`.\n",
        J::gap_threshold(),
        J::decided_bound()
    );
    out.push_str(
        "- `judgements.csv`: one row per judged ply.\n  - `game_id`, `player`, `rank` (the player's rank in the leaderboard snapshot, empty if none), `ply` (index in the game), `turn` (the game's turn), `phase` (`early_game`, `middle_game`, `endgame`).\n  - `played`, `ours`: the commands played and the commands our search chooses, joined by `;`.\n  - `played_score`, `ours_score`: our search's scores of the two plans; `gap` is `ours_score` less `played_score`.\n  - `match`: `same` (same commands), `same_first` (same first command only) or `different`.\n  - `situation`: `undecided`, `ours_wins` (our search has a forced win), `played_loses` (the played plan loses by force) or `decided_other`.\n- `summary.csv`: agreement and gap by player and phase (`all` too), in long format.\n  - `player`, `phase`, `metric`, `value`; the metrics are `plies`, `same`, `same_first`, `different` (all plies), `undecided`, `ours_wins`, `played_loses`, `decided_other` (the plies by situation), and, over the undecided plies only, `median_gap`, `gap_over_threshold` (a gap above the threshold) and `gap_over_threshold_share`.\n- `largest-gaps.md`: the 20 undecided plies with the largest gaps, each with both plans and the board before the ply.\n",
    );
    out
}

#[cfg(test)]
mod tests;
