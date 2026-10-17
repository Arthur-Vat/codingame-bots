use cg_core::rng::Rng;

use super::*;

// --- Helpers ---

/// A map with no void cell and three mine spots.
fn open_map() -> Map {
    Map {
        void: [false; CELLS],
        mine_spots: vec![(1, 0), (5, 5), (10, 11)],
    }
}

fn game(league: League) -> IceAndFireReferee {
    IceAndFireReferee::from_map(open_map(), league)
}

fn game_with_void(void: &[(usize, usize)]) -> IceAndFireReferee {
    let mut map = open_map();
    for &(x, y) in void {
        map.void[y * SIZE + x] = true;
    }
    IceAndFireReferee::from_map(map, League::Bronze)
}

fn cell(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn refresh(game: &mut IceAndFireReferee) {
    game.compute_active();
    game.compute_incomes();
}

/// Gives cells to a seat, without any rule.
fn own(game: &mut IceAndFireReferee, seat: usize, cells: &[(usize, usize)]) {
    for &(x, y) in cells {
        game.owner[cell(x, y)] = Some(seat);
    }
    refresh(game);
}

/// Puts a unit that may move on a cell, which its owner gets.
fn put_unit(game: &mut IceAndFireReferee, id: u32, seat: usize, level: u8, x: usize, y: usize) {
    game.owner[cell(x, y)] = Some(seat);
    game.units.push(Unit {
        id,
        owner: seat,
        level,
        cell: cell(x, y),
        can_move: true,
    });
    game.next_id = game.next_id.max(id + 1);
    refresh(game);
}

/// The player to act answers `line`.
fn say(game: &mut IceAndFireReferee, line: &str) -> Result<(), InvalidAnswer> {
    let seat = game.to_act;
    game.play(&[Answer {
        seat,
        lines: vec![line.to_string()],
    }])
}

fn unit_pos(game: &IceAndFireReferee, id: u32) -> Option<(usize, usize)> {
    game.units
        .iter()
        .find(|unit| unit.id == id)
        .map(|unit| (unit.cell % SIZE, unit.cell / SIZE))
}

fn count_units(game: &IceAndFireReferee, seat: usize) -> usize {
    game.units.iter().filter(|unit| unit.owner == seat).count()
}

fn lines_of(text: &str) -> Vec<&str> {
    text.lines().collect()
}

// --- League ---

#[test]
fn leagues_grow_the_rules() {
    let table = [
        (League::Wood3, 1, false, false, false),
        (League::Wood2, 3, false, false, false),
        (League::Wood1, 3, true, false, true),
        (League::Bronze, 3, true, true, true),
    ];
    for (league, level, mines, towers, spots) in table {
        assert_eq!(league.max_level(), level, "{league:?}");
        assert_eq!(league.allows_mines(), mines, "{league:?}");
        assert_eq!(league.allows_towers(), towers, "{league:?}");
        assert_eq!(league.has_mine_spots(), spots, "{league:?}");
    }
    assert_eq!(League::default(), League::Bronze);
}

// --- Start of the game and of a turn ---

#[test]
fn the_game_starts_with_gold_and_income() {
    let mut g = game(League::Bronze);
    // Red's start of turn has run: 20 gold plus an income of 1 (its HQ).
    assert_eq!(g.gold, [21, 20]);
    assert_eq!(g.income, [1, 2]);
    assert_eq!(g.game_turn(), 1);
    assert_eq!(g.players_to_act(), vec![0]);
    assert_eq!(g.outcome(), None);
    assert_eq!(
        g.time_limits(),
        TimeLimits {
            first_answer: Duration::from_millis(1000),
            later_answers: Duration::from_millis(50),
        }
    );
    // Blue starts with two cells and gets 2.
    say(&mut g, "WAIT").unwrap();
    assert_eq!(g.gold, [21, 22]);
    assert_eq!(g.players_to_act(), vec![1]);
    assert_eq!(g.game_turn(), 1);
    say(&mut g, "WAIT").unwrap();
    assert_eq!(g.gold, [22, 22]);
    assert_eq!(g.players_to_act(), vec![0]);
    assert_eq!(g.game_turn(), 2);
}

#[test]
fn the_start_of_turn_runs_its_steps_in_order() {
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0)]);
    // A unit that moved last turn, and a level 3 unit cut from the HQ.
    put_unit(&mut g, 1, 0, 1, 1, 0);
    g.units[0].can_move = false;
    put_unit(&mut g, 2, 0, 3, 5, 5);
    g.gold[0] = 0;
    g.start_turn(0);
    // Step 2 killed the cut unit before step 3 counted any upkeep: the income
    // is 2 cells minus 1, not the 2 - 21 that would wipe the army out.
    assert_eq!(unit_pos(&g, 2), None);
    assert_eq!(unit_pos(&g, 1), Some((1, 0)));
    assert_eq!(g.income[0], 1);
    assert_eq!(g.gold[0], 1);
    assert!(g.units[0].can_move, "step 6");
}

#[test]
fn a_wipe_out_shows_the_income_from_before_it() {
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0)]);
    put_unit(&mut g, 1, 0, 3, 1, 0);
    put_unit(&mut g, 2, 1, 3, 11, 10);
    g.gold[0] = 0;
    g.start_turn(0);
    // 2 cells minus the upkeep of 20 is -18: the gold becomes 0, the army dies.
    assert_eq!(g.gold[0], 0);
    assert_eq!(count_units(&g, 0), 0);
    assert_eq!(count_units(&g, 1), 1);
    assert_eq!(g.income[0], -18);
    let input = g.turn_input(0);
    assert_eq!(&lines_of(&input)[..2], ["0", "-18"]);
    // The next success recomputes it.
    g.gold[0] = 100;
    g.apply(
        0,
        Action::Train {
            level: 1,
            cell: cell(2, 0),
        },
    );
    assert_eq!(g.income[0], 3 - 1);
}

