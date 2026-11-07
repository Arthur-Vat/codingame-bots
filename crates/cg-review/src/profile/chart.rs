//! Static SVG line charts of the curves (no dependency: the SVG is written
//! as text). One chart a metric and set: the median at each game turn of the
//! two groups, with a legend, each line named at its right end, a grid of
//! round values and, when there is one, the turn from which the groups stay
//! apart. The colours follow the group (the first is blue, the second
//! orange) and switch to the dark palette when the viewer prefers it.

use std::fmt::Write as _;

use super::{Analysis, Curve, CurveSet, Point};

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 320.0;
const LEFT: f64 = 48.0;
/// The plot's right edge: the line names fit between it and the border.
const RIGHT: f64 = 528.0;
const BOTTOM: f64 = 276.0;
/// Characters of a title line (13 px, bold) that fit the width.
const TITLE_CHARS: usize = 96;

/// The style: light colours, then the dark ones when the viewer prefers.
const STYLE: &str = "text{font-family:system-ui,sans-serif}\
.bg{fill:#fcfcfb}.tx{fill:#0b0b0b}.mu{fill:#52514e}.gr{stroke:#e4e3df}.ds{stroke:#52514e}\
.s1{stroke:#2a78d6}.s2{stroke:#eb6834}.f1{fill:#2a78d6}.f2{fill:#eb6834}.halo{stroke:#fcfcfb}\
@media (prefers-color-scheme: dark){\
.bg{fill:#1a1a19}.tx{fill:#f0efec}.mu{fill:#c3c2b7}.gr{stroke:#3a3a37}.ds{stroke:#c3c2b7}\
.s1{stroke:#3987e5}.s2{stroke:#d95926}.f1{fill:#3987e5}.f2{fill:#d95926}.halo{stroke:#1a1a19}}";

