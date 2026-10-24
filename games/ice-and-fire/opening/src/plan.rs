//! Planners of the first turn (E004): each searches a plan of our turns up
//! to the middle game, the opponent simulated as v003's `econ`, and returns
//! the plan whose middle game scores best for us.
//!
//! A plan is a list of [`Step`]s, one a turn: the weights of the opening's
//! moves and how it buys. A step gives commands for any state, so a plan
//! stays playable when the opponent plays otherwise than the model. After its
//! last step a plan goes on with [`Context::rest`].
//!
//! The planner here is [`beam`] (P1 of E004), a beam search over whole
//! turns; the frame (steps, rollouts, predicted keys) also serves the
//! portfolio's choice that the `plan` bot plays (`planned::Planner::Choose`),
//! which beat v003 in the arena where the beam did not. A dynamic program over
//! the purchases of each turn (P2), simulated annealing (P4) and the
//! cross-entropy method (P5) are in E004's screening commit (92d96bd).
//!
//! Plans are ranked by the score of the middle game they reach
//! ([`Context::settings`]).

use std::collections::HashMap;
use std::time::Instant;

use ice_and_fire_engine::{Command, State};

use crate::econ::{self, Buy, Params};
use crate::middle;
use crate::rollout::{self, CAP};
use crate::score::{self, Settings};

/// One of our turns: the weights of the opening (an index into
/// [`Context::sets`]) and how it buys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Step {
    pub weights: u8,
    pub buy: Buy,
}

/// All that can be bought, as many as the gold allows.
const ALL: u8 = u8::MAX;

/// The purchases a planner tries on a turn.
pub const BUYS: [Buy; 8] = [
    Buy::Payback,
    Buy::Exact { mines: 0, units: 0 },
    Buy::Exact { mines: 0, units: 1 },
    Buy::Exact { mines: 0, units: 2 },
    Buy::Exact {
        mines: 0,
        units: ALL,
    },
    Buy::Exact { mines: 1, units: 0 },
    Buy::Exact { mines: 1, units: 1 },
    Buy::Exact {
        mines: 1,
        units: ALL,
    },
];

/// What a planner works with.
pub struct Context<'a> {
    /// The weight sets steps may use; the first is the default.
    pub sets: &'a [Params],
    /// The opponent's weights.
    pub model: Params,
    /// The step played after a plan's last.
    pub rest: Step,
    /// No new search work starts after this.
    pub deadline: Option<Instant>,
    /// How the middle game is scored.
    pub settings: Settings,
}

impl Context<'_> {
    fn out_of_time(&self) -> bool {
        self.deadline.is_some_and(|t| Instant::now() >= t)
    }

    fn params(&self, step: Step) -> &Params {
        &self.sets[usize::from(step.weights) % self.sets.len()]
    }

    /// The commands of `step` in `state`.
    pub fn commands(&self, state: &State, step: Step) -> Vec<Command> {
        econ::turn_with(state, self.params(step), step.buy)
    }

    /// The middle game reached by `plan` from `state`, our turn.
    pub fn play(&self, state: &State, plan: &[Step]) -> State {
        rollout::play(state, &self.model, |i, s| {
            self.commands(s, plan.get(i).copied().unwrap_or(self.rest))
        })
    }

    /// The score for us of the middle game reached by `plan`.
    pub fn value(&self, state: &State, plan: &[Step], settings: Settings) -> i64 {
        score::score(&self.play(state, plan), state.to_act(), settings)
    }

    /// The best of `plans` (each with its value), the first on a tie; the
    /// plan of [`Context::rest`] alone if there is none.
    fn finish(&self, plans: Vec<(i64, Vec<Step>)>) -> Vec<Step> {
        let mut best: Option<(i64, Vec<Step>)> = None;
        for (value, plan) in plans {
            if best.as_ref().is_none_or(|(top, _)| value > *top) {
                best = Some((value, plan));
            }
        }
        best.map(|(_, plan)| plan).unwrap_or_default()
    }

    /// The distinct steps worth trying in `state`: the default weights with
    /// each purchase of [`BUYS`], then the other weights with the payback
    /// rule; steps whose commands repeat an earlier step's are left out.
    fn steps(&self, state: &State) -> Vec<(Step, Vec<Command>)> {
        let mut out: Vec<(Step, Vec<Command>)> = Vec::new();
        let mut options: Vec<Step> = BUYS.iter().map(|&buy| Step { weights: 0, buy }).collect();
        for weights in 1..self.sets.len().min(usize::from(u8::MAX)) {
            options.push(Step {
                weights: weights as u8,
                buy: Buy::Payback,
            });
        }
        for step in options {
            let commands = self.commands(state, step);
            if !out.iter().any(|(_, c)| *c == commands) {
                out.push((step, commands));
            }
        }
        out
    }
}

