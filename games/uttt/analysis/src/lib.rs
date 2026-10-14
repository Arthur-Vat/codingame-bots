//! Ultimate Tic-Tac-Toe's analysis program (ADR 0032): one JSON request in,
//! JSON lines out, one report per step of the search. The protocol is
//! written in `studio/README.md`, "The analysis program".
//!
//! It searches with the current code and the settings of the newest release
//! in `games/uttt/releases/` (the bot `games/uttt/bots/mcts`): a test fails
//! when they differ. The search counts iterations, not time, and is seeded,
//! so the same request gives the same lines.
//!
//! This crate never ends up in a bot: it may use a crate from outside the
//! workspace and the bots' pattern and network weights by path.

use cg_search::{Budget, Mcts, MoveReport, Report};
use serde::Serialize;
use std::sync::OnceLock;
use uttt_engine::nnue::Network;
use uttt_engine::search::NnueBoard;
use uttt_engine::{PatternPolicy, Status};

mod request;

use request::Request;

// Packed characters include ones that clippy takes for invisible.
#[allow(clippy::invisible_characters)]
#[path = "../../bots/mcts/src/pattern_weights.rs"]
mod pattern_weights;

// The same.
#[allow(clippy::invisible_characters)]
#[path = "../../bots/mcts/src/nnue_weights.rs"]
mod nnue_weights;

/// The exploration constant of the newest release.
const EXPLORATION: f64 = 0.3;
/// Whether the tree uses the pattern policy's weights as priors.
const PRIORS: bool = false;
/// The weight of the priors' bias in selection.
const PRIOR_WEIGHT: f64 = 0.0;
/// Whether the search expands all the children of a node at once.
const EXPAND_ALL: bool = true;
/// The weight of a child's minimax value against its average in selection.
const MINIMAX_WEIGHT: f64 = 1.0;

/// The search stops, with the stop `"nodes"`, once the tree holds this many
/// nodes. With the `analysis` feature a node takes 40 bytes (a move, its mover, its
/// proof and whether it is expanded, one byte each, then nine 4-byte
/// numbers), so the limit is 320 MB of nodes. It is under 2^23 = 8,388,608
/// on purpose: the nodes vector doubles its capacity when full, so it stays
/// at 2^23 nodes (336 MB) and never doubles to 2^24 (671 MB, with a copy
/// peak of about 1 GB while it grows). The search looks at the tree after
/// at most [`STEP`] iterations and one iteration adds at most 81 nodes, so
/// the tree passes the limit by at most 81,000 nodes, far below 2^23.
const NODE_LIMIT: usize = 8_000_000;

/// The most iterations run between two looks at the size of the tree, so
/// that a large `report_every` cannot fill the memory.
const STEP: u64 = 1_000;

// The tree never needs more than 2^23 nodes of room (see `NODE_LIMIT`).
const _: () = assert!(NODE_LIMIT + 81 * STEP as usize <= 1 << 23);

/// The pattern policy, decoded once.
fn policy() -> &'static PatternPolicy {
    static POLICY: OnceLock<PatternPolicy> = OnceLock::new();
    POLICY.get_or_init(|| {
        let bytes =
            cg_pack::unpack(pattern_weights::PATTERN_PACKED).expect("the pattern weights unpack");
        PatternPolicy::decode_packed(&bytes).expect("the pattern weights decode")
    })
}

/// The network that judges the leaves, decoded once.
fn network() -> &'static Network<256> {
    static NETWORK: OnceLock<Network<256>> = OnceLock::new();
    NETWORK.get_or_init(|| {
        let bytes = cg_pack::unpack(nnue_weights::NNUE_WEIGHTS).expect("the NNUE weights unpack");
        Network::from_bytes(&bytes).expect("the NNUE weights decode")
    })
}

/// Answers one request of the analysis protocol (studio/README.md, "The
/// analysis program"): calls `emit` with each line, without its newline,
/// and stops after the line for which `emit` returns false.
pub fn analyse(request: &str, emit: &mut dyn FnMut(&str) -> bool) {
    analyse_with_stop(request, emit, &|| false);
}

/// [`analyse`], which also asks `stop` before each step of the search, at
/// most [`STEP`] (1000) iterations apart, whatever `report_every` is: when
/// it says true the search ends at once, with no further line. The lines
/// are the same as `analyse`'s up to there.
pub fn analyse_with_stop(
    request: &str,
    emit: &mut dyn FnMut(&str) -> bool,
    stop: &dyn Fn() -> bool,
) {
    match Request::parse(request) {
        Ok(request) => search(&request, NODE_LIMIT, emit, stop),
        Err(message) => {
            let line = serde_json::json!({ "error": message });
            emit(&line.to_string());
        }
    }
}

/// The line `{"report": ...}`.
#[derive(Serialize)]
struct Line {
    report: ReportJson,
}

#[derive(Serialize)]
struct ReportJson {
    iterations: u64,
    nodes: usize,
    to_move: Option<u8>,
    proven: Option<u8>,
    wdl: [f64; 3],
    moves: Vec<MoveJson>,
    done: bool,
    stop: Option<&'static str>,
}

#[derive(Serialize)]
struct MoveJson {
    #[serde(rename = "move")]
    mv: String,
    visits: u32,
    /// `None` for a move the search has not tried.
    wdl: Option<[f64; 3]>,
    proven: Option<u8>,
    line: Vec<String>,
}

