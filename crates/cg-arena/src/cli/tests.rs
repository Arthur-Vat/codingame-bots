use super::*;

#[test]
fn the_command_line_is_well_formed() {
    Cli::command().debug_assert();
}

#[test]
fn needs_two_differently_named_bots() {
    let one = vec!["a=bot".to_string()];
    assert!(two_bots(&one).unwrap_err().contains("exactly two"));
    let same = vec!["a=bot".to_string(), "a=other".to_string()];
    assert!(two_bots(&same).unwrap_err().contains("different names"));
    let ok = vec!["a=bot".to_string(), "b=bot --x".to_string()];
    assert_eq!(two_bots(&ok).unwrap()[1].args, vec!["--x".to_string()]);
}

#[test]
fn parses_each_command() {
    let cli = Cli::try_parse_from([
        "arena",
        "sprt",
        "--candidate",
        "v2=bot2",
        "--baseline",
        "v1=bot1",
        "--elo0",
        "-5",
        "--opening-plies",
        "4",
    ])
    .unwrap();
    match cli.command {
        Command::Sprt(args) => {
            assert_eq!(args.elo0, -5.0);
            assert_eq!(args.elo1, 10.0);
            assert_eq!(args.common.opening_plies, 4);
        }
        other => panic!("{other:?}"),
    }
    let cli = Cli::try_parse_from([
        "arena", "league", "--bot", "a=x", "--bot", "b=y", "--bot", "c=z",
    ])
    .unwrap();
    assert!(matches!(cli.command, Command::League(args) if args.bots.len() == 3));
}

#[test]
fn faults_can_be_expected_from_one_bot_only() {
    let cli = Cli::try_parse_from([
        "arena",
        "match",
        "--bot",
        "a=x",
        "--bot",
        "b=y",
        "--expect-no-faults-from",
        "a",
    ])
    .unwrap();
    let Command::Match(args) = cli.command else {
        panic!("not a match")
    };
    let names = ["a".to_string(), "b".to_string()];
    assert!(check_fault_name(&args.common, &names).is_ok());
    assert!(check_fault_name(&args.common, &["c".to_string()]).is_err());

    let mut summary = Summary::new(names);
    summary.faults[1].timeouts = 2;
    assert!(faults_ok(&args.common, &summary), "b's faults are b's loss");
    summary.faults[0].crashes = 1;
    assert!(!faults_ok(&args.common, &summary));
}

#[test]
fn rejects_a_non_positive_time_scale() {
    let cli = Cli::try_parse_from([
        "arena",
        "match",
        "--bot",
        "a=x",
        "--bot",
        "b=y",
        "--time-scale",
        "0",
    ])
    .unwrap();
    let Command::Match(args) = cli.command else {
        panic!("not a match")
    };
    let bots = two_bots(&args.bots).unwrap();
    assert!(tournament(bots, 1, &args.common).is_err());
}

#[test]
fn timeouts_within_the_allowed_rate_pass_the_fault_checks() {
    let cli = Cli::try_parse_from([
        "arena",
        "match",
        "--bot",
        "a=x",
        "--bot",
        "b=y",
        "--expect-no-faults",
        "--max-timeout-rate",
        "0.01",
    ])
    .unwrap();
    let Command::Match(args) = cli.command else {
        panic!("not a match")
    };
    let mut summary = Summary::new(["a".to_string(), "b".to_string()]);
    summary.wins = 1000;
    summary.faults[0].timeouts = 10;
    assert!(
        faults_ok(&args.common, &summary),
        "10 timeouts in 1,000 games"
    );
    summary.faults[1].timeouts = 11;
    assert!(!faults_ok(&args.common, &summary), "11 is too many");
    summary.faults[1].timeouts = 0;
    summary.faults[1].crashes = 1;
    assert!(!faults_ok(&args.common, &summary), "a crash still fails");
}

