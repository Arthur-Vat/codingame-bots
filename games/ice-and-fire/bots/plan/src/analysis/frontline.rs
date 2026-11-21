//! `strategy --positions FILE`: the strategic layer on each position of a
//! bank (`crate::strategy`): one tab-separated line per objective, with a
//! fresh role memory per position.
//!
//! Columns: name, ply, game turn, seat (the player to act), cell (`x,y`),
//! kind, value, held, our arrival turns and how, the enemy's arrival turns
//! and how, margin, role. `how` is `unit ID` or `train x,y`; `-` where there
//! is none (a held objective has no race); the role is `frontline ID`,
//! `train x,y` or `none`.

use std::fmt::Write as _;

use super::{read_bank, ready};
use crate::actions::xy;
use ice_and_fire_opening::frontline::objectives::{objectives, Kind};
use ice_and_fire_opening::frontline::race::{race, Arrival, How};
use ice_and_fire_opening::frontline::roles::{assign, Memory};

fn cell_text(cell: usize) -> String {
    let (x, y) = xy(cell);
    format!("{x},{y}")
}

fn arrival_text(arrival: Option<Arrival>) -> String {
    match arrival {
        None => "-\t-".to_string(),
        Some(Arrival {
            turns,
            how: How::Unit(id),
        }) => format!("{turns}\tunit {id}"),
        Some(Arrival {
            turns,
            how: How::Train(cell),
        }) => format!("{turns}\ttrain {}", cell_text(cell)),
    }
}

/// Runs `strategy` with its arguments (after the word `strategy`).
pub fn run(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--positions" => {
                positions = Some(args.next().cloned().ok_or("--positions needs a value")?);
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let path = positions.ok_or("strategy needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::from(
        "name\tply\tgame_turn\tseat\tcell\tkind\tvalue\theld\tours_turns\tours_how\ttheirs_turns\ttheirs_how\tmargin\trole\n",
    );
    for position in read_bank(&text)? {
        let (state, _) = match ready(&position) {
            Ok(ready) => ready,
            Err(err) => {
                eprintln!("left out: {err}");
                continue;
            }
        };
        let seat = state.to_act();
        let objectives = objectives(&state, seat);
        let races = race(&state, seat, &objectives);
        let roles = assign(&state, seat, &objectives, &races, &mut Memory::default());
        for o in &objectives {
            let race = races.iter().find(|r| r.cell == o.cell);
            let role = if let Some(&(id, _)) = roles.frontline.iter().find(|&&(_, c)| c == o.cell) {
                format!("frontline {id}")
            } else if let Some(&(cell, _)) = roles.trainings.iter().find(|&&(_, c)| c == o.cell) {
                format!("train {}", cell_text(cell))
            } else {
                "none".to_string()
            };
            let _ = writeln!(
                out,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                position.name,
                position.ply,
                position.game_turn,
                seat,
                cell_text(o.cell),
                match o.kind {
                    Kind::Choke => "choke",
                    Kind::Band => "band",
                },
                o.value,
                if o.held { "yes" } else { "no" },
                arrival_text(race.and_then(|r| r.ours)),
                arrival_text(race.and_then(|r| r.theirs)),
                race.map_or("-".to_string(), |r| r.margin.to_string()),
                role
            );
        }
    }
    Ok(out)
}
