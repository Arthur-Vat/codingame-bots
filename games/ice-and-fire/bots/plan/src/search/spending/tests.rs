use ice_and_fire_engine::{Command, State};

use super::*;
use crate::actions::generate;
use crate::tests::position;

/// Red holds the corner block x and y 0 to 3 and has 60 gold; blue holds
/// the row 11, at game turn 10.
fn board() -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    for row in rows.iter_mut().take(4) {
        for cell in row.iter_mut().take(4) {
            *cell = b'O';
        }
    }
    for cell in rows[11].iter_mut() {
        *cell = b'X';
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, [60, 0], &[(0, 1, 1, 2, 2)])
}

fn is_buy(action: &Action) -> bool {
    matches!(
        action,
        Action::One(Command::Train { .. } | Command::BuildTower { .. } | Command::BuildMine { .. })
    )
}

#[test]
fn the_filter_removes_only_single_purchases_and_never_more_for_a_larger_k() {
    let state = board();
    let spend = Spending::new(&state);
    let mut all = Vec::new();
    generate(&state, &mut all);
    all.push(Action::Many(vec![Command::Train {
        level: 1,
        x: 0,
        y: 4,
    }]));
    let rest = |list: &[Action]| -> Vec<Vec<Command>> {
        list.iter()
            .filter(|a| !is_buy(a))
            .map(|a| a.commands().to_vec())
            .collect()
    };
    let mut previous = 0;
    for k in [1, 2, 4, 18] {
        let mut kept = all.clone();
        spend.filter(k, &mut kept);
        assert_eq!(rest(&kept), rest(&all), "k {k}");
        let left = kept.iter().filter(|a| is_buy(a)).count();
        assert!(left >= previous, "k {k}: {left} after {previous}");
        previous = left;
    }
    // All the kinds: everything stays.
    let mut kept = all.clone();
    spend.filter(18, &mut kept);
    assert_eq!(kept.len(), all.len());
}

#[test]
fn ties_put_purchases_by_share_and_leave_the_rest_at_zero() {
    let state = board();
    let spend = Spending::new(&state);
    let mut all = Vec::new();
    generate(&state, &mut all);
    for action in &all {
        let tie = spend.tie(action, Some(200));
        match action {
            Action::One(Command::Move { .. }) => {
                assert_eq!(tie, if TYPE_ORDER { 457 } else { 0 });
            }
            Action::One(command) if is_buy(action) => {
                let buy = spend.typing.buy(*command).expect("a purchase");
                assert_eq!(tie, 1 + u16::from(spend.shares()[buy as usize]));
            }
            _ => assert_eq!(tie, 0),
        }
    }
    assert_eq!(spend.tie(&Action::Many(vec![Command::Wait]), None), 0);
}
