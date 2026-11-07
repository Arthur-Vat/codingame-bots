use cg_arena::record::{Player, PlayerKind, Record, RecordedAnswer, RECORD_FORMAT};
use cg_arena::runner::EndReason;
use ice_and_fire_engine::{Command, League, State};

use super::{board, losses, metrics, parse, side_metrics, trace, Flow, Losses};
use crate::test_util::random_record;

/// Red, to play, holds its HQ, a mine on (1, 0), a level 2 unit on (2, 0)
/// and (0, 1), and a cut-off cell (3, 1); blue holds its HQ corner.
fn position() -> State {
    let rows = [
        "OOO.........",
        "O..o........",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "...........X",
        ".........XXX",
    ];
    let mut turn = String::from("25\n4\n7\n4\n");
    for row in rows {
        turn.push_str(row);
        turn.push('\n');
    }
    turn.push_str("3\n0 0 0 0\n0 1 1 0\n1 0 11 11\n1\n0 1 2 2 0\n");
    State::read("1\n1 0\n", &turn, League::Bronze, 10).expect("a valid position")
}

#[test]
fn the_board_shows_owner_activity_and_content_in_two_characters() {
    let lines = board(&position());
    assert_eq!(lines.len(), 13);
    assert_eq!(lines[0], "    0  1  2  3  4  5  6  7  8  9 10 11");
    assert_eq!(lines[1], " 0 RH RM R2  .  .  .  .  .  .  .  .  .");
    assert_eq!(lines[2], " 1 R.  .  . r.  .  .  .  .  .  .  .  .");
    assert_eq!(lines[12], "11  .  .  .  .  .  .  .  .  . B. B. BH");
}

#[test]
fn the_metrics_follow_their_definitions() {
    let state = position();
    let keys: Vec<String> = metrics().into_iter().map(|m| m.key).collect();
    let red = side_metrics(&state, 0, &Flow::default());
    let get = |key: &str| red[keys.iter().position(|k| k == key).expect("a metric")];
    assert_eq!(red.len(), keys.len());
    assert_eq!(get("gold"), 25);
    // 4 active cells, +4 for the mine, -4 for the level 2 unit.
    assert_eq!(get("income"), 4);
    assert_eq!(get("upkeep"), 4);
    assert_eq!(get("cells"), 4);
    assert_eq!(get("cut_off"), 1);
    assert_eq!(get("units"), 1);
    assert_eq!(get("army"), 20);
    assert_eq!(get("mines"), 1);
    assert_eq!(get("towers"), 0);
    // 25 + 3 x 4 + 4 x 4 + 20.
    assert_eq!(get("material"), 73);
    // Blue's corner is far: no cut either way, a long chain to its HQ.
    assert_eq!(get("exposed30"), 0);
    assert_eq!(get("can_cut"), 0);
    assert!(get("hq_gap") > 0 && get("hq_gap") < super::NO_CHAIN);
    // Its 4 active cells and the mine, before upkeep; the one mine spot is
    // built.
    assert_eq!(get("gross_income"), 8);
    assert_eq!(get("free_spots"), 0);
    assert_eq!((get("level1"), get("level2"), get("level3")), (0, 1, 0));
    // (1, 0) joins (2, 0) to the HQ: one joint, cutting one cell off; the
    // tips of the lines are (2, 0) and (0, 1).
    assert_eq!((get("joints"), get("max_cut"), get("total_cut")), (1, 1, 1));
    assert_eq!(get("line_ends"), 2);
    assert_eq!(get("reach"), 20);
    // Every cell but the HQ touches a free cell: (1, 1), (3, 0), (2, 1) and
    // (0, 2) are room to grow.
    assert_eq!((get("frontier"), get("border"), get("room")), (0, 3, 4));
    assert_eq!(get("far_cells"), 0);
    // Distances 0, 1, 2 and 1 from the HQ: a mean of 1, times 10.
    assert_eq!(get("spread"), 10);
    // The joint holds a mine: neither empty nor guarded by a unit.
    assert_eq!((get("empty_joints"), get("guarded_joints")), (0, 0));
    // The unit on (2, 0) is far from blue, and 2 steps from the HQ.
    assert_eq!((get("frontier_units"), get("inner_units")), (0, 1));
    assert_eq!(
        (get("units_lost"), get("contact"), get("hq_guard")),
        (0, 0, 1)
    );
    assert_eq!((get("frontier_towers"), get("tower_cover")), (0, 0));
    // No last turn: the flows are 0.
    assert_eq!(
        (get("spent_units"), get("moves"), get("cells_change")),
        (0, 0, 0)
    );
}

#[test]
fn answers_parse_as_the_referee_reads_them() {
    assert_eq!(
        parse("TRAIN 1 2 3"),
        Some(Command::Train {
            level: 1,
            x: 2,
            y: 3
        })
    );
    assert_eq!(
        parse("MOVE 7 0 11"),
        Some(Command::Move { id: 7, x: 0, y: 11 })
    );
    assert_eq!(
        parse("BUILD TOWER 4 5"),
        Some(Command::BuildTower { x: 4, y: 5 })
    );
    assert_eq!(parse("WAIT"), None);
    assert_eq!(parse("TRAIN 1 2"), None);
}

