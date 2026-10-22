//! An opening played by a plan of the first turn (E004): the plan is made on
//! the first turn with its long time and replayed while our side of the
//! board is as the plan predicted (the opponent played as the model, or its
//! moves did not touch us), then the default step up to the predicted middle
//! game; otherwise, and after that, a new plan is made within the turn's
//! time.

use std::time::Instant;

use ice_and_fire_engine::{Command, State};

use crate::econ::{Buy, Params};
use crate::plan::{self, side_key, Context, Step};
use crate::score::Settings;

/// Which planner makes the plans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Planner {
    /// [`plan::beam`] with this width on the first turn and the replan width
    /// later.
    Beam { width: usize, replan: usize },
    /// [`plan::schedule`], this many schedules checked.
    Schedule(usize),
    /// [`plan::anneal`].
    Anneal,
    /// [`plan::cross_entropy`], this many plans a round.
    CrossEntropy(usize),
    /// S2: the weight set whose rollout scores best, played with the
    /// payback rule ([`crate::rollout::choose`]).
    Choose,
}

/// The opening's memory between turns.
pub struct Planned {
    pub planner: Planner,
    /// The weight sets of the steps; the first is the default.
    pub sets: Vec<Params>,
    /// The opponent's model.
    pub model: Params,
    plan: Vec<Step>,
    /// Our side's key expected at the start of each turn of the plan.
    expected: Vec<u64>,
    played: usize,
    /// Plans made so far, the first included.
    pub plans: u32,
}

impl Planned {
    pub fn new(planner: Planner, sets: Vec<Params>, model: Params) -> Planned {
        Planned {
            planner,
            sets,
            model,
            plan: Vec::new(),
            expected: Vec::new(),
            played: 0,
            plans: 0,
        }
    }

    /// The commands of our turn in `state`: the plan's next step, after a new
    /// plan if needed, searched until `deadline`.
    pub fn turn(&mut self, state: &State, deadline: Option<Instant>) -> Vec<Command> {
        let ctx = Context {
            sets: &self.sets,
            model: self.model,
            rest: Step {
                weights: 0,
                buy: Buy::Payback,
            },
            deadline,
            settings: Settings::DEFAULT,
        };
        let me = state.to_act();
        let on_track = self.expected.get(self.played) == Some(&side_key(state, me));
        if !on_track {
            let first = self.plans == 0;
            let seed = 0x5eed ^ u64::from(state.game_turn());
            self.plan = match self.planner {
                Planner::Beam { width, replan } => {
                    plan::beam(state, &ctx, if first { width } else { replan })
                }
                Planner::Schedule(checked) => plan::schedule(state, &ctx, checked),
                Planner::Anneal => plan::anneal(state, &ctx, seed, usize::MAX),
                Planner::CrossEntropy(samples) => {
                    plan::cross_entropy(state, &ctx, seed, samples, usize::MAX)
                }
                Planner::Choose => {
                    let weights = crate::rollout::choose(
                        state,
                        &self.sets,
                        &self.model,
                        ctx.settings,
                        deadline,
                    );
                    let step = Step {
                        weights: weights as u8,
                        buy: Buy::Payback,
                    };
                    vec![step; usize::from(crate::rollout::CAP as u8)]
                }
            };
            self.expected = plan::predicted_keys(state, &self.plan, &ctx);
            self.played = 0;
            self.plans += 1;
        }
        let step = self.plan.get(self.played).copied().unwrap_or(ctx.rest);
        self.played += 1;
        ctx.commands(state, step)
    }
}

#[cfg(test)]
mod tests;