/// Plays our `commands` and the model's reply from `state`, our turn. True
/// with the state when the rollout is over: the middle game (at either
/// start of turn), the end of the game or the cap.
fn advance(state: &State, commands: &[Command], model: &Params) -> (State, bool) {
    let mut next = state.clone();
    next.play_turn(commands);
    if next.outcome().is_some() || middle::reached(&next) {
        return (next, true);
    }
    let reply = econ::rules_turn_with(&next, model);
    next.play_turn(&reply);
    let over = next.outcome().is_some() || middle::reached(&next) || next.game_turn() > CAP;
    (next, over)
}

/// Our side's key at each of our starts of turn while `plan`, then
/// [`Context::rest`], is played from `state` against the model (the first is
/// `state`'s), up to the middle game or the cap.
pub fn predicted_keys(state: &State, plan: &[Step], ctx: &Context<'_>) -> Vec<u64> {
    let me = state.to_act();
    let mut keys = vec![side_key(state, me)];
    let mut sim = state.clone();
    for turn in 0..CAP as usize {
        let step = plan.get(turn).copied().unwrap_or(ctx.rest);
        let commands = ctx.commands(&sim, step);
        let (next, over) = advance(&sim, &commands, &ctx.model);
        if over {
            break;
        }
        sim = next;
        keys.push(side_key(&sim, me));
    }
    keys
}

/// A key of our side of a state: our cells, units, our mines and our gold.
/// The enemy's mines are not ours (E013): one built anywhere would make a
/// plan look off track.
pub fn side_key(state: &State, me: usize) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |word: u64| hash = (hash ^ word).wrapping_mul(0x0100_0000_01b3);
    let mines = state.mines() & state.owned(me);
    for (tag, bits) in [state.owned(me), state.units(me), mines]
        .into_iter()
        .enumerate()
    {
        for cell in bits.iter() {
            mix(((tag as u64) << 8) | cell as u64);
        }
        mix(0xff);
    }
    mix(state.gold(me) as u64);
    hash
}

/// P1: beam search over whole turns, `width` nodes a turn. Each node is
/// expanded with [`Context::steps`] and the model's reply, and valued by the
/// search score of its state; nodes that reach the middle game are kept
/// aside. When time is up the nodes left are played on to the middle game.
pub fn beam(state: &State, ctx: &Context<'_>, width: usize) -> Vec<Step> {
    let me = state.to_act();
    struct Node {
        state: State,
        plan: Vec<Step>,
    }
    let mut frontier = vec![Node {
        state: state.clone(),
        plan: Vec::new(),
    }];
    // The default plan is always a candidate.
    let mut done: Vec<(i64, Vec<Step>)> = vec![(ctx.value(state, &[], ctx.settings), Vec::new())];
    'search: while !frontier.is_empty() {
        let mut children: Vec<(i64, Node)> = Vec::new();
        let mut seen: HashMap<u64, usize> = HashMap::new();
        for node in &frontier {
            for (step, commands) in ctx.steps(&node.state) {
                if ctx.out_of_time() {
                    // Time is up: the best node of this depth so far is
                    // played on to the middle game.
                    if let Some((_, child)) = children.iter().max_by_key(|(v, _)| *v) {
                        done.push((
                            ctx.value(state, &child.plan, ctx.settings),
                            child.plan.clone(),
                        ));
                    }
                    break 'search;
                }
                let (next, over) = advance(&node.state, &commands, &ctx.model);
                let mut plan = node.plan.clone();
                plan.push(step);
                let value = score::score(&next, me, ctx.settings);
                if over {
                    done.push((value, plan));
                    continue;
                }
                let key = side_key(&next, me);
                match seen.get(&key) {
                    Some(&at) if children[at].0 >= value => {}
                    Some(&at) => children[at] = (value, Node { state: next, plan }),
                    None => {
                        seen.insert(key, children.len());
                        children.push((value, Node { state: next, plan }));
                    }
                }
            }
        }
        children.sort_by(|a, b| b.0.cmp(&a.0));
        children.truncate(width);
        frontier = children.into_iter().map(|(_, node)| node).collect();
    }
    ctx.finish(done)
}