/// Emits one report line; false when `emit` asked to stop.
fn send(report: ReportJson, emit: &mut dyn FnMut(&str) -> bool) -> bool {
    let line = serde_json::to_string(&Line { report }).expect("a report is JSON");
    emit(&line)
}

/// Searches the request's position, stopping when the tree holds
/// `node_limit` nodes (the tests lower it).
fn search(
    request: &Request,
    node_limit: usize,
    emit: &mut dyn FnMut(&str) -> bool,
    stop: &dyn Fn() -> bool,
) {
    let board = request.board;
    let over = match board.status() {
        Status::Ongoing => None,
        Status::Win(0) => Some([1.0, 0.0, 0.0]),
        Status::Win(_) => Some([0.0, 0.0, 1.0]),
        Status::Draw => Some([0.0, 1.0, 0.0]),
    };
    if let Some(wdl) = over {
        send(
            ReportJson {
                iterations: 0,
                nodes: 0,
                to_move: None,
                proven: None,
                wdl,
                moves: Vec::new(),
                done: true,
                stop: Some("over"),
            },
            emit,
        );
        return;
    }
    let to_move = board.to_move();
    let root = NnueBoard {
        board,
        network: network(),
        policy: policy(),
    };
    let mut mcts = Mcts::<NnueBoard>::new(EXPLORATION, request.seed);
    mcts.priors = PRIORS;
    mcts.prior_weight = PRIOR_WEIGHT;
    mcts.expand_all = EXPAND_ALL;
    mcts.minimax_weight = MINIMAX_WEIGHT;
    let mut done = 0;
    loop {
        // Up to the next report, in steps small enough to watch the tree.
        let target =
            ((done / request.report_every + 1) * request.report_every).min(request.iterations);
        let mut proven = false;
        let mut full = false;
        while done < target {
            if stop() {
                return;
            }
            let step = (target - done).min(STEP);
            let ran = mcts.think(&root, Budget::Iterations(step));
            done += ran;
            // `think` returns early only when the root is proven.
            proven = ran < step;
            let Some(look) = mcts.report(1) else {
                return;
            };
            proven |= look.proven.is_some();
            full = look.nodes >= node_limit;
            if proven || full {
                break;
            }
        }
        let Some(report) = mcts.report(request.depth) else {
            return;
        };
        let stop = if proven {
            Some("proven")
        } else if done >= request.iterations {
            Some("iterations")
        } else if full {
            Some("nodes")
        } else {
            None
        };
        let json = ReportJson {
            iterations: done,
            nodes: report.nodes,
            to_move: Some(to_move as u8),
            proven: report.proven.map(|wins| seat(to_move, wins)),
            wdl: report
                .moves
                .first()
                .map_or([0.0, 1.0, 0.0], |first| chances(first, to_move)),
            moves: moves(&report, to_move),
            done: stop.is_some(),
            stop,
        };
        if !send(json, emit) || stop.is_some() {
            return;
        }
    }
}

/// The seat proven to win, when the player to move `wins` or loses.
fn seat(to_move: usize, wins: bool) -> u8 {
    (if wins { to_move } else { 1 - to_move }) as u8
}

fn moves(report: &Report<uttt_engine::Move>, to_move: usize) -> Vec<MoveJson> {
    report
        .moves
        .iter()
        .map(|entry| MoveJson {
            mv: entry.mv.to_string(),
            visits: entry.visits,
            wdl: (entry.visits > 0).then(|| chances(entry, to_move)),
            proven: entry.proven.map(|wins| seat(to_move, wins)),
            line: entry.line.iter().map(ToString::to_string).collect(),
        })
        .collect()
}

/// `[seat 0 wins, draw, seat 1 wins]` for a move of the player in seat
/// `to_move`, in ten-thousandths that add up to exactly 1. From the move's
/// average score `s` and share of draws `d`: the mover wins `s - d/2` and
/// loses `1 - s - d/2`, both at least 0, then the three are scaled to add
/// up to 1. A proven move is exactly a win or a loss.
fn chances(entry: &MoveReport<uttt_engine::Move>, to_move: usize) -> [f64; 3] {
    const SCALE: f64 = 10_000.0;
    let (win, draw, loss) = match entry.proven {
        Some(true) => (1.0, 0.0, 0.0),
        Some(false) => (0.0, 0.0, 1.0),
        None => {
            let score = entry.score.clamp(0.0, 1.0);
            let draws = entry.draws.clamp(0.0, 1.0);
            (
                (score - draws / 2.0).max(0.0),
                draws,
                (1.0 - score - draws / 2.0).max(0.0),
            )
        }
    };
    let total = win + draw + loss;
    let (win, loss) = if total > 0.0 {
        (win / total, loss / total)
    } else {
        (0.0, 0.0)
    };
    let win = (win * SCALE).round().min(SCALE);
    let loss = (loss * SCALE).round().min(SCALE - win);
    let draw = SCALE - win - loss;
    let (first, last) = if to_move == 0 {
        (win, loss)
    } else {
        (loss, win)
    };
    [first / SCALE, draw / SCALE, last / SCALE]
}

#[cfg(test)]
mod tests;
