//! The plan search (see the crate's docs): lethal first, then the opening
//! until the middle game, then hill climbing over actions against a memory
//! of the opponent's replies.
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
//!
//! When a climb has nothing new to learn (it found a plan found before, or
//! the opponent's replies to its plan are in the memory already), the next
//! climb may not start with the first action of any plan found so far
//! (E014), so the climbs keep finding other plans until the time is up.
//!
//! When no first action left improves the plan, so that a climb ends on an
//! empty plan, the time left goes to climbs forced to start with each of the
//! root's other actions in turn, the best gain first (E029): an action that
//! does not pay alone can open a plan that does.
//!
//! Each plan keeps the opponent's replies found for it (E027): the shared
//! memory holds only the most damaging replies over all plans, so a plan's
//! own worst reply could leave it. A plan's final score is the worst over the
//! memory and its own replies, and the plan played must have had its replies
//! searched: a plan found as the time ran out gets them in the time left
//! before the hard limit, and if it then scores lower, the next best is
//! checked in turn.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::time::{Duration, Instant};

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, Outcome, State, HQ, SIZE};
use ice_and_fire_opening::planned::Planned;
use ice_and_fire_policy::greedy_turn;
use ice_and_fire_tactics::chain::{chain_to, cost_to, cost_to_moving, UNREACHABLE};
use ice_and_fire_tactics::cut::cuts;
use ice_and_fire_tactics::eval::WIN;

use crate::actions::{generate, xy, Action, CUT_LENGTH};
use crate::eval::eval;
use crate::strategy::Guide;

#[cfg(feature = "analysis")]
pub mod judge;
mod steer;

use steer::Steer;

/// How many times the caps below are multiplied: 1 at CodinGame's limits.
/// A teacher (E117) thinks `THINK_SCALE` times longer and scales its caps
/// with it, since at CodinGame's caps most of its turns end at a cap.
const CAP_SCALE: usize = 1;
/// Cells added to the opponent's cuts in its replies, beyond our own
/// `CUT_LENGTH` (E117's teacher).
const REPLY_CUT_EXTRA: usize = 0;
/// A deeper teacher (E118, `research/codingame/2026-11-28-deep-teacher.md`):
/// `Some(n)` makes the middle game's turn compare its `n` best plans, and the
/// plan today's search would play, by what our next turn can do after each of
/// the opponent's replies to them (a whole turn search of ours from the state
/// after the reply, its score taken, and the worst reply counting), and play
/// the best. `None`: today's search alone, as in every release. Offline only:
/// a deep turn takes [`DEEP_MS`], far beyond CodinGame's limit.
pub const DEEP: Option<usize> = None;
/// A deep turn's budget in milliseconds at CodinGame's time scale, multiplied
/// by `CG_TIME_SCALE` as the other budgets are (E118). Today's search keeps
/// its own budget at the start of it; the deep step runs until this.
pub const DEEP_MS: f64 = 2000.0;
/// What a lean deep leaf is (E119).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Leaf {
    /// A whole turn search of ours from the state after the reply, with no
    /// deep step, its final score taken (E118's leaf).
    Think,
    /// One climb of ours from that state, with no reply search, its end
    /// scored by `value`: a cheap estimate of our next turn.
    Climb,
    /// E120: a whole turn search where the time left per leaf is at least
    /// [`MIN_THINK_US`] (scaled by the time scale), else, for that candidate,
    /// one climb per leaf as `Climb`.
    Fit,
}

/// A lean deep step (E119, E118 made small enough for CodinGame's limit).
#[derive(Clone, Copy)]
pub struct Lean {
    /// The part of the turn, in percent, that today's search gets: its
    /// climbs, its final choice and its check end there.
    pub share: u32,
    /// The candidates compared: today's choice and the best others by
    /// today's score.
    pub candidates: usize,
    /// The most damaging replies of each candidate searched: its own and the
    /// memory's, without repeats, the worst for the candidate first.
    pub replies: usize,
    /// What a leaf is.
    pub leaf: Leaf,
    /// E120: today's search keeps its whole limits (`share` is not used) and
    /// the deep step runs only in the time it leaves before the hard limit
    /// (with `CG_FIXED_ITERS`, in [`FREE_TRIES_PCT`]% more tries), only when
    /// it ended before its time limit, its tries or the climb cap. A
    /// `Think` leaf that would get less than [`MIN_THINK_US`] is skipped,
    /// and its candidate not compared.
    pub free: bool,
}

/// E120: the smallest time, in microseconds at CodinGame's time scale
/// (multiplied by `CG_TIME_SCALE`), a `Think` leaf of a free-time deep step
/// gets; with less, the leaf is skipped.
const MIN_THINK_US: f64 = 1000.0;
/// E120: with `CG_FIXED_ITERS`, the tries of a free-time deep step, in
/// percent of the tries of today's search.
const FREE_TRIES_PCT: u128 = 50;

