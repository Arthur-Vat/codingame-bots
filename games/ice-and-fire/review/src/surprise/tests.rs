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
        shape: Shape::default(),
        reply: "WAIT".to_string(),
    };
    assert_eq!(row.surprise(), -60);
    let csv = rows_csv(&[row]);
    assert_eq!(csv.lines().count(), 2);
    assert!(csv.lines().nth(1).expect("a row").contains(",-60,"));
}

#[test]
fn the_report_has_the_row_of_the_release() {
    // Four plies of two games, one lost: a large miss, a large removal the
    // search foresaw (cells taken and cut off together), a quiet ply, and
    // a threat on our HQ the search did not foresee, which is not a normal
    // ply: it counts among all plies only.
    let row = |game_id, won, actual: i64, taken, cut| Row {
        game_id,
        ply: 2,
        turn: 9,
        seat: 0,
        opponent: "x".to_string(),
        top: false,
        won,
        turns_left: 5,
        judged: parse_line(&format!(
            "1\t2\texpected=0\tactual={actual}\tpass=10\tmodeled=3\tmodeled_best=5\tsame_reply=no"
        ))
        .expect("read"),
        effect: Effect {
            taken,
            cut,
            ..Effect::default()
        },
        shape: Shape::default(),
        reply: "WAIT".to_string(),
    };
    let rows = [
        row(1, false, -60, 0, 12),
        row(1, false, -5, 4, 6),
        row(2, true, 4, 0, 0),
        row(1, false, -THREAT, 0, 20),
    ];
    assert_eq!(rows[3].kind(), Kind::ThreatMissed);
    let request = Request {
        data: "data".into(),
        me: "me".to_string(),
        judge: "judge".into(),
        release: Some("ice-and-fire-v026".to_string()),
        top: 10,
        tries: 600,
        limit: None,
        jobs: 1,
        leaderboard: None,
        out: "out".into(),
        pass: false,
    };
    // A third game was read and had no judged ply: it is not counted.
    let md = report(&rows, &request, 3, 0, 0);
    assert!(
        md.contains("| ice-and-fire-v026 | 2 | 50% | 4 | 3 | 1 (33.3%) | 1 (33.3%) | 1 (25.0%) |"),
        "{md}"
    );
}

#[test]
fn the_shape_of_a_capture_at_the_tip_of_a_line() {
    // As in the effect test: a red trail ends at a unit next to blue's.
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
    let before = state.clone();
    state.end_turn();
    let red = unit(&state, 0).expect("a red unit");
    let (x, y) = ((red % 12) as u8, (red / 12) as u8);
    let s = shape(&before, &state, &[Command::Train { level: 2, x, y }], 0);
    // The tip of a line is no joint; the trail's other cells are.
    assert_eq!(s.joint_hit, 0);
    assert!(s.joints > 0);
    assert_eq!(
        (s.towers, s.towers_lost, s.move_captures, s.new_lost),
        (0, 0, 0, 0)
    );
    assert_eq!(s.depth, 1);
    assert!(s.hq_steps > 5);
    assert_eq!(s.opp_gold, state.gold(1));
}
