use uttt_engine::MoveList;

use super::*;

#[test]
fn games_are_reproducible_and_record_searched_positions() {
    let settings = SelfPlay {
        iterations: 300,
        opening_plies: 4,
        opening: Opening::Random,
        opening_temperature: 1.0,
        balance: None,
        exploration: 0.5,
        expand_all: false,
        minimax_weight: 0.0,
        network: None,
        policy: None,
        patterns: None,
        record_visits: true,
    };
    let game = settings.play(11).unwrap();
    assert_eq!(Some(&game), settings.play(11).as_ref());
    assert_ne!(Some(&game), settings.play(12).as_ref());
    let mut board = Board::new();
    for &mv in &game.moves {
        assert!(board.is_legal(mv));
        board.play(mv);
    }
    assert_ne!(board.status(), Status::Ongoing);
    assert!(!game.searched.is_empty());
    for (position, searched) in game.positions() {
        assert!(searched.ply >= 4, "openings are not searched");
        assert!(position.game_winning_move().is_none());
        let score = searched.score.unwrap();
        assert!((0.0..=1.0).contains(&score), "{score}");
        let mut legal = MoveList::new();
        position.legal_moves(&mut legal);
        assert_eq!(searched.visits.len(), legal.len());
        assert!(searched.visits.iter().all(|(mv, _)| legal.contains(mv)));
        // The move played is the most visited one.
        let best = searched.visits.iter().max_by_key(|(_, n)| *n).unwrap().0;
        assert_eq!(best, game.moves[usize::from(searched.ply)]);
    }
}

/// E011's class weights (`uttt-v008`), as the Train workflow reads them.
const CLASS_WEIGHTS: &str = include_str!("../../../training/class_weights.rs");

#[test]
fn class_weights_are_read_from_their_source() {
    let weights = read_policy_weights(CLASS_WEIGHTS).unwrap();
    assert_eq!(weights[0], 800);
    assert_eq!(weights[1], 10_000);
    assert_eq!(weights[31], 1070);
    assert!(read_policy_weights("no weights here").is_err());
    let short = "pub const PLAYOUT_WEIGHTS: [u32; 32] = [1, 2, 3];";
    assert_eq!(
        read_policy_weights(short).unwrap_err(),
        "3 weights instead of 32"
    );
}

#[test]
fn games_with_class_policy_playouts_can_skip_visits() {
    let weights = read_policy_weights(CLASS_WEIGHTS).unwrap();
    let policy: &'static PlayoutPolicy =
        Box::leak(Box::new(PlayoutPolicy::new(weights).for_plies(16)));
    let settings = SelfPlay {
        iterations: 300,
        opening_plies: 6,
        opening: Opening::Random,
        opening_temperature: 1.0,
        balance: None,
        exploration: 0.5,
        expand_all: false,
        minimax_weight: 0.0,
        network: None,
        policy: Some(policy),
        patterns: None,
        record_visits: false,
    };
    let game = settings.play(3).unwrap();
    assert_eq!(Some(&game), settings.play(3).as_ref());
    assert_ne!(game.status(), Status::Ongoing);
    assert!(!game.searched.is_empty());
    for searched in &game.searched {
        assert!(searched.visits.is_empty());
        assert!((0.0..=1.0).contains(&searched.score.unwrap()));
    }
}

/// The pattern weights of `uttt-v010` to `uttt-v012`, as the Train workflow reads them.
const BOT_PATTERNS: &str = include_str!("../../../training/pattern_weights.rs");

#[test]
fn the_bot_pattern_weights_are_read_from_its_source() {
    let policy = read_pattern_policy(BOT_PATTERNS).unwrap();
    // A pattern file's text gives the same policy as its packed form.
    let released = include_str!("../../../releases/uttt-v010.rs");
    assert_eq!(read_pattern_policy(released).unwrap(), policy);
    let digits = "A".repeat(uttt_policy::PATTERN_FEATURES);
    let text = format!("pub const PATTERN_TEXT: &str = \"\\\n{digits}\";");
    assert_eq!(
        read_pattern_policy(&text).unwrap(),
        PatternPolicy::decode(&digits).unwrap()
    );
    assert!(read_pattern_policy("pub const PATTERN_TEXT: &str = \"\\\nAB\\\nC/\";").is_err());
    assert!(read_pattern_policy("no text here").is_err());
    let truncated = BOT_PATTERNS.replace("PATTERN_PACKED: &str = \"", "PATTERN_PACKED: &str = \"x");
    assert!(read_pattern_policy(&truncated).is_err());
    assert!(read_pattern_policy("pub const PATTERN_PACKED: &str = \"").is_err());
}