/// The lean deep step (E119): after today's search, which has `share`% of the
/// turn, the candidates' leaves are searched one candidate after the other,
/// today's choice first, each leaf with the time left divided by the leaves
/// left, up to the turn's hard limit. A candidate with a leaf undone is not
/// compared; with no other complete candidate, today's choice is played.
/// `None`: today's search alone, as in every release.
pub const LEAN_DEPTH: Option<Lean> = Some(Lean {
    share: 100,
    candidates: 3,
    replies: 3,
    leaf: Leaf::Think,
    free: true,
});
/// Replies kept in the memory.
const MEMORY_CAP: usize = 8 * CAP_SCALE;
/// The opponent's best cuts added to the memory after a climb.
const REPLY_CUTS: usize = 3 * CAP_SCALE;
/// Most climbs in one turn.
const MAX_CLIMBS: usize = 32 * CAP_SCALE;
/// Most plans scored by the opponent's climb in one reply search (E019).
const REPLY_EVALS: u64 = 300 * CAP_SCALE as u64;
/// Where between the soft and the hard limit the middle game's climbs stop
/// (E027): at 36 ms of a 40 ms turn rather than 24; the rest is for the
/// final choice and its check.
const CLIMB_SHARE: f64 = 0.75;

/// The game turn from which the search plays whatever the opening would do
/// (E067): `None` for never (v026, where the opening plays until we can
/// pay a chain of trainings to an enemy cell).
pub const HANDOVER_TURN: Option<u32> = Some(7);
/// Whether the search also takes over, for good, at the first turn the
/// opening would buy nothing while we hold [`HANDOVER_GOLD`] or more, from
/// game turn [`HANDOVER_IDLE_FROM`] (E067).
pub const HANDOVER_IDLE: bool = false;
/// The gold of a level-1 training.
pub const HANDOVER_GOLD: i64 = 10;
pub const HANDOVER_IDLE_FROM: u32 = 6;

/// The number of the likeliest kinds of action (`types::Kind`, by the
/// table of zasmu's games for the unit's key) whose moves our own climbs
/// offer a unit (E106, stage 2 of
/// `research/codingame/2026-11-28-type-planner.md`); a move of another kind
/// is not offered. `None`: none removed. Trainings, buildings and macro
/// actions are never removed, and the opponent's model is never pruned.
pub const TYPE_KINDS: Option<usize> = None;
/// Whether our climbs break ties of the last gain by the table's prior (E106):
/// moves by their kind's prior, highest first, then the other actions in the
/// order generated. `false`: the order generated.
pub const TYPE_ORDER: bool = false;
/// The game turn from which [`TYPE_KINDS`] and [`TYPE_ORDER`] apply, by the
/// game turn of a climb's root (E106).
pub const TYPE_FROM_TURN: u32 = 7;

/// When the search stops.
#[derive(Clone, Copy)]
pub struct Limit {
    /// No new climb, reply search or plan scoring starts after this, and a
    /// climb stops at its next try.
    pub soft: Option<Instant>,
    /// The choice among the plans stops scoring more of them after this.
    pub hard: Option<Instant>,
    /// Where a deep turn ends (E118): the deep step, which follows today's
    /// search, stops at this. Only used with [`DEEP`].
    pub deep: Option<Instant>,
    /// Most plans scored (one try of an action is one) during the climbs. The
    /// tests use it in place of a deadline, so that they are deterministic.
    pub max_evals: u64,
}

/// What the search decided.
#[derive(Default)]
pub struct Thought {
    /// The commands to send, each accepted in order on the state searched.
    pub commands: Vec<Command>,
    /// Climbs started.
    pub climbs: usize,
    /// Plans scored during the climbs (actions tried).
    pub evals: u64,
    /// Replies in the memory at the end.
    pub memory: usize,
    /// Different plans found by the climbs.
    pub plans: usize,
    /// Plans whose replies were searched only for the final choice (E027).
    pub checks: usize,
    /// Whether the climbs ran out of first actions and were then forced to
    /// start with each of the others (E029).
    pub forced: usize,
    /// The final score of the plan played, when the search chose among plans
    /// (E118: a leaf's value); `WIN` for a lethal turn.
    pub score: Option<i64>,
    /// Whether the opening played the turn.
    pub opening: bool,
    /// Whether the plan played starts with the strategic turn (E077).
    pub strategic: bool,
    /// The strategic turn's pieces the plan played holds (E077).
    pub pieces: usize,
}

/// Whether the plan played must have had its own replies searched (E027).
const VERIFY: bool = true;

/// One opponent reply: commands, each kept only if the rules accept it, and
/// how much it hurt the plan it was found for (the evaluation after it, from
/// our seat), lower being worse for us.
#[derive(Clone)]
struct Reply {
    commands: Vec<Command>,
    damage: i64,
}

/// A plan: the commands and the state after them.
struct Plan {
    commands: Vec<Command>,
    end: State,
    /// The key of its first action, if any.
    first: Option<u64>,
    /// The opponent's replies to this plan, once searched (E027).
    own: Option<Vec<Reply>>,
}

/// The gain an action had when last tried, by [`key`].
type Gains = HashMap<u64, i64, BuildHasherDefault<DefaultHasher>>;

struct Search {
    me: usize,
    /// Whether this is the opponent's model, which the type switches never
    /// touch (E106).
    opponent: bool,
    limit: Limit,
    evals: u64,
    /// The best gain each action ever had in an earlier climb: where a new
    /// climb tries the actions first, since the first steps of a climb
    /// resemble those of the one before.
    hints: Gains,
    /// The actions of the climbs' root, generated once: every climb of a
    /// search starts from the same state.
    root_actions: Option<Vec<Action>>,
    /// Whether the strike reserve holds this turn (E069).
    strike: bool,
    /// The strategic layer's guide this turn (E077): its plan and the
    /// objectives' term.
    guide: Option<Guide>,
    /// The deep step's candidates and leaves (E118).
    deep_stats: (usize, usize),
}

