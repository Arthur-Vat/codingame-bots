//! The plan search (see the crate's docs): lethal first, then the opening
//! while the territories are far apart, then hill climbing over actions
//! against a memory of the opponent's replies.
//!
//! A climb starts from the state at the start of the turn and adds, one at a
//! time, the action that raises the score of the plan most. Trying every
//! action at every step would cost too much for 10 to 40 ms, so a step is
//! lazy: the actions are tried in the order of the gain each had when last
//! tried (new ones first), and the step stops at the first whose old gain is
//! no more than the best gain found in this step. When a lazy step finds no
//! gain, one full pass tries the actions it skipped, so a climb ends on a
//! plan that no single action improves. A later climb starts from the best
//! gains the actions had in the earlier ones, since its first steps resemble
//! theirs.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::time::Instant;

use ice_and_fire_engine::{Command, Outcome, State, HQ, SIZE};
use ice_and_fire_policy::greedy_turn;
use ice_and_fire_tactics::chain::chain_to;
use ice_and_fire_tactics::cut::cuts;
use ice_and_fire_tactics::eval::WIN;

use crate::actions::{generate, Action, CUT_LENGTH};
use crate::eval::eval;

/// Replies kept in the memory.
const MEMORY_CAP: usize = 8;
/// The opponent's best cuts added to the memory after a climb.
const REPLY_CUTS: usize = 3;
/// Most climbs in one turn.
const MAX_CLIMBS: usize = 8;

/// When the search stops.
#[derive(Clone, Copy, Debug)]
pub struct Limit {
    /// No new climb, reply search or plan scoring starts after this, and a
    /// climb stops at its next try.
    pub soft: Option<Instant>,
    /// The choice among the plans stops scoring more of them after this.
    pub hard: Option<Instant>,
    /// Most plans scored (one try of an action is one) during the climbs. The
    /// tests use it in place of a deadline, so that they are deterministic.
    pub max_evals: u64,
}

/// What the search decided.
pub struct Thought {
    /// The commands to send, each accepted in order on the state searched.
    pub commands: Vec<Command>,
    /// Climbs started.
    pub climbs: usize,
    /// Plans scored during the climbs (actions tried).
    pub evals: u64,
    /// Replies in the memory at the end.
    pub memory: usize,
}

/// One opponent reply: commands, each kept only if the rules accept it, and
/// how much it hurt the plan it was found for (the evaluation after it, from
/// our seat), lower being worse for us.
struct Reply {
    commands: Vec<Command>,
    damage: i64,
}

/// A plan: the commands and the state after them.
struct Plan {
    commands: Vec<Command>,
    end: State,
}

/// The gain an action had when last tried, by [`key`].
type Gains = HashMap<u64, i64, BuildHasherDefault<DefaultHasher>>;

struct Search {
    me: usize,
    limit: Limit,
    evals: u64,
    /// The best gain each action ever had in an earlier climb: where a new
    /// climb tries the actions first, since the first steps of a climb
    /// resemble those of the one before.
    hints: Gains,
}

/// Whether every command of `commands` is accepted in order by `state`.
fn apply_all(state: &mut State, commands: &[Command]) -> bool {
    commands.iter().all(|&command| state.apply(command))
}