#[test]
fn units_cut_from_the_hq_die_at_their_owners_next_start() {
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0), (2, 0), (3, 0)]);
    put_unit(&mut g, 5, 0, 1, 3, 0);
    put_unit(&mut g, 9, 1, 2, 2, 1);
    g.to_act = 1;
    // Blue takes (2, 0), which cuts (3, 0) off.
    g.apply(
        1,
        Action::Move {
            id: 9,
            cell: cell(2, 0),
        },
    );
    assert_eq!(g.owner[cell(2, 0)], Some(1));
    assert!(!g.active[cell(3, 0)]);
    assert_eq!(unit_pos(&g, 5), Some((3, 0)), "still alive");
    assert_eq!(g.income[0], 2, "(3, 0) and its unit no longer count");
    g.end_turn();
    assert_eq!(g.to_act, 0);
    assert_eq!(unit_pos(&g, 5), None, "dead at red's start");
}

#[test]
fn income_counts_active_cells_mines_and_upkeep() {
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0), (2, 0)]);
    g.building[cell(1, 0)] = Some(Building::Mine);
    put_unit(&mut g, 1, 0, 2, 2, 0);
    // An island with a mine and a unit earns and costs nothing.
    own(&mut g, 0, &[(8, 8)]);
    g.building[cell(8, 8)] = Some(Building::Mine);
    put_unit(&mut g, 2, 0, 3, 8, 8);
    // 3 active cells, +4 for the active mine, -4 for the level 2 unit.
    assert_eq!(g.income[0], 3 + 4 - 4);
    assert_eq!(g.income[1], 2);
}

#[test]
fn military_power_is_gold_plus_the_cost_of_the_units() {
    let mut g = game(League::Bronze);
    g.gold = [5, 7];
    put_unit(&mut g, 1, 0, 1, 1, 0);
    put_unit(&mut g, 2, 0, 3, 2, 0);
    put_unit(&mut g, 3, 1, 2, 10, 11);
    assert_eq!(g.military_power(0), 5 + 10 + 30);
    assert_eq!(g.military_power(1), 7 + 20);
}

// --- Input text ---

#[test]
fn the_initial_input_lists_the_mine_spots() {
    let g = game(League::Bronze);
    assert_eq!(g.initial_input(0), "3\n1 0\n5 5\n10 11\n");
    assert_eq!(g.initial_input(1), g.initial_input(0));
    let none = IceAndFireReferee::from_map(
        Map {
            void: [false; CELLS],
            mine_spots: Vec::new(),
        },
        League::Wood3,
    );
    assert_eq!(none.initial_input(0), "0\n");
    let setup = GameSetup {
        seed: 5,
        opening_plies: 0,
    };
    let generated = IceAndFireReferee::new(&setup);
    let input = generated.initial_input(0);
    assert_eq!(
        input.lines().next().unwrap(),
        generated.map.mine_spots.len().to_string()
    );
    assert_eq!(input.lines().count(), 1 + generated.map.mine_spots.len());
    let wood = IceAndFireReferee::with_league(&setup, League::Wood2);
    assert_eq!(wood.initial_input(1), "0\n");
}

#[test]
fn the_first_turn_input_for_both_seats() {
    let g = game(League::Bronze);
    let mut red = String::from("21\n1\n20\n2\n");
    let mut blue = String::from("20\n2\n21\n1\n");
    for y in 0..SIZE {
        let (red_row, blue_row) = match y {
            0 => ("O...........", "X..........."),
            10 | 11 => ("...........X", "...........O"),
            _ => ("............", "............"),
        };
        red.push_str(&format!("{red_row}\n"));
        blue.push_str(&format!("{blue_row}\n"));
    }
    red.push_str("2\n0 0 0 0\n1 0 11 11\n0\n");
    blue.push_str("2\n0 0 11 11\n1 0 0 0\n0\n");
    assert_eq!(g.turn_input(0), red);
    assert_eq!(g.turn_input(1), blue);
}

#[test]
fn the_map_shows_void_inactive_and_active_cells() {
    let mut g = game_with_void(&[(7, 5)]);
    own(&mut g, 0, &[(5, 5)]);
    own(&mut g, 1, &[(6, 5)]);
    let red = g.turn_input(0);
    assert_eq!(lines_of(&red)[4 + 5], ".....ox#....");
    let blue = g.turn_input(1);
    assert_eq!(lines_of(&blue)[4 + 5], ".....xo#....");
}

#[test]
fn buildings_and_units_are_ordered_own_first() {
    let mut g = game(League::Bronze);
    own(&mut g, 1, &[(9, 10), (9, 11), (10, 11), (11, 10)]);
    for (x, y) in [(9, 10), (9, 11), (10, 11)] {
        g.building[cell(x, y)] = Some(Building::Mine);
    }
    g.building[cell(11, 10)] = Some(Building::Tower);
    own(&mut g, 0, &[(1, 0), (0, 1)]);
    g.building[cell(1, 0)] = Some(Building::Mine);
    g.building[cell(0, 1)] = Some(Building::Tower);
    put_unit(&mut g, 4, 0, 2, 2, 1);
    put_unit(&mut g, 3, 1, 3, 10, 10);
    put_unit(&mut g, 2, 1, 2, 9, 9);
    put_unit(&mut g, 1, 0, 1, 2, 0);

    let red = g.turn_input(0);
    let tail = lines_of(&red)[16..].join("|");
    assert_eq!(
        tail,
        "8|0 0 0 0|0 1 1 0|0 2 0 1|1 0 11 11|1 1 9 10|1 1 9 11|1 1 10 11|1 2 11 10\
         |4|0 1 1 2 0|0 4 2 2 1|1 2 2 9 9|1 3 3 10 10"
    );
    let blue = g.turn_input(1);
    let tail = lines_of(&blue)[16..].join("|");
    assert_eq!(
        tail,
        "8|0 0 11 11|0 1 9 10|0 1 9 11|0 1 10 11|0 2 11 10|1 0 0 0|1 1 1 0|1 2 0 1\
         |4|0 2 2 9 9|0 3 3 10 10|1 1 1 2 0|1 4 2 2 1"
    );
}

