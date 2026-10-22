//! The plan search of the `plan` bot (experiment E002), after the contest's
//! winner (`games/ice-and-fire/research/README.md`, "What strong bots do"),
//! moved out of the bot so that other crates can play the middle game with
//! it (E005). Standard library only, so the bundler can copy it into bots.
//!
//! A turn is a list of commands, the plan. [`plan::think`] first looks for a
//! lethal move or chain of trainings and plays it. Else it climbs: from the
//! start of the turn it keeps adding the single action (a move, a training,
//! a tower, a mine, one of the best cuts, a lethal chain) that most raises
//! the score of the plan, until none does or the time is up. The score of a
//! plan is the worst evaluation of the state after it, our turn ended, over
//! the replies of the opponent in a memory (its lethal chain, its best cuts
//! and bronze's greedy turn, each answered by the plan). After a climb the
//! opponent's replies to the plan are added to the memory and the plan is
//! searched again against them; the plan that scores best against the whole
//! memory is played. The evaluation ([`eval`]) counts a side that cannot pay
//! its upkeep as having lost its units.

pub mod actions;
pub mod eval;
pub mod plan;
