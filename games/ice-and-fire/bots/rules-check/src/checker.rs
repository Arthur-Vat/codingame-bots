//! The checks: what CodinGame sent against what RULES.md and our engine say.
//!
//! `check_received` uses the turn's input alone. `check_start` knows the
//! first turn. `check_prediction` compares the input with the state the bot
//! predicted after its own commands, last turn: between that state and this
//! input only the opponent's turn happened, then the bot's own start of turn
//! (RULES.md, "Start of a player's turn").

use ice_and_fire_engine::{Bits, Command, League, State, CELLS, HQ, SIZE};

/// Most cells listed in one message.
const MAX_LISTED: usize = 10;

/// Gold per turn of a unit of level 1, 2 and 3 on an active cell.
const UPKEEP: [i64; 3] = [1, 4, 20];

/// The checks made and the differences found.
#[derive(Default)]
pub struct Report {
    pub checks: u32,
    pub differences: Vec<String>,
}

impl Report {
    /// Counts a check; `what` says both sides when it fails.
    pub fn check(&mut self, ok: bool, what: impl FnOnce() -> String) {
        self.checks += 1;
        if !ok {
            self.differences.push(what());
        }
    }

    /// Counts a check that failed.
    pub fn fail(&mut self, what: String) {
        self.check(false, || what);
    }
}

/// A command of the bot's turn that the engine carried out.
#[derive(Clone, Copy, Debug)]
pub struct Applied {
    pub command: Command,
}

/// What the bot predicted for the state after its own commands.
pub struct Prediction {
    /// The state after the commands, before the end of the turn.
    pub after: State,
    /// The first id the bot's own trainings could get.
    pub first_new_id: u32,
    pub applied: Vec<Applied>,
    /// The ids of the opponent's units in the input the commands answered.
    pub opponent_ids: Vec<u32>,
}

fn seat_name(seat: usize) -> &'static str {
    if seat == 0 {
        "red"
    } else {
        "blue"
    }
}

/// The cells as `(x,y)` pairs, the first few.
pub fn cells_text(cells: Bits) -> String {
    let mut text = String::new();
    for (index, cell) in cells.iter().enumerate() {
        if index == MAX_LISTED {
            text.push_str(" ...");
            break;
        }
        if index > 0 {
            text.push(' ');
        }
        text.push_str(&format!("({},{})", cell % SIZE, cell / SIZE));
    }
    if text.is_empty() {
        text.push_str("none");
    }
    text
}

/// The cells of `own` joined to `hq` through cells of `own`.
fn flood(own: Bits, hq: usize) -> Bits {
    if !own.test(hq) {
        return Bits::EMPTY;
    }
    let mut region = Bits::cell(hq);
    loop {
        let next = (region | region.neighbours()) & own;
        if next == region {
            return region;
        }
        region = next;
    }
}

fn upkeep(state: &State, cell: usize) -> i64 {
    UPKEEP[usize::from(state.unit_level(cell).unwrap_or(1)) - 1]
}

/// The income of a seat from RULES.md, "Income", on the cells and units shown.
fn income_formula(state: &State, seat: usize) -> i64 {
    let active = state.active(seat);
    let mut income = i64::from(active.count()) + 4 * i64::from((active & state.mines()).count());
    for cell in (active & state.units(seat)).iter() {
        income -= upkeep(state, cell);
    }
    income
}

/// Whether the bot's units were wiped out, as far as the input alone shows:
/// it had some before, has none, and has no gold.
fn wiped_out(previous: Option<&Prediction>, state: &State) -> bool {
    let me = state.to_act();
    previous.is_some_and(|p| {
        state.gold(me) == 0 && p.after.unit_count(me) > 0 && state.unit_count(me) == 0
    })
}