fn answer(seat: usize, line: &str) -> Vec<RecordedAnswer> {
    vec![RecordedAnswer {
        seat,
        lines: vec![line.to_string()],
        ms: 1.0,
    }]
}

fn player(name: &str) -> Player {
    Player {
        name: name.to_string(),
        kind: PlayerKind::Bot,
        bot_seed: Some(1),
        command: None,
        time_scale: 1.0,
        fixed_iters: None,
    }
}

#[test]
fn a_record_replays_into_plies_with_metrics_and_what_was_spent() {
    // (1, 0), next to red's HQ, is always a mine spot and free.
    let record = Record {
        format: RECORD_FORMAT,
        game: "ice-and-fire".to_string(),
        seed: 1,
        opening_plies: 0,
        unix_time: 0,
        source: "test".to_string(),
        players: [player("alpha"), player("beta")],
        turns: vec![
            answer(0, "TRAIN 1 1 0;MSG hi"),
            answer(1, "WAIT"),
            answer(0, "WAIT"),
        ],
        end: EndReason::Aborted {
            reason: "test".to_string(),
        },
        winner: None,
        origin: None,
    };
    let t = trace("g", &record);
    assert_eq!(t.plies.len(), 3);
    assert_eq!(t.players, ["alpha".to_string(), "beta".to_string()]);
    assert_eq!(t.end, "the arena stopped the game");
    assert_eq!(t.plies[0].spent, 10);
    assert_eq!(t.plies[0].commands, "TRAIN 1 1 0;MSG hi");
    assert_eq!(t.plies[1].to_act, 1);
    assert_eq!(t.value(0, 0, "cells"), Some(1));
    assert_eq!(t.value(2, 0, "cells"), Some(2));
    assert_eq!(t.value(2, 0, "units"), Some(1));
    // Red's last turn, seen at its next ply: one training of 10 gold that
    // took a neutral cell, everything else kept.
    assert_eq!(t.value(2, 0, "spent_units"), Some(10));
    assert_eq!(t.value(2, 0, "trains"), Some(1));
    assert_eq!(t.value(2, 0, "captured"), Some(0));
    // It took one neutral cell and no enemy cell, and killed nothing.
    assert_eq!(t.value(1, 0, "cum_neutral"), Some(1));
    assert_eq!(t.value(2, 0, "cum_neutral"), Some(1));
    assert_eq!(t.value(2, 0, "cum_captured"), Some(0));
    assert_eq!(t.value(2, 0, "exchange"), Some(0));
    assert_eq!(t.value(2, 0, "cells_change"), Some(1));
    assert!(t
        .value(2, 0, "kept_share")
        .is_some_and(|k| k > 0 && k < 100));
    // Blue waited.
    assert_eq!(t.value(2, 1, "trains"), Some(0));
    assert_eq!(t.value(2, 1, "kept_share"), Some(100));
    assert!(t.plies[2].board[1].starts_with(" 0 RH R1"));
    // Too early for the middle game, the endgame or any moment.
    assert_eq!((t.middle, t.endgame), (None, None));
    assert!(t.moments.is_empty());
    assert!(!t.primer.is_empty());
}

/// Blue's cells run along row 11 from its HQ at (11, 11) to (1, 11). Red's
/// unit stands on (7, 10), a cut-off red cell; `blue_units` are lines of
/// the input's unit list, `blue_gold` blue's gold.
fn corridor(red_level: u8, blue_units: &[&str], blue_gold: i64) -> State {
    let mut turn = format!("25\n4\n{blue_gold}\n4\n");
    for y in 0..10 {
        turn.push_str(if y == 0 {
            "O...........\n"
        } else {
            "............\n"
        });
    }
    turn.push_str(".......o....\n.XXXXXXXXXXX\n");
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n");
    turn.push_str(&format!("{}\n0 1 {red_level} 7 10\n", 1 + blue_units.len()));
    for unit in blue_units {
        turn.push_str(unit);
        turn.push('\n');
    }
    State::read("1\n1 0\n", &turn, League::Bronze, 10).expect("a valid position")
}

/// The losses of `commands` played in `before`.
fn losses_of(before: &State, commands: &[Command]) -> Losses {
    let mut after = before.clone();
    after.play_turn(commands);
    losses(before, commands, &after)
}

const TAKE_CORRIDOR: [Command; 1] = [Command::Move { id: 1, x: 7, y: 11 }];

#[test]
fn units_cut_off_by_a_move_count_as_kills_of_the_mover() {
    // Taking (7, 11) cuts (1..=6, 11) off: the level 2 on (5, 11) dies at
    // blue's turn; the level 1 on (9, 11) stays connected.
    let before = corridor(1, &["1 2 2 5 11", "1 3 1 9 11"], 7);
    let got = losses_of(&before, &TAKE_CORRIDOR);
    assert_eq!(
        got,
        Losses {
            fight: 0,
            cut: 20,
            debt: 0
        }
    );
    // Blue's turn really starts without the cut-off unit.
    let mut after = before.clone();
    after.play_turn(&TAKE_CORRIDOR);
    assert_eq!(after.unit_count(1), 1);
}

