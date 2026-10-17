//! The loop of a bot: read CodinGame's input, answer with the policy, flush.

use std::io::{BufRead, Write};
use std::time::Instant;

use ice_and_fire_engine::League;

/// Lines of a turn before the building count: four numbers, then the map.
const HEADER_LINES: usize = 4 + 12;

/// Plays turns of a game of `league` until the input ends. Prints the time
/// of each turn to stderr (debug output goes there only).
pub fn play(league: League, input: impl BufRead, mut output: impl Write) -> Result<(), String> {
    let mut lines = Lines { reader: input };

    // The initial input. End of input before it means there is no game.
    let Some(first) = lines.line()? else {
        return Ok(());
    };
    let mut initial = format!("{first}\n");
    lines.take(count(&first, "mine spot count")?, &mut initial)?;

    let mut turn = 0;
    loop {
        // End of input before a new turn starts means the game is over.
        let Some(first) = lines.line()? else {
            return Ok(());
        };
        turn += 1;
        let mut text = format!("{first}\n");
        lines.take(HEADER_LINES - 1, &mut text)?;
        // Buildings, then units: each a count and that many lines.
        for what in ["building count", "unit count"] {
            let line = lines.required()?;
            let n = count(&line, what)?;
            text.push_str(&line);
            text.push('\n');
            lines.take(n, &mut text)?;
        }

        let start = Instant::now();
        let answer = crate::answer(&initial, &text, league, turn);
        eprintln!("turn {turn} ({league:?}): {:?}", start.elapsed());
        writeln!(output, "{answer}")
            .and_then(|()| output.flush())
            .map_err(|err| format!("turn {turn}: cannot write the answer: {err}"))?;
    }
}

struct Lines<R> {
    reader: R,
}

impl<R: BufRead> Lines<R> {
    /// The next line without its line ending, or `None` at the end of input.
    fn line(&mut self) -> Result<Option<String>, String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(line.trim_end_matches(['\r', '\n']).to_string())),
            Err(err) => Err(format!("cannot read input: {err}")),
        }
    }

    fn required(&mut self) -> Result<String, String> {
        self.line()?
            .ok_or_else(|| "input ended in the middle of a turn".to_string())
    }

    /// Appends the next `n` lines to `text`.
    fn take(&mut self, n: usize, text: &mut String) -> Result<(), String> {
        for _ in 0..n {
            text.push_str(&self.required()?);
            text.push('\n');
        }
        Ok(())
    }
}

fn count(line: &str, what: &str) -> Result<usize, String> {
    line.trim()
        .parse()
        .map_err(|_| format!("bad {what} {line:?}"))
}