/// The strike reserve (E069, stage B of the curriculum in
/// `research/codingame/2026-11-18-zasmu-strategy.md`): when, at the start of
/// the turn, no strike of the opponent's threatens our HQ and our chain of
/// trainings to the enemy HQ (all our units stepping first) is affordable
/// with our gold plus this many turns of income, the search plays no plan
/// that leaves it unaffordable within as many turns. 0 for none (v028,
/// v029); 1 was the probe of E069.
pub const STRIKE_TURNS: i64 = 0;

/// The blockers offered (E044): towers on our cells that the opponent's
/// remembered replies enter. Towers alone won the screening.
pub const BLOCK_TOWERS: bool = true;
/// Towers also on our free cells next to those cells (a tower guards its
/// neighbours).
pub const BLOCK_NEIGHBOURS: bool = false;
/// A training at this level on those cells when they are empty (`None`:
/// none).
pub const BLOCK_LEVEL: Option<u8> = None;

/// Adds to `out` the blockers of the replies of `memory` (E044), as
/// [`BLOCK_TOWERS`], [`BLOCK_NEIGHBOURS`] and [`BLOCK_LEVEL`] say.
fn blockers(state: &State, memory: &[Reply], out: &mut Vec<Action>) {
    if !BLOCK_TOWERS && BLOCK_LEVEL.is_none() {
        return;
    }
    let replies = memory.iter().map(|reply| reply.commands.as_slice());
    blockers_of(
        state,
        replies,
        (BLOCK_TOWERS, BLOCK_NEIGHBOURS, BLOCK_LEVEL),
        out,
    );
}

/// Adds to `out` the blockers of the cells of the player to act in `state`
/// that `replies` enter by a training or a move (E044): with `towers`,
/// towers on them (and with `neighbours`, on our free cells next to them);
/// with a `level`, a training at that level on those that are empty; each
/// unless `out` holds the same action already.
pub(crate) fn blockers_of<'a>(
    state: &State,
    replies: impl Iterator<Item = &'a [Command]>,
    (towers, neighbours, level): (bool, bool, Option<u8>),
    out: &mut Vec<Action>,
) {
    let me = state.to_act();
    let mine = state.active(me);
    let mut entered = Bits::EMPTY;
    for reply in replies {
        for command in reply {
            if let Command::Train { x, y, .. } | Command::Move { x, y, .. } = *command {
                let cell = usize::from(y) * SIZE + usize::from(x);
                if mine.test(cell) {
                    entered |= Bits::cell(cell);
                }
            }
        }
    }
    if entered.is_empty() {
        return;
    }
    let free = state.empty_cells() & mine;
    let gold = state.gold(me);
    let push = |command: Command, out: &mut Vec<Action>| {
        if !out.iter().any(|a| a.commands() == [command]) {
            out.push(Action::One(command));
        }
    };
    if towers && gold >= BUILD_COST && state.league().allows_towers() {
        let mut cells = entered;
        if neighbours {
            cells |= entered.neighbours();
        }
        for cell in (cells & free).and_not(state.map().spots).iter() {
            let (x, y) = xy(cell);
            push(Command::BuildTower { x, y }, out);
        }
    }
    if let Some(level) = level.filter(|&level| level <= state.league().max_level()) {
        if gold >= 10 * i64::from(level) {
            for cell in (entered & free).iter() {
                let (x, y) = xy(cell);
                push(Command::Train { level, x, y }, out);
            }
        }
    }
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
        let (x, y) = xy(target);
        let command = Command::Move { id, x, y };
        let mut trial = state.clone();
        if trial.apply(command) && wins(&trial) {
            return Some(vec![command]);
        }
    }
    let chain = chain_to(state, target, state.gold(me))?;
    let mut trial = state.clone();
    (apply_all(&mut trial, &chain.commands) && wins(&trial)).then_some(chain.commands)
}

/// Whether `seat`'s chain of trainings to the enemy HQ, all its units
/// stepping first, is affordable in `state` with its gold plus `turns`
/// turns of income (E069).
pub(crate) fn strike_within(state: &State, seat: usize, turns: i64) -> bool {
    let cost = cost_to_moving(state, seat, state.units(seat), HQ[1 - seat]);
    cost != UNREACHABLE && state.gold(seat) + turns * state.income(seat) >= cost
}

/// Whether the strike reserve holds in `state`, the start of a turn of the
/// player to act (E069): [`STRIKE_TURNS`] is not 0, the opponent cannot take
/// our HQ at its next turn whatever we do, and our strike is affordable
/// within [`STRIKE_TURNS`] turns.
pub(crate) fn strike_reserve(state: &State) -> bool {
    let me = state.to_act();
    if STRIKE_TURNS == 0 || state.outcome().is_some() {
        return false;
    }
    let mut next = state.clone();
    next.end_turn();
    let threatened =
        next.outcome().is_none() && cost_to(&next, 1 - me, true, HQ[me]) <= next.gold(1 - me);
    !threatened && strike_within(state, me, STRIKE_TURNS)
}

impl Search {
    /// Whether the plan whose state is `end` breaks the strike reserve
    /// (E069): the reserve holds this turn, the plan does not win, and it
    /// leaves our strike unaffordable within [`STRIKE_TURNS`] turns.
    fn breaks_reserve(&self, end: &State) -> bool {
        self.strike
            && end.outcome() != Some(Outcome::Win(self.me))
            && !strike_within(end, self.me, STRIKE_TURNS)
    }

