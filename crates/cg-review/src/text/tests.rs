use super::{game_text, metrics_csv, ply_title};
use crate::test_util::game;

#[test]
fn the_game_file_names_every_ply_with_its_board_metrics_and_commands() {
    let mut trace = game(&[(1, 1), (2, 1), (2, 3), (4, 3)], Some(0));
    trace.middle = Some(2);
    let text = game_text(&trace);
    assert!(text.starts_with("# Game g1: red alpha against blue beta\n"));
    assert!(text.contains("**Result:** alpha (red) won; finished; 2 turns, 4 plies."));
    assert!(text.contains("- Red is R, blue is B."));
    assert!(text.contains("| cells | Cells held. |"));
    assert!(text.contains("- Early game: plies 0 to 1 (turns 1 to 1)."));
    assert!(text.contains("- Middle game: plies 2 to 3 (turns 2 to 2)."));
    assert!(text.contains("- Endgame: none."));
    assert!(text
        .contains("### Ply 3 · turn 2 · blue to play · middle game\n\n```text\nR. ##\n . B.\n```"));
    assert!(text.contains("| red | 4 |\n| blue | 3 |"));
    assert!(text.contains("Red played (spent 0): `WAIT`"));
    assert!(text.contains("Blue played (spent 0): (nothing)"));
    assert_eq!(
        ply_title(&trace, 0),
        "Ply 0 · turn 1 · red to play · early game"
    );
}

#[test]
fn the_csv_has_a_row_a_ply_and_a_column_a_side_and_metric() {
    let mut trace = game(&[(1, 2), (3, 4)], None);
    trace.endgame = Some(1);
    assert_eq!(
        metrics_csv(&trace),
        "ply,turn,to_act,phase,spent,red_cells,blue_cells\n0,1,red,early,0,1,2\n1,1,blue,end,0,3,4\n"
    );
}
