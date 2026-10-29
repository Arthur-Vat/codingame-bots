//! A record replayed into the state at the start of every ply, as the trace
//! replays it, for the commands that read a batch of records directly
//! (`bank`, `towers`, `screening`).

use std::fs;
use std::path::Path;

use cg_arena::record::Record;
use ice_and_fire_engine::{Command, League, Outcome, State, HQ};
use ice_and_fire_referee::map::generate;

use crate::trace::parse;

/// One ply: the state at its start (the player to act is `state.to_act()`),
/// the commands it answered and how long the answer took.
pub struct Step {
    pub state: State,
    pub commands: Vec<Command>,
    pub ms: Option<f64>,
}

/// A replayed game.
pub struct Game {
    /// The record's file name without `.json`.
    pub name: String,
    pub record: Record,
    pub steps: Vec<Step>,
    /// The state after the last ply.
    pub end: State,
}

impl Game {
    /// The seat that took the enemy HQ, if the game ended so.
    pub fn hq_taken_by(&self) -> Option<usize> {
        match self.end.outcome() {
            Some(Outcome::Win(winner)) if self.end.owned(winner).test(HQ[1 - winner]) => {
                Some(winner)
            }
            _ => None,
        }
    }
}

/// Replays `record`, a Bronze game, ply by ply.
pub fn replay(name: &str, record: Record) -> Game {
    let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut steps = Vec::new();
    for answers in &record.turns {
        if state.outcome().is_some() {
            break;
        }
        let seat = state.to_act();
        let answer = answers.iter().find(|a| a.seat == seat);
        let line = answer
            .and_then(|a| a.lines.first())
            .cloned()
            .unwrap_or_default();
        let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
        let before = state.clone();
        state.play_turn(&commands);
        steps.push(Step {
            state: before,
            commands,
            ms: answer.map(|a| a.ms),
        });
    }
    Game {
        name: name.to_string(),
        record,
        steps,
        end: state,
    }
}

/// Reads and replays the record at `path`.
pub fn load(path: &Path) -> Result<Game, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let record: Record =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    if record.game != "ice-and-fire" {
        return Err(format!(
            "{}: a record of {}, not of ice-and-fire",
            path.display(),
            record.game
        ));
    }
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("{}: no file name", path.display()))?;
    Ok(replay(name, record))
}

/// A record's result, once replayed.
type Slot<T> = Option<Result<T, String>>;

/// Applies `f` to each record of `paths`, read and replayed, on `jobs`
/// threads; the results come back in the order of `paths`.
pub fn map_records<T: Send>(
    paths: &[std::path::PathBuf],
    jobs: usize,
    f: impl Fn(&Game) -> T + Sync,
) -> Result<Vec<T>, String> {
    let jobs = jobs.clamp(1, 64);
    let mut slots: Vec<Slot<T>> = Vec::new();
    slots.resize_with(paths.len(), || None);
    let chunks: Vec<(usize, &mut [Slot<T>])> = {
        let size = paths.len().div_ceil(jobs).max(1);
        slots
            .chunks_mut(size)
            .enumerate()
            .map(|(i, c)| (i * size, c))
            .collect()
    };
    std::thread::scope(|scope| {
        for (start, chunk) in chunks {
            let f = &f;
            scope.spawn(move || {
                for (offset, slot) in chunk.iter_mut().enumerate() {
                    *slot = Some(load(&paths[start + offset]).map(|game| f(&game)));
                }
            });
        }
    });
    slots
        .into_iter()
        .map(|slot| slot.unwrap_or_else(|| Err("not replayed".to_string())))
        .collect()
}

#[cfg(test)]
mod tests;