    /// Whether the climbs must stop.
    fn spent(&self) -> bool {
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
            return self.value(end);
        }
        let mut worst = i64::MAX;
        for reply in memory {
            worst = worst.min(self.value(&after_reply(end, &reply.commands)));
            if worst <= bound {
                break;
            }
        }
        worst
    }

    /// The evaluation of `state` for us, with the objectives' term of the
    /// guide (E077).
    fn value(&self, state: &State) -> i64 {
        let base = eval(state, self.me);
        match &self.guide {
            Some(guide) if state.outcome().is_none() => base + guide.term(state, self.me),
            _ => base,
        }
    }

    /// The typing of a step of a climb from `root`, when [`TYPE_KINDS`] or
    /// [`TYPE_ORDER`] apply to it (E106): ours only, from [`TYPE_FROM_TURN`].
    fn steer(&self, root: &State, cur: &State) -> Option<Steer> {
        let on = (TYPE_KINDS.is_some() || TYPE_ORDER)
            && !self.opponent
            && root.game_turn() >= TYPE_FROM_TURN;
        on.then(|| Steer::new(cur))
    }

    /// Adds the strategic turn's pieces to `actions` (E077).
    fn add_pieces(&self, actions: &mut Vec<Action>) {
        if let Some(guide) = &self.guide {
            for piece in &guide.pieces {
                actions.push(Action::Many(piece.clone()));
            }
        }
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
        let mut best = None;
        let mut best_score = cur_score;
        for &(old, index) in order {
            if done[index] {
                continue;
            }
            if lazy && old != i64::MAX && old <= best_score - cur_score {
                return (best, true);
            }
            if self.spent() {
                return (best, false);
            }
            done[index] = true;
            let commands = actions[index].commands();
            let mut next = cur.clone();
            if !apply_all(&mut next, commands) || self.breaks_reserve(&next) {
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

    /// One climb from `root` against `memory`, never starting with an
    /// action whose key is in `banned`. With `cached`, `root` is the
    /// search's root, whose actions are generated once.
    fn climb(&mut self, root: &State, memory: &[Reply], banned: &[u64], cached: bool) -> Plan {
        let mut first = None;
        let mut cur = root.clone();
        let mut commands = Vec::new();
        let mut cur_score = self.score(&cur, memory, i64::MIN);
        let mut gains = Gains::default();
        let mut actions = Vec::new();
        let mut order = Vec::new();
        while cur.outcome().is_none() && !self.spent() {
            if commands.is_empty() && cached {
                match &self.root_actions {
                    Some(root_actions) => actions.clone_from(root_actions),
                    None => {
                        generate(&cur, &mut actions);
                        // The strategic turn as one macro action (E077).
                        if let Some(guide) = self.guide.as_ref().filter(|g| g.offered) {
                            actions.push(Action::Many(guide.plan.clone()));
                        }
                        self.add_pieces(&mut actions);
                        self.root_actions = Some(actions.clone());
                    }
                }
            } else {
                generate(&cur, &mut actions);
                self.add_pieces(&mut actions);
            }
            blockers(&cur, memory, &mut actions);
            let steer = self.steer(root, &cur);
            if let (Some(steer), Some(k)) = (&steer, TYPE_KINDS) {
                steer.filter(&cur, k, &mut actions);
            }
            order.clear();
            for (index, action) in actions.iter().enumerate() {
                let k = key(action.commands());
                let old = gains.get(&k).or(self.hints.get(&k)).copied();
                order.push((old.unwrap_or(i64::MAX), index));
            }
            // Highest last gain first, new actions before all, then in the
            // order generated.
            // With [`TYPE_ORDER`], ties go by the prior (E106).
            match steer.as_ref().filter(|_| TYPE_ORDER) {
                Some(steer) => {
                    let priors: Vec<u8> = actions.iter().map(|a| steer.prior(&cur, a)).collect();
                    order.sort_by(|a, b| {
                        (b.0.cmp(&a.0))
                            .then(priors[b.1].cmp(&priors[a.1]))
                            .then(a.1.cmp(&b.1))
                    });
                }
                None => order.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1))),
            }
            let mut done = vec![false; actions.len()];
            if commands.is_empty() {
                for (index, action) in actions.iter().enumerate() {
                    done[index] = banned.contains(&key(action.commands()));
                }
            }
            // A lazy pass, then a full one if the lazy one skipped actions
            // and found no gain.
            let mut best = None;
            for lazy in [true, false] {
                let (found, skipped) = self.pass(
                    &cur, cur_score, &actions, &order, &mut done, memory, &mut gains, lazy,
                );
                best = found;
                if best.is_some() || !skipped {
                    break;
                }
            }
            let Some((index, next, score)) = best else {
                break;
            };
            if commands.is_empty() {
                first = Some(key(actions[index].commands()));
            }
            commands.extend_from_slice(actions[index].commands());
            cur = next;
            cur_score = score;
        }
        for (k, gain) in gains {
            let hint = self.hints.entry(k).or_insert(gain);
            *hint = (*hint).max(gain);
        }
        Plan {
            commands,
            end: cur,
            first,
            own: None,
        }
    }

    /// The replies of the opponent to the plan that ends in `end`: its
    /// lethal chain, its best cuts, bronze's greedy turn and its own plan
    /// (one climb), on the state after our turn ends.
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
        for cut in cuts(&next, CUT_LENGTH + REPLY_CUT_EXTRA, gold, REPLY_CUTS) {
            out.push(cut.commands);
        }
        let mut greedy = Vec::new();
        greedy_turn(&next, &mut greedy);
        out.push(greedy);
        // The opponent's own plan (E019): a climb from its seat, with moves,
        // trainings and cuts, scored by the evaluation from its side.
        let mut theirs = Search {
            me: opponent,
            opponent: true,
            limit: Limit {
                max_evals: REPLY_EVALS,
                ..self.limit
            },
            evals: 0,
            hints: Gains::default(),
            root_actions: None,
            // The opponent's model keeps no reserve, as before E069.
            strike: false,
            guide: None,
            deep_stats: (0, 0),
        };
        out.push(theirs.climb(&next, &[], &[], true).commands);
        out.retain(|commands| !commands.is_empty());
        out
    }

    /// The replies to the plan that ends in `end`, each with its damage.
    fn answers(&self, end: &State) -> Vec<Reply> {
        self.replies(end)
            .into_iter()
            .map(|commands| {
                let damage = self.value(&after_reply(end, &commands));
                Reply { commands, damage }
            })
            .collect()
    }

    /// A climb from `root` forced to start with `action` (E029), or `None`
    /// when the rules refuse it.
    fn forced(&mut self, root: &State, action: &Action, memory: &[Reply]) -> Option<Plan> {
        let mut after = root.clone();
        if !apply_all(&mut after, action.commands()) || self.breaks_reserve(&after) {
            return None;
        }
        self.evals += 1;
        let rest = self.climb(&after, memory, &[], false);
        let mut commands = action.commands().to_vec();
        commands.extend_from_slice(&rest.commands);
        Some(Plan {
            commands,
            end: rest.end,
            first: Some(key(action.commands())),
            own: None,
        })
    }

    /// The score of `plan` for the final choice: the worst over `memory` and
    /// the plan's own replies (E027).
    fn final_score(&self, plan: &Plan, memory: &[Reply]) -> i64 {
        let score = self.rate(&plan.end, memory, i64::MIN);
        match &plan.own {
            Some(own) if !own.is_empty() => score.min(self.rate(&plan.end, own, i64::MIN)),
            _ => score,
        }
    }
}

