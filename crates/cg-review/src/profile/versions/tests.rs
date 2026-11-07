use super::*;

fn snapshot(name: &str, rows: &[(u32, &str, Option<u64>)]) -> Snapshot {
    Snapshot {
        name: name.to_string(),
        date: name[..10].to_string(),
        rows: rows
            .iter()
            .map(|&(rank, nickname, agent_id)| SnapshotRow {
                rank,
                nickname: nickname.to_string(),
                agent_id,
            })
            .collect(),
    }
}

fn options(top: usize) -> Options {
    Options {
        top,
        players: Vec::new(),
    }
}

fn seen(agent_id: Option<u64>, date: &str, theirs: Option<Option<bool>>) -> Seen {
    Seen {
        agent_id,
        date: Some(date.to_string()),
        theirs,
    }
}

#[test]
fn a_player_with_two_agent_ids_is_split_into_versions_with_their_results() {
    let rows = version_rows(&[
        seen(Some(2), "2026-11-05", Some(Some(false))),
        seen(Some(1), "2026-11-03", Some(Some(true))),
        seen(Some(1), "2026-11-04", Some(Some(true))),
        seen(Some(1), "2026-11-04", Some(None)),
        seen(Some(2), "2026-11-06", None),
    ]);
    assert_eq!(rows.len(), 2);
    // The earliest first.
    assert_eq!(rows[0].agent_id, Some(1));
    assert_eq!(rows[0].games, 3);
    assert_eq!(
        (rows[0].vs_us.won, rows[0].vs_us.lost, rows[0].vs_us.drawn),
        (2, 0, 1)
    );
    assert_eq!(rows[0].first_seen.as_deref(), Some("2026-11-03"));
    assert_eq!(rows[0].last_seen.as_deref(), Some("2026-11-04"));
    // A game we were not in counts as a game but not in the results.
    assert_eq!(rows[1].games, 2);
    assert_eq!(rows[1].vs_us.games(), 1);
    assert_eq!(rows[1].last_seen.as_deref(), Some("2026-11-06"));
}

#[test]
fn games_without_an_agent_id_make_one_unnamed_version() {
    let rows = version_rows(&[
        Seen {
            agent_id: None,
            date: None,
            theirs: Some(Some(true)),
        },
        seen(Some(7), "2026-11-06", None),
    ]);
    assert_eq!(rows[0].agent_id, Some(7));
    assert_eq!(rows[1].agent_id, None);
    assert_eq!(rows[1].first_seen, None);
}

#[test]
fn a_changed_agent_id_is_flagged_against_the_previous_days_snapshot() {
    let snapshots = [
        snapshot(
            "2026-11-05T100000Z.json",
            &[
                (1, "zasmu", Some(10)),
                (2, "iaz", Some(20)),
                (30, "far", Some(5)),
            ],
        ),
        snapshot(
            "2026-11-06T100000Z.json",
            &[(1, "zasmu", Some(11)), (2, "iaz", Some(20))],
        ),
        snapshot(
            "2026-11-07T100000Z.json",
            &[(1, "iaz", Some(20)), (2, "zasmu", Some(11))],
        ),
    ];
    let v = versions(&snapshots, &options(5));
    assert_eq!(v.days, ["2026-11-05", "2026-11-06", "2026-11-07"]);
    // By rank in the newest snapshot: iaz, then zasmu; "far" is outside the
    // top 5.
    let names: Vec<&str> = v.rows.iter().map(|r| r.player.as_str()).collect();
    assert_eq!(names, ["iaz", "iaz", "iaz", "zasmu", "zasmu", "zasmu"]);
    let flags: Vec<bool> = v.rows.iter().map(|r| r.changed).collect();
    assert_eq!(flags, [false, false, false, false, true, false]);
    assert_eq!(
        v.changes,
        [Change {
            player: "zasmu".to_string(),
            from_date: "2026-11-05".to_string(),
            to_date: "2026-11-06".to_string(),
            from_agent: 10,
            to_agent: 11,
        }]
    );
}

#[test]
fn two_snapshots_of_one_day_count_once_and_unknown_ids_never_change() {
    let snapshots = [
        snapshot("2026-11-06T100000Z.json", &[(1, "zasmu", Some(10))]),
        snapshot("2026-11-06T180000Z.json", &[(1, "zasmu", Some(11))]),
        snapshot("2026-11-07T100000Z.json", &[(1, "zasmu", None)]),
        snapshot("2026-11-08T100000Z.json", &[(1, "zasmu", Some(12))]),
    ];
    let v = versions(&snapshots, &options(1));
    assert_eq!(v.days.len(), 3);
    // The later snapshot of the sixth is the day's.
    assert_eq!(v.rows[0].agent_id, Some(11));
    assert_eq!(v.rows[0].snapshot, "2026-11-06T180000Z.json");
    assert!(v.rows.iter().all(|r| !r.changed));
    assert!(v.changes.is_empty());
}

#[test]
fn named_players_replace_the_top() {
    let snapshots = [snapshot(
        "2026-11-06T100000Z.json",
        &[(1, "zasmu", Some(10)), (50, "far", Some(5))],
    )];
    let v = versions(
        &snapshots,
        &Options {
            top: 1,
            players: vec!["far".to_string(), "nobody".to_string()],
        },
    );
    assert_eq!(v.rows.len(), 1);
    assert_eq!(v.rows[0].player, "far");
    assert_eq!(v.rows[0].rank, 50);
}
