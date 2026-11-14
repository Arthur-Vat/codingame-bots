use ice_and_fire_engine::{Command, League, State};

use super::*;

#[test]
fn a_line_of_the_bot_is_read() {
    let j = parse_line(
        "123\t40\texpected=-12\tactual=-80\tpass=30\tmodeled=7\tmodeled_best=25\tsame_reply=no",
    )
    .expect("read");
    assert_eq!((j.name.as_str(), j.ply), ("123", 40));
    assert_eq!((j.expected, j.actual, j.pass), (-12, -80, 30));
    assert_eq!((j.modeled, j.modeled_best, j.same_reply), (7, 25, false));
    assert!(parse_line("123\t40\texpected=1").is_err());
}

#[test]
fn the_effect_counts_cells_taken_and_units_killed() {
    // An open map: a red unit walks toward (5, 5), a blue one toward (6, 5),
    // each leaving a trail of its side's cells.
    let mut state = State::new(&[false; ice_and_fire_engine::CELLS], &[], League::Bronze);
    let unit = |s: &State, seat: usize| s.units(seat).iter().next();
    for _ in 0..40 {
        let seat = state.to_act();
        let (start, target) = if seat == 0 {
            ((1, 0), (5, 5))
        } else {
            ((10, 11), (6, 5))
        };
        let commands = match unit(&state, seat).and_then(|c| state.unit_id(c)) {
            None => vec![Command::Train {
                level: 1,
                x: start.0,
                y: start.1,
            }],
            Some(id) => vec![Command::Move {
                id,
                x: target.0,
                y: target.1,
            }],
        };
        state.play_turn(&commands);
    }
    assert_eq!(state.to_act(), 0);
    // Red passes; blue, rich by now, replies.
    state.end_turn();
    let none = effect(&state, &[], 0);
    assert_eq!(
        (none.taken, none.cut, none.units_lost, none.spent),
        (0, 0, 0, 0)
    );
    assert!(!none.hq);
    let red = unit(&state, 0).expect("a red unit");
    let (x, y) = ((red % 12) as u8, (red / 12) as u8);
    assert!(
        !(ice_and_fire_engine::Bits::cell(red).neighbours() & state.active(1)).is_empty(),
        "the units meet"
    );
    // A level-2 unit beats the red unit at the tip of red's trail.
    let hit = effect(&state, &[Command::Train { level: 2, x, y }], 0);
    assert_eq!((hit.trainings, hit.spent), ([0, 1, 0], 20));
    assert_eq!((hit.taken, hit.cut, hit.units_lost), (1, 0, 10));
    // A training needs no move: the same damage without moves.
    assert_eq!(hit.without_moves, 1);
    assert!(hit.hq_margin.is_some());
}

#[test]
fn the_csv_has_a_row_a_ply() {
    let row = Row {
        game_id: 1,
        ply: 2,
        turn: 3,
        seat: 0,
        opponent: "x".to_string(),
        top: true,
        won: false,
        turns_left: 5,
        judged: parse_line(
            "1\t2\texpected=0\tactual=-60\tpass=10\tmodeled=3\tmodeled_best=5\tsame_reply=no",
        )
        .expect("read"),
        effect: Effect::default(),
        reply: "WAIT".to_string(),
    };
    assert_eq!(row.surprise(), -60);
    let csv = rows_csv(&[row]);
    assert_eq!(csv.lines().count(), 2);
    assert!(csv.lines().nth(1).expect("a row").contains(",-60,"));
}
