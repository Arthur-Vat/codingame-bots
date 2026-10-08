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