impl Search {
    /// The deep step (E118): the index of the plan to play among `plans`
    /// (scored `scores`, today's choice `today`), compared by what our next
    /// turn can do. The candidates are the `width` best plans and today's; a
    /// candidate's score is the worst over its replies (the memory's and its
    /// own) of the final score of the plan a whole turn search of ours plays
    /// from the state after the reply. The searches share the time to
    /// `limit.deep` evenly, or the tries when there is no clock. On a tie
    /// today's choice wins, then the earlier candidate.
    fn deepen(
        &mut self,
        plans: &[Plan],
        scores: &[Option<i64>],
        memory: &[Reply],
        (today, width): (Option<usize>, usize),
        limit: Limit,
        opening: &mut Planned,
    ) -> Option<usize> {
        let today = today?;
        let score_of = |index: usize| scores[index].unwrap_or(i64::MIN);
        // The best first, the earlier on a tie.
        let mut order: Vec<usize> = (0..plans.len()).collect();
        order.sort_by_key(|&index| std::cmp::Reverse(score_of(index)));
        let mut candidates: Vec<usize> = order.into_iter().take(width).collect();
        if !candidates.contains(&today) {
            candidates.push(today);
        }
        // Each candidate's leaves: the states after its replies.
        let mut leaves: Vec<Vec<State>> = Vec::new();
        for &index in &candidates {
            let plan = &plans[index];
            let mut states = Vec::new();
            if plan.end.outcome().is_none() {
                let mut seen: Vec<&[Command]> = Vec::new();
                let own = plan.own.iter().flatten();
                for reply in memory.iter().chain(own) {
                    if !seen.contains(&reply.commands.as_slice()) {
                        seen.push(&reply.commands);
                        states.push(after_reply(&plan.end, &reply.commands));
                    }
                }
            }
            leaves.push(states);
        }
        let total: usize = leaves.iter().map(Vec::len).sum();
        self.deep_stats = (candidates.len(), total);
        let mut left = total;
        let mut best: Option<(i64, bool, usize)> = None;
        for (rank, &index) in candidates.iter().enumerate() {
            // A plan with no replies to search keeps its final score.
            let mut deep = if leaves[rank].is_empty() {
                self.final_score(&plans[index], memory)
            } else {
                i64::MAX
            };
            for leaf in &leaves[rank] {
                let value = self.leaf_value(leaf, limit, left, opening);
                left -= 1;
                deep = deep.min(value);
            }
            let key = (deep, index == today, std::cmp::Reverse(rank));
            let better = best.is_none_or(|(d, t, r)| key > (d, t, std::cmp::Reverse(r)));
            if better {
                best = Some((deep, index == today, rank));
            }
        }
        best.map(|(_, _, rank)| candidates[rank])
    }

    /// The value of the leaf `state`, the start of our next turn: the final
    /// score of the plan a turn search of ours plays from it, within an even
    /// share of the deep time (or tries) left for the `left` leaves not yet
    /// searched.
    fn leaf_value(&self, state: &State, limit: Limit, left: usize, opening: &mut Planned) -> i64 {
        if state.outcome().is_some() {
            return self.value(state);
        }
        let share = left.max(1);
        let leaf_limit = match limit.deep {
            Some(deep) => {
                let now = Instant::now();
                let room = deep.saturating_duration_since(now).div_f64(share as f64);
                Limit {
                    soft: Some(now + room.mul_f64(0.6)),
                    hard: Some(now + room),
                    deep: None,
                    max_evals: u64::MAX,
                }
            }
            None => Limit {
                soft: None,
                hard: None,
                deep: None,
                max_evals: (limit.max_evals / share as u64).max(1),
            },
        };
        let thought = think_in(state, leaf_limit, opening, None, (None, None));
        thought.score.unwrap_or_else(|| self.value(state))
    }
}

