use ice_and_fire_engine::League;

use super::*;

type Unit5 = (u64, u64, u64, u64, u64);

fn setup(seed: u64) -> GameSetup {
    GameSetup {
        seed,
        opening_plies: 0,
    }
}

fn turn(seat: usize, line: &str) -> Vec<RecordedAnswer> {
    vec![RecordedAnswer {
        seat,
        lines: vec![line.to_string()],
        ms: 0.0,
    }]
}

/// A game on the map of `seed` in which the seats in `policy` play the
/// bots' greedy policy and the others wait, up to the end of the game or
/// `limit` turns.
fn policy_game(seed: u64, policy: [bool; 2], limit: usize) -> Vec<Vec<RecordedAnswer>> {
    let mut referee = IceAndFireReferee::new(&setup(seed));
    let initial = referee.initial_input(0);
    let mut turns = Vec::new();
    while referee.outcome().is_none() && turns.len() < limit {
        let seat = referee.players_to_act()[0];
        let line = if policy[seat] {
            ice_and_fire_policy::answer(
                &initial,
                &referee.turn_input(seat),
                League::Bronze,
                referee.game_turn(),
            )
        } else {
            "WAIT".to_string()
        };
        let recorded = turn(seat, &line);
        referee
            .play(&[Answer {
                seat,
                lines: recorded[0].lines.clone(),
            }])
            .unwrap();
        turns.push(recorded);
    }
    turns
}

fn frames_of(seed: u64, turns: &[Vec<RecordedAnswer>]) -> Vec<Value> {
    IceAndFire.frames(&setup(seed), turns).unwrap()
}

/// The sorted keys of a JSON object.
fn keys(value: &Value) -> Vec<&str> {
    let mut keys: Vec<&str> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    keys
}

/// The units of a frame as `(id, owner, level, x, y)`.
fn unit_list(frame: &Value) -> Vec<Unit5> {
    frame["units"]
        .as_array()
        .unwrap()
        .iter()
        .map(|unit| {
            let get = |key: &str| unit[key].as_u64().unwrap();
            (get("id"), get("owner"), get("level"), get("x"), get("y"))
        })
        .collect()
}

/// A scripted game on seed 1: red trains a unit, blue waits, red moves the
/// unit.
fn scripted() -> Vec<Vec<RecordedAnswer>> {
    vec![
        turn(0, "TRAIN 1 1 0"),
        turn(1, "WAIT"),
        turn(0, "MOVE 1 2 0"),
    ]
}

#[test]
fn info_names_the_game() {
    let info = IceAndFire.info();
    assert_eq!(info.id, "ice-and-fire");
    assert_eq!(info.name, "A Code of Ice and Fire");
    assert_eq!(IceAndFire.opening_turns(&setup(1)), 0);
}

#[test]
fn a_game_of_no_turns_is_frame_zero() {
    let frames = frames_of(1, &[]);
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0]["last"], Value::Null);
}

#[test]
fn frame_zero_on_seed_one_is_the_start() {
    let start = &frames_of(1, &[])[0];
    assert_eq!(
        start["buildings"],
        json!([
            {"owner": 0, "kind": "hq", "x": 0, "y": 0},
            {"owner": 1, "kind": "hq", "x": 11, "y": 11},
        ])
    );
    assert_eq!(start["units"], json!([]));
    // Blue also owns the cell next to its HQ, (11, 10).
    assert_eq!(start["owner"][0][0], json!(0));
    assert_eq!(start["owner"][11][11], json!(1));
    assert_eq!(start["owner"][10][11], json!(1));
    assert_eq!(start["owner"][0][1], Value::Null);
    let owned = start["owner"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row.as_array().unwrap())
        .filter(|cell| !cell.is_null())
        .count();
    assert_eq!(owned, 3);
    // The HQs and blue's extra cell are active.
    assert_eq!(start["active"][0][0], json!(true));
    assert_eq!(start["active"][10][11], json!(true));
    assert_eq!(start["active"][0][1], json!(false));
    assert_eq!(start["void"][0][3], json!(true));
    assert_eq!(start["void"][0][0], json!(false));
    // Each seat starts with 20 gold; red's start of turn has paid its income
    // of 1 already.
    assert_eq!(start["gold"], json!([21, 20]));
    assert_eq!(start["income"], json!([1, 2]));
    assert_eq!(start["military"], json!([21, 20]));
    assert_eq!(start["turn"], json!(1));
    assert_eq!(start["to_move"], json!(0));
    assert_eq!(start["last"], Value::Null);
    assert_eq!(start["result"], Value::Null);
    let spots = start["mine_spots"].as_array().unwrap();
    assert_eq!(spots.len(), 12);
    assert_eq!(spots[0], json!([1, 0]));
    assert_eq!(spots[11], json!([10, 11]));
}

