//! The map of the game as one log line, for comparing our map generator with
//! CodinGame's (ADR 0039, decision 3). It reads the raw text, not the engine's
//! state, so that it still works when the engine rejects the input.

use ice_and_fire_engine::SIZE;

/// Spots a log line may list: the rules give far fewer.
const MAX_SPOTS: usize = SIZE * SIZE;

/// `MAP <seat> <144 characters, row by row> <spots as x,y separated by
/// spaces>`, without the `rules-check: ` prefix. `#` is a void cell and `.`
/// any other; the seat is `red` or `blue`.
pub fn map_line(initial: &str, turn: &str) -> Result<String, String> {
    let mut lines = initial.lines();
    let count: usize = lines
        .next()
        .and_then(|line| line.trim().parse().ok())
        .filter(|&count| count <= MAX_SPOTS)
        .ok_or("the mine spot count is not a number")?;
    let mut spots = Vec::with_capacity(count);
    for _ in 0..count {
        let line = lines.next().ok_or("the initial input ends early")?;
        let mut words = line.split_whitespace();
        let (x, y) = (words.next(), words.next());
        match (x.and_then(number), y.and_then(number), words.next()) {
            (Some(x), Some(y), None) => spots.push(format!("{x},{y}")),
            _ => return Err(format!("bad mine spot line {line:?}")),
        }
    }

    let rows: Vec<&str> = turn.lines().skip(4).take(SIZE).map(str::trim).collect();
    if rows.len() != SIZE || rows.iter().any(|row| row.len() != SIZE) {
        return Err("the map is not 12 rows of 12 characters".to_string());
    }
    let corner = |row: usize, column: usize| rows[row].as_bytes()[column];
    let seat = if matches!(corner(0, 0), b'O' | b'o') {
        "red"
    } else if matches!(corner(SIZE - 1, SIZE - 1), b'O' | b'o') {
        "blue"
    } else {
        return Err("neither HQ corner is the bot's own".to_string());
    };
    let grid: String = rows
        .iter()
        .flat_map(|row| row.chars())
        .map(|symbol| if symbol == '#' { '#' } else { '.' })
        .collect();
    let mut line = format!("MAP {seat} {grid}");
    for spot in spots {
        line.push(' ');
        line.push_str(&spot);
    }
    Ok(line)
}

fn number(word: &str) -> Option<usize> {
    word.parse().ok().filter(|&value| value < SIZE)
}