#[test]
fn games_with_the_bots_patterns_record_visits_of_every_move() {
    let policy: &'static PatternPolicy = Box::leak(Box::new(
        read_pattern_policy(BOT_PATTERNS).unwrap().for_plies(16),
    ));
    let settings = SelfPlay {
        iterations: 300,
        opening_plies: 6,
        opening: Opening::Random,
        opening_temperature: 1.0,
        balance: None,
        exploration: 0.5,
        expand_all: false,
        minimax_weight: 0.0,
        network: None,
        policy: None,
        patterns: Some(policy),
        record_visits: true,
    };
    let game = settings.play(5).unwrap();
    assert_eq!(Some(&game), settings.play(5).as_ref());
    assert_ne!(game.status(), Status::Ongoing);
    assert!(!game.searched.is_empty());
    for (position, searched) in game.positions() {
        let mut legal = MoveList::new();
        position.legal_moves(&mut legal);
        assert_eq!(searched.visits.len(), legal.len());
        let best = searched.visits.iter().max_by_key(|(_, n)| *n).unwrap().0;
        assert_eq!(best, game.moves[usize::from(searched.ply)]);
    }
}

/// The bot's network, as the Train workflow reads it.
const BOT_NETWORK: &str = include_str!("../../../bots/mcts/src/nnue_weights.rs");

fn network() -> &'static Network<256> {
    static NETWORK: std::sync::OnceLock<Network<256>> = std::sync::OnceLock::new();
    NETWORK.get_or_init(|| read_network(BOT_NETWORK).unwrap())
}

/// The newest release's settings, at a few iterations per move.
fn as_the_bot(opening: Opening, balance: Option<f64>) -> SelfPlay {
    SelfPlay {
        iterations: 200,
        opening_plies: 8,
        opening,
        opening_temperature: 1.0,
        balance,
        exploration: 0.2,
        expand_all: true,
        minimax_weight: 0.5,
        network: Some(network()),
        policy: None,
        patterns: None,
        record_visits: true,
    }
}

#[test]
fn sampled_openings_are_searched_recorded_and_varied() {
    let settings = as_the_bot(Opening::Sampled, None);
    let mut openings = std::collections::HashSet::new();
    let mut not_best = 0;
    for seed in 0..12 {
        let game = settings.play(seed).unwrap();
        assert_eq!(Some(&game), settings.play(seed).as_ref());
        assert_ne!(game.status(), Status::Ongoing);
        // Every position is searched, the opening's included.
        let plies: Vec<u8> = game.searched.iter().map(|searched| searched.ply).collect();
        assert_eq!(plies[..8], [0, 1, 2, 3, 4, 5, 6, 7]);
        for (position, searched) in game.positions() {
            let mut legal = MoveList::new();
            position.legal_moves(&mut legal);
            assert_eq!(searched.visits.len(), legal.len());
            let played = game.moves[usize::from(searched.ply)];
            let visits = |mv: Move| searched.visits.iter().find(|(m, _)| *m == mv).unwrap().1;
            let best = searched.visits.iter().max_by_key(|(_, n)| *n).unwrap().0;
            // Drawn among the moves the search visited; later, the search's
            // choice (a proven win, or the most visited move not proven
            // lost), which the other tests check.
            assert!(visits(played) > 0);
            if searched.ply < 8 {
                not_best += usize::from(visits(played) < visits(best));
            }
        }
        openings.insert(game.moves[..8].to_vec());
    }
    assert_eq!(openings.len(), 12, "every game opens its own way");
    assert!(
        not_best > 10,
        "only {not_best} opening moves were not the most visited"
    );
}

#[test]
fn games_decided_after_the_opening_are_dropped() {
    // A balance of 0.5 drops every game whose first search after the
    // opening is not exactly even; 1 drops none.
    for opening in [Opening::Random, Opening::Sampled] {
        for seed in 0..4 {
            assert!(as_the_bot(opening, Some(0.5)).play(seed).is_none());
            assert_eq!(
                as_the_bot(opening, Some(1.0)).play(seed),
                as_the_bot(opening, None).play(seed)
            );
        }
    }
}

#[test]
fn the_sampled_moves_follow_the_visits() {
    let mut rng = Rng::new(3);
    let a = Move::new(0, 0);
    let b = Move::new(4, 4);
    let c = Move::new(8, 8);
    let mut counts = [0u32; 3];
    for _ in 0..10_000 {
        let mv = sample([(a, 1), (b, 0), (c, 3)].into_iter(), 1.0, &mut rng).unwrap();
        counts[[a, b, c].iter().position(|&m| m == mv).unwrap()] += 1;
    }
    assert_eq!(counts[1], 0, "a move without visits is never drawn");
    assert!((2300..2700).contains(&counts[0]), "{counts:?}");
    assert_eq!(sample([(a, 0)].into_iter(), 1.0, &mut rng), None);
    assert_eq!(sample(std::iter::empty(), 1.0, &mut rng), None);
}