/// `text` safe inside SVG text and attributes.
fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// A file-name part: lower-case letters, digits and dashes.
pub(super) fn slug(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

/// The file name of the chart of `key` (an event if `event`) in `set`.
pub fn file_name(set: &CurveSet, key: &str, event: bool) -> String {
    format!(
        "{}-{}{}.svg",
        set.slug,
        if event { "event-" } else { "" },
        slug(key)
    )
}

/// Every chart of the analysis: file name and SVG, in a fixed order (the
/// metrics and the events of each set, then the economy sets).
pub fn files(a: &Analysis) -> Vec<(String, String)> {
    let c = &a.curves;
    [&c.top_vs_us, &c.wins_vs_losses]
        .into_iter()
        .chain(&c.players)
        .chain(&a.economy.sets)
        .flat_map(|set| {
            set.curves
                .iter()
                .chain(&set.events)
                .map(|curve| (file_name(set, &curve.key, curve.rate), svg(set, curve)))
        })
        .collect()
}

/// `text` cut into at most two lines of `max` characters at spaces, with an
/// ellipsis if it does not fit.
fn wrap(text: &str, max: usize) -> Vec<String> {
    let mut lines: Vec<String> = vec![String::new()];
    let mut cut = false;
    for word in text.split_whitespace() {
        let line = lines.last_mut().expect("a line");
        let needed = line.chars().count() + usize::from(!line.is_empty()) + word.chars().count();
        if needed <= max {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        } else if lines.len() < 2 {
            lines.push(word.to_string());
        } else {
            cut = true;
            break;
        }
    }
    if cut {
        if let Some(line) = lines.last_mut() {
            while line.chars().count() + 1 > max {
                line.pop();
            }
            line.push('…');
        }
    }
    lines
}

/// A step of 1, 2, 5 or 10 times a power of ten that gives about four
/// intervals over `range`.
fn nice_step(range: f64) -> f64 {
    let raw = range / 4.0;
    let magnitude = 10f64.powf(raw.log10().floor());
    let norm = raw / magnitude;
    let factor = [1.0, 2.0, 5.0, 10.0]
        .into_iter()
        .find(|&f| f >= norm - 1e-9)
        .unwrap_or(10.0);
    factor * magnitude
}

/// A tick label with the decimals the step needs.
fn tick_text(value: f64, step: f64) -> String {
    let decimals = if step >= 1.0 {
        0
    } else {
        (-step.log10()).ceil() as usize
    };
    let text = format!("{value:.decimals$}");
    if text.starts_with('-') && text.trim_start_matches(['-', '0', '.']).is_empty() {
        text.trim_start_matches('-').to_string()
    } else {
        text
    }
}

/// The turn labels' step: at most ten labels.
fn turn_step(last: u32) -> u32 {
    [1, 2, 5, 10, 20, 50, 100]
        .into_iter()
        .find(|s| last.div_ceil(*s) <= 10)
        .unwrap_or(100)
}

/// The value a point is drawn at: the median, or an event's share in percent.
fn drawn(curve: &Curve, p: &Point) -> Option<f64> {
    if curve.rate {
        p.mean.map(|m| m * 100.0)
    } else {
        p.median
    }
}

/// The chart of `curve` in `set`.
pub fn svg(set: &CurveSet, curve: &Curve) -> String {
    let title = if curve.rate {
        format!("Event {}: {}", curve.key, curve.meaning)
    } else {
        format!("{}: {}", curve.key, curve.meaning)
    };
    let title_lines = wrap(&title, TITLE_CHARS);
    let subtitle = format!(
        "{}{}; n = {} ({}) and {} ({})",
        if curve.rate {
            "share of games, in percent, per game turn"
        } else {
            "median per game turn"
        },
        if curve.lower_is_better {
            " (less is better)"
        } else {
            ""
        },
        set.games[0],
        set.labels[0],
        set.games[1],
        set.labels[1]
    );
    let subtitle_y = 20.0 + 16.0 * title_lines.len() as f64 + 2.0;
    let legend_y = subtitle_y + 18.0;
    let top = legend_y + 16.0;

    // The y range: from 0 (lower if a value is negative) to a round value.
    let values: Vec<f64> = curve
        .lines
        .iter()
        .flatten()
        .flat_map(|p| {
            if curve.rate {
                vec![drawn(curve, p)]
            } else {
                vec![p.p25, p.median, p.p75]
            }
        })
        .flatten()
        .collect();
    let low = values.iter().copied().fold(0.0, f64::min);
    let high = values.iter().copied().fold(0.0, f64::max);
    let step = nice_step(if high - low > 0.0 { high - low } else { 1.0 });
    let ymin = (low / step).floor() * step;
    let mut ymax = (high / step).ceil() * step;
    if ymax <= ymin {
        ymax = ymin + step;
    }
    let y_of = |v: f64| BOTTOM - (v - ymin) / (ymax - ymin) * (BOTTOM - top);
    let last = set.last_turn.max(1);
    let x_of = |turn: u32| {
        if last == 1 {
            (LEFT + RIGHT) / 2.0
        } else {
            LEFT + f64::from(turn - 1) / f64::from(last - 1) * (RIGHT - LEFT)
        }
    };

    let mut out = String::new();
    let _ = writeln!(
        out,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {WIDTH:.0} {HEIGHT:.0}\" width=\"{WIDTH:.0}\" height=\"{HEIGHT:.0}\" role=\"img\" aria-labelledby=\"t\">"
    );
    let _ = writeln!(out, "<title id=\"t\">{}</title>", esc(&title));
    let _ = writeln!(out, "<style>{STYLE}</style>");
    let _ = writeln!(
        out,
        "<rect class=\"bg\" width=\"{WIDTH:.0}\" height=\"{HEIGHT:.0}\"/>"
    );
    for (i, line) in title_lines.iter().enumerate() {
        let _ = writeln!(
            out,
            "<text class=\"tx\" x=\"16\" y=\"{:.1}\" font-size=\"13\" font-weight=\"600\">{}</text>",
            20.0 + 16.0 * i as f64,
            esc(line)
        );
    }
    let _ = writeln!(
        out,
        "<text class=\"mu\" x=\"16\" y=\"{subtitle_y:.1}\" font-size=\"11\">{}</text>",
        esc(&subtitle)
    );

    // The legend: a short line and the name, for each group.
    let mut x = 16.0;
    for (i, label) in set.labels.iter().enumerate() {
        let n = i + 1;
        let _ = writeln!(
            out,
            "<line class=\"s{n}\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke-width=\"2\" stroke-linecap=\"round\"/>",
            legend_y - 4.0,
            x + 18.0,
            legend_y - 4.0
        );
        let _ = writeln!(
            out,
            "<text class=\"tx\" x=\"{:.1}\" y=\"{legend_y:.1}\" font-size=\"12\">{}</text>",
            x + 24.0,
            esc(label)
        );
        x += 24.0 + label.chars().count() as f64 * 6.6 + 24.0;
    }

    if !curve.rate {
        let _ = writeln!(
            out,
            "<text class=\"mu\" x=\"{:.1}\" y=\"{legend_y:.1}\" font-size=\"11\" text-anchor=\"end\">line: median; band: middle half of the games</text>",
            WIDTH - 16.0
        );
    }

    // The grid and the y labels.
    let mut tick = ymin;
    let mut guard = 0;
    while tick <= ymax + step * 1e-6 && guard < 20 {
        let y = y_of(tick);
        let _ = writeln!(
            out,
            "<line class=\"gr\" x1=\"{LEFT:.1}\" y1=\"{y:.1}\" x2=\"{RIGHT:.1}\" y2=\"{y:.1}\" stroke-width=\"1\"/>"
        );
        let _ = writeln!(
            out,
            "<text class=\"mu\" x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" text-anchor=\"end\">{}</text>",
            LEFT - 6.0,
            y + 4.0,
            tick_text(tick, step)
        );
        tick += step;
        guard += 1;
    }

    // The x labels.
    let every = turn_step(last);
    for turn in (1..=last).filter(|t| t % every == 0) {
        let _ = writeln!(
            out,
            "<text class=\"mu\" x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" text-anchor=\"middle\">{turn}</text>",
            x_of(turn),
            BOTTOM + 16.0
        );
    }
    let _ = writeln!(
        out,
        "<text class=\"mu\" x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" text-anchor=\"middle\">game turn</text>",
        (LEFT + RIGHT) / 2.0,
        BOTTOM + 32.0
    );

    // The turn from which the groups stay apart.
    if let Some(turn) = curve.separates_from.filter(|t| *t <= last) {
        let x = x_of(turn);
        let _ = writeln!(
            out,
            "<line class=\"ds\" x1=\"{x:.1}\" y1=\"{top:.1}\" x2=\"{x:.1}\" y2=\"{BOTTOM:.1}\" stroke-width=\"1\" stroke-dasharray=\"4 3\"/>"
        );
        let (anchor, dx) = if x > RIGHT - 60.0 {
            ("end", -4.0)
        } else {
            ("start", 4.0)
        };
        let _ = writeln!(
            out,
            "<text class=\"mu halo\" x=\"{:.1}\" y=\"{:.1}\" font-size=\"11\" text-anchor=\"{anchor}\" stroke-width=\"3\" paint-order=\"stroke\">turn {turn}</text>",
            x + dx,
            top + 12.0
        );
    }

    // The bands (middle half of the games) behind the lines.
    if !curve.rate {
        for (i, line) in curve.lines.iter().enumerate() {
            let mut run: Vec<(f64, f64, f64)> = Vec::new();
            let mut bands = String::new();
            for p in line
                .iter()
                .filter(|p| p.turn <= last)
                .map(Some)
                .chain([None])
            {
                match p.and_then(|p| Some((p.turn, p.p25?, p.p75?))) {
                    Some((turn, low, high)) => run.push((x_of(turn), y_of(low), y_of(high))),
                    None => {
                        if run.len() > 1 {
                            let upper: Vec<String> = run
                                .iter()
                                .map(|(x, _, y)| format!("{x:.1} {y:.1}"))
                                .collect();
                            let lower: Vec<String> = run
                                .iter()
                                .rev()
                                .map(|(x, y, _)| format!("{x:.1} {y:.1}"))
                                .collect();
                            let _ = write!(bands, "M{} L{} Z ", upper.join(" L"), lower.join(" L"));
                        }
                        run.clear();
                    }
                }
            }
            if !bands.is_empty() {
                let _ = writeln!(
                    out,
                    "<path class=\"f{}\" fill-opacity=\"0.15\" d=\"{}\"/>",
                    i + 1,
                    bands.trim_end()
                );
            }
        }
    }

    // The lines, in segments where there is a value.
    let mut ends: Vec<(usize, f64)> = Vec::new();
    for (i, line) in curve.lines.iter().enumerate() {
        let n = i + 1;
        let mut path = String::new();
        let mut run: Vec<(f64, f64)> = Vec::new();
        let flush = |run: &mut Vec<(f64, f64)>, path: &mut String, out: &mut String| {
            match run.as_slice() {
                [] => {}
                [(x, y)] => {
                    let _ = writeln!(
                        out,
                        "<circle class=\"f{n}\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"2\"/>"
                    );
                }
                points => {
                    for (j, (x, y)) in points.iter().enumerate() {
                        let _ = write!(path, "{}{x:.1} {y:.1}", if j == 0 { "M" } else { " L" });
                    }
                    path.push(' ');
                }
            }
            run.clear();
        };
        let mut dots = String::new();
        for p in line.iter().filter(|p| p.turn <= last) {
            match drawn(curve, p) {
                Some(v) => run.push((x_of(p.turn), y_of(v))),
                None => flush(&mut run, &mut path, &mut dots),
            }
        }
        flush(&mut run, &mut path, &mut dots);
        if !path.is_empty() {
            let _ = writeln!(
                out,
                "<path class=\"s{n}\" d=\"{}\" fill=\"none\" stroke-width=\"2\" stroke-linejoin=\"round\" stroke-linecap=\"round\"/>",
                path.trim_end()
            );
        }
        out.push_str(&dots);
        if let Some(v) = line.iter().rev().find_map(|p| drawn(curve, p)) {
            ends.push((i, y_of(v)));
        }
    }

    // The names at the right end, apart from each other.
    ends.sort_by(|a, b| a.1.total_cmp(&b.1));
    if let [first, second] = ends.as_mut_slice() {
        let gap = 14.0;
        if second.1 - first.1 < gap {
            let push = (gap - (second.1 - first.1)) / 2.0;
            first.1 -= push;
            second.1 += push;
        }
    }
    for (i, y) in ends {
        let n = i + 1;
        let y = y.clamp(top + 6.0, BOTTOM - 2.0);
        let _ = writeln!(
            out,
            "<circle class=\"f{n}\" cx=\"{:.1}\" cy=\"{y:.1}\" r=\"4\"/>",
            RIGHT + 12.0
        );
        let _ = writeln!(
            out,
            "<text class=\"tx\" x=\"{:.1}\" y=\"{:.1}\" font-size=\"12\">{}</text>",
            RIGHT + 21.0,
            y + 4.0,
            esc(&set.labels[i])
        );
    }
    out.push_str("</svg>\n");
    out
}

#[cfg(test)]
mod tests;
