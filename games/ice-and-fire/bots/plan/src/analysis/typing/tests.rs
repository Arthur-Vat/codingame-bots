use ice_and_fire_engine::State;

use super::*;
use crate::tests::position;

/// Red's column x = 0 down to (0, 10) and blue's row 11, with red's units
/// 1 at (0, 10) and 2 at (0, 6).
fn state() -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    rows[0][0] = b'O';
    for row in rows.iter_mut().take(11).skip(1) {
        row[0] = b'O';
    }
    for x in 0..12 {
        rows[11][x] = b'X';
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, [0, 0], &[(0, 1, 1, 0, 10), (0, 2, 1, 0, 6)])
}

fn block(state: &State, played: Option<&str>) -> String {
    let seat = state.to_act();
    let mut text = format!(
        "position g1 3 {}\n{}turn\n{}",
        state.game_turn(),
        state.initial_input(seat),
        state.turn_input(seat)
    );
    if let Some(line) = played {
        text.push_str(&format!("played {line}\n"));
    }
    text.push_str("end\n");
    text
}

fn line_of(played: Option<&str>) -> Result<String, String> {
    let bank = read_bank(&block(&state(), played))?;
    unit_line(&bank[0])
}

#[test]
fn the_kinds_are_those_of_the_first_move_of_each_unit() {
    let state = state();
    let typing = Typing::new(&state);
    let commands = commands_of("MOVE 1 0 11;MOVE 1 0 9;MOVE 2 0 7;TRAIN 1 1 10;BUILD MINE 0 5");
    let kinds = kinds_of(&state, &typing, &commands);
    // Unit 1 captures; its second move does not count. Unit 2 steps on.
    assert_eq!(kinds.get(&(10 * 12)), Some(&Kind::Capture));
    assert_eq!(kinds.get(&(6 * 12)), Some(&Kind::ForwardO));
    assert_eq!(kinds.len(), 2);
    // No moves, or a move of a unit that is not ours: nobody moves.
    assert!(kinds_of(&state, &typing, &commands_of("WAIT")).is_empty());
    assert!(kinds_of(&state, &typing, &commands_of("MOVE 9 3 3")).is_empty());
}

#[test]
fn a_position_line_lists_the_units_with_raw_numbers_and_kinds() {
    let line = line_of(Some("MOVE 1 0 11")).expect("a line");
    assert!(
        line.starts_with("{\"game\":\"g1\",\"turn\":10,\"units\":[["),
        "{line}"
    );
    let Json::Object(top) = parse_json(&line).expect("JSON") else {
        panic!("not an object: {line}");
    };
    let Json::List(units) = &top[2].1 else {
        panic!("no units: {line}");
    };
    assert_eq!(units.len(), 2);
    // The units come in the order of their cells: (0, 6), then (0, 10).
    let (Json::List(back), Json::List(pusher)) = (&units[0], &units[1]) else {
        panic!("{line}");
    };
    assert_eq!(back.len(), 15);
    assert_eq!(back[0], Json::Text("back-free".to_string()));
    assert_eq!(back[13], Json::Text("stay".to_string()));
    assert_eq!(pusher[0], Json::Text("pusher".to_string()));
    assert_eq!(pusher[13], Json::Text("capture".to_string()));
    // The raw numbers: level, turn band, gold band, cap, en2, own2, tw2, d.
    assert_eq!(pusher[1], Json::Number(1.0));
    assert_eq!(pusher[2], Json::Text("9-12".to_string()));
    assert_eq!(pusher[3], Json::Text("<40".to_string()));
    assert_eq!(pusher[4], Json::Number(1.0));
    assert_eq!(pusher[8], Json::Number(1.0));
    // The embedded table's top three, or none.
    let Json::List(top3) = &pusher[14] else {
        panic!("{line}");
    };
    let empty = Table::embedded().levels[0].is_empty();
    assert_eq!(top3.len(), if empty { 0 } else { 3 });
    // Without a move, the units stay.
    let waiting = line_of(Some("WAIT")).expect("a line");
    assert_eq!(waiting.matches("\"stay\"").count(), 2, "{waiting}");
    assert_eq!(
        line_of(Some(""))
            .expect("a line")
            .matches("\"stay\"")
            .count(),
        2
    );
}

