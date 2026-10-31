use super::*;

/// A stored line whose pairs scored `points` (the number of pairs that
/// scored 0, ½, 1, 1½ and 2 points for `a`).
fn line(a: &str, b: &str, seed: u64, date: &str, points: [u32; 5]) -> PairingResult {
    let [c0, c1, c2, c3, c4] = points;
    PairingResult {
        a: a.to_string(),
        b: b.to_string(),
        pairs: points.iter().sum(),
        wins: 2 * c4 + c3 + c2,
        draws: c3 + c1,
        losses: c2 + c1 + 2 * c0,
        pair_points: points,
        faults: [0, 0],
        time_scale: 1.0,
        time_tolerance_ms: 0,
        opening_plies: 0,
        seed,
        date: date.to_string(),
        commit: "unknown".to_string(),
        cpu: "unknown".to_string(),
        speed: None,
    }
}

/// `pairs` pairs all drawn one game each (equal strength).
fn even(a: &str, b: &str, pairs: u32) -> PairingResult {
    line(a, b, 7, "2026-10-01", [0, 0, pairs, 0, 0])
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|name| name.to_string()).collect()
}

fn settings(pool: &[&str], releases: &[&str], anchor: &str) -> Settings {
    Settings {
        pool: names(pool),
        releases: names(releases),
        anchor: anchor.to_string(),
        fixed: Vec::new(),
        nearest: 2,
        new_pairs: 800,
        budget_pairs: 10_000,
        block_pairs: 200,
        top: 2,
        target_margin: 15.0,
        low_share: 0.2,
        night: "2026-11-01".to_string(),
        seed: 1,
        shards: 1,
    }
}

/// Every pair of `bots`, `pairs` pairs each, all even.
fn full(bots: &[&str], pairs: u32) -> Vec<PairingResult> {
    let mut lines = Vec::new();
    for (at, a) in bots.iter().enumerate() {
        for b in &bots[at + 1..] {
            lines.push(even(a, b, pairs));
        }
    }
    lines
}

fn of_kind(plan: &Plan, kind: Kind) -> Vec<&PlanLine> {
    plan.lines.iter().filter(|line| line.kind == kind).collect()
}

fn pairs_of(plan: &Plan, kind: Kind) -> u32 {
    of_kind(plan, kind).iter().map(|line| line.pairs).sum()
}

fn pair(line: &PlanLine) -> (&str, &str) {
    (line.a.as_str(), line.b.as_str())
}