/// `limit` with its clock and its tries cut to `share`% of what is left.
fn lean_limit(limit: Limit, share: u32) -> Limit {
    let now = Instant::now();
    let part = f64::from(share) / 100.0;
    let cut = |t: Instant| now + t.saturating_duration_since(now).mul_f64(part);
    Limit {
        soft: limit.soft.map(cut),
        hard: limit.hard.map(cut),
        deep: None,
        max_evals: (u128::from(limit.max_evals) * u128::from(share) / 100).max(1) as u64,
    }
}

impl Search {
    /// The lean deep step (E119): the index of the plan to play among
    /// `plans`, or `None` to keep today's choice. The candidates are today's
    /// choice, then the best scored others, `lean.candidates` in all. A
    /// candidate's leaves are the states after its `lean.replies` most
    /// damaging replies (the memory's and its own, by the evaluation after
    /// each from our seat); its deep score is the worst leaf, and only a
    /// candidate with all its leaves done is compared. The leaves share the
    /// time to `full.hard` evenly, each taking the time left divided by the
    /// leaves left; with no clock, the tries `full` keeps beyond today's
    /// share, in the same proportions. On a tie today's choice wins, then the
    /// earlier candidate.
    fn lean_step(
        &self,
        plans: &[Plan],
        scores: &[Option<i64>],
        memory: &[Reply],
        (today, lean): (Option<usize>, Lean),
        (full, opening): (Limit, &mut Planned),
    ) -> Option<usize> {
        let today = today?;
        // The leaf's kind, read from the switch.
        let kind = if lean.leaf == Leaf::Climb {
            Leaf::Climb
        } else {
            Leaf::Think
        };
        let mut order: Vec<usize> = (0..plans.len())
            .filter(|&index| index != today && scores[index].is_some())
            .collect();
        order.sort_by_key(|&index| std::cmp::Reverse(scores[index]));
        let candidates: Vec<usize> = std::iter::once(today)
            .chain(order)
            .take(lean.candidates.max(1))
            .collect();
        // Each candidate's leaves, the worst for it first.
        let mut leaves: Vec<Vec<State>> = Vec::new();
        for &index in &candidates {
            let plan = &plans[index];
            let mut rated: Vec<(i64, State)> = Vec::new();
            if plan.end.outcome().is_none() {
                let mut seen: Vec<&[Command]> = Vec::new();
                let own = plan.own.iter().flatten();
                for reply in memory.iter().chain(own) {
                    if !seen.contains(&reply.commands.as_slice()) {
                        seen.push(&reply.commands);
                        let after = after_reply(&plan.end, &reply.commands);
                        rated.push((self.value(&after), after));
                    }
                }
            }
            // Stable: the memory's order on a tie.
            rated.sort_by_key(|(damage, _)| *damage);
            rated.truncate(lean.replies.max(1));
            leaves.push(rated.into_iter().map(|(_, after)| after).collect());
        }
        let total: usize = leaves.iter().map(Vec::len).sum();
        let tries = if lean.free {
            u128::from(full.max_evals) * FREE_TRIES_PCT / 100
        } else {
            u128::from(full.max_evals) * u128::from(100 - lean.share.min(100)) / 100
        };
        let min_think = Duration::from_secs_f64(MIN_THINK_US / 1e6 * cg_core::time::time_scale());
        let each = (tries / total.max(1) as u128).max(1) as u64;
        let mut left = total;
        let mut best: Option<(i64, bool, usize)> = None;
        for (rank, &index) in candidates.iter().enumerate() {
            let mut deep = if leaves[rank].is_empty() {
                self.final_score(&plans[index], memory)
            } else {
                i64::MAX
            };
            // Whether the time left gives each leaf a whole turn search
            // (E120, free time only).
            let fits = |left: usize| match full.hard {
                Some(hard) if lean.free => {
                    hard.saturating_duration_since(Instant::now())
                        .div_f64(left.max(1) as f64)
                        >= min_think
                }
                _ => true,
            };
            let mut kind = kind;
            let mut complete = true;
            if lean.free && kind == Leaf::Think && !leaves[rank].is_empty() && !fits(left) {
                if lean.leaf == Leaf::Fit {
                    kind = Leaf::Climb;
                } else {
                    left -= leaves[rank].len();
                    continue;
                }
            }
            for leaf in &leaves[rank] {
                if full.hard.is_some_and(|hard| Instant::now() >= hard) {
                    return best.map(|(_, _, rank)| candidates[rank]);
                }
                if kind == Leaf::Think && !fits(left) {
                    complete = false;
                    left -= 1;
                    continue;
                }
                let mut leaf_limit = Limit {
                    soft: None,
                    hard: None,
                    deep: None,
                    max_evals: each,
                };
                if let Some(hard) = full.hard {
                    let now = Instant::now();
                    let room = hard
                        .saturating_duration_since(now)
                        .div_f64(left.max(1) as f64);
                    let soft = if kind == Leaf::Think { 0.6 } else { 0.9 };
                    leaf_limit.soft = Some(now + room.mul_f64(soft));
                    leaf_limit.hard = Some(now + room);
                    leaf_limit.max_evals = u64::MAX;
                }
                let value = self.lean_leaf(leaf, kind, leaf_limit, opening);
                left -= 1;
                deep = deep.min(value);
            }
            if !complete {
                continue;
            }
            let key = (deep, index == today, std::cmp::Reverse(rank));
            let better = best.is_none_or(|(d, t, r)| key > (d, t, std::cmp::Reverse(r)));
            if better {
                best = Some((deep, index == today, rank));
            }
        }
        best.map(|(_, _, rank)| candidates[rank])
    }