#[test]
fn the_mine_spots_are_the_initial_inputs() {
    let game = IceAndFireReferee::new(&setup(7));
    let spots: Vec<Value> = mine_spots(&game)
        .iter()
        .map(|&(x, y)| json!([x, y]))
        .collect();
    assert_eq!(frames_of(7, &[])[0]["mine_spots"], json!(spots));
    assert!(game
        .initial_input(0)
        .starts_with(&format!("{}\n", spots.len())));
}

#[test]
fn a_frame_has_the_fields_of_the_format() {
    let frames = frames_of(1, &scripted());
    let fields = [
        "active",
        "buildings",
        "gold",
        "income",
        "last",
        "military",
        "mine_spots",
        "owner",
        "result",
        "to_move",
        "turn",
        "units",
        "void",
    ];
    for frame in &frames {
        assert_eq!(keys(frame), fields);
        for grid in ["void", "owner", "active"] {
            let rows = frame[grid].as_array().unwrap();
            assert_eq!(rows.len(), 12);
            assert!(rows.iter().all(|row| row.as_array().unwrap().len() == 12));
        }
    }
    assert_eq!(frames[0]["last"], Value::Null);
    for frame in &frames[1..] {
        assert_eq!(
            keys(&frame["last"]),
            ["answer", "built", "moved", "removed", "seat", "trained"]
        );
    }
    // The shapes of the entries.
    assert_eq!(
        keys(&frames[1]["last"]["trained"][0]),
        ["id", "level", "x", "y"]
    );
    assert_eq!(keys(&frames[3]["last"]["moved"][0]), ["from", "id", "to"]);
    assert_eq!(
        keys(&frames[3]["units"][0]),
        ["id", "level", "owner", "x", "y"]
    );
    assert_eq!(
        keys(&frames[3]["buildings"][0]),
        ["kind", "owner", "x", "y"]
    );
}

#[test]
fn a_wait_turn_changes_nothing_on_the_board() {
    let frames = frames_of(1, &[turn(0, "WAIT"), turn(1, "WAIT")]);
    assert_eq!(frames.len(), 3);
    assert_eq!(
        frames[1]["last"],
        json!({"seat": 0, "answer": "WAIT", "moved": [], "trained": [],
               "built": [], "removed": []})
    );
    assert_eq!(frames[1]["to_move"], json!(1));
    assert_eq!(frames[1]["turn"], json!(1));
    assert_eq!(frames[2]["last"]["seat"], json!(1));
    assert_eq!(frames[2]["to_move"], json!(0));
    assert_eq!(frames[2]["turn"], json!(2));
    assert_eq!(frames[2]["owner"], frames[0]["owner"]);
    assert_eq!(frames[2]["units"], json!([]));
}

