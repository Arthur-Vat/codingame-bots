//! Reading the referee's input, line by line.

use std::fmt;
use std::io::BufRead;
use std::str::FromStr;

/// Why reading input failed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputError {
    /// The input ended: the game is over or the referee stopped.
    Eof,
    /// The input could not be read.
    Io(String),
    /// A line did not hold what was expected.
    Parse { line: String, expected: String },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::Eof => write!(f, "input ended"),
            InputError::Io(err) => write!(f, "cannot read input: {err}"),
            InputError::Parse { line, expected } => {
                write!(f, "expected {expected}, got {line:?}")
            }
        }
    }
}

impl std::error::Error for InputError {}

/// Reads the referee's input one line at a time.
///
/// ```
/// use cg_core::input::Input;
///
/// let mut input = Input::new("-1 -1\n2\n4 4\n0 8\n".as_bytes());
/// let opponent: (i32, i32) = input.two().unwrap();
/// let count: usize = input.one().unwrap();
/// assert_eq!((opponent, count), ((-1, -1), 2));
/// ```
pub struct Input<R> {
    reader: R,
    line: String,
}

impl<R: BufRead> Input<R> {
    pub fn new(reader: R) -> Self {
        Input {
            reader,
            line: String::new(),
        }
    }

    /// The next line, without its line ending.
    pub fn line(&mut self) -> Result<&str, InputError> {
        self.line.clear();
        match self.reader.read_line(&mut self.line) {
            Ok(0) => Err(InputError::Eof),
            Ok(_) => Ok(self.line.trim_end_matches(['\n', '\r'])),
            Err(err) => Err(InputError::Io(err.to_string())),
        }
    }

    /// The next line, parsed as one value.
    pub fn one<T: FromStr>(&mut self) -> Result<T, InputError> {
        let line = self.line()?;
        line.trim().parse().map_err(|_| parse_error::<T>(line, 1))
    }

    /// The next line, parsed as two whitespace-separated values.
    pub fn two<A: FromStr, B: FromStr>(&mut self) -> Result<(A, B), InputError> {
        let line = self.line()?;
        let mut parts = line.split_whitespace();
        match (
            parts.next().map(str::parse::<A>),
            parts.next().map(str::parse::<B>),
            parts.next(),
        ) {
            (Some(Ok(a)), Some(Ok(b)), None) => Ok((a, b)),
            _ => Err(InputError::Parse {
                line: line.to_string(),
                expected: format!(
                    "two values ({}, {})",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>()
                ),
            }),
        }
    }

    /// The next line, parsed as whitespace-separated values of one type.
    pub fn many<T: FromStr>(&mut self) -> Result<Vec<T>, InputError> {
        let line = self.line()?;
        line.split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| parse_error::<T>(line, 0))
    }
}

fn parse_error<T>(line: &str, count: usize) -> InputError {
    let what = std::any::type_name::<T>();
    InputError::Parse {
        line: line.to_string(),
        expected: match count {
            1 => format!("one value ({what})"),
            _ => format!("values ({what})"),
        },
    }
}

#[cfg(test)]
mod tests;
