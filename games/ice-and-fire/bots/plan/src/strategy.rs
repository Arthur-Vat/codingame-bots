//! The strategic layer of the frontline plan
//! (`research/codingame/2026-11-20-frontline-plan.md`): objectives with their
//! values, the race to them, and the roles of our units kept between turns
//! (Steps 2 and 3), and the strategic turn offered to the search (Step 4,
//! E077), each behind a switch.

pub mod objectives;
pub mod race;
pub mod roles;
#[cfg(test)]
mod tests;
pub mod turn;

use ice_and_fire_engine::{Command, State};

pub use roles::Memory;

/// How the strategic turn reaches the search (E077), bits added: 0, not at
/// all (v031); 1, offered as one macro action at the climbs' root; 2, the
/// first climb of each turn starts with it; 4, its pieces (each frontline
/// unit's step and the training at its tip) offered as macro actions at
/// every step of the climbs.
pub const TURN: u8 = 4;

/// Which trainings the strategic turn holds (E077): 1, at the frontline
/// units' tips only, after their moves (the owner's rule); 2, also the
/// trainings the roles give the objectives no unit covers, after those.
pub const TRAININGS: u8 = 1;

/// The objectives' term in the search's score (E077), in quarters of a cell
/// of land per cell an objective decides; 0 for none (v031).
pub const OBJECTIVE_QUARTERS: i64 = 4;

/// What the objectives' term counts for an objective of value `v`: with
/// false, `v` if we hold it, `-v` if the enemy does, 0 otherwise; with true,
/// a neutral objective counts `v` times our race margin at the turn's start
/// clamped to -1..=1 (the plan's definition).
pub const OBJECTIVE_RACE: bool = false;

/// The first and the last game turn of the strategic layer (E077: 7 to 14).
pub const FROM_TURN: u32 = 7;
/// See [`FROM_TURN`].
pub const UNTIL_TURN: u32 = 14;

/// Whether the bot computes objectives, races and roles each turn: when the
/// strategic turn or the objectives' term is on.
pub const STRATEGY: bool = TURN != 0 || OBJECTIVE_QUARTERS != 0;

/// What the layer gives the search for one turn.
#[derive(Clone, Debug, Default)]
pub struct Guide {
    /// The strategic turn's commands, empty for none.
    pub plan: Vec<Command>,
    /// Whether the plan is offered as a macro action.
    pub offered: bool,
    /// Whether the first climb starts with the plan.
    pub start: bool,
    /// The plan's pieces offered as macro actions, empty for none.
    pub pieces: Vec<Vec<Command>>,
    /// For the objectives' term: each objective's cell, value and the count
    /// it gets when neither side holds it (`OBJECTIVE_RACE`).
    pub objectives: Vec<(usize, i64, i64)>,
    /// The term's weight in quarters.
    pub quarters: i64,
}

impl Guide {
    /// The objectives' term for `seat` in `state`.
    pub fn term(&self, state: &State, seat: usize) -> i64 {
        if self.quarters == 0 {
            return 0;
        }
        let (mine, theirs) = (state.active(seat), state.active(1 - seat));
        let mut total = 0;
        for &(cell, value, neutral) in &self.objectives {
            total += if mine.test(cell) {
                value
            } else if theirs.test(cell) {
                -value
            } else {
                neutral
            };
        }
        self.quarters * total / 4
    }
}

/// The layer's guide for the turn of `state`, `memory` updated; none
/// outside game turns [`FROM_TURN`] to [`UNTIL_TURN`] or when [`STRATEGY`]
/// is off.
pub fn guide(state: &State, memory: &mut Memory) -> Option<Guide> {
    let now = state.game_turn();
    if !STRATEGY || !(FROM_TURN..=UNTIL_TURN).contains(&now) || state.outcome().is_some() {
        return None;
    }
    guide_with(
        state,
        memory,
        TURN,
        TRAININGS,
        OBJECTIVE_QUARTERS,
        OBJECTIVE_RACE,
    )
}

/// [`guide`] with its switches given, at any game turn.
pub fn guide_with(
    state: &State,
    memory: &mut Memory,
    mode: u8,
    trainings: u8,
    quarters: i64,
    race_counts: bool,
) -> Option<Guide> {
    let seat = state.to_act();
    let objectives = objectives::objectives(state, seat);
    let races = race::race(state, seat, &objectives);
    let roles = roles::assign(state, seat, &objectives, &races, memory);
    let plan = if mode != 0 {
        turn::strategic_turn(state, &roles, trainings == 2)
    } else {
        Vec::new()
    };
    let counted = objectives::counted(&objectives);
    let terms = counted
        .iter()
        .map(|o| {
            let margin = races
                .iter()
                .find(|r| r.cell == o.cell)
                .map_or(0, |r| r.margin.clamp(-1, 1));
            let neutral = if race_counts { o.value * margin } else { 0 };
            (o.cell, o.value, neutral)
        })
        .collect();
    let pieces = if mode & 4 != 0 {
        turn::pieces(state, &roles)
    } else {
        Vec::new()
    };
    Some(Guide {
        pieces,
        offered: !plan.is_empty() && mode & 1 != 0,
        start: !plan.is_empty() && mode & 2 != 0,
        plan,
        objectives: terms,
        quarters,
    })
}
