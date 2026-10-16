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
    assert_eq!(record.players[0].command.as_deref(), Some("x"));

    // A second run in the same directory must not overwrite the record.
    let mut again = Records::create(&args.common, "uttt", "arena match").unwrap();
    again.write(&tournament, &game(0, false));
    let error = again.finish().unwrap_err();
    assert!(error.contains("the file exists"), "{error}");
    assert_eq!(fs::read_to_string(dir_arg.join(&names[0])).unwrap(), json);
    fs::remove_dir_all(&dir).unwrap();
}

fn league_args(extra: &[&str]) -> LeagueArgs {
    let cli =
        Cli::try_parse_from(["arena", "league"].into_iter().chain(extra.iter().copied())).unwrap();
    match cli.command {
        Command::League(args) => args,
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_incremental_options_need_results() {
    let base = ["arena", "league", "--bot", "a=x", "--bot", "b=y"];
    let parse = |extra: &[&str]| {
        Cli::try_parse_from(base.into_iter().chain(extra.iter().copied())).map(|_| ())
    };
    parse(&["--results", "r", "--new", "a", "--new-results", "n"]).unwrap();
    assert!(parse(&["--results", "r"]).is_err(), "--new is required");
    assert!(parse(&["--results", "r", "--new", "a"]).is_err());
    assert!(parse(&["--results", "r", "--new-results", "n"]).is_err());
    for option in [
        ["--new", "a"],
        ["--new-results", "n"],
        ["--previous", "a"],
        ["--fixed-opponents", "a,b"],
        ["--nearest", "3"],
        ["--canary", "a,b"],
        ["--commit", "abc"],
    ] {
        assert!(parse(&option).is_err(), "{option:?} needs --results");
    }
    assert!(parse(&["--plan"]).is_err(), "--plan needs --results");
    // The defaults do not make the options need --results.
    let args = league_args(&["--bot", "a=x", "--bot", "b=y"]);
    assert_eq!((args.results, args.nearest), (None, 5));
    let args = league_args(&[
        "--bot",
        "a=x",
        "--results",
        "r",
        "--new",
        "a",
        "--new-results",
        "n",
        "--fixed-opponents",
        "greedy,v001",
        "--canary",
        "v002,v001",
        "--plan",
    ]);
    assert_eq!(args.fixed_opponents, ["greedy", "v001"]);
    assert_eq!(args.canary.as_deref(), Some("v002,v001"));
    assert!(args.plan);
    assert_eq!(args.nearest, 5);
}

fn stored_line(a: &str, b: &str) -> String {
    format!(
        r#"{{"a":"{a}","b":"{b}","pairs":2,"wins":2,"draws":0,"losses":2,"pair_points":[0,0,2,0,0],"faults":[0,0],"time_scale":1.0,"time_tolerance_ms":0,"opening_plies":0,"seed":7,"date":"2026-10-16","commit":"abc","cpu":"unknown"}}"#
    )
}

#[test]
fn a_canary_names_two_bots() {
    let bots = [
        BotSpec::parse("a=x").unwrap(),
        BotSpec::parse("b=y").unwrap(),
    ];
    let pairing = |a: &str, b: &str| league::parse_results(&stored_line(a, b), "f").unwrap();
    let stored = pairing("a", "b");
    assert_eq!(
        incremental::find_canary("b,a", &stored, &bots).unwrap(),
        stored[0]
    );
    let error = incremental::find_canary("a", &stored, &bots).unwrap_err();
    assert!(error.contains("A,B"), "{error}");
    let error = incremental::find_canary("a,c", &stored, &bots).unwrap_err();
    assert!(error.contains("no pairing"), "{error}");
    let stored = pairing("a", "c");
    let error = incremental::find_canary("a,c", &stored, &bots).unwrap_err();
    assert!(error.contains("no --bot"), "{error}");
}

#[cfg(unix)]
mod end_to_end {
    use std::time::Duration;

    use super::*;
    use crate::referee::{Answer, GameSetup, InvalidAnswer, Outcome, Referee, TimeLimits};

    /// Seat 0 wins when the seed is even, seat 1 when it is odd, after one
    /// turn: in a seat-swapped pair each bot wins one game.
    struct SeedDecides {
        seed: u64,
        played: bool,
    }

    impl Referee for SeedDecides {
        fn time_limits(&self) -> TimeLimits {
            TimeLimits {
                first_answer: Duration::from_secs(5),
                later_answers: Duration::from_secs(5),
            }
        }

        fn players_to_act(&self) -> Vec<usize> {
            vec![0]
        }

        fn turn_input(&self, _seat: usize) -> String {
            "go\n".to_string()
        }

        fn play(&mut self, _answers: &[Answer]) -> Result<(), InvalidAnswer> {
            self.played = true;
            Ok(())
        }

        fn outcome(&self) -> Option<Outcome> {
            self.played
                .then_some(Outcome::Win((self.seed % 2) as usize))
        }
    }

    fn new_referee(setup: &GameSetup) -> Box<dyn Referee> {
        Box::new(SeedDecides {
            seed: setup.seed,
            played: false,
        })
    }

    /// A fresh directory for one test.
    fn directory(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("cg-arena-league-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Runs an incremental league of `cat` bots named `bots`.
    fn run(dir: &Path, bots: &[&str], extra: &[&str]) -> Result<bool, String> {
        let results = dir.join("results.jsonl");
        let new_results = dir.join("new.jsonl");
        let mut line = vec![
            "--results",
            results.to_str().unwrap(),
            "--new-results",
            new_results.to_str().unwrap(),
            "--pairs",
            "2",
            "--jobs",
            "2",
            "--commit",
            "abc123",
        ];
        let bot_specs: Vec<String> = bots.iter().map(|name| format!("{name}=cat")).collect();
        for spec in &bot_specs {
            line.extend(["--bot", spec]);
        }
        line.extend(extra.iter().copied());
        run_league(league_args(&line), "uttt", &new_referee)
    }

    fn new_lines(dir: &Path) -> Vec<PairingResult> {
        league::read_results(&dir.join("new.jsonl")).unwrap()
    }

    #[test]
    fn plays_the_missing_pairings_and_nothing_twice() {
        let dir = directory("twice");
        let results = dir.join("results.jsonl");
        fs::write(&results, format!("{}\n", stored_line("greedy", "random"))).unwrap();
        let bots = ["random", "greedy", "v001"];
        let options = ["--new", "v001", "--anchor", "random"];

        assert_eq!(run(&dir, &bots, &options), Ok(true));
        let lines = new_lines(&dir);
        let opponents: Vec<&str> = lines.iter().map(|line| line.b.as_str()).collect();
        assert_eq!(opponents, ["greedy", "random"]);
        for line in &lines {
            assert_eq!(line.a, "v001");
            assert_eq!((line.pairs, line.wins, line.losses), (2, 2, 2));
            assert_eq!(line.pair_points, [0, 0, 2, 0, 0]);
            assert_eq!(line.seed, league::pairing_seed(1, "v001", &line.b));
            assert_eq!((line.commit.as_str(), line.time_scale), ("abc123", 1.0));
            assert_eq!(line.date.len(), 10);
        }

        // With the new lines copied in, a rerun plays nothing.
        let mut text = fs::read_to_string(&results).unwrap();
        text.push_str(&fs::read_to_string(dir.join("new.jsonl")).unwrap());
        fs::write(&results, text).unwrap();
        assert_eq!(run(&dir, &bots, &options), Ok(true));
        assert!(new_lines(&dir).is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn chooses_the_nearest_opponents_after_the_previous_release() {
        let dir = directory("nearest");
        let stored = [("greedy", "random"), ("v002", "random")];
        let text: Vec<String> = stored.iter().map(|(a, b)| stored_line(a, b)).collect();
        fs::write(dir.join("results.jsonl"), text.join("\n")).unwrap();
        let bots = ["random", "greedy", "v002", "v001"];
        let options = [
            "--new",
            "v001",
            "--previous",
            "greedy",
            "--nearest",
            "1",
            "--anchor",
            "random",
        ];
        assert_eq!(run(&dir, &bots, &options), Ok(true));
        let opponents: Vec<String> = new_lines(&dir).into_iter().map(|line| line.b).collect();
        assert_eq!(opponents, ["greedy", "random"], "ties go to the first name");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn plans_without_playing_and_replays_a_canary_without_storing_it() {
        let dir = directory("canary");
        let results = dir.join("results.jsonl");
        fs::write(&results, format!("{}\n", stored_line("greedy", "random"))).unwrap();
        let bots = ["random", "greedy", "v001"];
        let options = ["--new", "v001", "--canary", "random,greedy"];

        let mut plan = options.to_vec();
        plan.push("--plan");
        assert_eq!(run(&dir, &bots, &plan), Ok(true));
        assert!(!dir.join("new.jsonl").exists(), "a plan creates nothing");

        assert_eq!(run(&dir, &bots, &options), Ok(true));
        let opponents: Vec<String> = new_lines(&dir).into_iter().map(|line| line.b).collect();
        assert_eq!(opponents, ["greedy", "random"], "the canary is not stored");

        // A release that has pairings plays none, but the canary still runs.
        let mut text = fs::read_to_string(&results).unwrap();
        text.push_str(&format!("{}\n", stored_line("v001", "random")));
        fs::write(&results, text).unwrap();
        assert_eq!(run(&dir, &bots, &options), Ok(true));
        assert!(new_lines(&dir).is_empty());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn every_bot_of_the_file_needs_a_command_before_any_game() {
        let dir = directory("commands");
        let stored = [("greedy", "random"), ("v002", "random")];
        let text: Vec<String> = stored.iter().map(|(a, b)| stored_line(a, b)).collect();
        fs::write(dir.join("results.jsonl"), text.join("\n")).unwrap();
        // v002 is not the previous release, fixed or necessarily nearest, yet
        // it may be chosen: nothing is played without a command for it.
        let bots = ["random", "greedy", "v001"];
        let options = ["--new", "v001", "--previous", "greedy", "--nearest", "1"];
        let error = run(&dir, &bots, &options).unwrap_err();
        assert!(
            error.contains("\"v002\" is in the results file but has no --bot"),
            "{error}"
        );
        assert!(
            error.contains("give a --bot for every bot of the file"),
            "{error}"
        );
        assert!(!dir.join("new.jsonl").exists(), "no game was played");
        let mut plan = options.to_vec();
        plan.push("--plan");
        assert!(run(&dir, &bots, &plan).unwrap_err().contains("\"v002\""));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn an_empty_fixed_opponent_means_none() {
        let dir = directory("nofixed");
        let results = dir.join("results.jsonl");
        fs::write(&results, format!("{}\n", stored_line("greedy", "random"))).unwrap();
        let bots = ["random", "greedy", "v001"];
        let options = ["--new", "v001", "--fixed-opponents", ""];
        assert_eq!(run(&dir, &bots, &options), Ok(true));
        assert_eq!(new_lines(&dir).len(), 2);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reports_mistakes_before_playing() {
        let dir = directory("errors");
        let results = dir.join("results.jsonl");
        fs::write(&results, format!("{}\n", stored_line("greedy", "random"))).unwrap();
        let bots = ["random", "v001"];
        // greedy is in the file but has no --bot.
        let error = run(&dir, &bots, &["--new", "v001"]).unwrap_err();
        assert!(error.contains("no --bot"), "{error}");
        let error = run(&dir, &bots, &["--new", "v002"]).unwrap_err();
        assert!(error.contains("--new"), "{error}");
        let error = run(&dir, &bots, &["--new", "v001", "--previous", "v009"]).unwrap_err();
        assert!(error.contains("--previous"), "{error}");
        let error = run(&dir, &bots, &["--new", "v001", "--anchor", "v009"]).unwrap_err();
        assert!(error.contains("--anchor"), "{error}");
        let error = run(&dir, &bots, &["--new", "v001", "--canary", "v001,random"]).unwrap_err();
        assert!(error.contains("--canary"), "{error}");

        let same = results.to_str().unwrap();
        let args = league_args(&[
            "--bot",
            "v001=cat",
            "--bot",
            "random=cat",
            "--bot",
            "greedy=cat",
            "--results",
            same,
            "--new",
            "v001",
            "--new-results",
            same,
        ]);
        let error = run_league(args, "uttt", &new_referee).unwrap_err();
        assert!(error.contains("must not be"), "{error}");
        assert!(fs::read_to_string(&results).unwrap().contains("greedy"));

        fs::write(&results, "{oops\n").unwrap();
        let error = run(&dir, &bots, &["--new", "v001"]).unwrap_err();
        assert!(error.contains("results.jsonl:1:"), "{error}");
        fs::remove_dir_all(&dir).unwrap();
    }
}

mod leaderboard_command {
    use super::*;

    fn leaderboard_args(extra: &[&str]) -> Result<LeaderboardArgs, clap::Error> {
        let base = ["arena", "leaderboard"];
        let cli = Cli::try_parse_from(base.into_iter().chain(extra.iter().copied()))?;
        match cli.command {
            Command::Leaderboard(args) => Ok(args),
            other => panic!("{other:?}"),
        }
    }

    /// A fresh directory with a `releases` folder for one test.
    fn directory(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cg-arena-leaderboard-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("releases")).unwrap();
        dir
    }

    fn line(a: &str, b: &str, wins: u32, losses: u32) -> String {
        let pairs = (wins + losses) / 2;
        format!(
            r#"{{"a":"{a}","b":"{b}","pairs":{pairs},"wins":{wins},"draws":0,"losses":{losses},"pair_points":[0,0,{pairs},0,0],"faults":[0,0],"time_scale":1.0,"time_tolerance_ms":0,"opening_plies":4,"seed":1,"date":"2026-10-16","commit":"abc","cpu":"unknown"}}"#
        )
    }

    /// A folder with a results file, a journal and the release `v001`.
    fn setup(name: &str) -> PathBuf {
        let dir = directory(name);
        let results = [
            line("v001", "random", 150, 50),
            line("greedy", "random", 120, 80),
        ]
        .join("\n");
        fs::write(dir.join("league.jsonl"), results + "\n").unwrap();
        fs::write(
            dir.join("journal.md"),
            "| [E001](E001-mcts.md) | 2026-10-07 | MCTS | Accepted | Promoted: `uttt-v001` |\n",
        )
        .unwrap();
        fs::write(dir.join("releases/uttt-v001.rs"), "fn main() {}\n").unwrap();
        dir
    }

    fn run(dir: &Path, anchor: &str, extra: &[&str]) -> Result<bool, String> {
        let results = dir.join("league.jsonl");
        let journal = dir.join("journal.md");
        let mut line = vec![
            "--results",
            results.to_str().unwrap(),
            "--anchor",
            anchor,
            "--journal",
            journal.to_str().unwrap(),
        ];
        line.extend(extra.iter().copied());
        run_leaderboard(&leaderboard_args(&line).unwrap(), "uttt", "uttt-arena")
    }

    #[test]
    fn writes_the_file_that_check_then_accepts() {
        let dir = setup("write");
        let out = dir.join("LEADERBOARD.md");
        let out_arg = out.to_str().unwrap();
        assert_eq!(run(&dir, "random", &["--out", out_arg]), Ok(true));
        let text = fs::read_to_string(&out).unwrap();
        assert!(text.starts_with("# Leaderboard\n\n"), "{text}");
        assert!(text.ends_with("|\n") && !text.contains('\r'), "{text:?}");
        assert!(
            text.contains("| [v001](releases/uttt-v001.rs) | +"),
            "{text}"
        );
        assert!(text.contains("| [E001](journal/E001-mcts.md) | 2026-10-07 |"));
        assert!(text.contains("| greedy | +"));
        assert!(text.contains("| random | 0.0 | anchor | 400 | baseline | |"));
        assert_eq!(run(&dir, "random", &["--check", out_arg]), Ok(true));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn check_fails_on_a_different_or_missing_file() {
        let dir = setup("check");
        let out = dir.join("LEADERBOARD.md");
        let out_arg = out.to_str().unwrap();
        assert_eq!(run(&dir, "random", &["--check", out_arg]), Ok(false));
        assert!(!out.exists(), "--check must not write");

        assert_eq!(run(&dir, "random", &["--out", out_arg]), Ok(true));
        let mut edited = fs::read_to_string(&out).unwrap();
        edited.push('\n');
        fs::write(&out, edited).unwrap();
        assert_eq!(run(&dir, "random", &["--check", out_arg]), Ok(false));
        fs::write(&out, "").unwrap();
        assert_eq!(run(&dir, "random", &["--check", out_arg]), Ok(false));

        // A changed results file no longer matches the committed leaderboard.
        assert_eq!(run(&dir, "random", &["--out", out_arg]), Ok(true));
        let results = dir.join("league.jsonl");
        let mut text = fs::read_to_string(&results).unwrap();
        text.push_str(&line("v001", "greedy", 140, 60));
        text.push('\n');
        fs::write(&results, text).unwrap();
        assert_eq!(run(&dir, "random", &["--check", out_arg]), Ok(false));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn out_and_check_conflict_and_the_rest_is_required() {
        let base = ["--results", "r", "--anchor", "random", "--journal", "j"];
        let with = |extra: &[&str]| {
            let all: Vec<&str> = base.iter().chain(extra).copied().collect();
            leaderboard_args(&all)
        };
        assert!(with(&["--out", "a", "--check", "b"]).is_err());
        assert!(with(&["--out", "a"]).is_ok());
        assert!(with(&["--check", "b"]).is_ok());
        assert!(with(&[]).is_ok());
        assert!(leaderboard_args(&["--anchor", "random", "--journal", "j"]).is_err());
        assert!(leaderboard_args(&["--results", "r", "--journal", "j"]).is_err());
        assert!(leaderboard_args(&["--results", "r", "--anchor", "random"]).is_err());
    }

    #[test]
    fn rejects_an_anchor_that_is_not_in_the_file_but_accepts_an_empty_file() {
        let dir = setup("anchor");
        let error = run(&dir, "nobody", &[]).unwrap_err();
        assert!(
            error.contains("--anchor") && error.contains("nobody"),
            "{error}"
        );

        fs::write(dir.join("league.jsonl"), "").unwrap();
        let out = dir.join("LEADERBOARD.md");
        assert_eq!(
            run(&dir, "nobody", &["--out", out.to_str().unwrap()]),
            Ok(true)
        );
        let text = fs::read_to_string(&out).unwrap();
        assert!(text.contains("No pairings yet."), "{text}");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reports_a_bad_results_file_or_journal() {
        let dir = setup("errors");
        let results = fs::read_to_string(dir.join("league.jsonl")).unwrap();
        fs::write(dir.join("league.jsonl"), "{oops\n").unwrap();
        let error = run(&dir, "random", &[]).unwrap_err();
        assert!(error.contains("league.jsonl:1:"), "{error}");
        fs::write(dir.join("league.jsonl"), results).unwrap();

        fs::write(
            dir.join("journal.md"),
            "| [E001](a.md) | 2026-10-07 | A | B | Promoted: `uttt-v001` |\n\
             | [E002](b.md) | 2026-10-07 | A | B | Promoted: `uttt-v001` |\n",
        )
        .unwrap();
        let error = run(&dir, "random", &[]).unwrap_err();
        assert!(error.contains("twice"), "{error}");

        fs::remove_file(dir.join("journal.md")).unwrap();
        let error = run(&dir, "random", &[]).unwrap_err();
        assert!(error.contains("cannot read"), "{error}");
        fs::remove_dir_all(&dir).unwrap();
    }
}