    /// The value of the leaf `state`, the start of our next turn, within
    /// `limit`.
    fn lean_leaf(&self, state: &State, kind: Leaf, limit: Limit, opening: &mut Planned) -> i64 {
        if state.outcome().is_some() {
            return self.value(state);
        }
        match kind {
            Leaf::Think => {
                let thought = think_in(state, limit, opening, None, (None, None));
                thought.score.unwrap_or_else(|| self.value(state))
            }
            Leaf::Climb | Leaf::Fit => {
                if lethal(state).is_some() {
                    return WIN;
                }
                let mut search = Search {
                    me: self.me,
                    opponent: false,
                    limit,
                    evals: 0,
                    hints: Gains::default(),
                    root_actions: None,
                    strike: strike_reserve(state),
                    guide: None,
                    deep_stats: (0, 0),
                };
                let plan = search.climb(state, &[], &[], true);
                search.rate(&plan.end, &[], i64::MIN)
            }
        }
    }
}

/// Adds the replies `found` that the memory does not hold, then keeps the
/// [`MEMORY_CAP`] most damaging. Whether the memory changed.
fn remember(found: &[Reply], memory: &mut Vec<Reply>) -> bool {
    let before: Vec<_> = memory.iter().map(|r| r.commands.clone()).collect();
    for reply in found {
        if memory.iter().any(|r| r.commands == reply.commands) {
            continue;
        }
        memory.push(reply.clone());
    }
    // Stable: the older reply stays first on a tie.
    memory.sort_by_key(|reply| reply.damage);
    memory.truncate(MEMORY_CAP);
    memory.iter().map(|r| &r.commands).ne(&before)
}

/// The commands to play in `state` for the player to act, within `limit`;
/// `opening` plays until the middle game (`ice_and_fire_opening::middle`).
pub fn think(state: &State, limit: Limit, opening: &mut Planned) -> Thought {
    think_with(state, limit, opening, None)
}

/// [`think`] with the strategic layer's guide for the turn (E077): its plan
/// offered as a macro action or as the first climb's start, and the
/// objectives' term added to the score.
pub fn think_with(
    state: &State,
    limit: Limit,
    opening: &mut Planned,
    guide: Option<Guide>,
) -> Thought {
    think_in(state, limit, opening, guide, (DEEP, LEAN_DEPTH))
}