#[test]
fn a_train_then_a_move_show_in_their_turns() {
    let frames = frames_of(1, &scripted());
    assert_eq!(frames.len(), 4);
    assert_eq!(
        frames[1]["last"],
        json!({"seat": 0, "answer": "TRAIN 1 1 0", "moved": [],
               "trained": [{"id": 1, "level": 1, "x": 1, "y": 0}],
               "built": [], "removed": []})
    );
    assert_eq!(unit_list(&frames[1]), [(1, 0, 1, 1, 0)]);
    assert_eq!(frames[1]["owner"][0][1], json!(0));
    assert_eq!(frames[2]["last"]["trained"], json!([]));
    assert_eq!(frames[2]["last"]["moved"], json!([]));
    assert_eq!(
        frames[3]["last"],
        json!({"seat": 0, "answer": "MOVE 1 2 0",
               "moved": [{"id": 1, "from": [1, 0], "to": [2, 0]}],
               "trained": [], "built": [], "removed": []})
    );
    assert_eq!(unit_list(&frames[3]), [(1, 0, 1, 2, 0)]);
    assert_eq!(frames[3]["owner"][0][2], json!(0));
}

#[test]
fn a_build_shows_as_built() {
    // Red takes the mine spot (1, 0) with a unit, steps off it, saves gold
    // while blue waits, then builds a mine there.
    let mut turns = vec![
        turn(0, "TRAIN 1 1 0"),
        turn(1, "WAIT"),
        turn(0, "MOVE 1 1 1"),
    ];
    for _ in 0..4 {
        turns.push(turn(1, "WAIT"));
        turns.push(turn(0, "WAIT"));
    }
    turns.push(turn(1, "WAIT"));
    turns.push(turn(0, "BUILD MINE 1 0;MSG hello"));
    let frames = frames_of(1, &turns);
    let before = &frames[frames.len() - 2];
    let after = frames.last().unwrap();
    assert_eq!(after["last"]["answer"], json!("BUILD MINE 1 0;MSG hello"));
    assert_eq!(
        after["last"]["built"],
        json!([{"kind": "mine", "x": 1, "y": 0}])
    );
    assert_eq!(
        after["buildings"],
        json!([
            {"owner": 0, "kind": "hq", "x": 0, "y": 0},
            {"owner": 0, "kind": "mine", "x": 1, "y": 0},
            {"owner": 1, "kind": "hq", "x": 11, "y": 11},
        ])
    );
    assert_eq!(before["last"]["built"], json!([]));
    assert_eq!(before["buildings"].as_array().unwrap().len(), 2);
}

#[test]
fn a_building_replaced_on_its_cell_is_built() {
    let empty = Position::read(&IceAndFireReferee::new(&setup(1)));
    let building = |owner, kind| Building {
        owner,
        kind,
        x: 3,
        y: 3,
    };
    let mut before = empty.clone();
    before.buildings.push(building(0, "mine"));
    let mut kept = empty.clone();
    kept.buildings.push(building(0, "mine"));
    let mut replaced = empty;
    replaced.buildings.push(building(1, "tower"));
    assert_eq!(last_json(0, "WAIT", &before, &kept)["built"], json!([]));
    assert_eq!(
        last_json(1, "WAIT", &before, &replaced)["built"],
        json!([{"kind": "tower", "x": 3, "y": 3}])
    );
}

/// `last` of `after` lists exactly the units that differ from `before`.
fn check_last_against_units(before: &Value, after: &Value) {
    let (before_units, after_units) = (unit_list(before), unit_list(after));
    let last = &after["last"];
    let ids = |key: &str| -> Vec<u64> {
        last[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|unit| unit["id"].as_u64().unwrap())
            .collect()
    };
    let old = |id: u64| before_units.iter().find(|unit| unit.0 == id);
    let new = |id: u64| after_units.iter().find(|unit| unit.0 == id);
    let trained: Vec<u64> = after_units
        .iter()
        .filter(|unit| old(unit.0).is_none())
        .map(|unit| unit.0)
        .collect();
    let moved: Vec<u64> = after_units
        .iter()
        .filter(|unit| old(unit.0).is_some_and(|old| (old.3, old.4) != (unit.3, unit.4)))
        .map(|unit| unit.0)
        .collect();
    let removed: Vec<u64> = before_units
        .iter()
        .filter(|unit| new(unit.0).is_none())
        .map(|unit| unit.0)
        .collect();
    assert_eq!(ids("trained"), trained);
    assert_eq!(ids("moved"), moved);
    assert_eq!(ids("removed"), removed);
    for gone in last["removed"].as_array().unwrap() {
        let id = gone["id"].as_u64().unwrap();
        let old = old(id).unwrap();
        let get = |key: &str| gone[key].as_u64().unwrap();
        assert_eq!((id, get("owner"), get("level"), get("x"), get("y")), *old);
    }
}