#[test]
fn unit_ids_start_at_one_and_grow_with_every_unit_trained() {
    let mut g = game(League::Bronze);
    say(&mut g, "TRAIN 1 1 0").unwrap();
    say(&mut g, "TRAIN 1 10 11").unwrap();
    say(&mut g, "TRAIN 1 0 1").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((1, 0)));
    assert_eq!(unit_pos(&g, 2), Some((10, 11)));
    assert_eq!(unit_pos(&g, 3), Some((0, 1)));
    assert_eq!(g.units[1].owner, 1);
}

// --- TRAIN and capture ---

#[test]
fn train_needs_a_neighbour_of_an_active_cell() {
    let mut g = game(League::Bronze);
    // (5, 5) is far from any active cell: skipped, then (1, 0) works.
    say(&mut g, "TRAIN 1 5 5;TRAIN 1 1 0").unwrap();
    assert_eq!(count_units(&g, 0), 1);
    assert_eq!(unit_pos(&g, 1), Some((1, 0)));
    assert_eq!(g.owner[cell(1, 0)], Some(0));
    assert_eq!(g.gold[0], 11);
    assert_eq!(g.income[0], 2 - 1);

    // A neighbour of an inactive cell of its own is no better.
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(8, 8)]);
    say(&mut g, "TRAIN 1 8 9;TRAIN 1 8 8").unwrap();
    assert_eq!(count_units(&g, 0), 0);
    assert_eq!(g.gold[0], 21);
}

#[test]
fn train_checks_the_level_and_the_gold() {
    let mut g = game(League::Bronze);
    // Level 3 costs 30 and red has 21; level 2 costs 20.
    say(&mut g, "TRAIN 3 1 0;TRAIN 2 1 0").unwrap();
    assert_eq!(g.units[0].level, 2);
    assert_eq!(g.gold[0], 1);
    // Level 2 does not exist in Wood 3: the command is dropped while reading.
    let mut g = game(League::Wood3);
    assert_eq!(g.read_answer(0, "TRAIN 2 1 0"), Ok(vec![]));
    say(&mut g, "TRAIN 2 1 0;TRAIN 1 1 0").unwrap();
    assert_eq!(g.units[0].level, 1);
}

/// Whether a level `attacker` unit trained on a cell with a level `defender`
/// unit of the opponent takes it.
fn trained_takes(league: League, attacker: u8, defender: u8) -> bool {
    let mut g = game(league);
    put_unit(&mut g, 7, 1, defender, 1, 0);
    g.gold[0] = 100;
    // Not through `play`, since the defender, cut from its HQ, would die at
    // blue's start of turn.
    g.apply(
        0,
        Action::Train {
            level: attacker,
            cell: cell(1, 0),
        },
    );
    let took = g.owner[cell(1, 0)] == Some(0);
    assert_eq!(
        unit_pos(&g, 7).is_none(),
        took,
        "the defender dies if beaten"
    );
    took
}

#[test]
fn a_higher_level_beats_a_lower_one() {
    for (attacker, defender, takes) in [
        (1, 1, false),
        (1, 2, false),
        (1, 3, false),
        (2, 1, true),
        (2, 2, false),
        (2, 3, false),
        (3, 1, true),
        (3, 2, true),
        (3, 3, true),
    ] {
        assert_eq!(
            trained_takes(League::Bronze, attacker, defender),
            takes,
            "{attacker} against {defender}"
        );
        assert_eq!(
            trained_takes(League::Wood2, attacker, defender),
            takes,
            "{attacker} against {defender} in Wood 2"
        );
    }
}

#[test]
fn level_1_beats_level_1_only_in_wood_3() {
    assert!(trained_takes(League::Wood3, 1, 1));
    assert!(!trained_takes(League::Wood2, 1, 1));
    assert!(!trained_takes(League::Wood1, 1, 1));
    assert!(!trained_takes(League::Bronze, 1, 1));
}

#[test]
fn a_move_captures_by_level_too() {
    for (league, takes) in [(League::Wood3, true), (League::Bronze, false)] {
        let mut g = game(league);
        put_unit(&mut g, 1, 0, 1, 4, 4);
        put_unit(&mut g, 2, 1, 1, 5, 4);
        g.apply(
            0,
            Action::Move {
                id: 1,
                cell: cell(5, 4),
            },
        );
        assert_eq!(unit_pos(&g, 1), Some(if takes { (5, 4) } else { (4, 4) }));
        assert_eq!(unit_pos(&g, 2).is_none(), takes);
    }
}

#[test]
fn a_cell_with_a_unit_or_building_of_its_owner_cannot_be_entered() {
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 1, 0);
    g.gold[0] = 100;
    own(&mut g, 0, &[(0, 1)]);
    g.building[cell(0, 1)] = Some(Building::Mine);
    say(&mut g, "TRAIN 3 1 0;TRAIN 3 0 1;TRAIN 3 0 0").unwrap();
    assert_eq!(count_units(&g, 0), 1);
    assert_eq!(g.gold[0], 100);
}

