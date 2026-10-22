//! Planners of the first turn (E004): each searches a plan of our turns up
//! to the middle game, the opponent simulated as v003's `econ`, and returns
//! the plan whose middle game scores best for us.
//!
//! A plan is a list of [`Step`]s, one a turn: the weights of the opening's
//! moves and how it buys. A step gives commands for any state, so a plan
//! stays playable when the opponent plays otherwise than the model. After its
//! last step a plan goes on with [`Context::rest`].
//!
//! - [`beam`] (P1): beam search over whole turns.
//! - [`schedule`] (P2): a dynamic program over the purchases of each turn on
//!   a model of the economy, its best schedules checked by rollouts.
//! - [`anneal`] (P4): simulated annealing over plans.
//! - [`cross_entropy`] (P5): the cross-entropy method over plans.
//!
//! Plans are ranked by the score of the middle game they reach
//! ([`Context::settings`]).

use std::collections::HashMap;
use std::time::Instant;

use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, State, HQ};

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

/// A key of our side of a state: our cells, units, the mines and our gold.
pub fn side_key(state: &State, me: usize) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |word: u64| hash = (hash ^ word).wrapping_mul(0x0100_0000_01b3);
    for (tag, bits) in [state.owned(me), state.units(me), state.mines()]
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

/// P4: simulated annealing over plans of the length the default plan takes
/// to reach the middle game. A move changes one step to another of
/// [`Context::steps`]'s kinds; a worse plan is kept with probability
/// exp(Δ / T), T falling from 8 to 0.5 gold over the time (or `max_iters`).
pub fn anneal(state: &State, ctx: &Context<'_>, seed: u64, max_iters: usize) -> Vec<Step> {
    let mut rng = Rng::new(seed);
    let length = plan_length(state, ctx);
    let kinds = step_kinds(ctx);
    let mut current = vec![ctx.rest; length];
    let mut current_value = ctx.value(state, &current, ctx.settings);
    let mut best = vec![(current_value, current.clone())];
    let start = Instant::now();
    let total = ctx.deadline.map(|t| t.saturating_duration_since(start));
    for iter in 0..max_iters {
        if ctx.out_of_time() {
            break;
        }
        let progress = match total {
            Some(total) if !total.is_zero() => start.elapsed().as_secs_f64() / total.as_secs_f64(),
            _ => iter as f64 / max_iters as f64,
        };
        let temperature = 8.0 * (0.5f64 / 8.0).powf(progress.min(1.0));
        let mut trial = current.clone();
        let at = rng.below(length as u64) as usize;
        trial[at] = kinds[rng.below(kinds.len() as u64) as usize];
        if trial == current {
            continue;
        }
        let value = ctx.value(state, &trial, ctx.settings);
        let delta = (value - current_value) as f64;
        if delta >= 0.0 || rng.unit() < (delta / temperature).exp() {
            current = trial;
            current_value = value;
            if value > best[0].0 {
                best.insert(0, (value, current.clone()));
            }
        }
    }
    ctx.finish(best)
}

/// P5: the cross-entropy method. Each step of the plan is drawn from its own
/// distribution over the kinds of steps; each round draws `samples` plans,
/// and the distributions move 70% of the way toward the frequencies of the
/// best tenth.
pub fn cross_entropy(
    state: &State,
    ctx: &Context<'_>,
    seed: u64,
    samples: usize,
    rounds: usize,
) -> Vec<Step> {
    let mut rng = Rng::new(seed);
    let length = plan_length(state, ctx);
    let kinds = step_kinds(ctx);
    let k = kinds.len();
    // Start near the default plan: half its weight on the default step.
    let rest = kinds.iter().position(|&s| s == ctx.rest).unwrap_or(0);
    let mut probs = vec![vec![0.5 / k as f64; k]; length];
    for row in &mut probs {
        row[rest] += 0.5;
    }
    let base = vec![ctx.rest; length];
    let mut all = vec![(ctx.value(state, &base, ctx.settings), base)];
    let elite = (samples / 10).max(2);
    'rounds: for _ in 0..rounds {
        let mut drawn: Vec<(i64, Vec<usize>)> = Vec::new();
        for _ in 0..samples {
            if ctx.out_of_time() {
                break 'rounds;
            }
            let picks: Vec<usize> = probs.iter().map(|row| draw(row, &mut rng)).collect();
            let plan: Vec<Step> = picks.iter().map(|&i| kinds[i]).collect();
            let value = ctx.value(state, &plan, ctx.settings);
            all.push((value, plan));
            drawn.push((value, picks));
        }
        drawn.sort_by(|a, b| b.0.cmp(&a.0));
        for (t, row) in probs.iter_mut().enumerate() {
            let mut freq = vec![0.0; k];
            for (_, picks) in drawn.iter().take(elite) {
                freq[picks[t]] += 1.0 / elite as f64;
            }
            for (p, f) in row.iter_mut().zip(freq) {
                *p = 0.3 * *p + 0.7 * f;
            }
        }
    }
    ctx.finish(all)
}

/// An index drawn from the distribution `row`.
fn draw(row: &[f64], rng: &mut Rng) -> usize {
    let mut x = rng.unit() * row.iter().sum::<f64>();
    for (i, &p) in row.iter().enumerate() {
        if x < p {
            return i;
        }
        x -= p;
    }
    row.len() - 1
}

