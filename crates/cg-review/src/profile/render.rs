//! The report of an [`Analysis`] as Markdown, narrow enough for a phone:
//! no table has more than seven columns.

use std::fmt::Write as _;

use super::{Analysis, Profile, Sources, Tally, FEW_GAMES, SMALL};

/// A number with at most one decimal.
pub(crate) fn num(value: Option<f64>) -> String {
    match value {
        None => "-".to_string(),
        Some(v) if (v - v.round()).abs() < 0.05 => format!("{:.0}", v.round()),
        Some(v) => format!("{v:.1}"),
    }
}

/// " [n = 2]" for a small sample.
fn flag(n: usize) -> String {
    if n > 0 && n < SMALL {
        format!(" [n = {n}]")
    } else {
        String::new()
    }
}

/// A median or mean with the games it comes from, flagged when few.
fn num_n(value: Option<f64>, n: usize) -> String {
    match value {
        None => "-".to_string(),
        Some(_) => format!("{}{}", num(value), flag(n)),
    }
}

/// "60% (3/5)", flagged when `n` is small.
pub(crate) fn share(k: usize, n: usize) -> String {
    if n == 0 {
        return "- (0)".to_string();
    }
    format!("{}% ({k}/{n}){}", (k * 100 + n / 2) / n, flag(n))
}

fn tally(t: &Tally) -> String {
    if t.drawn > 0 {
        format!("{}-{}-{}", t.won, t.lost, t.drawn)
    } else {
        format!("{}-{}", t.won, t.lost)
    }
}

pub(crate) fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

pub(crate) fn table(out: &mut String, headers: &[&str], rows: &[Vec<String>]) {
    if rows.is_empty() {
        out.push_str("(none)\n\n");
        return;
    }
    let _ = writeln!(out, "| {} |", headers.join(" | "));
    let _ = writeln!(
        out,
        "|{}|",
        headers
            .iter()
            .enumerate()
            .map(|(i, _)| if i == 0 { "---" } else { "---:" })
            .collect::<Vec<_>>()
            .join("|")
    );
    for row in rows {
        let _ = writeln!(
            out,
            "| {} |",
            row.iter().map(|c| cell(c)).collect::<Vec<_>>().join(" | ")
        );
    }
    out.push('\n');
}

