//! The fast engine must agree with the reference referee on every turn of
//! many random games: same input text for both seats, same outcome, same
//! military power, whatever the commands, valid or not.

use cg_arena::referee::{Answer, Outcome as RefereeOutcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, Command, League, Outcome, State, HQ};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::{IceAndFireReferee, League as RefereeLeague};

/// Games per league, as required: 2,000 in Bronze (four tests of 500 seeds, so
/// that they run on parallel threads), 300 in each wood league.
const WOOD_GAMES: u64 = 300;

fn referee_league(league: League) -> RefereeLeague {
    match league {
        League::Wood3 => RefereeLeague::Wood3,
        League::Wood2 => RefereeLeague::Wood2,
        League::Wood1 => RefereeLeague::Wood1,
        League::Bronze => RefereeLeague::Bronze,
    }
}

fn outcome_of(outcome: Option<RefereeOutcome>) -> Option<Outcome> {
    outcome.map(|outcome| match outcome {
        RefereeOutcome::Win(seat) => Outcome::Win(seat),
        RefereeOutcome::Draw => Outcome::Draw,
    })
}

/// Commands on random coordinates and ids, valid or not, inserted at random
/// places of `commands`.
fn add_noise(commands: &mut Vec<Command>, next_id: u32, rng: &mut Rng) {
    for _ in 0..1 + rng.below(4) {
        let x = if rng.below(8) == 0 {
            rng.below(256)
        } else {
            rng.below(13)
        } as u8;
        let y = if rng.below(8) == 0 {
            rng.below(256)
        } else {
            rng.below(13)
        } as u8;
        let noise = match rng.below(5) {
            0 => Command::Move {
                id: rng.below(u64::from(next_id) + 3) as u32,
                x,
                y,
            },
            1 | 2 => Command::Train {
                level: rng.below(5) as u8,
                x,
                y,
            },
            3 => Command::BuildMine { x, y },
            _ => Command::BuildTower { x, y },
        };
        let at = rng.below(commands.len() as u64 + 1) as usize;
        commands.insert(at, noise);
    }
}

/// Plays one game on both engines; returns whether an HQ was captured and
/// the number of turns compared. Panics with the seed, the turn and both
/// texts on the first difference.
fn play_game(seed: u64, league: League, rng: &mut Rng) -> (bool, u64) {
    let map = generate(seed, referee_league(league));
    let mut referee = IceAndFireReferee::from_map(map.clone(), referee_league(league));
    let mut state = State::new(&map.void, &map.mine_spots, league);
    let mut turns = 0;
    let mut commands = Vec::new();
    for seat in 0..2 {
        assert_eq!(
            state.initial_input(seat),
            referee.initial_input(seat),
            "seed {seed} {league:?}: initial input of seat {seat}"
        );
    }
    loop {
        let to_act = referee.players_to_act();
        assert_eq!(
            outcome_of(referee.outcome()),
            state.outcome(),
            "seed {seed} {league:?}: outcome after {turns} turns"
        );
        for seat in 0..2 {
            assert_eq!(
                state.military_power(seat),
                referee.military_power(seat),
                "seed {seed} {league:?}: military power of seat {seat} after {turns} turns"
            );
        }
        if to_act.is_empty() {
            break;
        }
        let seat = to_act[0];
        assert_eq!(state.to_act(), seat, "seed {seed} {league:?}: turn {turns}");
        assert_eq!(
            state.game_turn(),
            referee.game_turn(),
            "seed {seed} {league:?}"
        );
        for viewer in 0..2 {
            let (ours, theirs) = (state.turn_input(viewer), referee.turn_input(viewer));
            assert!(
                ours == theirs,
                "seed {seed} {league:?}, turn {turns} (game turn {}, seat {seat} to act), \
                 input of seat {viewer}\n--- engine ---\n{ours}--- referee ---\n{theirs}",
                state.game_turn()
            );
        }
        state.random_turn(rng, &mut commands);
        // random_turn promises that each of its commands succeeds.
        let mut replay = state.clone();
        for &command in &commands {
            assert!(
                replay.apply(command),
                "seed {seed} {league:?}, turn {turns}: {command:?} of {commands:?} fails"
            );
        }
        if rng.below(10) == 0 {
            add_noise(&mut commands, state.next_unit_id(), rng);
        }
        let line = answer_line(&commands);
        let answer = Answer {
            seat,
            lines: vec![line.clone()],
        };
        referee
            .play(&[answer])
            .unwrap_or_else(|error| panic!("seed {seed} {league:?}: {error:?} for {line:?}"));
        state.play_turn(&commands);
        turns += 1;
    }
    // A game decided by military power after 100 turns leaves both HQs home.
    let captured = (0..2).any(|seat| !state.owned(seat).test(HQ[seat]));
    (captured, turns)
}

fn run(league: League, seeds: std::ops::Range<u64>, rng_seed: u64) -> (u64, u64) {
    let mut rng = Rng::new(rng_seed);
    let mut captures = 0;
    let mut turns = 0;
    let games = seeds.end - seeds.start;
    for seed in seeds {
        let (captured, played) = play_game(seed, league, &mut rng);
        captures += u64::from(captured);
        turns += played;
    }
    println!("{league:?}: {games} games, {turns} turns compared, {captures} HQ captures");
    (captures, turns)
}

#[test]
fn bronze_seeds_0_to_500_agree_with_the_referee() {
    let (captures, _) = run(League::Bronze, 0..500, 11);
    assert!(captures > 0, "no Bronze game ended by an HQ capture");
}

#[test]
fn bronze_seeds_500_to_1000_agree_with_the_referee() {
    let (captures, _) = run(League::Bronze, 500..1000, 21);
    assert!(captures > 0, "no Bronze game ended by an HQ capture");
}

#[test]
fn bronze_seeds_1000_to_1500_agree_with_the_referee() {
    let (captures, _) = run(League::Bronze, 1000..1500, 31);
    assert!(captures > 0, "no Bronze game ended by an HQ capture");
}

#[test]
fn bronze_seeds_1500_to_2000_agree_with_the_referee() {
    let (captures, _) = run(League::Bronze, 1500..2000, 41);
    assert!(captures > 0, "no Bronze game ended by an HQ capture");
}

#[test]
fn wood1_games_agree_with_the_referee() {
    let (captures, _) = run(League::Wood1, 0..WOOD_GAMES, 12);
    assert!(captures > 0, "no Wood 1 game ended by an HQ capture");
}

#[test]
fn wood2_games_agree_with_the_referee() {
    let (captures, _) = run(League::Wood2, 0..WOOD_GAMES, 13);
    assert!(captures > 0, "no Wood 2 game ended by an HQ capture");
}

#[test]
fn wood3_games_agree_with_the_referee() {
    let (captures, _) = run(League::Wood3, 0..WOOD_GAMES, 14);
    assert!(captures > 0, "no Wood 3 game ended by an HQ capture");
}
