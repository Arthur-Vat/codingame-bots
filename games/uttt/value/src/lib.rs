//! E012's value network for Ultimate Tic-Tac-Toe and its searches, for the
//! trainer; bots never use it, which is why it is not part of `uttt-engine`,
//! whose code the bundler copies into every bot.
//!
//! The network gives the expected score of the side to move (ADR 0017).
//!
//! The inputs describe a position from the side to move's view, as 0 or 1
//! each; only the ones that are 1 are listed, and the first layer adds up
//! their rows. Two hidden layers use a clipped ReLU (between 0 and 1) and
//! the output a sigmoid. The arithmetic is in `f32`, in loops the compiler
//! vectorizes for any x86-64 processor.
//!
//! Inputs, by index:
//!
//! - `0..81`: cells of open small boards marked by the side to move, at
//!   `9 * board + cell`;
//! - `81..162`: the same for the opponent;
//! - `162..189`: closed small boards, at `162 + 3 * board`, plus 0 when the
//!   side to move won it, 1 when the opponent did, 2 when it is full;
//! - `189..199`: the board the side to move is sent to, at `189 + board`,
//!   or 198 for a free choice;
//! - `199..208`: open boards where the side to move has a free cell that
//!   wins the board, at `199 + board`;
//! - `208..217`: the same for the opponent.

use cg_core::rng::Rng;
use cg_search::Game;
use uttt_engine::search::score;
use uttt_engine::{Board, Move};
use uttt_policy::{PlayoutPolicy, PolicyMoves};

/// Inputs of the network.
pub const INPUTS: usize = 217;
/// Index of the first input of the opponent's cells.
pub const THEIR_CELLS: usize = 81;
/// Index of the first input of the closed boards.
pub const CLOSED: usize = 162;
/// Index of the first input of the target board.
pub const TARGET: usize = 189;
/// Index of the first input of the side to move's threats; the opponent's
/// follow.
pub const THREATS: usize = 199;
/// Inputs that can be 1 at once: 81 cells or closed boards, a target, and
/// 18 threats fit.
pub const MAX_ACTIVE: usize = 128;

/// Writes the indices of the inputs that are 1 in `board` into `list`, in
/// increasing order, and returns how many there are.
pub fn active_inputs(board: &Board, list: &mut [u16; MAX_ACTIVE]) -> usize {
    let me = board.to_move();
    let them = 1 - me;
    let mut count = 0;
    let mut push = |input: usize, count: &mut usize| {
        list[*count] = input as u16;
        *count += 1;
    };
    let closed = board.closed_boards();
    for (seat, offset) in [(me, 0), (them, THEIR_CELLS)] {
        for small in 0..9 {
            if closed & (1 << small) != 0 {
                continue;
            }
            let mut cells = board.cells(seat, small);
            while cells != 0 {
                push(
                    offset + 9 * small + cells.trailing_zeros() as usize,
                    &mut count,
                );
                cells &= cells - 1;
            }
        }
    }
    let mut boards = closed;
    while boards != 0 {
        let small = boards.trailing_zeros() as usize;
        let status = if board.won_boards(me) & (1 << small) != 0 {
            0
        } else if board.won_boards(them) & (1 << small) != 0 {
            1
        } else {
            2
        };
        push(CLOSED + 3 * small + status, &mut count);
        boards &= boards - 1;
    }
    push(TARGET + board.target().unwrap_or(9), &mut count);
    for (seat, offset) in [(me, THREATS), (them, THREATS + 9)] {
        let mut threats = board.threat_boards(seat);
        while threats != 0 {
            push(offset + threats.trailing_zeros() as usize, &mut count);
            threats &= threats - 1;
        }
    }
    count
}

/// The network of ADR 0017: 217 inputs, 64 and 16 hidden units.
pub type ValueNetwork = Network<64, 16>;

