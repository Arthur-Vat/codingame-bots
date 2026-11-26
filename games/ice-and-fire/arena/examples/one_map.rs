//! Plays one game (E080's turn-1 planner prototype) on the map of a given seed
//! and writes its record as JSON, or prints the map's initial input.
//! Usage: one_map SEED [MAX_TURNS TIME_SCALE NAME=CMD NAME=CMD OUT]

use cg_arena::referee::{GameSetup, Referee};
use cg_arena::runner::{run_match, BotSpec, MatchOptions};
use ice_and_fire_referee::IceAndFireReferee;

fn spec(s: &str) -> BotSpec {
    let (name, cmd) = s.split_once('=').expect("NAME=CMD");
    let mut parts = cmd.split(' ').map(String::from);
    BotSpec {
        name: name.into(),
        program: parts.next().expect("a program"),
        args: parts.collect(),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let seed: u64 = a[1].parse::<i64>().map(|s| s as u64).expect("seed");
    let mut referee = IceAndFireReferee::new(&GameSetup {
        seed,
        opening_plies: 0,
    });
    if a.len() == 2 {
        println!("{}", referee.turn_input(0));
        return;
    }
    let options = MatchOptions {
        time_scale: a[3].parse().expect("time scale"),
        max_turns: a[2].parse().expect("max turns"),
        show_bot_stderr: true,
        ..MatchOptions::default()
    };
    let (b0, b1) = (spec(&a[4]), spec(&a[5]));
    let record = run_match(&mut referee, [&b0, &b1], seed, &options).expect("the match");
    std::fs::write(&a[6], serde_json::to_string(&record).expect("json")).expect("write");
    eprintln!("end {:?} turns {}", record.end, record.turns);
}