#[test]
fn entering_an_opponents_building_destroys_it() {
    let mut g = game(League::Bronze);
    own(&mut g, 1, &[(1, 0)]);
    g.building[cell(1, 0)] = Some(Building::Mine);
    say(&mut g, "TRAIN 1 1 0").unwrap();
    assert_eq!(g.owner[cell(1, 0)], Some(0));
    assert_eq!(g.building[cell(1, 0)], None);
}

/// A red territory along the left and bottom edges that reaches blue's HQ
/// corner, where blue has an active tower on (10, 10) and the cell (10, 11).
fn fortress() -> IceAndFireReferee {
    let mut g = game(League::Bronze);
    let red: Vec<(usize, usize)> = (0..SIZE)
        .map(|y| (0, y))
        .chain((1..=9).map(|x| (x, 11)))
        .chain([(9, 10)])
        .collect();
    own(&mut g, 0, &red);
    own(&mut g, 1, &[(10, 11), (10, 10)]);
    g.building[cell(10, 10)] = Some(Building::Tower);
    refresh(&mut g);
    g.gold[0] = 100;
    g
}

#[test]
fn an_active_tower_protects_the_active_cells_around_it_below_level_3() {
    assert!(fortress().active[cell(10, 10)] && fortress().active[cell(9, 11)]);
    for (line, takes) in [
        ("TRAIN 1 10 11", false),
        ("TRAIN 2 10 11", false),
        ("TRAIN 3 10 11", true),
        // The tower's own cell is protected too.
        ("TRAIN 1 10 10", false),
        ("TRAIN 2 10 10", false),
        ("TRAIN 3 10 10", true),
    ] {
        let mut g = fortress();
        say(&mut g, line).unwrap();
        let (x, y) = if line.ends_with("10 11") {
            (10, 11)
        } else {
            (10, 10)
        };
        assert_eq!(g.owner[cell(x, y)] == Some(0), takes, "{line}");
    }
    // Taking the tower's cell destroys the tower.
    let mut g = fortress();
    say(&mut g, "TRAIN 3 10 10").unwrap();
    assert_eq!(g.building[cell(10, 10)], None);
    // A neutral cell next to a tower is nobody's to protect.
    let mut g = fortress();
    say(&mut g, "TRAIN 1 9 9").unwrap();
    assert_eq!(g.owner[cell(9, 9)], Some(0));
}

#[test]
fn a_tower_protects_an_hq() {
    for (level, wins) in [(1, false), (2, false), (3, true)] {
        let mut g = game(League::Bronze);
        g.building[cell(11, 10)] = Some(Building::Tower);
        put_unit(&mut g, 1, 0, level, 10, 11);
        say(&mut g, "MOVE 1 11 11").unwrap();
        assert_eq!(
            g.outcome(),
            wins.then_some(Outcome::Win(0)),
            "level {level}"
        );
    }
}

#[test]
fn an_inactive_tower_protects_nothing_but_still_blocks_its_own_cell() {
    let island = |level: u8, x: usize, y: usize, target: &str| {
        let mut g = game(League::Bronze);
        own(&mut g, 1, &[(5, 5), (5, 6)]);
        g.building[cell(5, 5)] = Some(Building::Tower);
        put_unit(&mut g, 3, 0, level, x, y);
        assert!(!g.active[cell(5, 5)] && !g.active[cell(5, 6)]);
        say(&mut g, &format!("MOVE 3 {target}")).unwrap();
        unit_pos(&g, 3)
    };
    // The cell next to the tower is inactive: a level 1 unit walks in.
    assert_eq!(island(1, 4, 6, "5 6"), Some((5, 6)));
    // The tower's own cell stays closed to levels 1 and 2, and opens to 3.
    assert_eq!(island(1, 5, 4, "5 5"), Some((5, 4)));
    assert_eq!(island(2, 5, 4, "5 5"), Some((5, 4)));
    assert_eq!(island(3, 5, 4, "5 5"), Some((5, 5)));
}

// --- MOVE ---

/// Where the unit of `seat` standing on `from` goes when told to move toward
/// `target` on an open map.
fn moved_to(seat: usize, from: (usize, usize), target: (usize, usize)) -> (usize, usize) {
    let mut g = game(League::Bronze);
    g.to_act = seat;
    put_unit(&mut g, 1, seat, 1, from.0, from.1);
    say(&mut g, &format!("MOVE 1 {} {}", target.0, target.1)).unwrap();
    unit_pos(&g, 1).unwrap()
}

#[test]
fn a_move_breaks_ties_in_each_colours_order() {
    // Up, right, down, left for red; down, left, up, right for blue.
    assert_eq!(moved_to(0, (5, 5), (7, 7)), (6, 5));
    assert_eq!(moved_to(1, (5, 5), (7, 7)), (5, 6));
    assert_eq!(moved_to(0, (5, 5), (3, 3)), (5, 4));
    assert_eq!(moved_to(1, (5, 5), (3, 3)), (4, 5));
    assert_eq!(moved_to(0, (5, 5), (7, 3)), (5, 4));
    assert_eq!(moved_to(1, (5, 5), (7, 3)), (5, 4));
    assert_eq!(moved_to(0, (5, 5), (3, 7)), (5, 6));
    assert_eq!(moved_to(1, (5, 5), (3, 7)), (5, 6));
    // A straight way has no tie.
    assert_eq!(moved_to(0, (5, 5), (5, 11)), (5, 6));
    assert_eq!(moved_to(1, (5, 5), (0, 5)), (4, 5));
}

