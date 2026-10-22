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
use crate::rollout::{choose, search, CAP};
use crate::score::Settings;

/// Which planner makes the plans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Planner {
    /// [`plan::beam`] with this width on the first turn and the replan width
    /// later.
    Beam { width: usize, replan: usize },
    /// S2: the weight set whose rollout scores best, played with the payback
    /// rule ([`crate::rollout::choose`]).
    Choose,
    /// S2, then hill climbing on the chosen weights until the deadline
    /// ([`crate::rollout::search`]).
    Search,
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
    /// How the middle game is scored.
    pub settings: Settings,
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
            settings: Settings::DEFAULT,
        }
    }

    /// Whether the opening still plays in `state`, at our start of turn. It
    /// ends for good at the middle game: when we can pay a chain of trainings
    /// to an enemy cell ([`crate::middle::reached`]), or when the plan's
    /// prediction, on track to its end, has reached the middle game (at the
    /// enemy's start of turn, which we do not see).
    pub fn playing(&mut self, state: &State) -> bool {
        let predicted = self.plans > 0 && self.played >= self.expected.len();
        if !self.over && (predicted || crate::middle::reached(state)) {
            self.over = true;
        }
        !self.over
    }

    /// The commands of our turn in `state`: the plan's next step, after a new
    /// plan if needed, searched until `deadline`.
    pub fn turn(&mut self, state: &State, deadline: Option<Instant>) -> Vec<Command> {
        let me = state.to_act();
        let on_track = self.expected.get(self.played) == Some(&side_key(state, me));
        let rest = Step {
            weights: 0,
            buy: Buy::Payback,
        };
        if !on_track {
            let first = self.plans == 0;
            // The weights of a choice, or of a search, added to the sets if new.
            let chosen = |found: Params, sets: &mut Vec<Params>| {
                let weights = sets.iter().position(|p| *p == found).unwrap_or_else(|| {
                    sets.push(found);
                    sets.len() - 1
                });
                let step = Step {
                    weights: weights as u8,
                    buy: Buy::Payback,
                };
                vec![step; CAP as usize]
            };
            self.plan = match self.planner {
                Planner::Beam { width, replan } => {
                    let ctx = self.context(deadline);
                    plan::beam(state, &ctx, if first { width } else { replan })
                }
                Planner::Search => {
                    let seed = 0x5eed ^ u64::from(state.game_turn());
                    let sets = &self.sets;
                    let found = search(
                        state,
                        sets,
                        &self.model,
                        self.settings,
                        deadline,
                        usize::MAX,
                        seed,
                    );
                    chosen(found, &mut self.sets)
                }
                Planner::Choose => {
                    let index = choose(state, &self.sets, &self.model, self.settings, deadline);
                    chosen(self.sets[index], &mut self.sets)
                }
            };
            let ctx = self.context(deadline);
            self.expected = plan::predicted_keys(state, &self.plan, &ctx);
            self.played = 0;
            self.plans += 1;
        }
        let step = self.plan.get(self.played).copied().unwrap_or(rest);
        self.played += 1;
        self.context(deadline).commands(state, step)
    }

    /// The planners' context over this opening's sets.
    fn context(&self, deadline: Option<Instant>) -> Context<'_> {
        Context {
            sets: &self.sets,
            model: self.model,
            rest: Step {
                weights: 0,
                buy: Buy::Payback,
            },
            deadline,
            settings: self.settings,
        }
    }
}

#[cfg(test)]
mod tests;
