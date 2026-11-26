use ice_and_fire_engine::{Command, SIZE};

use super::entered;

#[test]
fn a_reply_enters_the_cells_it_moves_onto_and_trains_on() {
    let reply = [
        Command::Move { id: 4, x: 3, y: 2 },
        Command::Train {
            level: 1,
            x: 5,
            y: 6,
        },
        Command::BuildTower { x: 7, y: 7 },
    ];
    let cells: Vec<usize> = entered(&reply).iter().collect();
    assert_eq!(cells, vec![2 * SIZE + 3, 6 * SIZE + 5]);
}
