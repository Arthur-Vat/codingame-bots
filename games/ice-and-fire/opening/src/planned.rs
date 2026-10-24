//! An opening played by a plan of the first turn (E004): the plan is made on
//! the first turn with its long time and replayed while our side of the
//! board is as the plan predicted (the opponent played as the model, or its
//! moves did not touch us), then the default step up to the predicted middle
//! game; otherwise, and after that, a new plan is made within the turn's
//! time.
//!
//! With [`Planner::Solo`] (E010) the plan is [`crate::solo::plan`]'s, our
//! side alone for a few turns, and it is made again whenever it runs out
//! before the middle game.

use std::time::Instant;

use ice_and_fire_engine::{Command, State};

use crate::econ::{Buy, Params};
use crate::plan::{self, side_key, Context, Step};
use crate::rollout::{choose, CAP};
use crate::score::Settings;
use crate::solo::{self, Objective};

/// Which planner makes the plans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Planner {
    /// [`plan::beam`] with this width on the first turn and the replan width
    /// later.
    Beam { width: usize, replan: usize },
    /// S2: the weight set whose rollout scores best, played with the payback
    /// rule ([`crate::rollout::choose`]).
    Choose,
    /// E010: [`solo::plan`] over `turns` of our turns, the opponent passing,
    /// with [`solo::SETS`] as the weight sets (`sets` must hold them), the
    /// `objective`, and beams up to `width` when there is no deadline; again
    /// for `turns` turns whenever the plan runs out or our side differs from
    /// it. The opening ends only at the middle game.
    Solo {
        turns: usize,
        width: usize,
        objective: Objective,
    },
}

/// What the opening does when our side of the board is not what the plan
/// predicted for this turn (E013).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffTrack {
    /// Make a new plan from the board (E004, E010).
    Replan,
    /// Keep playing the plan's steps on the board; a new plan is made only
    /// when the plan runs out.
    Keep,
    /// End the opening: the middle-game search plays from this turn.
    End,
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
    /// Whether the middle game has begun: the opening is then over for good.
    over: bool,
    /// What to do off track ([`OffTrack::Replan`] by default).
    pub off_track: OffTrack,
    /// The most of our turns the opening plays, if any (E013).
    pub max_turns: Option<usize>,
    /// The fewest of our turns the opening plays before the middle game may
    /// end it, if any (E013).
    pub min_turns: Option<usize>,
    /// Our turns the opening has played.
    turns: usize,
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
            over: false,
            off_track: OffTrack::Replan,
            max_turns: None,
            min_turns: None,
            turns: 0,
        }
    }

    /// Whether the opening still plays in `state`, at our start of turn. It
    /// ends for good at the middle game: when we can pay a chain of trainings
    /// to an enemy cell ([`crate::middle::reached`]), or when the plan's
    /// prediction, on track to its end, has reached the middle game (at the
    /// enemy's start of turn, which we do not see).
    pub fn playing(&mut self, state: &State) -> bool {
        let solo = matches!(self.planner, Planner::Solo { .. });
        let predicted = !solo && self.plans > 0 && self.played >= self.expected.len();
        let capped = self.max_turns.is_some_and(|max| self.turns >= max);
        let early = self.min_turns.is_some_and(|min| self.turns < min);
        let middle = !early && crate::middle::reached(state);
        if !self.over && (predicted || capped || middle) {
            self.over = true;
        }
        if !self.over && self.off_track == OffTrack::End && self.diverged(state) {
            self.over = true;
        }
        !self.over
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
        let replan = match self.off_track {
            OffTrack::Keep => self.plans == 0 || self.played >= self.plan.len(),
            OffTrack::Replan | OffTrack::End => {
                self.expected.get(self.played) != Some(&side_key(state, state.to_act()))
            }
        };
        self.turns += 1;
        if replan {
            let first = self.plans == 0;
            self.plan = match self.planner {
                Planner::Beam { width, replan } => {
                    plan::beam(state, &ctx, if first { width } else { replan })
                }
                Planner::Choose => {
                    let sets = &self.sets;
                    let weights = choose(state, sets, &self.model, ctx.settings, deadline);
                    let step = Step {
                        weights: weights as u8,
                        buy: Buy::Payback,
                    };
                    vec![step; CAP as usize]
                }
                Planner::Solo {
                    turns,
                    width,
                    objective,
                } => {
                    let objective = &objective;
                    let rest = solo::REST;
                    let mut steps =
                        solo::plan(state, &self.sets, rest, turns, width, deadline, objective);
                    // The plan was rated with `rest` after its steps.
                    steps.resize(turns, rest);
                    steps
                }
            };
            self.expected = match self.planner {
                Planner::Solo { .. } => solo_keys(state, &self.sets, &self.plan),
                _ => plan::predicted_keys(state, &self.plan, &ctx),
            };
            self.played = 0;
            self.plans += 1;
        }
        let step = self.plan.get(self.played).copied().unwrap_or(ctx.rest);
        self.played += 1;
        ctx.commands(state, step)
    }
}

impl Planned {
    /// Whether a plan is being played and our side in `state` is not what it
    /// predicted for this turn.
    fn diverged(&self, state: &State) -> bool {
        self.plans > 0
            && self.played < self.expected.len()
            && self.expected[self.played] != side_key(state, state.to_act())
    }
}

/// Our side's key at the start of each turn of the solo `plan` from
/// `state`, the opponent passing.
fn solo_keys(state: &State, sets: &[Params], plan: &[Step]) -> Vec<u64> {
    let me = state.to_act();
    let mut keys = Vec::with_capacity(plan.len());
    let mut sim = state.clone();
    for &step in plan {
        if sim.outcome().is_some() {
            break;
        }
        keys.push(side_key(&sim, me));
        sim = solo::advance(&sim, &solo::commands(&sim, sets, step));
    }
    keys
}

#[cfg(test)]
mod tests;