#[test]
fn a_block_without_a_played_line_or_with_a_bad_state_is_an_error() {
    let err = line_of(None).expect_err("no played line");
    assert!(err.contains("no played line"), "{err}");
    assert!(read_bank("position g 1\n").is_err());
    let broken = block(&state(), Some("WAIT")).replace("turn\n", "turn\nx\n");
    let bank = read_bank(&broken).expect("a block");
    assert!(unit_line(&bank[0]).is_err());
    let dir = std::env::temp_dir().join(format!("plan-types-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let file = dir.join("bad.bank");
    std::fs::write(&file, "position a b c d\n").expect("file");
    let args = ["--positions".to_string(), file.display().to_string()];
    assert!(run_positions(&args).is_err());
    assert!(run_positions(&["--positions".to_string()]).is_err());
    assert!(run_positions(&["--nope".to_string(), "x".to_string()]).is_err());
    assert!(run_positions(&[
        "--positions".to_string(),
        dir.join("none").display().to_string()
    ])
    .is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn json_is_read_and_bad_json_is_an_error() {
    let value =
        parse_json(" {\"a\": [1, -2.5e1, \"x\\\"y\", true, null], \"b\": {}} ").expect("JSON");
    assert_eq!(
        value,
        Json::Object(vec![
            (
                "a".to_string(),
                Json::List(vec![
                    Json::Number(1.0),
                    Json::Number(-25.0),
                    Json::Text("x\"y".to_string()),
                    Json::Other,
                    Json::Other
                ])
            ),
            ("b".to_string(), Json::Object(vec![]))
        ])
    );
    for bad in [
        "",
        "[",
        "[1,]",
        "{\"a\" 1}",
        "\"abc",
        "[1] 2",
        "tru",
        "-",
        "[1 2]",
        "{1:2}",
    ] {
        assert!(parse_json(bad).is_err(), "{bad:?}");
    }
    assert!(parse_json(&"[".repeat(50)).is_err(), "too deep");
}

const SMALL: &str = r#"{"levels":[
  [[["pusher",1],[0.5,0.25,0.25,0,0,0,0,0]],[["guard",2],[0,0,0,0,0,0,0,1]]],
  [[["pusher",1,"9-12"],[0.4,0.2,0.2,0.2,0,0,0,0]]],
  [],
  [],
  []]}"#;

#[test]
fn the_builders_json_becomes_a_table() {
    let table = table_of_json(SMALL).expect("a table");
    let lens: Vec<usize> = table.levels.iter().map(Vec::len).collect();
    assert_eq!(lens, [2, 1, 0, 0, 0]);
    // Entries sum to 255, and the likeliest kinds keep their order.
    for (_, p) in table.levels.iter().flatten() {
        assert_eq!(p.iter().map(|&q| u32::from(q)).sum::<u32>(), 255);
    }
    let (_, p) = table.levels[0][0];
    assert_eq!(p[0], 127);
    assert!(p[0] > p[1] && p[1] >= p[3]);
    assert_eq!(table.levels[0][1].1, [0, 0, 0, 0, 0, 0, 0, 255]);
    // The pusher at level 1 (a level-1 unit, turn band 9-12) has 4 even kinds.
    let (_, q) = table.levels[1][0];
    assert!(q[0] > q[1] && q[1] >= q[3] && q[3] > q[4], "{q:?}");
}

#[test]
fn a_bad_table_is_an_error_naming_the_fault() {
    let cases: Vec<(String, &str)> = vec![
        ("[]".to_string(), "object"),
        ("{}".to_string(), "levels"),
        ("{\"levels\":[[],[],[]]}".to_string(), "levels"),
        (
            SMALL.replace("\"pusher\",1,\"9-12\"", "\"pusher\",1"),
            "has 2 fields",
        ),
        (SMALL.replace("\"guard\"", "\"nobody\""), "field 0"),
        (SMALL.replace("\"9-12\"", "\"soon\""), "field 2"),
        (SMALL.replace("\"guard\",2", "\"guard\",3"), "field 1"),
        (SMALL.replace("0.5,0.25", "5,0.25"), "not in 0 to 1"),
        (
            SMALL.replace("0,0,0,0,0,0,0,1", "0,0,0,0,0,0,0,0"),
            "sum to 0",
        ),
        (
            SMALL.replace("0,0,0,0,0,0,0,1", "0,0,0,0,0,0,1"),
            "7 probabilities",
        ),
        (
            SMALL.replace("0.25,0.25,0,0,0,0,0", "\"a\",0.25,0,0,0,0,0"),
            "not a number",
        ),
        (
            SMALL.replace(
                "[[\"guard\",2],[0,0,0,0,0,0,0,1]]",
                "[[\"pusher\",1],[0,0,0,0,0,0,0,1]]",
            ),
            "twice",
        ),
    ];
    for (bad, what) in &cases {
        let err = table_of_json(bad).expect_err(bad);
        assert!(err.contains(what), "{what}: {err}");
    }
}

#[test]
fn quantizing_keeps_the_sum_and_the_order_of_the_likeliest() {
    assert_eq!(quantize(&[0.0; KINDS]), None);
    assert_eq!(
        quantize(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).expect("q")[0],
        255
    );
    // Three kinds that 255 cannot tell apart from the fourth by rounding.
    let weights = [0.3, 0.2004, 0.2003, 0.2002, 0.0991, 0.0, 0.0, 0.0];
    let q = quantize(&weights).expect("quantized");
    assert_eq!(q.iter().map(|&v| u32::from(v)).sum::<u32>(), 255);
    assert!(q[1].min(q[2]) > q[3], "{q:?}");
    let mut top: Vec<usize> = likeliest(&q).iter().take(3).map(|&k| k as usize).collect();
    top.sort_unstable();
    assert_eq!(top, [0, 1, 2], "{q:?}");
}

#[test]
fn types_table_writes_a_module_the_bot_decodes() {
    let dir = std::env::temp_dir().join(format!("plan-types-table-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let (json, out) = (dir.join("t.json"), dir.join("table.rs"));
    std::fs::write(&json, SMALL).expect("json");
    let args = [
        "--json".to_string(),
        json.display().to_string(),
        "--out".to_string(),
        out.display().to_string(),
    ];
    let said = run_table(&args).expect("written");
    assert!(said.contains("keys by level 2/1/0/0/0"), "{said}");
    let module = std::fs::read_to_string(&out).expect("module");
    let packed = module
        .split('"')
        .nth(1)
        .expect("a string constant")
        .to_string();
    let table = Table::decode(&packed).expect("decoded");
    assert_eq!(table.levels[0].len(), 2);
    assert_eq!(table.levels[1].len(), 1);
    // Bad input is an error and writes nothing.
    std::fs::write(&json, "{\"levels\": 3}").expect("json");
    let before = std::fs::read_to_string(&out).expect("module");
    assert!(run_table(&args).is_err());
    assert_eq!(std::fs::read_to_string(&out).expect("module"), before);
    assert!(run_table(&args[..2]).is_err(), "no --out");
    let missing = [
        "--json".to_string(),
        dir.join("none.json").display().to_string(),
        "--out".to_string(),
        out.display().to_string(),
    ];
    assert!(run_table(&missing).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}
