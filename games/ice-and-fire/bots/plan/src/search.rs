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
use std::time::Instant;

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

/// Replies kept in the memory.
const MEMORY_CAP: usize = 8;
/// The opponent's best cuts added to the memory after a climb.
const REPLY_CUTS: usize = 3;
/// Most climbs in one turn.
const MAX_CLIMBS: usize = 64;
/// Whether the replies hold bronze's greedy turn (E101: it decides 2 to 4%
/// of the scores for about 6% of the turn).
const REPLY_GREEDY: bool = true;
/// When the climbs end before their deadline, the plans that score best
/// have their replies searched again with this many climbs of the opponent,
/// each barred from the first actions of the earlier ones, in the time left
/// before that deadline (E101); `None`: no such check (v034).
const SPARE_CHECK: Option<usize> = None;
/// The plans checked so, the best first (E101).
const SPARE_PLANS: usize = 3;
/// Whether, when every first action has started a climb and time is left,
/// the climbs start again from the best plan less one of its commands, each
/// in turn (E101): a climb from a plan's neighbour can find a combination
/// that no climb from the root does.
const RESTART: bool = false;
/// Most plans scored by the opponent's climb in one reply search (E019).
const REPLY_EVALS: u64 = 300;
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

/// When the search stops.
#[derive(Clone, Copy)]
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
    ///
    /// With more than one of `climbs`, the opponent climbs that many times,
    /// each barred from the first actions of the earlier ones (E101's spare
    /// check).
    fn replies_with(&self, end: &State, climbs: usize) -> Vec<Vec<Command>> {
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
        if REPLY_GREEDY {
            let mut greedy = Vec::new();
            greedy_turn(&next, &mut greedy);
            out.push(greedy);
        }
        // The opponent's own plan (E019): a climb from its seat, with moves,
        // trainings and cuts, scored by the evaluation from its side.
        let mut theirs = Search {
            me: opponent,
            limit: Limit {
                max_evals: REPLY_EVALS * climbs as u64,
                ..self.limit
            },
            evals: 0,
            hints: Gains::default(),
            root_actions: None,
            // The opponent's model keeps no reserve, as before E069.
            strike: false,
            guide: None,
        };
        let mut banned = Vec::new();
        let mut found: Vec<Vec<Command>> = Vec::new();
        for _ in 0..climbs {
            let plan = theirs.climb(&next, &[], &banned, true);
            if !found.contains(&plan.commands) {
                found.push(plan.commands.clone());
                out.push(plan.commands);
            }
            match plan.first {
                Some(first) if !theirs.spent() => banned.push(first),
                _ => break,
            }
        }
        out.retain(|commands| !commands.is_empty());
        out
    }

    /// The replies to the plan that ends in `end`, each with its damage.
    fn answers(&self, end: &State) -> Vec<Reply> {
        self.answers_with(end, 1)
    }

    /// [`Self::answers`] with `climbs` climbs of the opponent.
    fn answers_with(&self, end: &State, climbs: usize) -> Vec<Reply> {
        self.replies_with(end, climbs)
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
    let me = state.to_act();
    let mut thought = Thought::default();
    if state.outcome().is_some() {
        return thought;
    }
    if let Some(commands) = lethal(state) {
        thought.commands = commands;
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
    let mut search = Search {
        me,
        limit: Limit {
            soft: at(CLIMB_SHARE),
            ..limit
        },
        evals: 0,
        hints: Gains::default(),
        root_actions: None,
        strike: strike_reserve(state),
        guide,
    };
    let mut memory = Vec::new();
    let mut plans: Vec<Plan> = Vec::new();
    let mut banned = Vec::new();
    // Set when no first action left improves the plan (E029).
    let mut exhausted = false;
    let mut forced: Vec<Action> = Vec::new();
    // Set once the climbs started again from the best plan (E101).
    let mut restarted = false;
    // The longest reply search of the turn (E101's spare check).
    let mut reply_cost = std::time::Duration::ZERO;
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
            if forced.is_empty() && RESTART && !restarted {
                // Every first action started a climb: start again from the
                // best plan so far, less one of its commands each time.
                restarted = true;
                let best = plans
                    .iter()
                    .max_by_key(|p| search.rate(&p.end, &memory, i64::MIN));
                if let Some(best) = best {
                    for skip in 0..best.commands.len() {
                        let mut rest = best.commands.clone();
                        rest.remove(skip);
                        if !rest.is_empty() {
                            forced.push(Action::Many(rest));
                        }
                    }
                }
            }
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
            let started = Instant::now();
            let found = search.answers(&plan.end);
            reply_cost = reply_cost.max(started.elapsed());
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
                    let gain = |a: &Action| {
                        search
                            .hints
                            .get(&key(a.commands()))
                            .copied()
                            .unwrap_or(i64::MIN)
                    };
                    // Stable, so that ties keep the generator's order; the
                    // best gain last, since the climbs pop from the end.
                    forced.sort_by_key(gain);
                }
            }
        }
    }
    // The climbs ended early: the best plans get their replies searched
    // again, with more climbs of the opponent, in the time left (E101).
    if let Some(climbs) = SPARE_CHECK {
        let mut order: Vec<(i64, usize)> = plans
            .iter()
            .enumerate()
            .map(|(i, p)| (search.final_score(p, &memory), i))
            .collect();
        order.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        let cost = reply_cost.mul_f64(climbs as f64);
        for &(_, index) in order.iter().take(SPARE_PLANS) {
            let fits = search
                .limit
                .soft
                .is_none_or(|soft| Instant::now() + cost <= soft);
            if !fits {
                break;
            }
            let more = search.answers_with(&plans[index].end, climbs);
            let own = plans[index].own.get_or_insert_with(Vec::new);
            for reply in more {
                if !own.iter().any(|r| r.commands == reply.commands) {
                    own.push(reply);
                }
            }
        }
    }
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