/// Checks that need the input alone.
pub fn check_received(report: &mut Report, state: &State, previous: Option<&Prediction>) {
    let me = state.to_act();
    let opponent = 1 - me;
    let map = state.map();

    for (seat, &hq) in HQ.iter().enumerate() {
        let connected = flood(state.owned(seat), hq);
        let shown = state.active(seat);
        report.check(connected == shown, || {
            format!(
                "{}'s active cells: CodinGame shows active but not joined to the HQ: {}; \
                 joined to the HQ but shown inactive: {}",
                seat_name(seat),
                cells_text(shown.and_not(connected)),
                cells_text(connected.and_not(shown))
            )
        });
    }

    let stranded = state.units(me).and_not(state.active(me));
    report.check(stranded.is_empty(), || {
        format!(
            "units of the bot on its own inactive cells: CodinGame {}, engine none",
            cells_text(stranded)
        )
    });
    let stranded = state.units(opponent).and_not(state.active(opponent));
    report.check(stranded.is_empty(), || {
        format!(
            "units of the opponent on its own inactive cells at the bot's turn: \
             CodinGame {}, engine none",
            cells_text(stranded)
        )
    });

    let misplaced = state.mines().and_not(map.spots) | (state.towers() & map.spots);
    report.check(misplaced.is_empty(), || {
        format!(
            "mines off the mine spots or towers on them: CodinGame {}, engine none",
            cells_text(misplaced)
        )
    });

    // The bot's own income, unless a wipe-out shows the income from before.
    if wiped_out(previous, state) {
        let bare = state.active(me).count() as i64
            + 4 * i64::from((state.active(me) & state.mines()).count());
        let before = previous.map_or(0, |p| {
            p.after
                .units(me)
                .iter()
                .map(|cell| upkeep(&p.after, cell))
                .sum()
        });
        let shown = state.income(me);
        report.check(bare - before <= shown && shown <= bare, || {
            format!(
                "income of the bot after a wipe-out: CodinGame {shown}, engine between {} and {bare}",
                bare - before
            )
        });
    } else {
        let formula = income_formula(state, me);
        report.check(formula == state.income(me), || {
            format!(
                "income of the bot: CodinGame {}, engine {formula}",
                state.income(me)
            )
        });
    }
    let formula = income_formula(state, opponent);
    report.check(formula == state.income(opponent), || {
        format!(
            "income of the opponent: CodinGame {}, engine {formula}",
            state.income(opponent)
        )
    });
}

/// The first turn: the start of the game is known (RULES.md, "Start of the
/// game"). Red sees the whole start; blue sees red's first turn too, so only
/// its own side is known.
pub fn check_start(report: &mut Report, state: &State, raw: &str) {
    let me = state.to_act();
    if me == 0 {
        let mut void = [false; CELLS];
        for cell in state.map().void.iter() {
            void[cell] = true;
        }
        let spots: Vec<(usize, usize)> = state
            .map()
            .spots
            .iter()
            .map(|cell| (cell % SIZE, cell / SIZE))
            .collect();
        let start = State::new(&void, &spots, League::Bronze).turn_input(0);
        let shown: Vec<&str> = raw.lines().map(str::trim_end).collect();
        let engine: Vec<&str> = start.lines().collect();
        let first = (0..shown.len().max(engine.len())).find(|&k| shown.get(k) != engine.get(k));
        report.check(first.is_none(), || {
            let k = first.unwrap_or(0);
            format!(
                "the first turn as red, line {}: CodinGame {:?}, engine {:?}",
                k + 1,
                shown.get(k),
                engine.get(k)
            )
        });
    } else {
        let blue_cells = Bits::cell(HQ[1]) | Bits::cell(HQ[1] - SIZE);
        report.check(state.gold(1) == 22 && state.income(1) == 2, || {
            format!(
                "the first turn as blue, gold and income: CodinGame {} and {}, engine 22 and 2",
                state.gold(1),
                state.income(1)
            )
        });
        report.check(
            state.owned(1) == blue_cells && state.unit_count(1) == 0,
            || {
                format!(
                    "the first turn as blue, own cells: CodinGame {} with {} units, \
                     engine {} with none",
                    cells_text(state.owned(1)),
                    state.unit_count(1),
                    cells_text(blue_cells)
                )
            },
        );
    }
}