#[test]
fn units_that_are_killed_show_as_removed() {
    // Both seats play the policy: their units meet and kill each other.
    let turns = policy_game(2, [true, true], 400);
    let frames = frames_of(2, &turns);
    let mut removed = 0;
    for pair in frames.windows(2) {
        check_last_against_units(&pair[0], &pair[1]);
        removed += pair[1]["last"]["removed"].as_array().unwrap().len();
    }
    assert!(removed > 0, "the game has kills");
}

#[test]
fn a_game_won_by_hq_capture_ends_the_frames() {
    // Blue captures red's HQ on seed 1, red captures blue's on seed 2.
    for (seed, winner, (loser_x, loser_y)) in [(1, 1, (0, 0)), (2, 0, (11, 11))] {
        let turns = policy_game(seed, [true, true], 400);
        let frames = frames_of(seed, &turns);
        assert_eq!(frames.len(), 1 + turns.len());
        let (last, before) = frames.split_last().unwrap();
        assert_eq!(last["result"], json!({"winner": winner, "end": "hq"}));
        assert_eq!(last["to_move"], Value::Null);
        assert_eq!(last["owner"][loser_y][loser_x], json!(winner));
        assert_eq!(last["last"]["seat"], json!(winner));
        for frame in before {
            assert_eq!(frame["result"], Value::Null);
            assert!(frame["to_move"].is_u64());
        }
    }
}

#[test]
fn a_game_played_to_the_100th_turn_ends_by_military_power() {
    // Nobody does anything: 100 game turns, 200 answers.
    let turns = policy_game(1, [false, false], 1000);
    assert_eq!(turns.len(), 200);
    let frames = frames_of(1, &turns);
    assert_eq!(frames.len(), 201);
    // Blue's extra cell gives it the larger income, so the larger military
    // power.
    let last = &frames[200];
    assert_eq!(last["turn"], json!(100));
    assert_eq!(last["to_move"], Value::Null);
    assert_eq!(last["result"], json!({"winner": 1, "end": "turns"}));
    let military: Vec<i64> = last["military"]
        .as_array()
        .unwrap()
        .iter()
        .map(|power| power.as_i64().unwrap())
        .collect();
    assert!(military[1] > military[0]);
    // One answer earlier it is still going on.
    assert_eq!(frames[199]["to_move"], json!(1));
    assert_eq!(frames[199]["turn"], json!(100));
    assert_eq!(frames[199]["result"], Value::Null);
    // Nothing more can be played.
    let mut more = turns;
    more.push(turn(0, "WAIT"));
    assert_eq!(IceAndFire.frames(&setup(1), &more).unwrap_err().turn, 200);
}

#[test]
fn a_draw_has_no_winner_and_ends_by_turns() {
    let position = Position::read(&IceAndFireReferee::new(&setup(1)));
    assert_eq!(
        result_json(Outcome::Draw, &position),
        json!({"winner": null, "end": "turns"})
    );
    // The loser's HQ is still its own: the winner won on military power.
    assert_eq!(
        result_json(Outcome::Win(0), &position),
        json!({"winner": 0, "end": "turns"})
    );
}