#[test]
fn a_move_goes_to_the_nearest_neighbour_it_may_enter() {
    // The right neighbour is taken by a friend: the tie goes down.
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 5, 5);
    put_unit(&mut g, 2, 0, 1, 6, 5);
    say(&mut g, "MOVE 1 7 7").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((5, 6)));
}

#[test]
fn a_move_fails_unless_a_neighbour_is_strictly_nearer() {
    // The target is the unit's own cell.
    assert_eq!(moved_to(0, (5, 5), (5, 5)), (5, 5));
    // The only nearer neighbour holds a friend.
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 5, 5);
    put_unit(&mut g, 2, 0, 1, 5, 6);
    say(&mut g, "MOVE 1 5 8").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((5, 5)));
}

#[test]
fn a_move_to_a_blocked_neighbour_fails() {
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 5, 5);
    put_unit(&mut g, 2, 0, 1, 6, 5);
    say(&mut g, "MOVE 1 6 5").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((5, 5)));
    assert_eq!(unit_pos(&g, 2), Some((6, 5)));
    // An opponent's unit that is too strong blocks as well.
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 2, 5, 5);
    put_unit(&mut g, 2, 1, 2, 6, 5);
    say(&mut g, "MOVE 1 6 5").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((5, 5)));
}

#[test]
fn a_move_follows_shortest_paths_around_void_cells() {
    // Void at (6, 5) and (6, 6): from (5, 5) toward (7, 5) the way is up.
    let mut g = game_with_void(&[(6, 5), (6, 6)]);
    put_unit(&mut g, 1, 0, 1, 5, 5);
    say(&mut g, "MOVE 1 7 5").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((5, 4)));
    assert_eq!(g.distance(cell(5, 5), cell(7, 5)), 4);
    assert_eq!(g.distance(cell(5, 5), cell(6, 5)), UNREACHABLE);
}

#[test]
fn a_move_takes_the_cell_and_leaves_the_old_one_owned() {
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 5, 5);
    say(&mut g, "MOVE 1 6 5").unwrap();
    assert_eq!(g.owner[cell(5, 5)], Some(0));
    assert_eq!(g.owner[cell(6, 5)], Some(0));
}

#[test]
fn a_unit_moves_once_per_turn_and_not_the_turn_it_is_trained() {
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 5, 5);
    say(&mut g, "MOVE 1 9 5;MOVE 1 9 5").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((6, 5)), "one cell only");

    let mut g = game(League::Bronze);
    g.apply(
        0,
        Action::Train {
            level: 1,
            cell: cell(1, 0),
        },
    );
    // The unit exists when the line is read, but was trained this turn.
    assert_eq!(unit_pos(&g, 1), Some((1, 0)));
    say(&mut g, "MOVE 1 5 0").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((1, 0)));
}

#[test]
fn a_move_of_a_unit_trained_in_the_same_line_is_dropped_while_reading() {
    let g = game(League::Bronze);
    let line = "TRAIN 1 1 0;MOVE 1 5 0";
    assert_eq!(
        g.read_answer(0, line),
        Ok(vec![Action::Train {
            level: 1,
            cell: cell(1, 0)
        }])
    );
    let mut g = game(League::Bronze);
    say(&mut g, line).unwrap();
    assert_eq!(unit_pos(&g, 1), Some((1, 0)));
    // Next turn it moves.
    say(&mut g, "WAIT").unwrap();
    say(&mut g, "MOVE 1 5 0").unwrap();
    assert_eq!(unit_pos(&g, 1), Some((2, 0)));
}

#[test]
fn a_move_of_an_opponents_or_a_missing_unit_is_dropped_without_a_loss() {
    let mut g = game(League::Bronze);
    // Next to blue's start cell, so that it is active and survives blue's start.
    put_unit(&mut g, 4, 1, 1, 11, 9);
    assert_eq!(g.read_answer(0, "MOVE 4 11 8;MOVE 9 11 8"), Ok(vec![]));
    say(&mut g, "MOVE 4 11 8;MOVE 9 11 8").unwrap();
    assert_eq!(unit_pos(&g, 4), Some((11, 9)));
}

// --- BUILD ---

/// Whether `line` built something for red in `league`, with red owning
/// (1, 0) and (0, 1) next to its HQ and holding 100 gold.
fn builds(league: League, line: &str) -> bool {
    let mut g = game(league);
    own(&mut g, 0, &[(1, 0), (0, 1), (1, 1)]);
    g.gold[0] = 100;
    say(&mut g, line).unwrap();
    g.gold[0] == 85
}

#[test]
fn build_checks_the_league() {
    for (league, mine, tower) in [
        (League::Wood3, false, false),
        (League::Wood2, false, false),
        (League::Wood1, true, false),
        (League::Bronze, true, true),
    ] {
        assert_eq!(builds(league, "BUILD MINE 1 0"), mine, "{league:?}");
        assert_eq!(builds(league, "BUILD TOWER 1 1"), tower, "{league:?}");
    }
}

#[test]
fn a_mine_goes_on_a_mine_spot_and_a_tower_never() {
    assert!(builds(League::Bronze, "BUILD MINE 1 0"));
    assert!(!builds(League::Bronze, "BUILD MINE 1 1"));
    assert!(!builds(League::Bronze, "BUILD TOWER 1 0"));
    assert!(builds(League::Bronze, "BUILD TOWER 0 1"));
}

