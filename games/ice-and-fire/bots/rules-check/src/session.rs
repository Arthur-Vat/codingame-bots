//! One game as the bot sees it: the turns' inputs in, the answers and the log
//! lines out.

use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, League, Outcome, State, MAX_TURNS};

use crate::checker::{check_prediction, check_received, check_start, Applied, Prediction, Report};
use crate::map_log::map_line;

/// The league the bot is meant for: the full rules apply from Bronze.
const LEAGUE: League = League::Bronze;

/// What the bot does with one turn's input.
pub struct TurnOutput {
    /// The lines for stderr, the last being the running totals.
    pub log: Vec<String>,
    /// The answer for stdout.
    pub answer: String,
}

pub struct Session {
    initial: String,
    rng: Rng,
    turn: u32,
    checks: u32,
    differences: u32,
    /// The turn of the first difference.
    first_difference: Option<u32>,
    prediction: Option<Prediction>,
}

fn seat_name(seat: usize) -> &'static str {
    if seat == 0 {
        "red"
    } else {
        "blue"
    }
}

impl Session {
    /// A game whose initial input is `initial`; the random turns follow `seed`.
    pub fn new(initial: String, seed: u64) -> Session {
        Session {
            initial,
            rng: Rng::new(seed),
            turn: 0,
            checks: 0,
            differences: 0,
            first_difference: None,
            prediction: None,
        }
    }

    /// Differences found so far.
    pub fn differences(&self) -> u32 {
        self.differences
    }

    /// The number of the last turn read, from 1.
    pub fn turn(&self) -> u32 {
        self.turn
    }

    /// Notes input that stopped in the middle of a turn, or any other fault
    /// outside a turn's checks.
    pub fn note_difference(&mut self, what: &str) -> String {
        self.checks += 1;
        self.differences += 1;
        self.first_difference.get_or_insert(self.turn + 1);
        format!("rules-check: DIFFERENCE turn {}: {what}", self.turn + 1)
    }

    /// Reads a turn's input, checks it, and plays a random turn. A parse
    /// failure is a difference and the answer is `WAIT`; this never panics.
    pub fn play_turn(&mut self, input: &str) -> TurnOutput {
        self.turn += 1;
        let turn = self.turn;
        let mut report = Report::default();
        let mut info = Vec::new();
        if turn == 1 {
            match map_line(&self.initial, input) {
                Ok(line) => info.push(format!("rules-check: {line}")),
                Err(err) => report.fail(format!("the map line: {err}")),
            }
        }

        let previous = self.prediction.take();
        let mut answer = "WAIT".to_string();
        match State::read(&self.initial, input, LEAGUE, turn) {
            Err(err) => report.fail(format!("the input does not parse: {err}")),
            Ok(state) => {
                check_received(&mut report, &state, previous.as_ref());
                if turn == 1 {
                    check_start(&mut report, &state, input);
                }
                if let Some(previous) = &previous {
                    check_prediction(&mut report, &state, previous);
                }

                let mut commands = Vec::new();
                state.random_turn(&mut self.rng, &mut commands);
                answer = answer_line(&commands);
                let prediction = predict(&state, &commands);
                self.describe(&prediction.after, &mut info);
                self.prediction = Some(prediction);
            }
        }

        self.checks += report.checks;
        self.differences += report.differences.len() as u32;
        if !report.differences.is_empty() {
            self.first_difference.get_or_insert(turn);
        }
        let mut log = info;
        for difference in &report.differences {
            log.push(format!("rules-check: DIFFERENCE turn {turn}: {difference}"));
        }
        let first = self
            .first_difference
            .map_or(String::new(), |first| format!(" (first at turn {first})"));
        log.push(format!(
            "rules-check: turn {turn}, {} checks, {} differences{first}",
            self.checks, self.differences
        ));
        TurnOutput { log, answer }
    }

    /// The lines about the end of the game: a captured HQ, and the last turn.
    fn describe(&self, after: &State, info: &mut Vec<String>) {
        let seat = after.to_act();
        if let Some(Outcome::Win(winner)) = after.outcome() {
            info.push(format!(
                "rules-check: EXPECT win for {} (HQ captured)",
                seat_name(winner)
            ));
        }
        if self.turn == MAX_TURNS {
            let result = if after.outcome().is_some() {
                "decided by the HQ capture above".to_string()
            } else if seat == 1 {
                let mut last = after.clone();
                last.end_turn();
                match last.outcome() {
                    Some(Outcome::Win(winner)) => {
                        format!("win for {} (military power)", seat_name(winner))
                    }
                    Some(Outcome::Draw) => {
                        "draw (equal military power; RULES.md does not confirm it)".to_string()
                    }
                    None => "unknown".to_string(),
                }
            } else {
                "decided after blue's last turn, which is still to come".to_string()
            };
            info.push(format!(
                "rules-check: turn {}: military power red {}, blue {} after our commands; \
                 expected result: {result}",
                self.turn,
                after.military_power(0),
                after.military_power(1)
            ));
        }
    }
}

/// The state after `commands`, applied in the engine one by one.
fn predict(state: &State, commands: &[ice_and_fire_engine::Command]) -> Prediction {
    let mut after = state.clone();
    let mut applied = Vec::with_capacity(commands.len());
    for &command in commands {
        if after.apply(command) {
            applied.push(Applied { command });
        }
    }
    let opponent = 1 - state.to_act();
    let opponent_ids = state
        .units(opponent)
        .iter()
        .filter_map(|cell| state.unit_id(cell))
        .collect();
    Prediction {
        after,
        first_new_id: state.next_unit_id(),
        applied,
        opponent_ids,
    }
}