fn counts(map: &std::collections::BTreeMap<String, usize>) -> String {
    let mut items: Vec<(&String, &usize)> = map.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    items
        .iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// The report: sources, opponents, profiles, our weaknesses and the games.
pub fn markdown(a: &Analysis) -> String {
    let mut out = String::new();
    sources(&mut out, &a.sources);
    opponents(&mut out, a);
    out.push_str("## Profiles\n\n");
    if a.profiles.is_empty() {
        out.push_str("No focus player has a game.\n\n");
    }
    for p in &a.profiles {
        profile(&mut out, p);
    }
    weaknesses(&mut out, a);
    games(&mut out, a);
    out
}

pub(crate) fn sources(out: &mut String, sources: &Sources) {
    out.push_str("## Sources\n\n");
    for w in &sources.warnings {
        let _ = writeln!(out, "**Warning:** {w}\n");
    }
    for n in &sources.notes {
        let _ = writeln!(out, "{n}\n");
    }
    let rows: Vec<Vec<String>> = sources
        .by_kind
        .iter()
        .map(|k| {
            vec![
                k.kind.clone(),
                k.batch.clone().unwrap_or_else(|| "-".to_string()),
                k.games.to_string(),
            ]
        })
        .collect();
    table(out, &["kind", "batch", "games"], &rows);
    let total: usize = sources.by_kind.iter().map(|k| k.games).sum();
    let _ = writeln!(out, "{total} games used.");
    match &sources.snapshot {
        Some(s) => {
            let _ = writeln!(out, "Ranks from the leaderboard snapshot {s}.");
        }
        None => out.push_str("No leaderboard snapshot: players are unranked.\n"),
    }
    if sources.left_out.is_empty() {
        out.push_str("No game left out.\n\n");
    } else {
        let _ = writeln!(out, "\n{} games left out:\n", sources.left_out.len());
        for l in &sources.left_out {
            let id = l.game_id.map_or("-".to_string(), |g| g.to_string());
            let _ = writeln!(out, "- {id}: {}", l.why);
        }
        out.push('\n');
    }
}

fn opponents(out: &mut String, a: &Analysis) {
    out.push_str("## Opponents\n\n");
    out.push_str(
        "Results are ours (won-lost, draws last when any). Seat columns: our seat 0 and our seat 1.\n\n",
    );
    let rows: Vec<Vec<String>> = a
        .opponents
        .iter()
        .map(|o| {
            vec![
                o.player.clone(),
                o.rank.map_or("-".to_string(), |r| r.to_string()),
                format!("{}{}", o.games, flag(o.games)),
                tally(&o.ours),
                format!("{} / {}", tally(&o.by_seat[0]), tally(&o.by_seat[1])),
                num(o.median_turns),
            ]
        })
        .collect();
    table(
        out,
        &["player", "rank", "games", "won-lost", "seat 0 / 1", "turns"],
        &rows,
    );
    // The most common ending overall is the usual one; only the opponents
    // with another ending are listed, the rest would repeat it.
    let mut overall: std::collections::BTreeMap<String, usize> = Default::default();
    for o in &a.opponents {
        for (end, n) in &o.ends {
            *overall.entry(end.clone()).or_default() += n;
        }
    }
    let usual = overall
        .iter()
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
        .map(|(end, _)| end.clone());
    let _ = writeln!(
        out,
        "How the games ended, over all opponents: {}.\n",
        counts(&overall)
    );
    let rows: Vec<Vec<String>> = a
        .opponents
        .iter()
        .filter(|o| o.ends.keys().any(|end| Some(end) != usual.as_ref()))
        .map(|o| vec![o.player.clone(), counts(&o.ends)])
        .collect();
    if !rows.is_empty() {
        let _ = writeln!(
            out,
            "Opponents with a game that did not end with \"{}\":\n",
            usual.unwrap_or_default()
        );
        table(out, &["player", "endings"], &rows);
    }
}

fn profile(out: &mut String, p: &Profile) {
    let rank = p
        .rank
        .map_or("unranked".to_string(), |r| format!("rank {r}"));
    let _ = writeln!(out, "### {} ({rank}, {} games)\n", p.player, p.games);
    if p.few_games {
        let _ = writeln!(out, "Few games (fewer than {FEW_GAMES}): read with care.\n");
    }
    if p.games == 0 {
        return;
    }
    out.push_str("**Results against us**, by their seat (won-lost):\n\n");
    let mut rows = Vec::new();
    for (seat, t) in p.vs_us.iter().enumerate() {
        rows.push(vec![
            format!("seat {seat}"),
            format!("{}{}", t.games(), flag(t.games())),
            tally(t),
        ]);
    }
    let all = Tally {
        won: p.vs_us[0].won + p.vs_us[1].won,
        lost: p.vs_us[0].lost + p.vs_us[1].lost,
        drawn: p.vs_us[0].drawn + p.vs_us[1].drawn,
    };
    rows.push(vec![
        "both".to_string(),
        format!("{}{}", all.games(), flag(all.games())),
        tally(&all),
    ]);
    table(out, &["their seat", "games", "won-lost"], &rows);

    out.push_str("**Key metrics at game turns**: their median / ours, each side read right after its own play of that turn (the value its play left). Row `games` counts the games behind each turn (theirs / ours).\n\n");
    let mut turns: Vec<u32> = p
        .metrics
        .iter()
        .flat_map(|m| m.at_turn.iter().map(|c| c.turn))
        .collect();
    turns.sort_unstable();
    turns.dedup();
    let heads: Vec<String> = std::iter::once("metric".to_string())
        .chain(turns.iter().map(|t| format!("t{t}")))
        .collect();
    let heads: Vec<&str> = heads.iter().map(String::as_str).collect();
    let mut rows = Vec::new();
    if let Some(first) = p.metrics.first() {
        let mut row = vec!["games".to_string()];
        for t in &turns {
            row.push(
                first
                    .at_turn
                    .iter()
                    .find(|c| c.turn == *t)
                    .map_or("-".to_string(), |c| {
                        format!("{} / {}{}", c.games, c.our_games, flag(c.games))
                    }),
            );
        }
        rows.push(row);
    }
    for m in &p.metrics {
        let mut row = vec![m.key.clone()];
        for t in &turns {
            row.push(
                m.at_turn
                    .iter()
                    .find(|c| c.turn == *t)
                    .map_or("-".to_string(), |c| {
                        format!("{} / {}", num(c.theirs), num(c.ours))
                    }),
            );
        }
        rows.push(row);
    }
    table(out, &heads, &rows);
    if !p.turns_dropped.is_empty() {
        let dropped: Vec<String> = p.turns_dropped.iter().map(|t| format!("t{t}")).collect();
        let _ = writeln!(
            out,
            "Left out: {}, which fewer than half of the games reach.\n",
            dropped.join(", ")
        );
    }

    out.push_str("**Key metrics by phase**: mean of the games' means, theirs / ours; each value is read at the start of that side's own plies in the phase.\n\n");
    let mut rows = Vec::new();
    if let Some(first) = p.metrics.first() {
        let mut row = vec!["games".to_string()];
        for c in &first.by_phase {
            row.push(format!("{} / {}{}", c.games, c.our_games, flag(c.games)));
        }
        rows.push(row);
    }
    for m in &p.metrics {
        let mut row = vec![m.key.clone()];
        for c in &m.by_phase {
            row.push(format!("{} / {}", num(c.theirs), num(c.ours)));
        }
        rows.push(row);
    }
    let heads: Vec<&str> = std::iter::once("metric")
        .chain(
            p.metrics
                .first()
                .map(|m| {
                    m.by_phase
                        .iter()
                        .map(|c| c.phase.as_str())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        )
        .collect();
    table(out, &heads, &rows);

    out.push_str("**Events**: share of games where it happens, for them and for us (games against us); their median first game turn; mean count a game, theirs / ours.\n\n");
    let rows: Vec<Vec<String>> = p
        .events
        .iter()
        .map(|e| {
            vec![
                e.name.clone(),
                share(e.theirs_in, e.games),
                share(e.ours_in, e.our_games),
                num_n(e.first_turn, e.theirs_in),
                format!("{} / {}", num(e.mean_theirs), num(e.mean_ours)),
            ]
        })
        .collect();
    table(
        out,
        &["event", "theirs", "ours", "first turn", "per game"],
        &rows,
    );

    out.push_str("**Moments**: games with at least one, in their wins and in their losses; all moments counted; median game turn.\n\n");
    let rows: Vec<Vec<String>> = p
        .moments
        .iter()
        .map(|m| {
            vec![
                m.kind.clone(),
                format!("for {}", m.favours),
                share(m.in_wins.with, m.in_wins.games),
                share(m.in_losses.with, m.in_losses.games),
                m.total.to_string(),
                num_n(m.median_turn, m.total),
            ]
        })
        .collect();
    table(
        out,
        &["kind", "favours", "in wins", "in losses", "all", "turn"],
        &rows,
    );

    let _ = writeln!(
        out,
        "**Openings**: their most common first {} own plies.\n",
        p.opening_plies
    );
    let rows: Vec<Vec<String>> = p
        .openings
        .iter()
        .map(|o| {
            vec![
                format!("`{}`", o.opening),
                format!("{}{}", o.games, flag(o.games)),
                share(o.wins, o.games),
            ]
        })
        .collect();
    table(out, &["opening", "games", "win rate"], &rows);

    let _ = writeln!(out, "**How its wins came** ({} wins):\n", p.wins.wins);
    if p.wins.wins > 0 {
        let _ = writeln!(out, "Endings: {}.\n", counts(&p.wins.ends));
        let rows: Vec<Vec<String>> = p
            .wins
            .first_moments
            .iter()
            .map(|m| {
                vec![
                    m.kind.clone(),
                    share(m.wins, p.wins.wins),
                    num_n(m.median_turn, m.wins),
                ]
            })
            .collect();
        out.push_str("First moment favouring it in each win:\n\n");
        table(out, &["kind", "wins", "median turn"], &rows);
    }
}

fn weaknesses(out: &mut String, a: &Analysis) {
    let w = &a.weaknesses;
    out.push_str("## Our weaknesses\n\n");
    let _ = writeln!(
        out,
        "Our games only: {} losses, {} of them against the focus players.\n",
        w.losses, w.focus_losses
    );
    if w.losses == 0 {
        out.push_str("No loss.\n\n");
        return;
    }
    out.push_str("Turns are compared with each side read right after its own ply of that turn (for its last ply, at its start); a turn at which either side has no ply is skipped.\n\n");
    out.push_str("Moments against us in our losses, by phase (early / middle / end), counted:\n\n");
    let rows: Vec<Vec<String>> = w
        .against_us
        .iter()
        .map(|m| {
            vec![
                m.kind.clone(),
                m.group.clone(),
                m.by_phase[0].to_string(),
                m.by_phase[1].to_string(),
                m.by_phase[2].to_string(),
            ]
        })
        .collect();
    table(out, &["kind", "against", "early", "middle", "end"], &rows);
    out.push_str("Metrics of ours that stay below the opponent's from some turn to the end of our losses (for metrics where less is better, such as a gap or an exposure, read it the other way):\n\n");
    let rows: Vec<Vec<String>> = w
        .stays_below
        .iter()
        .map(|s| {
            vec![
                s.key.clone(),
                share(s.below, s.losses),
                num_n(s.median_turn, s.below),
            ]
        })
        .collect();
    table(
        out,
        &["metric", "losses where below", "median from turn"],
        &rows,
    );
    let _ = writeln!(
        out,
        "The {} losses where the lead (`{}`) fell behind for good earliest:\n",
        w.earliest.len(),
        w.lead
    );
    let rows: Vec<Vec<String>> = w
        .earliest
        .iter()
        .map(|e| {
            vec![
                e.game_id.to_string(),
                e.opponent.clone(),
                e.turn.to_string(),
            ]
        })
        .collect();
    table(out, &["game", "opponent", "from turn"], &rows);
}

fn games(out: &mut String, a: &Analysis) {
    out.push_str("## Games\n\nGame ids by opponent, with our result (W, L, D).\n\n");
    for l in &a.games {
        let ids: Vec<String> = l.games.iter().map(|(id, r)| format!("{id} {r}")).collect();
        let _ = writeln!(
            out,
            "- **{}** ({}): {}",
            l.label,
            l.games.len(),
            ids.join(", ")
        );
    }
}
