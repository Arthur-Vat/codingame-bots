use super::*;

/// A suite written and read back gives the same lines (whole values bare,
/// three decimals at most); a foreign header is refused.
#[test]
fn a_suite_reads_back() {
    let width = TURNS.len() * MEASURES.len() + 1;
    let line = Line {
        game_id: 42,
        kind: "test".to_string(),
        seed: 7,
        seat: 1,
        player: "zasmu".to_string(),
        opponent: "ice-and-fire-v026".to_string(),
        values: (0..width).map(|i| i as f64 / 4.0).collect(),
    };
    assert_eq!(
        (number(3.0), number(0.25)),
        ("3".to_string(), "0.250".to_string())
    );
    let text = format_suite(std::slice::from_ref(&line), "a comment\nand another");
    assert!(text.starts_with("# a comment\n# and another\n"));
    // Blank lines are skipped.
    let text = format!("{text}\n");
    assert_eq!(read_suite(&text).unwrap(), vec![line]);
    assert!(read_suite("game_id\tother\n").is_err());
}

/// A game of waits, replayed: both sides reach turn 12, and red's values
/// start with its one active cell at turn 5 (nobody moves or trains).
#[test]
fn the_values_of_a_game_of_waits() {
    let map = generate(3, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let zones = Zones::of(&state);
    let mut states = Vec::new();
    while state.game_turn() <= 12 {
        states.push(state.clone());
        state.play_turn(&[]);
    }
    let refs: Vec<&State> = states.iter().collect();
    let red = side_values(&refs, 0, &zones).unwrap();
    assert_eq!(red.len(), TURNS.len() * MEASURES.len() + 1);
    assert_eq!(red[0], 1.0);
    assert_eq!(red[3], 0.0);
    assert!(red[red.len() - 1].is_finite());
    let short: Vec<&State> = refs
        .iter()
        .copied()
        .filter(|s| s.game_turn() < 12)
        .collect();
    assert!(side_values(&short, 0, &zones).is_none());
}
