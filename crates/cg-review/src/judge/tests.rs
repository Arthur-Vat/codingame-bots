use super::*;

struct Words;

impl Judging for Words {
    fn compare(played: &str, ours: &str) -> MoveMatch {
        if played == ours {
            MoveMatch::Same
        } else if played.split(' ').next() == ours.split(' ').next() {
            MoveMatch::First
        } else {
            MoveMatch::Different
        }
    }
    fn gap_threshold() -> i64 {
        10
    }
}

fn j(
    player: &str,
    game: u64,
    ply: usize,
    phase: Phase,
    played: &str,
    scores: [i64; 2],
) -> Judgement {
    Judgement {
        player: player.to_string(),
        rank: (player == "ann").then_some(3),
        game_id: game,
        ply,
        turn: ply as u32 / 2 + 1,
        phase,
        ours: "a b".to_string(),
        played: played.to_string(),
        ours_score: scores[0],
        played_score: scores[1],
        board: vec![format!("board {game}-{ply}")],
    }
}

fn sample() -> Vec<Judgement> {
    vec![
        j("ann", 1, 0, Phase::Early, "a b", [5, 5]),
        j("ann", 1, 2, Phase::Middle, "a c", [20, 5]),
        j("ann", 1, 4, Phase::Middle, "x y", [30, -10]),
        j("ann", 2, 4, Phase::Middle, "x y", [0, 4]),
        j("bob", 3, 6, Phase::End, "a b", [7, 7]),
    ]
}

#[test]
fn the_csv_has_a_row_a_ply() {
    let text = judgements_csv::<Words>(&sample());
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 6);
    assert_eq!(
        lines[0],
        "game_id,player,rank,ply,turn,phase,played,ours,played_score,ours_score,gap,match"
    );
    assert_eq!(lines[1], "1,ann,3,0,1,early_game,a b,a b,5,5,0,same");
    assert_eq!(
        lines[2],
        "1,ann,3,2,2,middle_game,a c,a b,5,20,15,same_first"
    );
    assert_eq!(
        lines[3],
        "1,ann,3,4,3,middle_game,x y,a b,-10,30,40,different"
    );
    assert_eq!(lines[5], "3,bob,,6,4,endgame,a b,a b,7,7,0,same");
}

#[test]
fn fields_with_commas_are_quoted() {
    assert_eq!(field("a,b"), "\"a,b\"");
    assert_eq!(field("say \"x\""), "\"say \"\"x\"\"\"");
    assert_eq!(field("plain; text"), "plain; text");
}

#[test]
fn the_summary_counts_agreement_and_gaps_by_player_and_phase() {
    let text = summary_csv::<Words>(&sample());
    let has = |row: &str| assert!(text.lines().any(|l| l == row), "{row}\n{text}");
    has("ann,early_game,plies,1");
    has("ann,early_game,same,1");
    has("ann,middle_game,plies,3");
    has("ann,middle_game,same_first,1");
    has("ann,middle_game,different,2");
    // Middle gaps are 15, 40 and -4: the median 15, two over 10.
    has("ann,middle_game,median_gap,15.0");
    has("ann,middle_game,gap_over_threshold,2");
    has("ann,middle_game,gap_over_threshold_share,0.667");
    // Even count: 0, 15, 40 and -4 give 7.5.
    has("ann,all,median_gap,7.5");
    has("bob,all,gap_over_threshold,0");
    assert!(!text.contains("bob,early_game"), "{text}");
}

#[test]
fn the_largest_gaps_come_first_with_their_boards() {
    let text = largest_gaps::<Words>(&sample());
    let first = text.find("## 1. ann in game 1, turn 3").expect("first");
    let second = text.find("## 2. ann in game 1, turn 2").expect("second");
    assert!(first < second);
    assert!(text.contains("gap 40"), "{text}");
    assert!(text.contains("```text\nboard 1-4\n```"), "{text}");
    assert!(text.contains("- played (-10): `x y`"), "{text}");
    assert!(text.contains("- ours (30): `a b`"), "{text}");
}

#[test]
fn at_most_twenty_plies_are_listed_and_none_is_fine() {
    let many: Vec<Judgement> = (0..30)
        .map(|i| j("ann", 1, i, Phase::Middle, "x", [i as i64, 0]))
        .collect();
    let text = largest_gaps::<Words>(&many);
    assert_eq!(text.matches("\n## ").count(), WORST_SHOWN);
    assert!(text.contains("## 1. ann in game 1, turn 15 (middle game), gap 29"));
    assert!(!largest_gaps::<Words>(&[]).contains("\n## "));
    assert_eq!(summary_csv::<Words>(&[]), "player,phase,metric,value\n");
}

#[test]
fn the_readme_names_every_file_and_the_threshold() {
    let text = readme::<Words>(&["a note".to_string()]);
    assert!(text.contains("- a note"), "{text}");
    for file in ["judgements.csv", "summary.csv", "largest-gaps.md"] {
        assert!(text.contains(file), "{file}");
    }
    assert!(text.contains("above 10"), "{text}");
}

#[test]
fn the_median_of_nothing_is_none() {
    assert_eq!(median(&mut []), None);
    assert_eq!(median(&mut [3, 1, 2]), Some(2.0));
}