/// A network with `H` units in its first hidden layer and `H2` in its
/// second.
///
/// Its parameters, in order, are the input weights (one row of `H` per
/// input), the first layer's biases, the second layer's weights (one row
/// of `H` per unit of the second layer), its biases, the output weights
/// and the output bias.
#[derive(Clone, Debug, PartialEq)]
pub struct Network<const H: usize, const H2: usize> {
    input: Vec<[f32; H]>,
    input_bias: [f32; H],
    hidden: Vec<[f32; H]>,
    hidden_bias: [f32; H2],
    output: [f32; H2],
    output_bias: f32,
}

impl<const H: usize, const H2: usize> Network<H, H2> {
    /// The sizes of the parameter groups, in order.
    pub const GROUPS: [usize; 6] = [INPUTS * H, H, H2 * H, H2, H2, 1];
    /// The number of parameters.
    pub const PARAMETERS: usize = INPUTS * H + H + H2 * H + H2 + H2 + 1;

    /// The network with these parameters, in the order of the type's
    /// documentation.
    pub fn from_parameters(parameters: &[f32]) -> Result<Self, String> {
        if parameters.len() != Self::PARAMETERS {
            return Err(format!(
                "{} parameters instead of {}",
                parameters.len(),
                Self::PARAMETERS
            ));
        }
        let mut rest = parameters;
        let mut take = |count: usize| {
            let (taken, after) = rest.split_at(count);
            rest = after;
            taken
        };
        let rows = |values: &[f32]| -> Vec<[f32; H]> {
            values
                .chunks_exact(H)
                .map(|row| std::array::from_fn(|unit| row[unit]))
                .collect()
        };
        Ok(Network {
            input: rows(take(INPUTS * H)),
            input_bias: copy(take(H)),
            hidden: rows(take(H2 * H)),
            hidden_bias: copy(take(H2)),
            output: copy(take(H2)),
            output_bias: take(1)[0],
        })
    }

    /// Decodes weights written by `uttt-trainer`: base64 of, for each
    /// parameter group in order, its scale as a little-endian `f32`, then
    /// one signed byte per parameter, which is multiplied by the scale.
    pub fn decode(text: &str) -> Result<Self, String> {
        let bytes = base64(text)?;
        let expected = 4 * Self::GROUPS.len() + Self::PARAMETERS;
        if bytes.len() != expected {
            return Err(format!("{} bytes instead of {expected}", bytes.len()));
        }
        let mut parameters = Vec::with_capacity(Self::PARAMETERS);
        let mut rest = &bytes[..];
        for count in Self::GROUPS {
            let scale = f32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
            let values = &rest[4..4 + count];
            parameters.extend(values.iter().map(|&byte| f32::from(byte as i8) * scale));
            rest = &rest[4 + count..];
        }
        Self::from_parameters(&parameters)
    }

    /// The expected score of the side to move in `board`, from 0 to 1.
    pub fn evaluate(&self, board: &Board) -> f32 {
        let mut list = [0; MAX_ACTIVE];
        let count = active_inputs(board, &mut list);
        self.evaluate_inputs(&list[..count])
    }

    /// The output when the inputs listed in `active` are 1 and the others
    /// 0. Each index must be below [`INPUTS`].
    pub fn evaluate_inputs(&self, active: &[u16]) -> f32 {
        let mut first = self.input_bias;
        for &input in active {
            for (value, weight) in first.iter_mut().zip(&self.input[usize::from(input)]) {
                *value += weight;
            }
        }
        for value in &mut first {
            *value = value.clamp(0.0, 1.0);
        }
        let mut second = self.hidden_bias;
        for (value, row) in second.iter_mut().zip(&self.hidden) {
            *value = (*value + dot(&first, row)).clamp(0.0, 1.0);
        }
        let z = self.output_bias + dot(&second, &self.output);
        1.0 / (1.0 + (-z).exp())
    }
}

/// The first `N` values of `values`.
fn copy<const N: usize>(values: &[f32]) -> [f32; N] {
    std::array::from_fn(|index| values[index])
}

/// The dot product of two equally long slices, summed in 8 lanes so that
/// the compiler can vectorize it.
fn dot(a: &[f32], b: &[f32]) -> f32 {
    let mut lanes = [0.0f32; 8];
    for (x, y) in a.chunks_exact(8).zip(b.chunks_exact(8)) {
        for (lane, (x, y)) in lanes.iter_mut().zip(x.iter().zip(y)) {
            *lane += x * y;
        }
    }
    let tail: f32 = a
        .chunks_exact(8)
        .remainder()
        .iter()
        .zip(b.chunks_exact(8).remainder())
        .map(|(x, y)| x * y)
        .sum();
    lanes.iter().sum::<f32>() + tail
}