#[test]
fn build_needs_an_own_empty_cell_near_the_active_territory_and_15_gold() {
    // Next to the HQ but not owned.
    let mut g = game(League::Bronze);
    say(&mut g, "BUILD MINE 1 0").unwrap();
    assert_eq!(g.gold[0], 21);
    // Owned but cut from the HQ, on a mine spot.
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(5, 5)]);
    say(&mut g, "BUILD MINE 5 5").unwrap();
    assert_eq!(g.gold[0], 21);
    // A unit, or a building, is in the way.
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 1, 0);
    say(&mut g, "BUILD MINE 1 0").unwrap();
    assert_eq!(g.gold[0], 21);
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0)]);
    say(&mut g, "BUILD MINE 1 0;BUILD MINE 1 0").unwrap();
    assert_eq!(g.gold[0], 6, "built once");
    // 15 gold is enough, 14 is not.
    for (gold, built) in [(14, false), (15, true)] {
        let mut g = game(League::Bronze);
        own(&mut g, 0, &[(1, 0)]);
        g.gold[0] = gold;
        say(&mut g, "BUILD MINE 1 0").unwrap();
        assert_eq!(g.building[cell(1, 0)].is_some(), built, "{gold} gold");
    }
}

#[test]
fn a_mine_adds_to_the_income_at_once() {
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0)]);
    say(&mut g, "BUILD MINE 1 0").unwrap();
    assert_eq!(g.building[cell(1, 0)], Some(Building::Mine));
    assert_eq!(g.income[0], 2 + 4);
    // Blue destroys it by entering the cell.
    own(&mut g, 1, &[(2, 0)]);
}

// --- End of the game ---

#[test]
fn an_hq_capture_ends_the_game_in_the_middle_of_a_line() {
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 10, 11);
    say(&mut g, "MOVE 1 11 11;TRAIN 1 1 0").unwrap();
    assert_eq!(g.outcome(), Some(Outcome::Win(0)));
    assert_eq!(g.players_to_act(), Vec::<usize>::new());
    assert_eq!(count_units(&g, 0), 1, "the TRAIN did not run");
    assert_eq!(g.gold, [21, 20], "blue's start of turn did not run");
    assert_eq!(g.to_act, 0);
    assert!(say(&mut g, "WAIT").is_err());

    // Blue captures red's HQ.
    let mut g = game(League::Bronze);
    g.to_act = 1;
    put_unit(&mut g, 1, 1, 1, 0, 1);
    say(&mut g, "MOVE 1 0 0").unwrap();
    assert_eq!(g.outcome(), Some(Outcome::Win(1)));
}

#[test]
fn an_hq_survives_its_capture_and_is_not_entered_by_its_owner() {
    let mut g = game(League::Bronze);
    put_unit(&mut g, 1, 0, 1, 0, 1);
    say(&mut g, "TRAIN 1 0 0").unwrap();
    assert_eq!(count_units(&g, 0), 1, "the HQ is not empty");
    assert_eq!(g.building[0], Some(Building::Hq));
}

/// Starts the game again with the cells and units as they are: 20 gold each,
/// and red's start of turn.
fn restart(g: &mut IceAndFireReferee) {
    g.gold = [START_GOLD; 2];
    g.start_turn(0);
}

/// Plays `WAIT` until the game ends; the number of answers.
fn wait_to_the_end(g: &mut IceAndFireReferee) -> usize {
    let mut answers = 0;
    while g.outcome().is_none() {
        assert_eq!(answers % 2, g.to_act);
        say(g, "WAIT").unwrap();
        answers += 1;
        assert!(answers <= 200);
    }
    answers
}

#[test]
fn the_game_ends_after_the_hundredth_turn_of_each_player() {
    // Red earns 1 a turn and blue 2: blue wins.
    let mut g = game(League::Bronze);
    for _ in 0..199 {
        say(&mut g, "WAIT").unwrap();
    }
    assert_eq!(g.outcome(), None);
    assert_eq!(g.game_turn(), 100);
    assert_eq!(g.players_to_act(), vec![1]);
    say(&mut g, "WAIT").unwrap();
    assert_eq!(g.outcome(), Some(Outcome::Win(1)));
    assert_eq!(g.military_power(0), 20 + 100);
    assert_eq!(g.military_power(1), 20 + 200);
    assert_eq!(g.players_to_act(), Vec::<usize>::new());
    assert!(say(&mut g, "WAIT").is_err());

    // Both earn 2 a turn: a draw (provisional, see the referee's comment).
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0)]);
    restart(&mut g);
    assert_eq!(wait_to_the_end(&mut g), 200);
    assert_eq!(g.military_power(0), 20 + 200);
    assert_eq!(g.outcome(), Some(Outcome::Draw));
    assert_eq!(g.military_power(0), g.military_power(1));

    // Red earns 3 a turn: red wins.
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0), (2, 0)]);
    restart(&mut g);
    assert_eq!(wait_to_the_end(&mut g), 200);
    assert_eq!(g.outcome(), Some(Outcome::Win(0)));
}

#[test]
fn units_count_in_the_military_power_at_the_end() {
    let mut g = game(League::Bronze);
    own(&mut g, 0, &[(1, 0)]);
    put_unit(&mut g, 1, 0, 1, 1, 0);
    restart(&mut g);
    wait_to_the_end(&mut g);
    // Red earns 2 - 1 a turn, blue 2.
    assert_eq!(g.gold[0], 20 + 100);
    assert_eq!(g.military_power(0), 20 + 100 + 10);
    assert_eq!(g.outcome(), Some(Outcome::Win(1)));
}

// --- Reading the answer ---

fn verdict(line: &str) -> Result<(), InvalidAnswer> {
    say(&mut game(League::Bronze), line)
}

fn assert_fine(line: &str) {
    assert_eq!(verdict(line), Ok(()), "{line:?}");
}

fn assert_loses(line: &str) {
    let err = verdict(line).expect_err(line);
    assert_eq!(err.seat, 0, "{line:?}");
    assert!(
        err.reason.contains("unrecognised command"),
        "{line:?}: {err:?}"
    );
}

