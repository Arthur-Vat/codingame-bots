//! Measures the game's numbers (stage 6): the seat advantage, the length of a
//! game, how games end, the draw rate, and how many commands a turn offers.
//!
//! Each game is played in-process through the reference referee, fed with its
//! text exactly as CodinGame would send it, on the map of the game's index
//! (one game per map, so that games are independent). The bots are the ones
//! the arena plays: `random` is the random bot's turn (`State::random_turn`),
//! `bronze` the policy of the Bronze bot.
//!
//! ```sh
//! cargo run --release -p ice-and-fire-policy --example numbers -- [GAMES]
//! ```
//!
//! Prints Markdown tables. The arena's `match` command shows the same games
//! with real processes; this example adds what its output lacks: the end of
//! the game (HQ capture or turn 100) and the turn it came on.

use std::thread;

use cg_arena::referee::{Answer, GameSetup, Outcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, Bits, Command, League, State, CELLS, SIZE};
use ice_and_fire_policy::answer;
use ice_and_fire_referee::IceAndFireReferee;

/// Game turns after which the game ends on military power.
const LAST_TURN: u32 = 100;

/// Game turns at which the commands on offer are counted.
const SAMPLE_TURNS: [u32; 7] = [1, 2, 5, 10, 20, 40, 60];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Bot {
    Random,
    Bronze,
}

impl Bot {
    fn name(self) -> &'static str {
        match self {
            Bot::Random => "random",
            Bot::Bronze => "bronze",
        }
    }
}

struct Game {
    /// Red's score: 1, 0.5 or 0.
    red: f64,
    /// The game turn the game ended in.
    turn: u32,
    /// Ended by the capture of an HQ (else by military power at turn 100).
    captured: bool,
    /// Commands on offer at the sampled turns, when the game was sampled.
    offers: Vec<Offer>,
}

/// What the player to act could do in a position.
struct Offer {
    turn: u32,
    units: usize,
    /// Cells that a unit can end a move on, summed over the units: the
    /// distinct results of moving one unit.
    moves: usize,
    trains: usize,
    builds: usize,
    /// log10 of the product, over the units, of (1 + the unit's moves): the
    /// move sets of a turn when units do not get in each other's way.
    log10_move_sets: f64,
}

fn main() {
    let games: u64 = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(1000);
    println!(
        "Games per row: {games}, one per map (maps 0 to {}).\n",
        games - 1
    );
    let rows = [
        (Bot::Random, Bot::Random),
        (Bot::Bronze, Bot::Bronze),
        (Bot::Bronze, Bot::Random),
    ];
    println!(
        "| Red | Blue | Red's score | 95% interval | Red wins | Draws | Blue wins | \
         Mean game turns | Median | Reach turn 100 | HQ captured | Ended on power |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |");
    let mut offers: Vec<(Bot, Bot, Vec<Offer>)> = Vec::new();
    for (red, blue) in rows {
        let results = play_all(games, [red, blue]);
        report(red, blue, &results);
        offers.push((
            red,
            blue,
            results.into_iter().flat_map(|game| game.offers).collect(),
        ));
    }
    for (red, blue, offers) in offers.iter().filter(|(red, blue, _)| red == blue) {
        println!("\nCommands on offer to the player to act, in `{}` against `{}` games (means over the positions of both seats at that game turn):\n", red.name(), blue.name());
        report_offers(offers);
    }
}