#[test]
fn the_network_is_read_from_the_bot_and_from_a_release() {
    let network = read_network(BOT_NETWORK).unwrap();
    let release = include_str!("../../../releases/uttt-v015.rs");
    assert!(read_network(release).unwrap() == network);
    assert!(read_network("no weights here").is_err());
    assert!(read_network("pub const NNUE_WEIGHTS: &str = \"abc\";").is_err());
}

/// The text after `flag` in `options`, up to the next space.
fn flag_value<'a>(options: &'a str, flag: &str) -> Option<&'a str> {
    let at = options.find(&format!("{flag} "))? + flag.len() + 1;
    options[at..].split_whitespace().next()
}

#[test]
fn the_train_workflow_plays_as_the_newest_release() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let workflow = std::fs::read_to_string(root.join(".github/workflows/train.yml")).unwrap();
    let options = workflow
        .lines()
        .find(|line| line.trim_start().starts_with("options=(--network "))
        .expect("the nnue stage's self-play options");
    let folder = root.join("games/uttt/releases");
    let newest = std::fs::read_dir(&folder)
        .unwrap()
        .filter_map(|entry| {
            let name = entry.unwrap().file_name().into_string().unwrap();
            let number = name
                .strip_prefix("uttt-v")?
                .strip_suffix(".rs")?
                .parse::<u32>()
                .ok()?;
            Some((number, name))
        })
        .max()
        .expect("a release")
        .1;
    let release = std::fs::read_to_string(folder.join(&newest))
        .unwrap()
        .replace("\r\n", "\n");
    let has = |line: String| {
        assert!(
            release.lines().any(|l| l == line),
            "{newest} has no line `{line}`: update the Train workflow's nnue self-play to its settings"
        );
    };
    let number = |flag: &str| flag_value(options, flag).unwrap().parse::<f64>().unwrap();
    has(format!(
        "const EXPLORATION: f64 = {:?};",
        number("--exploration")
    ));
    has(format!(
        "const MINIMAX_WEIGHT: f64 = {:?};",
        number("--minimax-weight")
    ));
    has(format!(
        "const EXPAND_ALL: bool = {};",
        options.contains("--expand-all")
    ));
    assert!(!release
        .lines()
        .any(|line| line == "const PRIORS: bool = true;"));
    let path = flag_value(options, "--network").unwrap();
    let network = std::fs::read_to_string(root.join(path)).unwrap();
    assert_eq!(
        read_constant(&network, "NNUE_WEIGHTS"),
        read_constant(&release, "NNUE_WEIGHTS"),
        "{path} is not {newest}'s network"
    );
}

#[test]
fn progress_lines_group_digits_and_give_the_share() {
    assert_eq!(thousands(0), "0");
    assert_eq!(thousands(999), "999");
    assert_eq!(thousands(1_000), "1,000");
    assert_eq!(thousands(528_079), "528,079");
    assert_eq!(thousands(12_345_678), "12,345,678");
    assert_eq!(
        progress(10_800, 25_000, 528_079, 5_649.4),
        "10,800 of 25,000 games (43%), 528,079 positions, 5,649 s"
    );
}

#[test]
fn a_higher_temperature_flattens_the_draw() {
    let mut rng = Rng::new(5);
    let a = Move::new(0, 0);
    let b = Move::new(4, 4);
    let c = Move::new(8, 8);
    let visits = [(a, 1), (b, 0), (c, 9)];
    let mut counts = [0u32; 3];
    for _ in 0..10_000 {
        let mv = sample(visits.into_iter(), 2.0, &mut rng).unwrap();
        counts[[a, b, c].iter().position(|&m| m == mv).unwrap()] += 1;
    }
    // In proportion to 1 and 3, the square roots of the visits.
    assert_eq!(counts[1], 0, "a move without visits is never drawn");
    assert!((2300..2700).contains(&counts[0]), "{counts:?}");
    assert_eq!(sample([(a, 0)].into_iter(), 2.0, &mut rng), None);
    // Very hot, nearly uniform among the visited moves.
    let mut a_count = 0;
    for _ in 0..10_000 {
        a_count += u32::from(sample(visits.into_iter(), 1e6, &mut rng) == Some(a));
    }
    assert!((4800..5200).contains(&a_count), "{a_count}");
}
