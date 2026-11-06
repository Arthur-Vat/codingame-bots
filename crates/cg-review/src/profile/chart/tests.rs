use crate::profile::Point;

use super::*;

fn line(values: &[Option<f64>]) -> Vec<Point> {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| Point::new(i as u32 + 1, usize::from(v.is_some()) * 6, *v))
        .collect()
}

fn set(first: &[Option<f64>], second: &[Option<f64>], separates_from: Option<u32>) -> CurveSet {
    CurveSet {
        slug: "wins-vs-losses".to_string(),
        labels: ["our wins".to_string(), "our losses".to_string()],
        games: [6, 7],
        last_turn: first.len() as u32,
        either: false,
        events: Vec::new(),
        curves: vec![Curve {
            key: "income".to_string(),
            meaning: "Gold earned each turn.".to_string(),
            lower_is_better: false,
            rate: false,
            lines: [line(first), line(second)],
            auc: vec![None; first.len()],
            separates_from,
        }],
    }
}

fn some(values: &[f64]) -> Vec<Option<f64>> {
    values.iter().map(|v| Some(*v)).collect()
}

#[test]
fn a_chart_has_the_design_of_the_guidance() {
    let s = set(
        &some(&[1.0, 2.0, 4.0, 8.0]),
        &some(&[1.0, 1.0, 2.0, 3.0]),
        Some(3),
    );
    let svg = svg(&s, &s.curves[0]);
    assert!(svg.starts_with("<svg "), "{svg}");
    assert!(svg.contains("viewBox=\"0 0 640 320\""), "{svg}");
    // Palettes, light and dark.
    for colour in [
        "#2a78d6", "#eb6834", "#3987e5", "#d95926", "#fcfcfb", "#1a1a19", "#0b0b0b", "#f0efec",
        "#52514e", "#c3c2b7",
    ] {
        assert!(svg.contains(colour), "{colour}");
    }
    assert!(svg.contains("@media (prefers-color-scheme: dark)"));
    assert!(svg.contains("font-family:system-ui,sans-serif"));
    // Two lines of 2 px with round joins, a legend and names at the ends.
    assert_eq!(svg.matches("<path class=\"s").count(), 2);
    assert_eq!(
        svg.matches("stroke-width=\"2\" stroke-linejoin=\"round\"")
            .count(),
        2
    );
    assert_eq!(svg.matches(">our wins</text>").count(), 2);
    assert_eq!(svg.matches(">our losses</text>").count(), 2);
    // The title, the subtitle with n, the axis and the separation turn.
    assert!(svg.contains("income: Gold earned each turn."));
    assert!(svg.contains("n = 6 (our wins) and 7 (our losses)"));
    assert!(!svg.contains("less is better"));
    assert!(svg.contains(">game turn</text>"));
    assert!(svg.contains(">turn 3</text>"));
    assert!(svg.contains("stroke-dasharray"));
    // The y axis starts at 0 and ends at a round value above the maximum.
    assert!(svg.contains(">0</text>"));
    assert!(svg.contains(">8</text>"));
    assert!(!svg.contains("NaN") && !svg.contains("inf"), "{svg}");
}

#[test]
fn the_subtitle_says_when_less_is_better() {
    let mut s = set(&some(&[1.0, 2.0]), &some(&[1.0, 2.0]), None);
    s.curves[0].lower_is_better = true;
    assert!(svg(&s, &s.curves[0]).contains("median per game turn (less is better); n = 6"));
}

#[test]
fn the_same_input_gives_the_same_file() {
    let s = set(&some(&[1.0, 3.0]), &some(&[2.0, 2.5]), None);
    assert_eq!(svg(&s, &s.curves[0]), svg(&s, &s.curves[0]));
    assert!(!svg(&s, &s.curves[0]).contains("dasharray"));
}

#[test]
fn text_is_escaped() {
    let mut s = set(&some(&[1.0, 2.0]), &some(&[1.0, 2.0]), None);
    s.curves[0].meaning = "a < b & c > \"d\"".to_string();
    s.labels[0] = "wins & draws".to_string();
    let svg = svg(&s, &s.curves[0]);
    assert!(svg.contains("a &lt; b &amp; c &gt; &quot;d&quot;"), "{svg}");
    assert!(svg.contains("wins &amp; draws"));
    assert!(!svg.contains("a < b"));
}

#[test]
fn gaps_split_a_line_and_a_lone_point_is_a_dot() {
    let s = set(
        &[
            Some(1.0),
            Some(2.0),
            None,
            Some(3.0),
            None,
            Some(4.0),
            Some(5.0),
        ],
        &some(&[1.0; 7]),
        None,
    );
    let svg = svg(&s, &s.curves[0]);
    // The first line has two segments in one path and one dot; the second one segment.
    let path = svg
        .lines()
        .find(|l| l.contains("class=\"s1\" d="))
        .expect("path");
    assert_eq!(path.matches('M').count(), 2, "{path}");
    assert_eq!(svg.matches("<circle class=\"f1\" cx").count(), 2); // the dot and the end mark
}

