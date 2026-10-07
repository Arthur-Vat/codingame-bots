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
