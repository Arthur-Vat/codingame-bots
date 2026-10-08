//! The self-play data format: games as move lists, with the search's
//! result at every searched position.
//!
//! A file starts with [`MAGIC`], then holds games one after another. A
//! game is its number of moves (one byte), the moves (one byte each, `9 *
//! board + cell`), then its number of searched positions (one byte), each
//! being:
//!
//! - its ply (one byte, the number of moves played before it);
//! - the root's average score for the side to move (four bytes, an `f32`);
//! - its number of candidate moves with their visits (one byte; 0 when
//!   visits were not recorded);
//! - each candidate as a move (one byte) and its visits (four bytes).
//!
//! Numbers of several bytes are little-endian. Version 1 ([`MAGIC_V1`]),
//! still read, had no root score.

use std::io::{self, Read, Write};

use uttt_engine::{Board, Move, Status};

/// The first bytes of a data file, with the format's version.
pub const MAGIC: &[u8; 8] = b"UTTTSP02";
/// The first bytes of a file of the first version, without root scores.
pub const MAGIC_V1: &[u8; 8] = b"UTTTSP01";

/// A position searched during a game.
#[derive(Clone, Debug, PartialEq)]
pub struct Searched {
    /// Moves played before this position.
    pub ply: u8,
    /// The root's average score for the side to move after the search, from
    /// 0 to 1; `None` in files of the first version.
    pub score: Option<f32>,
    /// The visits of each move; empty when they were not recorded.
    pub visits: Vec<(Move, u32)>,
}

/// One self-play game.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameRecord {
    pub moves: Vec<Move>,
    pub searched: Vec<Searched>,
}

impl GameRecord {
    /// How the game ended.
    pub fn status(&self) -> Status {
        let mut board = Board::new();
        for &mv in &self.moves {
            board.play(mv);
        }
        board.status()
    }

    /// Every searched position with its visits, in order.
    pub fn positions(&self) -> impl Iterator<Item = (Board, &Searched)> + '_ {
        let mut board = Board::new();
        let mut played = 0;
        self.searched.iter().map(move |searched| {
            while played < usize::from(searched.ply) {
                board.play(self.moves[played]);
                played += 1;
            }
            (board, searched)
        })
    }
}

fn index(mv: Move) -> u8 {
    (9 * mv.board() + mv.cell()) as u8
}

fn from_index(byte: u8) -> io::Result<Move> {
    if byte >= 81 {
        return Err(invalid(&format!("no cell {byte}")));
    }
    Ok(Move::new(usize::from(byte / 9), usize::from(byte % 9)))
}

/// Writes the header of a data file.
pub fn write_header(out: &mut impl Write) -> io::Result<()> {
    out.write_all(MAGIC)
}

/// Writes one game.
pub fn write_game(out: &mut impl Write, game: &GameRecord) -> io::Result<()> {
    let len = |count: usize| {
        u8::try_from(count).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "too long"))
    };
    out.write_all(&[len(game.moves.len())?])?;
    let moves: Vec<u8> = game.moves.iter().map(|&mv| index(mv)).collect();
    out.write_all(&moves)?;
    out.write_all(&[len(game.searched.len())?])?;
    for searched in &game.searched {
        out.write_all(&[searched.ply])?;
        out.write_all(&searched.score.unwrap_or(f32::NAN).to_le_bytes())?;
        out.write_all(&[len(searched.visits.len())?])?;
        for &(mv, visits) in &searched.visits {
            out.write_all(&[index(mv)])?;
            out.write_all(&visits.to_le_bytes())?;
        }
    }
    Ok(())
}

/// Reads bytes one field at a time.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, count: usize) -> io::Result<&'a [u8]> {
        let slice = self
            .bytes
            .get(self.at..self.at + count)
            .ok_or_else(|| invalid("truncated data file"))?;
        self.at += count;
        Ok(slice)
    }

    fn at_end(&self) -> bool {
        self.at == self.bytes.len()
    }
}

fn invalid(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, what.to_string())
}

/// Reads a whole data file.
pub fn read_games(input: &mut impl Read) -> io::Result<Vec<GameRecord>> {
    let mut bytes = Vec::new();
    input.read_to_end(&mut bytes)?;
    let version = match bytes.get(..MAGIC.len()) {
        Some(magic) if magic == MAGIC => 2,
        Some(magic) if magic == MAGIC_V1 => 1,
        _ => return Err(invalid("not a self-play data file")),
    };
    let mut cursor = Cursor {
        bytes: &bytes,
        at: MAGIC.len(),
    };
    let mut games = Vec::new();
    // The end of the file may only come between games.
    while !cursor.at_end() {
        let count = cursor.take(1)?[0];
        let moves = cursor
            .take(usize::from(count))?
            .iter()
            .map(|&byte| from_index(byte))
            .collect::<io::Result<Vec<Move>>>()?;
        let searched_count = cursor.take(1)?[0];
        let mut searched = Vec::with_capacity(usize::from(searched_count));
        for _ in 0..searched_count {
            let ply = cursor.take(1)?[0];
            let score = if version == 1 {
                None
            } else {
                let bytes = cursor.take(4)?;
                Some(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            };
            let children = cursor.take(1)?[0];
            if ply > count {
                return Err(invalid("a searched position after the game's end"));
            }
            let mut visits = Vec::with_capacity(usize::from(children));
            for _ in 0..children {
                let entry = cursor.take(5)?;
                let visit_bytes = [entry[1], entry[2], entry[3], entry[4]];
                visits.push((from_index(entry[0])?, u32::from_le_bytes(visit_bytes)));
            }
            searched.push(Searched { ply, score, visits });
        }
        games.push(GameRecord { moves, searched });
    }
    Ok(games)
}

#[cfg(test)]
mod tests;
