//! The commands of an answer, and the answer's text (RULES.md, "Commands").

/// One command of a turn. Coordinates are `u8` so that tests can write cells
/// outside the map; the rules drop those.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Move { id: u32, x: u8, y: u8 },
    Train { level: u8, x: u8, y: u8 },
    BuildMine { x: u8, y: u8 },
    BuildTower { x: u8, y: u8 },
    Wait,
}

impl Command {
    /// The command as CodinGame reads it, for example `MOVE 3 4 5`.
    pub fn to_text(&self) -> String {
        match *self {
            Command::Move { id, x, y } => format!("MOVE {id} {x} {y}"),
            Command::Train { level, x, y } => format!("TRAIN {level} {x} {y}"),
            Command::BuildMine { x, y } => format!("BUILD MINE {x} {y}"),
            Command::BuildTower { x, y } => format!("BUILD TOWER {x} {y}"),
            Command::Wait => "WAIT".to_string(),
        }
    }
}

/// A turn's answer: the commands joined by `;`, or `WAIT` when there is none.
pub fn answer_line(commands: &[Command]) -> String {
    if commands.is_empty() {
        return "WAIT".to_string();
    }
    let mut line = String::with_capacity(16 * commands.len());
    for (index, command) in commands.iter().enumerate() {
        if index > 0 {
            line.push(';');
        }
        line.push_str(&command.to_text());
    }
    line
}