#[test]
fn rejects_a_timeout_rate_outside_zero_to_one() {
    for rate in ["1", "1.5", "-0.1"] {
        let cli = Cli::try_parse_from([
            "arena",
            "match",
            "--bot",
            "a=x",
            "--bot",
            "b=y",
            "--max-timeout-rate",
            rate,
        ])
        .unwrap();
        let Command::Match(args) = cli.command else {
            panic!("not a match")
        };
        let bots = two_bots(&args.bots).unwrap();
        assert!(tournament(bots, 1, &args.common).is_err(), "{rate}");
    }
}

#[test]
fn the_game_id_is_the_binary_name_without_arena() {
    assert_eq!(game_id("uttt-arena"), "uttt");
    assert_eq!(game_id("uttt"), "uttt");
}

#[test]
fn a_record_sample_needs_records() {
    let base = ["arena", "match", "--bot", "a=x", "--bot", "b=y"];
    let without = Cli::try_parse_from(base.into_iter().chain(["--records-sample", "10"]));
    assert!(without.is_err());
    let with =
        Cli::try_parse_from(
            base.into_iter()
                .chain(["--records", "dir", "--records-sample", "10"]),
        )
        .unwrap();
    let Command::Match(args) = with.command else {
        panic!("not a match")
    };
    assert_eq!(args.common.records_sample, Some(10));
    // Records built without the command line refuse the same mistake.
    let mut common = args.common;
    common.records = None;
    assert!(Records::create(&common, "uttt", "arena match").is_err());
}

fn game(pair: u32, swapped: bool) -> GameRecord {
    GameRecord {
        pair,
        swapped,
        game: crate::runner::MatchRecord {
            seed: 9,
            seats: ["a".to_string(), "b".to_string()],
            winner: Some(0),
            end: crate::runner::EndReason::Finished,
            turns: 1,
            max_answer_ms: [1.0, 0.0],
            mean_answer_ms: [1.0, 0.0],
            later_answer_ms: Default::default(),
            recorded_turns: vec![vec![crate::record::RecordedAnswer {
                seat: 0,
                lines: vec!["4 4".to_string()],
                ms: 1.0,
            }]],
        },
    }
}

#[test]
fn record_files_are_named_by_game_seed_pair_and_swap() {
    assert_eq!(
        record_file_name("uttt-5", &game(3, false)),
        "uttt-5-p3-0.json"
    );
    assert_eq!(
        record_file_name("uttt-5", &game(3, true)),
        "uttt-5-p3-1.json"
    );
}

#[test]
fn writes_parseable_record_files_in_a_new_directory() {
    let dir = std::env::temp_dir().join(format!("cg-arena-records-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let dir_arg = dir.join("nested");
    let cli = Cli::try_parse_from([
        "arena",
        "match",
        "--bot",
        "a=x",
        "--bot",
        "b=y",
        "--seed",
        "5",
        "--records",
        dir_arg.to_str().unwrap(),
        "--records-sample",
        "1",
    ])
    .unwrap();
    let Command::Match(args) = cli.command else {
        panic!("not a match")
    };
    let bots = two_bots(&args.bots).unwrap();
    let tournament = tournament(bots, 2, &args.common).unwrap();
    let mut records = Records::create(&args.common, "uttt", "arena match").unwrap();
    // A limit of 1 keeps one win (ceil(1 / 3) = 1) and drops the second.
    records.write(&tournament, &game(0, false));
    records.write(&tournament, &game(1, false));
    records.finish().unwrap();

    let mut names: Vec<String> = fs::read_dir(&dir_arg)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(names, ["uttt-5-p0-0.json"]);
    let json = fs::read_to_string(dir_arg.join(&names[0])).unwrap();
    let record: Record = serde_json::from_str(&json).unwrap();
    assert_eq!(record.format, 1);
    assert_eq!(record.game, "uttt");
    assert_eq!(record.source, "arena match");
    assert_eq!(record.turns.len(), 1);
    fs::remove_dir_all(&dir).unwrap();
}
