//! Records of random games, for the tests.

use cg_arena::record::{Player, PlayerKind, Record, RecordedAnswer, RECORD_FORMAT};
use cg_arena::runner::EndReason;
use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, League, Outcome, State};
use ice_and_fire_referee::map::generate;

fn player(name: &str) -> Player {
    Player {
        name: name.to_string(),
        kind: PlayerKind::Bot,
        bot_seed: Some(1),
        command: None,
        time_scale: 1.0,
        fixed_iters: None,
    }
}

/// A game of random turns on the referee's map of `seed`, as the arena
/// records it, between bots named `names`; each answer takes `ply` + 1
/// milliseconds.
pub fn random_record(seed: u64, names: [&str; 2]) -> Record {
    let map = generate(seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut rng = Rng::new(seed ^ 0x51ed_270b);
    let mut commands = Vec::new();
    let mut turns = Vec::new();
    while state.outcome().is_none() {
        let seat = state.to_act();
        state.random_turn(&mut rng, &mut commands);
        turns.push(vec![RecordedAnswer {
            seat,
            lines: vec![answer_line(&commands)],
            ms: turns.len() as f64 + 1.0,
        }]);
        state.play_turn(&commands);
    }
    Record {
        format: RECORD_FORMAT,
        game: "ice-and-fire".to_string(),
        seed,
        opening_plies: 0,
        unix_time: 0,
        source: "test".to_string(),
        players: names.map(player),
        turns,
        end: EndReason::Finished,
        winner: match state.outcome() {
            Some(Outcome::Win(w)) => Some(w),
            _ => None,
        },
    }
}

/// A fresh empty directory for a test's files.
pub fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir =
        std::env::temp_dir().join(format!("ice-and-fire-review-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    dir
}