/// Decodes standard base64 (`A-Z`, `a-z`, `0-9`, `+`, `/`), stopping at
/// padding.
fn base64(text: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(text.len() / 4 * 3);
    let (mut buffer, mut bits) = (0u32, 0);
    for byte in text.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => return Err(format!("{:?} is not base64", char::from(byte))),
        };
        buffer = buffer << 6 | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

/// A position searched with a value network (ADR 0017): [`Board`]'s game,
/// except that a new leaf gets an estimate instead of a playout. A side to
/// move with a move that wins the game scores a win, exactly; otherwise
/// the network judges the position. The network has `H` and `H2` hidden
/// units, those of [`ValueNetwork`] by default.
#[derive(Clone, Copy)]
pub struct ValueBoard<const H: usize = 64, const H2: usize = 16> {
    pub board: Board,
    pub network: &'static Network<H, H2>,
}

impl<const H: usize, const H2: usize> ValueBoard<H, H2> {
    /// Seat 0's expected score in a position where the game goes on.
    pub fn estimate(&self) -> f64 {
        network_estimate(&self.board, self.network)
    }
}

/// Seat 0's expected score in `board`, where the game goes on: a win when
/// the side to move can win the game at once, otherwise the network's.
fn network_estimate<const H: usize, const H2: usize>(
    board: &Board,
    network: &Network<H, H2>,
) -> f64 {
    let value = if board.game_winning_move().is_some() {
        1.0
    } else {
        f64::from(network.evaluate(board))
    };
    if board.to_move() == 0 {
        value
    } else {
        1.0 - value
    }
}

/// Positions are equal when their boards are: a search compares them to
/// find the current position in its tree.
impl<const H: usize, const H2: usize> PartialEq for ValueBoard<H, H2> {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl<const H: usize, const H2: usize> Game for ValueBoard<H, H2> {
    type Move = Move;

    fn to_move(&self) -> usize {
        self.board.to_move()
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        Game::legal_moves(&self.board, moves);
    }

    fn play(&mut self, mv: Move) {
        self.board.play(mv);
    }

    fn score(&self) -> Option<f64> {
        score(self.board.status())
    }

    fn playout(&mut self, _rng: &mut Rng) -> f64 {
        self.estimate()
    }
}

/// A position searched with a value network and playouts together, the
/// fallback of ADR 0017: a new leaf's estimate is `share` of the network's
/// ([`ValueBoard::estimate`]) and the rest the result of a playout that
/// follows `policy`. A side to move that can win the game at once still
/// scores a win, exactly.
#[derive(Clone, Copy)]
pub struct MixBoard<const H: usize = 64, const H2: usize = 16> {
    pub board: Board,
    pub network: &'static Network<H, H2>,
    pub policy: &'static PlayoutPolicy,
    /// The network's share of the estimate, from 0 to 1.
    pub share: f64,
}

impl<const H: usize, const H2: usize> PartialEq for MixBoard<H, H2> {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl<const H: usize, const H2: usize> Game for MixBoard<H, H2> {
    type Move = Move;

    fn to_move(&self) -> usize {
        self.board.to_move()
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        Game::legal_moves(&self.board, moves);
    }

    fn play(&mut self, mv: Move) {
        self.board.play(mv);
    }

    fn score(&self) -> Option<f64> {
        score(self.board.status())
    }

    fn playout(&mut self, rng: &mut Rng) -> f64 {
        if self.board.game_winning_move().is_some() {
            return network_estimate(&self.board, self.network);
        }
        let estimate = network_estimate(&self.board, self.network);
        let mut board = self.board;
        let played =
            score(board.policy_playout(self.policy, rng)).expect("a playout ends the game");
        self.share * estimate + (1.0 - self.share) * played
    }
}

#[cfg(test)]
mod tests;