/// A key to remember an action by.
fn key(commands: &[Command]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for command in commands {
        let (kind, a, b, c) = match *command {
            Command::Move { id, x, y } => (1, u64::from(id), x, y),
            Command::Train { level, x, y } => (2, u64::from(level), x, y),
            Command::BuildMine { x, y } => (3, 0, x, y),
            Command::BuildTower { x, y } => (4, 0, x, y),
            Command::Wait => (5, 0, 0, 0),
        };
        let word = kind | a << 8 | u64::from(b) << 40 | u64::from(c) << 48;
        hash = (hash ^ word).wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// The state after our plan `end`, the opponent's turn with `reply` and the
/// start of ours.
fn after_reply(end: &State, reply: &[Command]) -> State {
    let mut next = end.clone();
    next.end_turn();
    for &command in reply {
        next.apply(command);
    }
    next.end_turn();
    next
}

/// The plan, a lethal move or chain, that takes the enemy HQ now, if there is
/// one.
fn lethal(state: &State) -> Option<Vec<Command>> {
    let me = state.to_act();
    let target = HQ[1 - me];
    let wins = |trial: &State| trial.outcome() == Some(Outcome::Win(me));
    // A unit next to the HQ steps in.
    for cell in state.movable_units().iter() {
        if state.map().distance(cell, target) != 1 {
            continue;
        }
        let Some((id, _)) = state.unit_at(cell) else {
            continue;
        };
        let command = Command::Move {
            id,
            x: (target % SIZE) as u8,
            y: (target / SIZE) as u8,
        };
        let mut trial = state.clone();
        if trial.apply(command) && wins(&trial) {
            return Some(vec![command]);
        }
    }
    let chain = chain_to(state, target, state.gold(me))?;
    let mut trial = state.clone();
    (apply_all(&mut trial, &chain.commands) && wins(&trial)).then_some(chain.commands)
}

impl Search {
    /// Whether the climbs must stop.
    fn out_of_time(&self) -> bool {
        self.evals >= self.limit.max_evals || self.limit.soft.is_some_and(|t| Instant::now() >= t)
    }

    /// The score of the plan whose state is `end` against `memory`: the worst
    /// evaluation after a reply, or the evaluation of `end` itself with an
    /// empty memory. The replies are tried most damaging first and the
    /// scoring stops once the score is no more than `bound`, so a result
    /// above `bound` is exact and any other is an upper bound.
    fn score(&mut self, end: &State, memory: &[Reply], bound: i64) -> i64 {
        self.evals += 1;
        self.rate(end, memory, bound)
    }

    fn rate(&self, end: &State, memory: &[Reply], bound: i64) -> i64 {
        if end.outcome() == Some(Outcome::Win(self.me)) {
            return WIN;
        }
        if memory.is_empty() {
            return eval(end, self.me);
        }
        let mut worst = i64::MAX;
        for reply in memory {
            worst = worst.min(eval(&after_reply(end, &reply.commands), self.me));
            if worst <= bound {
                break;
            }
        }
        worst
    }

    /// One pass over `actions` in `order` from the plan state `cur`: the best
    /// action that scores above `cur_score`, with its state and score. With
    /// `lazy`, stops at the first action whose last gain is no more than the
    /// best gain so far, and says so in the second result. Actions with
    /// `done` set were tried in this step already.
    #[allow(clippy::too_many_arguments)]
    fn pass(
        &mut self,
        cur: &State,
        cur_score: i64,
        actions: &[Action],
        order: &[(i64, usize)],
        done: &mut [bool],
        memory: &[Reply],
        gains: &mut Gains,
        lazy: bool,
    ) -> (Option<(usize, State, i64)>, bool) {
        let mut best: Option<(usize, State, i64)> = None;
        let mut best_score = cur_score;
        for &(old, index) in order {
            if done[index] {
                continue;
            }
            if lazy && old != i64::MAX && old <= best_score - cur_score {
                return (best, true);
            }
            if self.out_of_time() {
                return (best, false);
            }
            done[index] = true;
            let commands = actions[index].commands();
            let mut next = cur.clone();
            if !apply_all(&mut next, commands) {
                gains.insert(key(commands), i64::MIN / 4);
                continue;
            }
            let score = self.score(&next, memory, best_score);
            gains.insert(key(commands), score - cur_score);
            if score > best_score {
                best_score = score;
                best = Some((index, next, score));
            }
        }
        (best, false)
    }

    /// One climb from `root` against `memory`.
    fn climb(&mut self, root: &State, memory: &[Reply]) -> Plan {
        let mut cur = root.clone();
        let mut commands = Vec::new();
        let mut cur_score = self.score(&cur, memory, i64::MIN);
        let mut gains = Gains::default();
        let mut actions = Vec::new();
        let mut order: Vec<(i64, usize)> = Vec::new();
        while cur.outcome().is_none() && !self.out_of_time() {
            generate(&cur, &mut actions);
            order.clear();
            for (index, action) in actions.iter().enumerate() {
                let k = key(action.commands());
                let old = gains.get(&k).or(self.hints.get(&k)).copied();
                order.push((old.unwrap_or(i64::MAX), index));
            }
            // Highest last gain first, new actions before all, then in the
            // order generated.
            order.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            let mut done = vec![false; actions.len()];
            let (mut best, skipped) = self.pass(
                &cur, cur_score, &actions, &order, &mut done, memory, &mut gains, true,
            );
            if best.is_none() && skipped {
                best = self
                    .pass(
                        &cur, cur_score, &actions, &order, &mut done, memory, &mut gains, false,
                    )
                    .0;
            }
            let Some((index, next, score)) = best else {
                break;
            };
            commands.extend_from_slice(actions[index].commands());
            cur = next;
            cur_score = score;
        }
        for (k, gain) in gains {
            let hint = self.hints.entry(k).or_insert(gain);
            *hint = (*hint).max(gain);
        }
        Plan { commands, end: cur }
    }

    /// The replies of the opponent to the plan that ends in `end`: its
    /// lethal chain, its best cuts and bronze's greedy turn, on the state
    /// after our turn ends.
    fn replies(&self, end: &State) -> Vec<Vec<Command>> {
        let mut next = end.clone();
        next.end_turn();
        let mut out = Vec::new();
        if next.outcome().is_some() {
            return out;
        }
        let opponent = next.to_act();
        let gold = next.gold(opponent);
        if let Some(chain) = chain_to(&next, HQ[self.me], gold) {
            out.push(chain.commands);
        }
        for cut in cuts(&next, CUT_LENGTH, gold, REPLY_CUTS) {
            out.push(cut.commands);
        }
        let mut greedy = Vec::new();
        greedy_turn(&next, &mut greedy);
        out.push(greedy);
        out.retain(|commands| !commands.is_empty());
        out
    }

    /// Adds the replies to the plan that ends in `end` that the memory does
    /// not hold, then keeps the [`MEMORY_CAP`] most damaging. Whether the
    /// memory changed.
    fn remember(&self, end: &State, memory: &mut Vec<Reply>) -> bool {
        let before: Vec<Vec<Command>> = memory.iter().map(|r| r.commands.clone()).collect();
        for commands in self.replies(end) {
            if memory.iter().any(|r| r.commands == commands) {
                continue;
            }
            let damage = eval(&after_reply(end, &commands), self.me);
            memory.push(Reply { commands, damage });
        }
        // Stable: the older reply stays first on a tie.
        memory.sort_by_key(|reply| reply.damage);
        memory.truncate(MEMORY_CAP);
        memory.len() != before.len() || memory.iter().zip(&before).any(|(r, b)| &r.commands != b)
    }
}

/// The commands to play in `state` for the player to act, within `limit`.
pub fn think(state: &State, limit: Limit) -> Thought {
    let me = state.to_act();
    let mut thought = Thought {
        commands: Vec::new(),
        climbs: 0,
        evals: 0,
        memory: 0,
    };
    if state.outcome().is_some() {
        return thought;
    }
    if let Some(commands) = lethal(state) {
        thought.commands = commands;
        return thought;
    }
    if let Some(commands) = crate::opening::turn(state) {
        thought.commands = commands;
        return thought;
    }
    let mut search = Search {
        me,
        limit,
        evals: 0,
        hints: Gains::default(),
    };
    let mut memory: Vec<Reply> = Vec::new();
    let mut plans: Vec<Plan> = Vec::new();
    while thought.climbs < MAX_CLIMBS && (plans.is_empty() || !search.out_of_time()) {
        let plan = search.climb(state, &memory);
        thought.climbs += 1;
        // A climb that ends on a plan found before has nothing new to learn.
        let same = plans.iter().any(|p| p.commands == plan.commands);
        if same || search.out_of_time() {
            if !same {
                plans.push(plan);
            }
            break;
        }
        let more = search.remember(&plan.end, &mut memory);
        plans.push(plan);
        // A climb against the same memory would find the same plan again.
        if !more {
            break;
        }
    }
    thought.evals = search.evals;
    thought.memory = memory.len();

    // The plan that scores best against the whole memory, the latest on a
    // tie. The latest is always scored; earlier ones while time remains.
    let mut best: Option<(i64, usize)> = None;
    for (index, plan) in plans.iter().enumerate().rev() {
        if best.is_some() && limit.hard.is_some_and(|t| Instant::now() >= t) {
            break;
        }
        let score = search.rate(&plan.end, &memory, i64::MIN);
        if best.is_none_or(|(top, _)| score > top) {
            best = Some((score, index));
        }
    }
    if let Some((_, index)) = best {
        // Each command is checked once more, in order, on a scratch copy.
        let mut scratch = state.clone();
        for &command in &plans[index].commands {
            if scratch.apply(command) {
                thought.commands.push(command);
            }
        }
    }
    thought
}