#[test]
fn wait_and_msg_are_fine() {
    for line in [
        "WAIT",
        "WAIT;",
        "WAIT;;",
        "WAIT;WAIT",
        "WAIT; WAIT",
        " WAIT ",
        "\tWAIT\t",
        "WAIT\r",
        "WAIT\u{1};",
        "MSG hi",
        "MSG hi there; WAIT",
        "MSG   spaced",
        "MSG a b c d e f g h i j k l m n o p q r s t u v w x y z",
        "WAIT;MSG hi;",
        ";",
        ";;",
    ] {
        assert_fine(line);
    }
}

#[test]
fn an_unrecognised_part_loses() {
    for line in [
        "",
        " ",
        ";WAIT",
        ";;WAIT",
        "WAIT;;WAIT",
        "WAIT; ",
        "MOVE 1 2 3; ",
        "WAIT;\t",
        "MSG",
        "MSG ",
        "MSG;WAIT",
        "wait",
        "Wait",
        "WAIT 1",
        "WAIT WAIT",
        "move 1 2 3",
        "train 1 1 0",
        "TRAIN  1 1 0",
        "TRAIN 1  1 0",
        "MOVE 1 2",
        "MOVE 1 2 3 4",
        "TRAIN 1 2",
        "MOVE\t1 2 3",
        "MOVE 1 2 x",
        "MOVE a 2 3",
        "TRAIN 1 -1 0",
        "TRAIN 1 +1 0",
        "TRAIN 1.5 1 0",
        "TRAIN 1 0x1 0",
        "BUILD",
        "BUILD MINE",
        "BUILD MINE 1",
        "BUILD HOUSE 1 1",
        "BUILD mine 1 1",
        "BUILD  MINE 1 1",
        "BUILD MINE 1 1 1",
        "MSGhi",
        "msg hi",
        "FOO",
        "WAIT,WAIT",
        "\u{ff37}AIT",
    ] {
        assert_loses(line);
    }
}

#[test]
fn integers_must_fit_32_bits_signed() {
    assert_loses("TRAIN 1 2147483648 0");
    assert_loses("MOVE 2147483648 1 1");
    assert_loses("MOVE 1 0 4294967296");
    assert_loses("BUILD MINE 99999999999999999999999999 0");
    assert_loses(&format!("MOVE 1 2 {}", "9".repeat(5000)));
    // The largest value is read, then dropped for being off the map.
    assert_fine("TRAIN 1 2147483647 0");
    assert_fine("MOVE 2147483647 2147483647 2147483647");
    assert_fine("BUILD TOWER 0 2147483647");
    // Leading zeros are digits.
    assert_fine("TRAIN 001 01 00");
}

#[test]
fn off_map_void_and_out_of_range_commands_are_dropped_without_a_loss() {
    let g = game_with_void(&[(5, 5)]);
    for line in [
        "TRAIN 1 12 0",
        "TRAIN 1 0 12",
        "TRAIN 1 5 5",
        "BUILD MINE 99 99",
        "BUILD TOWER 5 5",
        "MOVE 1 12 0",
        "MOVE 1 5 5",
        "TRAIN 0 1 0",
        "TRAIN 4 1 0",
        "TRAIN 300 1 0",
    ] {
        assert_eq!(g.read_answer(0, line), Ok(vec![]), "{line:?}");
        let mut g = game_with_void(&[(5, 5)]);
        put_unit(&mut g, 1, 0, 1, 2, 2);
        assert_eq!(say(&mut g, line), Ok(()), "{line:?}");
    }
    assert_eq!(
        g.read_answer(0, "TRAIN 1 1 0"),
        Ok(vec![Action::Train {
            level: 1,
            cell: cell(1, 0)
        }])
    );
}

#[test]
fn an_unrecognised_part_stops_the_whole_line() {
    let mut g = game(League::Bronze);
    let err = say(&mut g, "TRAIN 1 1 0;FOO").unwrap_err();
    assert_eq!(err.seat, 0);
    assert_eq!(count_units(&g, 0), 0);
    assert_eq!(g.gold[0], 21);
    assert_eq!(g.to_act, 0);
    // The reason names the part, shortened when it is long.
    assert!(err.reason.contains("FOO"));
    let long = format!("{}Z", "A".repeat(100_000));
    let err = verdict(&long).unwrap_err();
    assert!(err.reason.len() < 200, "{}", err.reason.len());
}

#[test]
fn java_split_drops_only_trailing_empty_parts() {
    assert_eq!(split_answer(""), vec![""]);
    assert_eq!(split_answer(";"), Vec::<&str>::new());
    assert_eq!(split_answer("A;;"), vec!["A"]);
    assert_eq!(split_answer(";A"), vec!["", "A"]);
    assert_eq!(split_answer("A;;B"), vec!["A", "", "B"]);
    assert_eq!(split_answer("A; "), vec!["A", " "]);
}

#[test]
fn trim_removes_characters_up_to_space() {
    assert_eq!(trim("\u{0}\t\r\n WAIT \u{1f}\u{20}"), "WAIT");
    assert_eq!(trim("\u{a0}WAIT"), "\u{a0}WAIT");
}

#[test]
fn an_answer_without_lines_or_from_the_wrong_seat_loses() {
    let mut g = game(League::Bronze);
    let err = g
        .play(&[Answer {
            seat: 0,
            lines: vec![],
        }])
        .unwrap_err();
    assert_eq!(err.seat, 0);
    let err = g
        .play(&[Answer {
            seat: 1,
            lines: vec!["WAIT".to_string()],
        }])
        .unwrap_err();
    assert_eq!(err.seat, 1);
}