#[test]
fn constant_empty_and_negative_curves_draw_without_nan() {
    let flat = set(&some(&[0.0; 3]), &some(&[0.0; 3]), None);
    let text = svg(&flat, &flat.curves[0]);
    assert!(!text.contains("NaN") && !text.contains("inf"), "{text}");
    assert!(text.contains(">0.0</text>") && text.contains(">0.5</text>"));
    let constant = set(&some(&[5.0; 3]), &some(&[5.0; 3]), None);
    let text = svg(&constant, &constant.curves[0]);
    assert!(!text.contains("NaN"), "{text}");
    let empty = set(&[None, None], &[None, None], None);
    let text = svg(&empty, &empty.curves[0]);
    assert!(!text.contains("NaN") && !text.contains("<path"), "{text}");
    let one = set(&some(&[3.0]), &some(&[4.0]), None);
    assert!(!svg(&one, &one.curves[0]).contains("NaN"));
    let negative = set(&some(&[-5.0, -2.0, 3.0]), &some(&[0.0, 1.0, 2.0]), None);
    let text = svg(&negative, &negative.curves[0]);
    assert!(text.contains(">-6</text>"), "{text}");
    assert!(!text.contains("NaN"));
}

#[test]
fn names_at_the_right_end_stay_apart() {
    let s = set(&some(&[3.0, 4.0]), &some(&[3.0, 4.1]), None);
    let svg = svg(&s, &s.curves[0]);
    let ys: Vec<f64> = svg
        .lines()
        .filter(|l| l.contains("r=\"4\""))
        .map(|l| {
            let cy = l.split("cy=\"").nth(1).expect("cy");
            cy.split('"')
                .next()
                .expect("value")
                .parse()
                .expect("number")
        })
        .collect();
    assert_eq!(ys.len(), 2);
    assert!((ys[0] - ys[1]).abs() >= 13.9, "{ys:?}");
}

#[test]
fn a_long_title_wraps_then_is_cut() {
    assert_eq!(wrap("short one", 20), ["short one"]);
    assert_eq!(wrap("aaa bbb ccc ddd", 7), ["aaa bbb", "ccc ddd"]);
    let cut = wrap("aaa bbb ccc ddd eee fff", 7);
    assert_eq!(cut.len(), 2);
    assert!(
        cut[1].ends_with('…') && cut[1].chars().count() <= 7,
        "{cut:?}"
    );
}

#[test]
fn ticks_are_round() {
    assert_eq!(nice_step(8.0), 2.0);
    assert_eq!(nice_step(1.0), 0.5);
    assert_eq!(nice_step(300.0), 100.0);
    assert_eq!(nice_step(40.0), 10.0);
    assert_eq!(tick_text(2.0, 2.0), "2");
    assert_eq!(tick_text(0.5, 0.5), "0.5");
    assert_eq!(tick_text(-0.0, 1.0), "0");
    assert_eq!(turn_step(20), 2);
    assert_eq!(turn_step(10), 1);
    assert_eq!(turn_step(100), 10);
}

#[test]
fn files_are_named_by_set_and_metric() {
    let s = set(&some(&[1.0, 2.0]), &some(&[1.0, 2.0]), None);
    assert_eq!(file_name(&s, "income", false), "wins-vs-losses-income.svg");
    assert_eq!(
        file_name(&s, "Hq Gap/x", false),
        "wins-vs-losses-hq-gap-x.svg"
    );
    let a = Analysis {
        curves: crate::profile::Curves {
            wins_vs_losses: s,
            ..Default::default()
        },
        ..Default::default()
    };
    let files = files(&a);
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, "wins-vs-losses-income.svg");
}

#[test]
fn the_middle_half_is_a_light_band_behind_each_line() {
    let mut s = set(&some(&[1.0, 2.0, 3.0]), &some(&[1.0, 1.0, 1.0]), None);
    for p in &mut s.curves[0].lines[0] {
        p.p25 = p.median.map(|m| m - 0.5);
        p.p75 = p.median.map(|m| m + 0.5);
    }
    let text = svg(&s, &s.curves[0]);
    assert!(text.contains("fill-opacity=\"0.15\""), "{text}");
    assert!(text.contains("line: median; band: middle half of the games"));
    // The band comes before the lines.
    assert!(
        text.find("fill-opacity").expect("band") < text.find("<path class=\"s1\"").expect("line")
    );
}

#[test]
fn an_event_is_drawn_as_a_share_in_percent_without_bands() {
    let mut s = set(&some(&[0.0, 0.0]), &some(&[0.0, 0.0]), None);
    let mut event = s.curves[0].clone();
    event.key = "mine".to_string();
    event.rate = true;
    for (line, share) in event.lines.iter_mut().zip([0.4, 0.1]) {
        for p in line.iter_mut() {
            p.mean = Some(share);
        }
    }
    s.events.push(event);
    let text = svg(&s, &s.events[0]);
    assert!(text.contains("Event mine:"), "{text}");
    assert!(text.contains("share of games, in percent"), "{text}");
    assert!(!text.contains("fill-opacity") && !text.contains("band:"));
    assert!(text.contains(">40</text>"), "{text}");
    assert_eq!(file_name(&s, "mine", true), "wins-vs-losses-event-mine.svg");
    // The files of a set: its metrics, then its events.
    let a = Analysis {
        curves: crate::profile::Curves {
            wins_vs_losses: s,
            ..Default::default()
        },
        ..Default::default()
    };
    let names: Vec<String> = files(&a).into_iter().map(|f| f.0).collect();
    assert_eq!(
        names,
        ["wins-vs-losses-income.svg", "wins-vs-losses-event-mine.svg"]
    );
}