#[test]
fn a_turn_the_referee_rejects_is_an_error_with_its_index() {
    let mut turns = scripted();
    turns.push(turn(1, "JUMP 3 4"));
    let error = IceAndFire.frames(&setup(1), &turns).unwrap_err();
    assert_eq!(error.turn, 3);
    assert_eq!(error.invalid.seat, 1);
    assert!(
        error.invalid.reason.contains("unrecognised command"),
        "{}",
        error.invalid.reason
    );
    // The turns before it are fine.
    assert_eq!(frames_of(1, &turns[..3]).len(), 4);
}

#[test]
fn turns_from_the_wrong_seats_are_errors() {
    let frames = |turns: &[Vec<RecordedAnswer>]| IceAndFire.frames(&setup(1), turns);
    // Blue cannot start.
    assert_eq!(frames(&[turn(1, "WAIT")]).unwrap_err().turn, 0);
    // Red cannot play twice in a row.
    assert_eq!(
        frames(&[turn(0, "WAIT"), turn(0, "WAIT")])
            .unwrap_err()
            .turn,
        1
    );
    // A turn needs exactly one answer.
    assert_eq!(frames(&[Vec::new()]).unwrap_err().turn, 0);
    let both = vec![turn(0, "WAIT")[0].clone(), turn(1, "WAIT")[0].clone()];
    assert_eq!(frames(&[both]).unwrap_err().turn, 0);
}

#[test]
fn odd_answers_never_panic() {
    let lines = [
        "",
        " ",
        ";",
        ";;WAIT;;",
        "MOVE",
        "MOVE 1 2",
        "MOVE -1 0 0",
        "MOVE 99999999999999999999 0 0",
        "MOVE 1 99 99",
        "TRAIN 4 0 1",
        "TRAIN 1 0 0",
        "BUILD TOWER 0 1",
        "BUILD MINE 11 11",
        "TRAIN 1 1 0;TRAIN 1 1 0;TRAIN 1 1 0",
        "wait",
        "\u{0}\u{1f980}",
    ];
    for line in lines {
        match IceAndFire.frames(&setup(1), &[turn(0, line)]) {
            Ok(frames) => assert_eq!(frames.len(), 2, "{line:?}"),
            Err(error) => assert_eq!(error.turn, 0, "{line:?}"),
        }
    }
    // An answer without lines is an empty answer line, which the referee
    // does not recognise.
    let no_lines = vec![RecordedAnswer {
        seat: 0,
        lines: Vec::new(),
        ms: 0.0,
    }];
    assert_eq!(
        IceAndFire.frames(&setup(1), &[no_lines]).unwrap_err().turn,
        0
    );
}

#[test]
fn the_frames_are_deterministic_for_a_seed() {
    let turns = policy_game(5, [true, true], 60);
    let first = frames_of(5, &turns);
    assert_eq!(first, frames_of(5, &turns));
    assert_eq!(first.len(), 1 + turns.len());
    assert_ne!(first[0], frames_of(6, &[])[0]);
    // A prefix of the turns gives the prefix of the frames.
    assert_eq!(frames_of(5, &turns[..10])[..], first[..11]);
}

#[test]
fn the_game_has_no_human_play() {
    assert!(!IceAndFire.human_play());
    for seat in 0..2 {
        assert!(IceAndFire
            .human_moves(&setup(1), &[], seat)
            .unwrap()
            .is_empty());
        assert!(IceAndFire
            .human_moves(&setup(1), &scripted(), seat)
            .unwrap()
            .is_empty());
    }
    // Broken turns are still an error.
    let error = IceAndFire
        .human_moves(&setup(1), &[turn(1, "WAIT")], 0)
        .unwrap_err();
    assert_eq!(error.turn, 0);
}

#[test]
fn the_referee_of_the_adapter_is_the_bronze_one() {
    let from_adapter = IceAndFire.new_referee(&setup(3));
    let bronze = IceAndFireReferee::new(&setup(3));
    assert_eq!(from_adapter.initial_input(0), bronze.initial_input(0));
    assert_eq!(from_adapter.turn_input(0), bronze.turn_input(0));
}