/// The cell of the bot's unit `id` in `state`, if it has one.
fn find_unit(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&cell| state.unit_id(cell) == Some(id))
}

/// Checks the input against the prediction.
pub fn check_prediction(report: &mut Report, now: &State, prediction: &Prediction) {
    let me = now.to_act();
    let opponent = 1 - me;
    let before = &prediction.after;
    if before.to_act() != me {
        report.fail(format!(
            "the bot's seat: CodinGame {}, engine {}",
            seat_name(me),
            seat_name(before.to_act())
        ));
        return;
    }

    report.check(
        now.map().void == before.map().void && now.map().spots == before.map().spots,
        || {
            format!(
                "the map changed: void cells now {}, before {}; mine spots now {}, before {}",
                cells_text(now.map().void),
                cells_text(before.map().void),
                cells_text(now.map().spots),
                cells_text(before.map().spots)
            )
        },
    );

    // Gold: the prediction plus the income shown, or nothing after a wipe-out.
    let owed = before.gold(me) + now.income(me);
    let wiped = owed < 0;
    if wiped {
        report.check(now.gold(me) == 0 && now.unit_count(me) == 0, || {
            format!(
                "wipe-out ({} gold plus income {} is below 0): CodinGame shows {} gold and {} units, \
                 engine 0 gold and no unit",
                before.gold(me),
                now.income(me),
                now.gold(me),
                now.unit_count(me)
            )
        });
    } else {
        report.check(now.gold(me) == owed, || {
            format!(
                "gold of the bot: CodinGame {}, engine {} (predicted {} plus income {})",
                now.gold(me),
                owed,
                before.gold(me),
                now.income(me)
            )
        });
    }

    // Cells.
    let extra = now.owned(me).and_not(before.owned(me));
    report.check(extra.is_empty(), || {
        format!(
            "cells of the bot not owned after its commands: CodinGame {}, engine none",
            cells_text(extra)
        )
    });
    let lost = before
        .owned(me)
        .and_not(now.owned(me))
        .and_not(now.owned(opponent));
    report.check(lost.is_empty(), || {
        format!(
            "cells of the bot that are lost but not the opponent's: CodinGame {}, engine none",
            cells_text(lost)
        )
    });
    let freed = before.owned(opponent).and_not(now.owned(opponent));
    report.check(freed.is_empty(), || {
        format!(
            "cells of the opponent that it no longer owns: CodinGame {}, engine none",
            cells_text(freed)
        )
    });

    // A unit or building is gone legitimately when its cell was captured, the
    // cell is now inactive (units die at the start of the turn), or all units
    // were wiped out.
    let captured = now.owned(opponent);
    let inactive = now.owned(me).and_not(now.active(me));

    // Units of the bot.
    let mut wrong = Vec::new();
    for cell in now.units(me).iter() {
        let (id, level) = (now.unit_id(cell), now.unit_level(cell));
        let same_level = before.unit_level(cell) == level;
        // Ids of trained units are the bot's guess (it cannot see ids of
        // units that died), so only their side of the first new id is known.
        let first = prediction.first_new_id;
        let same_id = match (id, before.unit_id(cell)) {
            (Some(shown), Some(predicted)) => {
                (shown >= first && predicted >= first) || shown == predicted
            }
            _ => false,
        };
        if !(before.units(me).test(cell) && same_level && same_id) {
            wrong.push(format!(
                "({},{}) CodinGame id {:?} level {:?}, engine id {:?} level {:?}",
                cell % SIZE,
                cell / SIZE,
                id,
                level,
                before.unit_id(cell).filter(|_| before.units(me).test(cell)),
                before
                    .unit_level(cell)
                    .filter(|_| before.units(me).test(cell)),
            ));
        }
    }
    report.check(wrong.is_empty(), || {
        format!("units of the bot: {}", list(&wrong))
    });
    let gone = before.units(me).and_not(now.units(me));
    let unexplained = if wiped {
        Bits::EMPTY
    } else {
        gone.and_not(captured | inactive)
    };
    report.check(unexplained.is_empty(), || {
        format!(
            "units of the bot gone without a capture, an inactive cell or a wipe-out: \
             CodinGame none on {}, engine they stay",
            cells_text(unexplained)
        )
    });

    // Buildings of the bot.
    let mut wrong = Vec::new();
    for (kind, now_set, before_set) in [
        ("mine", now.mines(), before.mines()),
        ("tower", now.towers(), before.towers()),
    ] {
        let appeared = (now_set & now.owned(me)).and_not(before_set & before.owned(me));
        if !appeared.is_empty() {
            wrong.push(format!(
                "{kind} CodinGame {}, engine none",
                cells_text(appeared)
            ));
        }
        let vanished = (before_set & before.owned(me))
            .and_not(now_set)
            .and_not(captured);
        if !vanished.is_empty() {
            wrong.push(format!(
                "{kind} gone from {} though they were not captured, engine they stay",
                cells_text(vanished)
            ));
        }
    }
    report.check(wrong.is_empty(), || {
        format!("buildings of the bot: {}", list(&wrong))
    });

    // Units of the opponent: ids are unique and grow.
    let newest = (0..CELLS)
        .filter_map(|cell| before.unit_id(cell))
        .max()
        .unwrap_or(0);
    let mut wrong = Vec::new();
    for cell in now.units(opponent).iter() {
        let Some(id) = now.unit_id(cell) else {
            continue;
        };
        if prediction.opponent_ids.contains(&id) {
            match find_unit(before, opponent, id) {
                None => wrong.push(format!("unit {id} was captured and is back")),
                Some(old) if before.unit_level(old) != now.unit_level(cell) => wrong.push(format!(
                    "unit {id} has level {:?}, engine {:?}",
                    now.unit_level(cell),
                    before.unit_level(old)
                )),
                Some(_) => {}
            }
        } else if id <= newest {
            wrong.push(format!("new unit with id {id}, engine more than {newest}"));
        }
    }
    report.check(wrong.is_empty(), || {
        format!("units of the opponent: {}", list(&wrong))
    });

    // What each accepted command should show, unless the opponent undid it.
    for applied in &prediction.applied {
        let (what, cell, shown) = match applied.command {
            Command::Train { level, x, y } => {
                let cell = usize::from(y) * SIZE + usize::from(x);
                let shown = now.units(me).test(cell) && now.unit_level(cell) == Some(level);
                ("a unit of that level there", cell, shown)
            }
            Command::Move { id, .. } => {
                let Some(cell) = find_unit(before, me, id) else {
                    continue;
                };
                let shown = now.units(me).test(cell) && now.unit_id(cell) == Some(id);
                ("that unit there", cell, shown)
            }
            Command::BuildMine { x, y } => {
                let cell = usize::from(y) * SIZE + usize::from(x);
                ("a mine there", cell, now.mines().test(cell))
            }
            Command::BuildTower { x, y } => {
                let cell = usize::from(y) * SIZE + usize::from(x);
                ("a tower there", cell, now.towers().test(cell))
            }
            Command::Wait => continue,
        };
        let is_unit = matches!(
            applied.command,
            Command::Train { .. } | Command::Move { .. }
        );
        let excused = captured.test(cell) || (is_unit && (wiped || inactive.test(cell)));
        report.check(shown || excused, || {
            format!(
                "{} was carried out by the engine: CodinGame shows no {what} ({},{}), \
                 and the cell was neither captured nor inactive",
                applied.command.to_text(),
                cell % SIZE,
                cell / SIZE
            )
        });
    }
}

fn list(items: &[String]) -> String {
    let mut text = items
        .iter()
        .take(MAX_LISTED)
        .cloned()
        .collect::<Vec<_>>()
        .join("; ");
    if items.len() > MAX_LISTED {
        text.push_str("; ...");
    }
    text
}