#[test]
fn a_long_line_of_commands_is_read_and_carried_out() {
    let mut g = game(League::Bronze);
    let line = "WAIT;".repeat(100_000) + "TRAIN 1 1 0";
    say(&mut g, &line).unwrap();
    assert_eq!(count_units(&g, 0), 1);
}

// --- Whole games ---

fn setup(seed: u64) -> GameSetup {
    GameSetup {
        seed,
        opening_plies: 0,
    }
}

const LEAGUES: [League; 4] = [League::Wood3, League::Wood2, League::Wood1, League::Bronze];

/// Checks what must hold between two plies.
fn check_invariants(g: &IceAndFireReferee) {
    for seat in 0..2 {
        assert!(g.gold[seat] >= 0);
        assert!(g.owner[hq_cell(seat)].is_some());
        assert_eq!(g.building[hq_cell(seat)], Some(Building::Hq));
    }
    let mut seen = std::collections::HashSet::new();
    for unit in &g.units {
        assert!(!g.map.void[unit.cell]);
        assert_eq!(g.owner[unit.cell], Some(unit.owner));
        assert!(seen.insert(unit.cell), "two units on one cell");
        assert!(unit.id < g.next_id);
        assert!((1..=g.league.max_level()).contains(&unit.level));
    }
    let ids: std::collections::HashSet<u32> = g.units.iter().map(|unit| unit.id).collect();
    assert_eq!(ids.len(), g.units.len());
    let mut fresh = g.clone();
    refresh(&mut fresh);
    assert_eq!(fresh.active, g.active);
    // The input has the announced number of lines.
    for seat in 0..2 {
        let input = g.turn_input(seat);
        let lines = lines_of(&input);
        let buildings: usize = lines[16].parse().unwrap();
        let units: usize = lines[17 + buildings].parse().unwrap();
        assert_eq!(lines.len(), 18 + buildings + units);
        assert!(input.ends_with('\n'));
    }
}

#[test]
fn two_hundred_games_of_waiting_end() {
    for seed in 0..200 {
        let league = LEAGUES[(seed % 4) as usize];
        let mut g = IceAndFireReferee::with_league(&setup(seed), league);
        assert_eq!(wait_to_the_end(&mut g), 200, "seed {seed}");
        assert!(g.outcome().is_some());
        check_invariants(&g);
    }
}

/// A well-formed command line over random coordinates, some of them beside
/// the acting seat's territory.
fn random_line(rng: &mut Rng, g: &IceAndFireReferee) -> String {
    let mine: Vec<usize> = (0..CELLS)
        .filter(|&c| g.owner[c] == Some(g.to_act))
        .collect();
    let place = |rng: &mut Rng| -> (u64, u64) {
        if rng.below(10) < 7 {
            let base = mine[rng.below(mine.len() as u64) as usize];
            let (dx, dy) = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)][rng.below(5) as usize];
            let x = ((base % SIZE) as i64 + dx).clamp(0, 11) as u64;
            let y = ((base / SIZE) as i64 + dy).clamp(0, 11) as u64;
            (x, y)
        } else {
            (rng.below(14), rng.below(14))
        }
    };
    let count = 1 + rng.below(6);
    let mut parts = Vec::new();
    for _ in 0..count {
        let (x, y) = place(rng);
        parts.push(match rng.below(7) {
            0 => "WAIT".to_string(),
            1 => format!("MSG hello {}", rng.below(100)),
            2 | 3 => {
                // One of the seat's units, mostly toward the opponent's HQ.
                let own: Vec<u32> = g
                    .units
                    .iter()
                    .filter(|unit| unit.owner == g.to_act)
                    .map(|unit| unit.id)
                    .collect();
                let id = match rng.pick(&own) {
                    Some(&id) if rng.below(10) < 8 => u64::from(id),
                    _ => 1 + rng.below(u64::from(g.next_id) + 1),
                };
                let (x, y) = if rng.below(2) == 0 {
                    let hq = hq_cell(1 - g.to_act);
                    ((hq % SIZE) as u64, (hq / SIZE) as u64)
                } else {
                    (x, y)
                };
                format!("MOVE {id} {x} {y}")
            }
            4 | 5 => format!("TRAIN {} {x} {y}", rng.below(5)),
            _ => {
                let kind = if rng.below(2) == 0 { "MINE" } else { "TOWER" };
                format!("BUILD {kind} {x} {y}")
            }
        });
    }
    parts.join(";")
}

#[test]
fn two_hundred_games_of_random_commands_end() {
    let mut trained = 0;
    for seed in 0..200 {
        let league = LEAGUES[(seed % 4) as usize];
        let mut g = IceAndFireReferee::with_league(&setup(seed), league);
        let mut rng = Rng::new(seed ^ 0xFEED);
        let mut answers = 0;
        while g.outcome().is_none() {
            let line = random_line(&mut rng, &g);
            say(&mut g, &line).unwrap_or_else(|err| panic!("seed {seed}: {line:?}: {err:?}"));
            check_invariants(&g);
            answers += 1;
            assert!(answers <= 200, "seed {seed}");
        }
        trained += g.next_id - 1;
    }
    // The games must be lively: units are trained and fight.
    assert!(trained > 1000, "only {trained} units trained");
}

#[test]
fn the_same_seed_and_answers_give_the_same_game() {
    let play = || {
        let mut g = IceAndFireReferee::new(&setup(3));
        let mut rng = Rng::new(9);
        while g.outcome().is_none() {
            let line = random_line(&mut rng, &g);
            say(&mut g, &line).unwrap();
        }
        (g.outcome(), g.gold, g.turn_input(0))
    };
    assert_eq!(play(), play());
}