/// [`think_with`] with the deep step's width: `deep` is [`DEEP`] for a turn
/// and `None` inside the deep step's own searches (E118), never recursive.
fn think_in(
    state: &State,
    limit: Limit,
    opening: &mut Planned,
    guide: Option<Guide>,
    (deep, lean): (Option<usize>, Option<Lean>),
) -> Thought {
    let me = state.to_act();
    let mut thought = Thought::default();
    if state.outcome().is_some() {
        return thought;
    }
    if let Some(commands) = lethal(state) {
        thought.commands = commands;
        thought.score = Some(WIN);
        return thought;
    }
    // A time between the soft and the hard limit.
    let at = |share: f64| match (limit.soft, limit.hard) {
        (Some(soft), Some(hard)) => Some(soft + (hard - soft).mul_f64(share)),
        _ => None,
    };
    if HANDOVER_TURN.is_some_and(|turn| state.game_turn() >= turn) {
        opening.end();
    }
    if opening.playing(state) {
        // The opening's choice stops at 76% of the turn: the hard limit is
        // the whole budget.
        let commands = opening.turn(state, at(0.4));
        let buys = commands.iter().any(|command| {
            matches!(
                command,
                Command::Train { .. } | Command::BuildMine { .. } | Command::BuildTower { .. }
            )
        });
        let idle = HANDOVER_IDLE
            && !buys
            && state.game_turn() >= HANDOVER_IDLE_FROM
            && state.gold(me) >= HANDOVER_GOLD;
        if !idle {
            thought.opening = true;
            thought.commands = commands;
            return thought;
        }
        // The opening would keep its gold: the search plays from here on.
        opening.end();
    }
    // A lean deep turn (E119) gives today's search a share of the turn.
    let full = limit;
    // A free-time one (E120) leaves today's search all of it.
    let limit = lean
        .filter(|lean| !lean.free)
        .map_or(limit, |lean| lean_limit(limit, lean.share));
    let at = |share: f64| match (limit.soft, limit.hard) {
        (Some(soft), Some(hard)) => Some(soft + (hard - soft).mul_f64(share)),
        _ => None,
    };
    let mut search = Search {
        me,
        opponent: false,
        limit: Limit {
            soft: at(CLIMB_SHARE),
            ..limit
        },
        evals: 0,
        hints: Gains::default(),
        root_actions: None,
        strike: strike_reserve(state),
        guide,
        deep_stats: (0, 0),
    };
    let mut memory = Vec::new();
    let mut plans: Vec<Plan> = Vec::new();
    let mut banned = Vec::new();
    // Set when no first action left improves the plan (E029).
    let mut exhausted = false;
    let mut forced: Vec<Action> = Vec::new();
    // The first climb starts with the strategic turn (E077).
    let mut start = search
        .guide
        .as_ref()
        .filter(|g| g.start)
        .map(|g| Action::Many(g.plan.clone()));
    while thought.climbs < MAX_CLIMBS && (plans.is_empty() || !search.spent()) {
        let started = match start.take() {
            Some(action) => search.forced(state, &action, &memory),
            None => None,
        };
        let plan = if let Some(plan) = started {
            plan
        } else if exhausted {
            let Some(action) = forced.pop() else {
                break;
            };
            match search.forced(state, &action, &memory) {
                Some(plan) => plan,
                None => continue,
            }
        } else {
            search.climb(state, &memory, &banned, true)
        };
        thought.climbs += 1;
        // A climb that ends on a plan found before has nothing new to learn.
        let same = plans.iter().any(|p| p.commands == plan.commands);
        let first = plan.first;
        // Out of time, the plan is kept and no reply searched yet.
        let stop = search.spent();
        let mut plan = plan;
        let mut more = false;
        if !stop && !same {
            let found = search.answers(&plan.end);
            more = remember(&found, &mut memory);
            plan.own = Some(found);
        }
        if !same {
            plans.push(plan);
        }
        if stop {
            break;
        }
        // A climb against the same memory would find the same plan again:
        // the next one starts elsewhere (E014). When the plan was empty, no
        // first action left improves it: the next climbs are forced to start
        // with each of the others, the best gain so far first (E029).
        if !more && !exhausted {
            match first {
                Some(first) => banned.push(first),
                None => {
                    exhausted = true;
                    forced = search.root_actions.clone().unwrap_or_default();
                    forced.retain(|a| !banned.contains(&key(a.commands())));
                    let steer = search.steer(state, state);
                    if let (Some(steer), Some(k)) = (&steer, TYPE_KINDS) {
                        steer.filter(state, k, &mut forced);
                    }
                    let gain = |a: &Action| {
                        search
                            .hints
                            .get(&key(a.commands()))
                            .copied()
                            .unwrap_or(i64::MIN)
                    };
                    // Stable, so that ties keep the generator's order; the
                    // best gain last, since the climbs pop from the end.
                    match steer.as_ref().filter(|_| TYPE_ORDER) {
                        Some(steer) => {
                            forced.sort_by_key(|a| (gain(a), steer.prior(state, a)));
                        }
                        None => forced.sort_by_key(gain),
                    }
                }
            }
        }
    }
    // E120: whether today's search ended before its limits.
    let ended_early = !search.spent();
    thought.forced = usize::from(exhausted);
    thought.evals = search.evals;
    thought.memory = memory.len();
    thought.plans = plans.len();

    // The plan that scores best against the whole memory and its own
    // replies, the latest on a tie. The latest is always scored; earlier ones
    // while time remains.
    let late = || limit.hard.is_some_and(|t| Instant::now() >= t);
    let mut scores: Vec<Option<i64>> = vec![None; plans.len()];
    for index in (0..plans.len()).rev() {
        if index + 1 < plans.len() && late() {
            break;
        }
        scores[index] = Some(search.final_score(&plans[index], &memory));
    }
    let pick = |scores: &[Option<i64>]| {
        let mut best: Option<(i64, usize)> = None;
        for (index, score) in scores.iter().enumerate().rev() {
            if let Some(score) = *score {
                if best.is_none_or(|(top, _)| score > top) {
                    best = Some((score, index));
                }
            }
        }
        best
    };
    let mut best = pick(&scores);
    // The plan played must have had its replies searched (E027): until the
    // best has, its replies are searched in the time left, the opponent's
    // climb stopping at the hard limit, and the choice made again.
    search.limit.soft = limit.hard;
    while let Some((_, index)) = best {
        if !VERIFY || plans[index].own.is_some() || late() {
            break;
        }
        let found = search.answers(&plans[index].end);
        plans[index].own = Some(found);
        thought.checks += 1;
        scores[index] = Some(search.final_score(&plans[index], &memory));
        best = pick(&scores);
    }
    #[cfg(feature = "analysis")]
    judge::observe(&search, state, &memory, best);
    if let Some(lean) = lean.filter(|lean| !lean.free || ended_early) {
        let today = best.map(|(_, index)| index);
        let chosen = search.lean_step(&plans, &scores, &memory, (today, lean), (full, opening));
        if let Some(index) = chosen {
            best = Some((scores[index].unwrap_or(i64::MIN), index));
        }
    } else if let Some(width) = deep {
        let started = Instant::now();
        for (index, score) in scores.iter_mut().enumerate() {
            score.get_or_insert_with(|| search.final_score(&plans[index], &memory));
        }
        best = pick(&scores);
        let today = best.map(|(_, index)| index);
        let deep_best = search.deepen(&plans, &scores, &memory, (today, width), limit, opening);
        if let Some(index) = deep_best {
            best = Some((scores[index].unwrap_or(i64::MIN), index));
        }
        eprintln!(
            "DEEP turn={} candidates={} leaves={} differs={} ms={:.1}",
            state.game_turn(),
            search.deep_stats.0,
            search.deep_stats.1,
            u8::from(deep_best.is_some() && deep_best != today),
            started.elapsed().as_secs_f64() * 1000.0
        );
    }
    thought.score = best.map(|(score, _)| score);
    if let Some((_, index)) = best {
        thought.strategic = search
            .guide
            .as_ref()
            .is_some_and(|g| !g.plan.is_empty() && plans[index].commands.starts_with(&g.plan));
        thought.pieces = search.guide.as_ref().map_or(0, |g| {
            g.pieces
                .iter()
                .filter(|p| p.iter().all(|c| plans[index].commands.contains(c)))
                .count()
        });
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
