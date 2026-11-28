use ice_and_fire_engine::State;

use super::*;
use crate::tests::position;

fn state() -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    rows[0][0] = b'O';
    for row in rows.iter_mut().take(9).skip(1) {
        row[0] = b'O';
    }
    for cell in rows[11].iter_mut() {
        *cell = b'X';
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, [0, 0], &[(0, 1, 1, 0, 8)])
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
    buy_line(&bank[0])
}

#[test]
fn a_position_line_lists_the_purchases_and_not_the_moves() {
    let line = line_of(Some(
        "MOVE 1 0 9;TRAIN 1 3 11;TRAIN 2 3 10;BUILD TOWER 0 3;BUILD MINE 4 4",
    ))
    .expect("a line");
    assert!(
        line.contains("\"band\":\"9-12\"") && line.contains("\"gold\":\"<40\""),
        "{line}"
    );
    assert!(
        line.ends_with("\"buys\":[\"t1-capture\",\"t2-front\",\"tower-joint\",\"mine\"]}"),
        "{line}"
    );
    let none = line_of(Some("WAIT")).expect("a line");
    assert!(none.ends_with("\"buys\":[]}"), "{none}");
    assert!(line_of(None).is_err());
}

#[test]
fn quantizing_sums_to_255_and_keeps_the_order() {
    let q = quantize(&[0.5, 0.3, 0.2, 0.0]);
    assert_eq!(q.iter().map(|&v| u32::from(v)).sum::<u32>(), 255);
    assert!(q[0] > q[1] && q[1] > q[2] && q[3] == 0, "{q:?}");
    assert_eq!(quantize(&[0.0, 0.0]), vec![0, 0]);
}

fn json(rows: usize) -> String {
    let mut text = String::from("{\"rows\":[");
    for i in 0..rows {
        if i > 0 {
            text.push(',');
        }
        // Nothing 0.25, then a quarter and three quarters on two kinds.
        let mut row = vec!["0.25".to_string()];
        row.extend((0..BUYS).map(|k| match k {
            1 => "0.25".to_string(),
            2 => "0.75".to_string(),
            _ => "0".to_string(),
        }));
        text.push_str(&format!("[{}]", row.join(",")));
    }
    text.push_str("]}");
    text
}

#[test]
fn the_table_round_trips_through_the_packed_module() {
    let dir = std::env::temp_dir().join(format!("plan-spending-table-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let (path, out) = (dir.join("t.json"), dir.join("table.rs"));
    std::fs::write(&path, json(ROWS)).expect("json");
    let args = [
        "--json".to_string(),
        path.display().to_string(),
        "--out".to_string(),
        out.display().to_string(),
    ];
    let said = run_table(&args).expect("written");
    assert!(said.contains("20 rows; 380 bytes"), "{said}");
    let module = std::fs::read_to_string(&out).expect("module");
    let packed = module.split('"').nth(1).expect("a string constant");
    let table = SpendTable::decode(packed).expect("decoded");
    assert_eq!(table, table_of_json(&json(ROWS)).expect("table"));
    for row in &table.rows {
        assert_eq!(row[BUYS], 64);
        assert_eq!(u32::from(row[1]) + u32::from(row[2]), 255);
        assert!(row[2] > row[1]);
    }
    // Bad input writes nothing.
    std::fs::write(&path, json(3)).expect("json");
    let before = std::fs::read_to_string(&out).expect("module");
    assert!(run_table(&args).is_err());
    assert_eq!(std::fs::read_to_string(&out).expect("module"), before);
    assert!(run_table(&args[..2]).is_err(), "no --out");
    std::fs::remove_dir_all(&dir).ok();
}