/// Plays `games` games on 4 threads.
fn play_all(games: u64, bots: [Bot; 2]) -> Vec<Game> {
    let threads = 4;
    let mut results: Vec<(u64, Game)> = thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                scope.spawn(move || {
                    (t..games)
                        .step_by(threads as usize)
                        .map(|seed| (seed, play(seed, bots)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().expect("a game thread"))
            .collect()
    });
    results.sort_by_key(|(seed, _)| *seed);
    results.into_iter().map(|(_, game)| game).collect()
}

fn play(seed: u64, bots: [Bot; 2]) -> Game {
    let mut referee = IceAndFireReferee::new(&GameSetup {
        seed,
        opening_plies: 0,
    });
    let initial = referee.initial_input(0);
    let mut rng = Rng::new(seed ^ 0x5eed);
    let mut offers = Vec::new();
    let mut commands = Vec::new();
    while let Some(&seat) = referee.players_to_act().first() {
        let text = referee.turn_input(seat);
        let turn = referee.game_turn();
        if SAMPLE_TURNS.contains(&turn) {
            let state =
                State::read(&initial, &text, League::Bronze, turn).expect("the referee's text");
            offers.push(offer(&state, turn));
        }
        let line = match bots[seat] {
            Bot::Random => {
                let state =
                    State::read(&initial, &text, League::Bronze, turn).expect("the referee's text");
                state.random_turn(&mut rng, &mut commands);
                answer_line(&commands)
            }
            Bot::Bronze => answer(&initial, &text, League::Bronze, turn),
        };
        let answers = [Answer {
            seat,
            lines: vec![line],
        }];
        referee.play(&answers).expect("the bots answer validly");
    }
    let red = match referee.outcome() {
        Some(Outcome::Win(0)) => 1.0,
        Some(Outcome::Win(_)) => 0.0,
        _ => 0.5,
    };
    Game {
        red,
        turn: referee.game_turn(),
        captured: hq_captured(&referee),
        offers,
    }
}

/// Whether an HQ changed hands, read from the map of the final position:
/// red's HQ at (0, 0) and blue's at (11, 11) are owned by their players
/// until captured.
fn hq_captured(referee: &IceAndFireReferee) -> bool {
    let text = referee.turn_input(0);
    let rows: Vec<&str> = text.lines().skip(4).take(SIZE).collect();
    let red_hq = rows[0].as_bytes()[0];
    let blue_hq = rows[SIZE - 1].as_bytes()[SIZE - 1];
    !matches!(red_hq, b'O' | b'o') || !matches!(blue_hq, b'X' | b'x')
}

fn report(red: Bot, blue: Bot, games: &[Game]) {
    let n = games.len() as f64;
    let mean = games.iter().map(|game| game.red).sum::<f64>() / n;
    let variance = games
        .iter()
        .map(|game| (game.red - mean).powi(2))
        .sum::<f64>()
        / (n - 1.0);
    let half = 1.96 * (variance / n).sqrt();
    let count = |score: f64| games.iter().filter(|game| game.red == score).count();
    let mut turns: Vec<u32> = games.iter().map(|game| game.turn).collect();
    turns.sort_unstable();
    let mean_turns = turns.iter().map(|&t| f64::from(t)).sum::<f64>() / n;
    let median = (turns[turns.len() / 2] + turns[(turns.len() - 1) / 2]) as f64 / 2.0;
    let reach = games.iter().filter(|game| game.turn >= LAST_TURN).count();
    let captured = games.iter().filter(|game| game.captured).count();
    println!(
        "| {} | {} | {:.1}% | {:.1}% to {:.1}% | {} | {} | {} | {:.1} | {} | {:.1}% | {:.1}% | {:.1}% |",
        red.name(),
        blue.name(),
        100.0 * mean,
        100.0 * (mean - half),
        100.0 * (mean + half),
        count(1.0),
        count(0.5),
        count(0.0),
        mean_turns,
        median,
        100.0 * reach as f64 / n,
        100.0 * captured as f64 / n,
        100.0 * (n - captured as f64) / n,
    );
}

/// The commands the player to act could send in `state`, each tried on a
/// copy of the state.
fn offer(state: &State, turn: u32) -> Offer {
    let try_command = |command: Command| state.clone().apply(command);
    let seat = state.to_act();
    let before = state.units(seat);
    let mut moves = 0;
    let mut units = 0;
    let mut log10_move_sets = 0.0;
    for cell in state.movable_units().iter() {
        let id = state.unit_id(cell).expect("a unit on its cell");
        // A move steps one cell toward its target: count the cells a unit
        // can end on, whatever the target.
        let mut ends = Bits::EMPTY;
        for to in 0..CELLS {
            let mut after = state.clone();
            let command = Command::Move {
                id,
                x: (to % SIZE) as u8,
                y: (to / SIZE) as u8,
            };
            if after.apply(command) {
                ends |= after.units(seat).and_not(before);
            }
        }
        let options = ends.count() as usize;
        units += 1;
        moves += options;
        log10_move_sets += ((1 + options) as f64).log10();
    }
    let mut trains = 0;
    let mut builds = 0;
    for cell in 0..CELLS {
        let (x, y) = ((cell % SIZE) as u8, (cell / SIZE) as u8);
        for level in 1..=3 {
            trains += usize::from(try_command(Command::Train { level, x, y }));
        }
        builds += usize::from(try_command(Command::BuildMine { x, y }));
        builds += usize::from(try_command(Command::BuildTower { x, y }));
    }
    Offer {
        turn,
        units,
        moves,
        trains,
        builds,
        log10_move_sets,
    }
}

fn report_offers(offers: &[Offer]) {
    println!("| Game turn | Positions | Units that can move | Cells units can move to | Trainings (level, cell) | Buildings | log10 of the move sets |");
    println!("| --- | --- | --- | --- | --- | --- | --- |");
    for turn in SAMPLE_TURNS {
        let rows: Vec<&Offer> = offers.iter().filter(|offer| offer.turn == turn).collect();
        if rows.is_empty() {
            continue;
        }
        let n = rows.len() as f64;
        let mean = |f: fn(&Offer) -> f64| rows.iter().map(|offer| f(offer)).sum::<f64>() / n;
        println!(
            "| {turn} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |",
            rows.len(),
            mean(|offer| offer.units as f64),
            mean(|offer| offer.moves as f64),
            mean(|offer| offer.trains as f64),
            mean(|offer| offer.builds as f64),
            mean(|offer| offer.log10_move_sets),
        );
    }
}