/// The kinds of steps the stochastic planners draw from.
fn step_kinds(ctx: &Context<'_>) -> Vec<Step> {
    let mut kinds: Vec<Step> = BUYS.iter().map(|&buy| Step { weights: 0, buy }).collect();
    for weights in 1..ctx.sets.len().min(usize::from(u8::MAX)) {
        kinds.push(Step {
            weights: weights as u8,
            buy: Buy::Payback,
        });
    }
    if !kinds.contains(&ctx.rest) {
        kinds.push(ctx.rest);
    }
    kinds
}

/// Our turns until the middle game with the default plan, plus 2, at most
/// the cap.
fn plan_length(state: &State, ctx: &Context<'_>) -> usize {
    let end = ctx.play(state, &[]);
    let turns = end.game_turn().saturating_sub(state.game_turn()) as usize + 2;
    turns.clamp(1, CAP as usize)
}

/// The economy of one side in [`schedule`]'s model, at the start of a turn.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Economy {
    gold: i64,
    units: i64,
    mines: i64,
    cells: i64,
}

/// P2: a dynamic program over the purchases of each turn. The model: each
/// unit takes a free cell a turn and a training takes one, up to the cells of
/// our side of the map (nearer our territory than the enemy's); a mine can
/// be built once the territory holds the cells nearer the HQ than its spot;
/// the income is cells + 4 mines − units. At the turn of the default plan's
/// middle game it values cells at a training each, and gold + the score's
/// turns × income. The best `checked`
/// schedules are played as plans of exact purchases and the best kept.
pub fn schedule(state: &State, ctx: &Context<'_>, checked: usize) -> Vec<Step> {
    let me = state.to_act();
    let end = ctx.play(state, &[]);
    let turns = (end.game_turn().saturating_sub(state.game_turn()) as usize).clamp(1, CAP as usize);
    let economy_turns = ctx.settings.turns;
    let from_hq = econ::distances(state, Bits::cell(HQ[me]));
    let theirs = econ::distances(state, state.owned(1 - me));
    let mine = econ::distances(state, state.owned(me));
    // Our side: our cells and the free cells at least as near us.
    let mut side: Vec<u8> = state
        .map()
        .valid
        .iter()
        .filter(|&c| {
            state.owned(me).test(c) || (!state.owned(1 - me).test(c) && mine[c] <= theirs[c])
        })
        .map(|c| from_hq[c])
        .collect();
    side.sort_unstable();
    let room = side.len() as i64;
    // For each safe spot on our side, the cells the territory needs first.
    let safe = ctx.params(ctx.rest).safe_spot;
    let mut unlock: Vec<i64> = state
        .map()
        .spots
        .iter()
        .filter(|&c| mine[c] <= theirs[c] && theirs[c] > safe && !state.mines().test(c))
        .map(|c| side.iter().filter(|&&d| d < from_hq[c]).count() as i64 + 1)
        .collect();
    unlock.sort_unstable();
    let start = Economy {
        gold: state.gold(me),
        units: state.unit_count(me) as i64,
        mines: i64::from((state.mines() & state.owned(me)).count()),
        cells: i64::from(state.active(me).count()),
    };
    let built = start.mines;
    // Each layer: the economies reached, with a schedule reaching each.
    let mut layer: HashMap<Economy, Vec<(u8, u8)>> = HashMap::new();
    layer.insert(start, Vec::new());
    for _ in 0..turns {
        if ctx.out_of_time() {
            break;
        }
        let mut next: HashMap<Economy, Vec<(u8, u8)>> = HashMap::new();
        for (e, plan) in &layer {
            for mines in 0..=2u8 {
                for units in 0..=4u8 {
                    let cost = 15 * i64::from(mines) + 10 * i64::from(units);
                    let open = unlock.iter().filter(|&&need| need <= e.cells).count() as i64
                        - (e.mines - built);
                    if cost > e.gold || i64::from(mines) > open.max(0) {
                        continue;
                    }
                    let grown = (e.cells + e.units + i64::from(units)).min(room.max(e.cells));
                    let units_after = e.units + i64::from(units);
                    let mines_after = e.mines + i64::from(mines);
                    let income = grown + 4 * mines_after - units_after;
                    let economy = Economy {
                        gold: e.gold - cost + income,
                        units: units_after,
                        mines: mines_after,
                        cells: grown,
                    };
                    next.entry(economy).or_insert_with(|| {
                        let mut p = plan.clone();
                        p.push((mines, units));
                        p
                    });
                }
            }
        }
        // Keep the most promising economies.
        let mut ranked: Vec<(Economy, Vec<(u8, u8)>)> = next.into_iter().collect();
        ranked.sort_by_key(|(e, _)| {
            std::cmp::Reverse(
                e.gold + score::CELL * e.cells + economy_turns * (e.cells + 4 * e.mines - e.units),
            )
        });
        ranked.truncate(20_000);
        layer = ranked.into_iter().collect();
    }
    let mut ranked: Vec<(i64, Vec<(u8, u8)>)> = layer
        .into_iter()
        .map(|(e, plan)| {
            (
                e.gold + score::CELL * e.cells + economy_turns * (e.cells + 4 * e.mines - e.units),
                plan,
            )
        })
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let mut plans: Vec<(i64, Vec<Step>)> = vec![(ctx.value(state, &[], ctx.settings), Vec::new())];
    for (_, purchases) in ranked.into_iter().take(checked) {
        if ctx.out_of_time() {
            break;
        }
        let plan: Vec<Step> = purchases
            .iter()
            .map(|&(mines, units)| Step {
                weights: ctx.rest.weights,
                buy: Buy::Exact { mines, units },
            })
            .collect();
        plans.push((ctx.value(state, &plan, ctx.settings), plan));
    }
    ctx.finish(plans)
}
