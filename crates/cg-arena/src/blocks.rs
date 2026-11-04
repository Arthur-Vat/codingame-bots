//! The SPRT over blocks played apart (ADR 0049, decision 1).
//!
//! An SPRT split over runners plays blocks of seat-swapped pairs, each
//! runner with seeds of its own ([`crate::tournament::block_seed`]), and
//! writes each block's games to a file. This module adds the blocks up and
//! checks the test's bounds.
//!
//! To make a verdict reproducible, blocks are counted in a fixed order, in
//! rounds: a round is the next `round_blocks` blocks of the list given, one
//! per runner. The bounds are checked after each whole round, in order, and
//! the first check that decides ends the test, whatever came after it and
//! however many rounds the caller handed over at once. A group of blocks
//! smaller than a round is left out. Checking only between rounds keeps
//! Wald's error bounds (they hold for any stopping rule that looks at the
//! evidence, here less often than after every pair); the simulations in the
//! tests measure the error rates and the pairs played.

use crate::sprt::{SprtSettings, Verdict};
use crate::summary::Summary;
use crate::tournament::GameRecord;

/// How a test over blocks ended, or that it has not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// H1: the candidate is stronger.
    Accepted,
    /// H0: the candidate is not stronger.
    Rejected,
    /// Still undecided at the cap on pairs.
    Capped,
    /// The candidate lost too many games by a fault (a crash, an invalid
    /// answer, or timeouts beyond the allowed rate).
    Fault,
    /// No bound is passed yet and the cap is not reached.
    Continue,
}

/// A fault check on one of the two bots.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaultCheck {
    /// The bot checked: 0 for the candidate, 1 for the baseline.
    pub bot: usize,
    /// Timeouts tolerated as a fraction of the games counted so far.
    pub max_timeout_rate: f64,
}

/// What a test over blocks checks.
#[derive(Clone, Debug)]
pub struct BlockTest {
    pub settings: SprtSettings,
    /// Blocks in a round: one per runner.
    pub round_blocks: usize,
    /// Pairs at which a test still undecided after a round stops.
    pub max_pairs: u32,
    /// The candidate and the baseline.
    pub names: [String; 2],
    pub faults: Option<FaultCheck>,
}

/// The state of a test over blocks after the rounds it counted.
#[derive(Clone, Debug)]
pub struct Checked {
    pub outcome: Outcome,
    /// The games of the counted blocks, from the candidate's side.
    pub summary: Summary,
    /// Blocks counted, in whole rounds up to the deciding one.
    pub counted_blocks: usize,
    /// Blocks handed over but not counted: after the deciding round, or in
    /// a last round that is not whole.
    pub left_out_blocks: usize,
    pub llr: f64,
}

impl Checked {
    /// Complete pairs counted.
    pub fn counted_pairs(&self) -> u32 {
        self.summary.pairs.iter().sum()
    }
}

impl BlockTest {
    /// Adds the blocks up in the order given, round by round, and stops at
    /// the first round after which a fault, a bound or the cap decides.
    /// Fails when a block is not made of complete seat-swapped pairs of the
    /// two bots.
    pub fn check(&self, blocks: &[Vec<GameRecord>]) -> Result<Checked, String> {
        if self.round_blocks == 0 {
            return Err("a round needs at least one block".to_string());
        }
        self.settings.validate()?;
        let mut summary = Summary::new(self.names.clone());
        let mut counted_blocks = 0;
        let mut outcome = Outcome::Continue;
        for round in blocks.chunks_exact(self.round_blocks) {
            for (offset, block) in round.iter().enumerate() {
                add_block(&mut summary, block, &self.names)
                    .map_err(|err| format!("block {}: {err}", counted_blocks + offset + 1))?;
            }
            counted_blocks += round.len();
            outcome = self.decide(&summary);
            if outcome != Outcome::Continue {
                break;
            }
        }
        Ok(Checked {
            outcome,
            counted_blocks,
            left_out_blocks: blocks.len() - counted_blocks,
            llr: self.settings.llr(&summary.pairs),
            summary,
        })
    }

    /// The outcome after a round: a fault first, then the bounds, then the cap.
    fn decide(&self, summary: &Summary) -> Outcome {
        if let Some(check) = &self.faults {
            let found = &summary.faults[check.bot];
            if !found.within(summary.games(), check.max_timeout_rate) {
                return Outcome::Fault;
            }
        }
        match self.settings.verdict(&summary.pairs) {
            Verdict::Accepted => Outcome::Accepted,
            Verdict::Rejected => Outcome::Rejected,
            Verdict::Continue if summary.pairs.iter().sum::<u32>() >= self.max_pairs => {
                Outcome::Capped
            }
            Verdict::Continue => Outcome::Continue,
        }
    }
}

/// Adds one block's games to the summary after checking them: every pair has
/// its two games, one with each bot in seat 0, played by the two bots.
fn add_block(
    summary: &mut Summary,
    block: &[GameRecord],
    names: &[String; 2],
) -> Result<(), String> {
    let mut seen: std::collections::BTreeMap<u32, [bool; 2]> = std::collections::BTreeMap::new();
    for record in block {
        let first_seat = usize::from(record.swapped);
        let seats = &record.game.seats;
        if seats[first_seat] != names[0] || seats[1 - first_seat] != names[1] {
            return Err(format!(
                "pair {} was played by {:?} and {:?}, not by {:?} and {:?}",
                record.pair, seats[0], seats[1], names[0], names[1]
            ));
        }
        let games = seen.entry(record.pair).or_default();
        if std::mem::replace(&mut games[usize::from(record.swapped)], true) {
            return Err(format!(
                "pair {} has two games with the same bot in seat 0",
                record.pair
            ));
        }
    }
    if let Some((pair, _)) = seen.iter().find(|(_, games)| !(games[0] && games[1])) {
        return Err(format!("pair {pair} is not complete"));
    }
    for record in block {
        summary.add(record);
    }
    Ok(())
}

/// Reads the games of a block from a `--out` file: one JSON object per line.
pub fn read_block(path: &std::path::Path) -> Result<Vec<GameRecord>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|err| format!("cannot read {}: {err}", path.display()))?;
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(number, line)| {
            serde_json::from_str(line)
                .map_err(|err| format!("{}, line {}: {err}", path.display(), number + 1))
        })
        .collect()
}

#[cfg(test)]
mod tests;
