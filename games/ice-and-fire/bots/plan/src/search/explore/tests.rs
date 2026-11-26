use ice_and_fire_engine::Command;

use super::{is_purchase, perturbations};
use crate::actions::Action;

#[test]
fn perturbations_drop_the_purchases_then_each_command() {
    let step = Command::Move { id: 3, x: 4, y: 5 };
    let train = Command::Train {
        level: 1,
        x: 5,
        y: 5,
    };
    let tower = Command::BuildTower { x: 4, y: 4 };
    let plan = [step, train, tower];
    assert!(is_purchase(&Action::One(train)));
    assert!(!is_purchase(&Action::One(step)));
    let starts = perturbations(&plan, 3);
    // The first to try is last.
    let commands: Vec<Vec<Command>> = starts.iter().rev().map(|a| a.commands().to_vec()).collect();
    assert_eq!(
        commands,
        vec![vec![step], vec![train, tower], vec![step, tower]]
    );
    // A plan of moves alone has no start without purchases.
    assert_eq!(perturbations(&[step], 3).len(), 0);
}
