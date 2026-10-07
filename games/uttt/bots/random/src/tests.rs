use super::*;

fn run(seed: u64, input: &str) -> (Result<(), InputError>, String) {
    let mut output = Vec::new();
    let result = play(
        &mut Rng::new(seed),
        Input::new(input.as_bytes()),
        &mut output,
    );
    (result, String::from_utf8(output).expect("moves are ASCII"))
}

#[test]
fn plays_a_valid_action_each_turn_until_the_input_ends() {
    let input = "-1 -1\n3\n4 4\n0 0\n8 8\n4 4\n2\n3 3\n3 4\n";
    let (result, output) = run(1, input);
    assert_eq!(result, Err(InputError::Eof));
    let moves: Vec<&str> = output.lines().collect();
    assert_eq!(moves.len(), 2);
    assert!(["4 4", "0 0", "8 8"].contains(&moves[0]), "{output}");
    assert!(["3 3", "3 4"].contains(&moves[1]), "{output}");
}

#[test]
fn same_seed_same_moves() {
    let input = "-1 -1\n5\n0 0\n1 1\n2 2\n3 3\n4 4\n";
    assert_eq!(run(9, input).1, run(9, input).1);
}

#[test]
fn every_action_gets_played_sometimes() {
    let input = "-1 -1\n3\n0 0\n1 1\n2 2\n";
    let mut seen = std::collections::HashSet::new();
    for seed in 0..50 {
        seen.insert(run(seed, input).1);
    }
    assert_eq!(seen.len(), 3, "{seen:?}");
}

#[test]
fn rejects_a_turn_without_actions() {
    let (result, _) = run(1, "-1 -1\n0\n");
    assert!(matches!(result, Err(InputError::Parse { .. })));
}