#[test]
fn a_plan_line_is_written_in_the_documented_key_order() {
    let line = PlanLine {
        a: "v014".to_string(),
        b: "v013".to_string(),
        pairs: 800,
        seed: 123,
        kind: Kind::New,
        shard: 0,
    };
    let text = r#"{"a":"v014","b":"v013","pairs":800,"seed":123,"kind":"new","shard":0}"#;
    assert_eq!(line.to_line(), text);
    assert_eq!(parse_plan(text, "plan").unwrap(), [line]);
    assert_eq!(
        parse_plan(&format!("\n{text}\n  \n"), "plan")
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn bad_plan_lines_are_rejected_with_their_place() {
    let good = r#"{"a":"x","b":"y","pairs":2,"seed":1,"kind":"top","shard":0}"#;
    for (bad, expected) in [
        (good.replace("\"pairs\":2", "\"pairs\":0"), "0 pairs"),
        (good.replace("\"b\":\"y\"", "\"b\":\"x\""), "with itself"),
        (good.replace("top", "mid"), "mid"),
        (good.replace("\"shard\"", "\"extra\":1,\"shard\""), "extra"),
        ("{oops".to_string(), "key"),
    ] {
        let error = parse_plan(&format!("{good}\n{bad}\n"), "plan.jsonl").unwrap_err();
        assert!(error.starts_with("plan.jsonl:2: "), "{error}");
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn settings_are_checked() {
    let base = settings(&["a", "b", "c"], &["b", "c"], "a");
    assert!(base.validate().is_ok());
    let check = |change: &dyn Fn(&mut Settings), expected: &str| {
        let mut changed = base.clone();
        change(&mut changed);
        let error = changed.validate().unwrap_err();
        assert!(error.contains(expected), "{error}");
        assert!(plan(&[], &changed).is_err());
    };
    check(&|s| s.pool.push("a".to_string()), "twice");
    check(&|s| s.releases.push("z".to_string()), "not in the --pool");
    check(&|s| s.releases.push("b".to_string()), "twice");
    check(&|s| s.anchor = "z".to_string(), "--anchor");
    check(&|s| s.fixed = names(&["z"]), "--fixed-opponents");
    check(&|s| s.block_pairs = 0, "--block-pairs");
    check(&|s| s.top = 0, "--top");
    check(&|s| s.shards = 0, "--shards");
    check(&|s| s.new_pairs = 0, "--new-pairs");
    check(&|s| s.low_share = 1.5, "--low-share");
    check(&|s| s.low_share = f64::NAN, "--low-share");
    check(&|s| s.low_share = -0.1, "--low-share");
    for night in [
        "",
        "2026-1-01",
        "2026/11/01",
        "2026-13-01",
        "2026-11-32",
        "x026-11-01",
        "2026-11-01-",
        "2026-11-01x",
        "2026-11-01-a",
        "2026-11-01-1234",
    ] {
        check(&|s| s.night = night.to_string(), "--night");
    }
    for night in ["2026-11-01", "2026-11-01-2"] {
        let mut s = settings(&["a", "b"], &["b"], "a");
        s.night = night.to_string();
        assert_eq!(s.validate(), Ok(()), "{night}");
    }
}

#[test]
fn an_empty_file_plans_each_bot_as_new() {
    let mut s = settings(
        &["greedy", "v001", "v002", "v003"],
        &["v001", "v002", "v003"],
        "greedy",
    );
    s.fixed = names(&["greedy"]);
    let plan = plan(&[], &s).unwrap();
    // The first release plays the fixed opponent and, the pool being small,
    // every other bot; later releases only their previous release and the
    // fixed opponent; the baseline needs ratings first.
    let pairs: Vec<(&str, &str)> = plan.lines.iter().map(pair).collect();
    assert_eq!(
        pairs,
        [
            ("v001", "greedy"),
            ("v001", "v002"),
            ("v001", "v003"),
            ("v002", "greedy"),
            ("v003", "v002"),
            ("v003", "greedy"),
        ]
    );
    for line in &plan.lines {
        assert_eq!((line.kind, line.pairs), (Kind::New, 800));
        assert_eq!(line.seed, pairing_seed(1, &line.a, &line.b));
    }
    assert!(plan.report.last().unwrap().starts_with("target not met: "));
}

#[test]
fn a_release_that_played_its_previous_release_gets_its_nearest() {
    let stored = [
        line("v002", "greedy", 1, "2026-10-01", [0, 0, 0, 0, 10]),
        line("near", "v002", 2, "2026-10-01", [0, 0, 10, 0, 0]),
        line("far", "greedy", 3, "2026-10-01", [10, 0, 0, 0, 0]),
        line("v001", "greedy", 4, "2026-10-01", [0, 0, 10, 0, 0]),
        line("v003", "v002", 5, "2026-10-01", [0, 0, 10, 0, 0]),
    ];
    let mut s = settings(
        &["greedy", "far", "near", "v001", "v002", "v003"],
        &["v001", "v002", "v003"],
        "greedy",
    );
    s.fixed = names(&["greedy"]);
    s.nearest = 1;
    s.budget_pairs = 0;
    let plan = plan(&stored, &s).unwrap();
    let v003: Vec<&str> = plan
        .lines
        .iter()
        .filter(|line| line.a == "v003")
        .map(|line| line.b.as_str())
        .collect();
    assert_eq!(v003, ["greedy", "near"], "the fixed one, then the closest");
    assert!(plan
        .lines
        .iter()
        .filter(|line| line.kind == Kind::New)
        .all(|line| line.pairs == 800));
}

#[test]
fn a_bot_without_games_waits_for_its_previous_release_to_be_rated() {
    // v002 has a line with nobody; it plays its previous release and the
    // fixed opponent only, whatever the number of unrated bots around.
    let stored = [even("v001", "greedy", 100), even("near", "greedy", 100)];
    let mut s = settings(
        &["greedy", "near", "v001", "v002"],
        &["v001", "v002"],
        "greedy",
    );
    s.fixed = names(&["greedy"]);
    s.nearest = 1;
    let plan = plan(&stored, &s).unwrap();
    let v002: Vec<&str> = plan
        .lines
        .iter()
        .filter(|line| line.a == "v002")
        .map(|line| line.b.as_str())
        .collect();
    assert_eq!(v002, ["v001", "greedy"]);
}

#[test]
fn bots_outside_the_pool_are_rated_but_never_planned() {
    let mut stored = full(&["a", "b", "c", "d"], 3000);
    stored.push(even("old", "a", 3000));
    let s = settings(&["a", "b", "c", "d"], &[], "a");
    let plan = plan(&stored, &s).unwrap();
    assert!(plan
        .lines
        .iter()
        .all(|line| line.a != "old" && line.b != "old"));
    assert!(!plan.lines.is_empty());
}

#[test]
fn a_complete_pool_on_target_plans_only_the_other_bots() {
    let stored = full(&["a", "b", "c", "d"], 3000);
    let mut s = settings(&["a", "b", "c", "d"], &[], "a");
    s.budget_pairs = 1000;
    let plan = plan(&stored, &s).unwrap();
    assert_eq!(plan.lines.len(), 1, "{:?}", plan.lines);
    let low = &plan.lines[0];
    assert_eq!((low.kind, low.pairs), (Kind::Low, 200));
    // The top 2 are a and b (equal ratings, by name), so c plays first.
    assert!(low.a == "c" || low.b == "c", "{low:?}");
    assert_eq!(
        plan.report.last().unwrap(),
        "target met: top 2 within ±15 Elo"
    );
}

#[test]
fn a_budget_of_zero_plans_only_the_new_bots() {
    let mut s = settings(&["a", "b", "c"], &["b", "c"], "a");
    s.budget_pairs = 0;
    let all = plan(&[], &s).unwrap();
    assert!(!all.lines.is_empty());
    assert!(all.lines.iter().all(|line| line.kind == Kind::New));
    // With stored games that leave the top off target, nothing else either.
    let stored = [even("a", "b", 20), even("b", "c", 20), even("a", "c", 20)];
    let mut s = settings(&["a", "b", "c", "d", "e"], &[], "a");
    s.budget_pairs = 0;
    let plan = plan(&stored, &s).unwrap();
    assert!(plan.lines.iter().all(|line| line.kind == Kind::New));
}

#[test]
fn the_new_bots_are_planned_even_over_the_budget() {
    let mut s = settings(&["a", "b", "c"], &["b", "c"], "a");
    s.budget_pairs = 100;
    let plan = plan(&[], &s).unwrap();
    assert_eq!(pairs_of(&plan, Kind::New), 800 * plan.lines.len() as u32);
}

/// Five bots, each pair played `pairs` times; the top 2 are off target.
fn wide_top(pairs: u32) -> Vec<PairingResult> {
    full(&["a", "b", "c", "d", "e"], pairs)
}

#[test]
fn the_budget_is_split_in_whole_blocks() {
    let stored = wide_top(20);
    let run = |budget: u64, share: f64| {
        let mut s = settings(&["a", "b", "c", "d", "e"], &[], "a");
        s.target_margin = 0.001;
        s.budget_pairs = budget;
        s.low_share = share;
        let plan = plan(&stored, &s).unwrap();
        (pairs_of(&plan, Kind::Top), pairs_of(&plan, Kind::Low))
    };
    // 1,000 pairs: 200 low and 800 top.
    assert_eq!(run(1000, 0.2), (800, 200));
    // 1,100: 220 low, 880 top, both rounded down; the 100 left is unused.
    assert_eq!(run(1100, 0.2), (800, 200));
    // 1,500 at a quarter: 375 low, 1,125 top; 175 and 125 left make a block.
    assert_eq!(run(1500, 0.25), (1200, 200));
    // Nothing for the others.
    assert_eq!(run(1000, 0.0), (1000, 0));
    assert_eq!(run(1000, 1.0), (0, 1000));
    // Less than a block.
    assert_eq!(run(199, 0.2), (0, 0));
}

#[test]
fn the_top_pairs_go_to_neighbours_of_bots_over_the_target() {
    // a is the anchor; b has few games and a wide interval, the others many.
    let stored = [
        even("a", "b", 20),
        even("a", "c", 4000),
        even("a", "d", 4000),
        even("c", "d", 4000),
        even("b", "c", 20),
    ];
    let mut s = settings(&["a", "b", "c", "d"], &[], "a");
    s.top = 3;
    s.nearest = 1;
    s.budget_pairs = 1000; // 4 blocks of 200 for top, 1 for low
    let plan = plan(&stored, &s).unwrap();
    let top = of_kind(&plan, Kind::Top);
    assert!(!top.is_empty());
    assert!(
        top.iter().all(|line| line.a == "b" || line.b == "b"),
        "only b is over the target: {top:?}"
    );
    assert_eq!(pairs_of(&plan, Kind::Top), 800);
    // The projected margin of b shrinks.
    let report = plan.report.join("\n");
    assert!(report.contains("b margin"), "{report}");
    assert!(plan.report.last().unwrap().starts_with("target not met: "));
    assert!(plan.report.last().unwrap().contains("over ±15 Elo now: b"));
}

#[test]
fn the_top_is_not_planned_for_bots_that_cannot_improve() {
    // Every pair is over target but a lone block cannot be placed: the pool
    // is the anchor and one unlinked bot.
    let s = settings(&["a", "z"], &[], "a");
    let planned = plan(&[], &s).unwrap();
    assert!(of_kind(&planned, Kind::Top).is_empty());
    assert!(of_kind(&planned, Kind::Low).is_empty());
    // A pool of one bot plans nothing.
    let mut one = settings(&["a"], &[], "a");
    one.nearest = 5;
    let plan = plan(&[], &one).unwrap();
    assert!(plan.lines.is_empty());
    assert_eq!(
        plan.report.last().unwrap(),
        "target met: top 1 within ±15 Elo"
    );
}

#[test]
fn the_others_play_oldest_first_against_their_nearest() {
    let bots = ["a", "b", "c", "d", "e", "f"];
    let mut stored = full(&bots, 3000);
    for line in &mut stored {
        line.date = if line.involves("e") && line.involves("f") {
            "2026-01-01".to_string()
        } else if line.involves("e") || line.involves("f") {
            "2026-10-01".to_string()
        } else {
            "2026-10-05".to_string()
        };
    }
    // e and f are tied by date; f has fewer games.
    stored.retain(|line| !(line.involves("f") && line.involves("d") && line.a == "d"));
    stored.push(line("d", "f", 99, "2026-10-01", [0, 0, 1, 0, 0]));
    let mut s = settings(&bots, &[], "a");
    s.budget_pairs = 1000;
    s.nearest = 2;
    let plan = plan(&stored, &s).unwrap();
    assert!(of_kind(&plan, Kind::Top).is_empty());
    let low = of_kind(&plan, Kind::Low);
    assert_eq!(low.len(), 1);
    // The latest dates: a to d have 2026-10-05, e and f 2026-10-01. The top
    // is a and b, so e and f come first, and f has fewer games than e.
    assert!(low[0].a == "f" || low[0].b == "f", "{:?}", low[0]);
}

#[test]
fn several_low_blocks_for_a_pair_add_up_and_avoid_the_top_pairs() {
    let stored = wide_top(20);
    let mut s = settings(&["a", "b", "c", "d", "e"], &[], "a");
    s.target_margin = 0.001;
    s.budget_pairs = 2000;
    s.low_share = 0.5;
    let plan = plan(&stored, &s).unwrap();
    let mut seen = HashSet::new();
    for line in &plan.lines {
        assert!(seen.insert(key(&line.a, &line.b)), "one line per pair");
    }
    let top: HashSet<_> = of_kind(&plan, Kind::Top)
        .iter()
        .map(|line| key(&line.a, &line.b))
        .collect();
    for line in of_kind(&plan, Kind::Low) {
        assert!(!top.contains(&key(&line.a, &line.b)));
    }
    assert_eq!(pairs_of(&plan, Kind::Low), 1000);
    assert!(of_kind(&plan, Kind::Low)
        .iter()
        .any(|line| line.pairs > 200));
}

#[test]
fn lines_come_as_new_then_top_then_low_with_the_higher_rated_bot_first() {
    let mut stored = wide_top(20);
    stored.push(line("hi", "a", 5, "2026-10-01", [0, 0, 0, 0, 20]));
    stored.push(line("hi", "b", 6, "2026-10-01", [0, 0, 0, 0, 20]));
    let mut s = settings(&["a", "b", "c", "d", "e", "hi", "new"], &[], "a");
    s.target_margin = 0.001;
    s.budget_pairs = 3000;
    s.nearest = 1;
    let plan = plan(&stored, &s).unwrap();
    let kinds: Vec<Kind> = plan.lines.iter().map(|line| line.kind).collect();
    let mut sorted = kinds.clone();
    sorted.sort_by_key(|kind| match kind {
        Kind::New => 0,
        Kind::Top => 1,
        Kind::Low => 2,
    });
    assert_eq!(kinds, sorted);
    assert!(kinds.contains(&Kind::New) && kinds.contains(&Kind::Top));
    let ratings = {
        let names = league::known_bots(&stored, ["a"]);
        league::rate(&names, &stored, "a").unwrap()
    };
    let rating = |name: &str| ratings.iter().find(|r| r.name == name).unwrap().rating;
    for line in plan.lines.iter().filter(|line| line.kind != Kind::New) {
        assert!(rating(&line.a) >= rating(&line.b), "{line:?}");
    }
}

#[test]
fn night_seeds_differ_between_nights_and_avoid_stored_seeds() {
    let stored = wide_top(20);
    let run = |night: &str| {
        let mut s = settings(&["a", "b", "c", "d", "e"], &[], "a");
        s.target_margin = 0.001;
        s.budget_pairs = 1000;
        s.night = night.to_string();
        plan(&stored, &s).unwrap()
    };
    let (monday, tuesday) = (run("2026-11-01"), run("2026-11-02"));
    assert_eq!(monday.lines.len(), tuesday.lines.len());
    assert!(!monday.lines.is_empty());
    for (x, y) in monday.lines.iter().zip(&tuesday.lines) {
        assert_eq!(pair(x), pair(y));
        assert_ne!(x.seed, y.seed, "{x:?}");
    }

    // A stored seed is skipped.
    let mut s = settings(&["a", "b"], &[], "a");
    s.night = "2026-11-01".to_string();
    let first = {
        let base = pairing_seed(1, "b", "a");
        let mut rng = Rng::new(base ^ fnv1a(b"2026-11-01"));
        (rng.next_u64(), rng.next_u64())
    };
    let taken = line("a", "b", first.0, "2026-10-01", [0, 0, 5, 0, 0]);
    let builder = Builder::new(&s, &[taken]);
    assert_eq!(builder.night_seed("b", "a"), first.1);
    let elsewhere = Builder::new(&s, &[]);
    assert_eq!(elsewhere.night_seed("b", "a"), first.0);

    // A `new` line keeps the incremental league's seed unless it is stored.
    let own = pairing_seed(1, "b", "a");
    let mut builder = Builder::new(&s, &[line("a", "b", own, "2026-10-01", [0, 0, 5, 0, 0])]);
    builder.add("b", "a", 800, Kind::New);
    assert_ne!(builder.lines[0].0.seed, own);
    let mut builder = Builder::new(&s, &[]);
    builder.add("b", "a", 800, Kind::New);
    assert_eq!(builder.lines[0].0.seed, own);
}

#[test]
fn lines_are_shared_between_shards_largest_first() {
    assert_eq!(
        assign_shards(&[100, 300, 200, 300, 100], 2),
        [1, 0, 0, 1, 1]
    );
    assert_eq!(assign_shards(&[800; 6], 3), [0, 1, 2, 0, 1, 2]);
    assert_eq!(assign_shards(&[5, 5], 1), [0, 0]);
    assert!(assign_shards(&[], 4).is_empty());

    let mut s = settings(&["a", "b", "c", "d"], &["b", "c", "d"], "a");
    s.shards = 3;
    let plan = plan(&[], &s).unwrap();
    let shards: Vec<usize> = plan.lines.iter().map(|line| line.shard).collect();
    assert_eq!(shards, assign_shards(&vec![800; plan.lines.len()], 3));
    assert!(shards.iter().all(|&shard| shard < 3));
}

#[test]
fn the_same_inputs_give_the_same_plan() {
    let mut stored = wide_top(20);
    stored.push(even("a", "x", 5));
    let mut s = settings(&["a", "b", "c", "d", "e", "x", "y"], &["x", "y"], "a");
    s.budget_pairs = 4000;
    s.shards = 2;
    let first = plan(&stored, &s).unwrap();
    let second = plan(&stored, &s).unwrap();
    assert_eq!(first, second);
    assert!(!first.lines.is_empty());
}

#[test]
fn an_opponent_with_a_wide_margin_adds_less_information() {
    let information = |margin: f64| (1.96_f64 / margin).powi(2);
    let base = 0.01;
    let narrow = added_information(base, information(5.0));
    let wide = added_information(base, information(50.0));
    assert!(wide < narrow && narrow < base, "{wide} {narrow} {base}");
    assert_eq!(added_information(base, f64::INFINITY), base, "the anchor");
    assert_eq!(added_information(base, 0.0), 0.0);
    assert_eq!(added_information(0.0, 1.0), 0.0);
    // So the margin of the bot that plays shrinks less against the wide one.
    let margin_after = |added: f64| 1.96 / (information(30.0) + added).sqrt();
    assert!(margin_after(wide) > margin_after(narrow));
}

#[test]
fn the_other_bots_play_among_themselves_not_against_the_top() {
    let bots = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let stored = full(&bots, 3000);
    let mut s = settings(&bots, &[], "a");
    s.top = 6;
    s.budget_pairs = 5000;
    s.low_share = 1.0;
    let plan = plan(&stored, &s).unwrap();
    let low = of_kind(&plan, Kind::Low);
    assert_eq!(low.iter().map(|line| line.pairs).sum::<u32>(), 5000);
    for line in low {
        assert!(
            ["g", "h"].contains(&line.a.as_str()) && ["g", "h"].contains(&line.b.as_str()),
            "{line:?}"
        );
    }
}