#[test]
fn a_unit_taken_by_a_move_is_a_fight_kill_and_the_cut_it_causes_a_cut_kill() {
    // Level 2 enters (7, 11), where a level 1 stands: 10 taken in the fight;
    // the level 2 on (4, 11) is then cut off: 20.
    let before = corridor(2, &["1 2 1 7 11", "1 3 2 4 11"], 7);
    let got = losses_of(&before, &TAKE_CORRIDOR);
    assert_eq!(
        got,
        Losses {
            fight: 10,
            cut: 20,
            debt: 0
        }
    );
    assert_eq!(got.total(), 30);
}

#[test]
fn a_debt_wipe_out_is_a_loss_and_not_a_kill() {
    // Blue has nothing in hand and a level 3 (upkeep 20) on its 4 remaining
    // active cells: its income is negative, and all its units die.
    let before = corridor(2, &["1 2 1 7 11", "1 3 3 10 11"], 0);
    let got = losses_of(&before, &TAKE_CORRIDOR);
    assert_eq!(
        got,
        Losses {
            fight: 10,
            cut: 0,
            debt: 30
        }
    );
}

#[test]
fn nobody_dies_at_a_start_of_turn_when_the_game_ended() {
    // Red enters blue's HQ cell (11, 11) from (11, 10).
    let mut turn = String::from("25\n4\n7\n4\n");
    for y in 0..12 {
        turn.push_str(match y {
            0 => "O...........\n",
            10 => "...........o\n",
            11 => "...........X\n",
            _ => "............\n",
        });
    }
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n1\n0 1 3 11 10\n");
    let before = State::read("1\n1 0\n", &turn, League::Bronze, 10).expect("a position");
    let commands = [Command::Move {
        id: 1,
        x: 11,
        y: 11,
    }];
    let mut after = before.clone();
    after.play_turn(&commands);
    assert!(after.outcome().is_some());
    assert_eq!(losses(&before, &commands, &after), Losses::default());
}

#[test]
fn the_cumulative_metrics_add_up_in_random_games() {
    let keys: Vec<String> = metrics().into_iter().map(|m| m.key).collect();
    for seed in [3u64, 11, 29] {
        let record = random_record(seed, ["a", "b"]);
        let t = trace("r", &record);
        let get = |ply: usize, side: usize, key: &str| {
            t.plies[ply].metrics[side][keys.iter().position(|k| k == key).expect("a key")]
        };
        for side in 0..2 {
            for ply in 1..t.plies.len() {
                for key in [
                    "cum_neutral",
                    "cum_captured",
                    "cum_killed",
                    "cum_killed_fight",
                    "cum_killed_cut",
                    "cum_lost",
                ] {
                    assert!(
                        get(ply, side, key) >= get(ply - 1, side, key),
                        "{key} fell, seed {seed}"
                    );
                }
                assert_eq!(
                    get(ply, side, "cum_killed"),
                    get(ply, side, "cum_killed_fight") + get(ply, side, "cum_killed_cut")
                );
                assert_eq!(
                    get(ply, side, "exchange"),
                    get(ply, side, "cum_killed") - get(ply, side, "cum_lost")
                );
            }
        }
        // What a side lost is at least what the other killed.
        let last = t.plies.len() - 1;
        for side in 0..2 {
            assert!(get(last, 1 - side, "cum_lost") >= get(last, side, "cum_killed"));
        }
    }
}

#[test]
fn every_unit_trained_is_alive_or_lost() {
    for seed in [5u64, 17, 23, 41] {
        let record = random_record(seed, ["a", "b"]);
        let map = ice_and_fire_referee::map::generate(seed, ice_and_fire_referee::League::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let (mut trained, mut lost) = ([0i64; 2], [0i64; 2]);
        for answers in &record.turns {
            if state.outcome().is_some() {
                break;
            }
            let seat = state.to_act();
            let line = answers[0].lines[0].clone();
            let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
            // The cost of each training the rules accept.
            let mut sim = state.clone();
            for &command in &commands {
                let gold = sim.gold(seat);
                if sim.apply(command) && matches!(command, Command::Train { .. }) {
                    trained[seat] += gold - sim.gold(seat);
                }
            }
            let before = state.clone();
            state.play_turn(&commands);
            lost[1 - seat] += losses(&before, &commands, &state).total();
            if state.outcome().is_some() {
                break;
            }
            for side in 0..2 {
                let alive: i64 = state
                    .units(side)
                    .iter()
                    .map(|c| 10 * i64::from(state.unit_level(c).expect("a unit")))
                    .sum();
                assert_eq!(trained[side], alive + lost[side], "seed {seed}");
            }
        }
    }
}
