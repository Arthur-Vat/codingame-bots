//! The files of the report besides the charts and the turn-by-turn data:
//! one CSV for each kind of table (long format, a `player` or `group`
//! column), `README.md` and `signals.md`.

use std::fmt::Write as _;

use super::data::{field, num};
use super::signals::signals;
use super::{Analysis, CurveSet, SpendGroup, Tally};

/// A CSV file being written.
struct Csv(String);

impl Csv {
    fn new(header: &[&str]) -> Csv {
        Csv(format!("{}\n", header.join(",")))
    }
    fn row(&mut self, cells: &[String]) {
        let cells: Vec<String> = cells.iter().map(|c| field(c)).collect();
        let _ = writeln!(self.0, "{}", cells.join(","));
    }
}

fn opt(value: Option<f64>) -> String {
    value.map_or(String::new(), num)
}

fn text<T: ToString>(value: T) -> String {
    value.to_string()
}

fn tally(t: &Tally) -> [String; 3] {
    [text(t.won), text(t.lost), text(t.drawn)]
}

/// `(file name, description with its columns)` of each table, in order.
pub const TABLES: [(&str, &str); 28] = [
    ("sources.csv", "the games used, by `kind` (ranking, test, replay) and `batch`: `games`."),
    ("left_out.csv", "the games left out: `game_id` (empty for a bad index line) and `why`."),
    ("opponents.csv", "our opponents: `player`, `rank` (leaderboard snapshot, empty if unranked), `games`, our `won`, `lost`, `drawn`, the same when we sat in seat 0 (`seat0_won`, `seat0_lost`, `seat0_drawn`) and in seat 1 (`seat1_won`, `seat1_lost`, `seat1_drawn`), `median_turns`."),
    ("opponent_ends.csv", "how our games against each `player` ended: `end` and its number of `games`."),
    ("profile_results.csv", "each focus `player`: `rank`, `games`, and their results against us by `their_seat`: `won`, `lost`, `drawn`."),
    ("profile_metrics_turn.csv", "a focus `player`'s key `metric` at a game `turn`: the median over `games` of theirs (`theirs`) and over `our_games` of ours in the same games (`ours`), each side read right after its own ply of that turn; only turns that half of their games reach."),
    ("profile_metrics_phase.csv", "a focus `player`'s key `metric` by `phase`: the mean over games (`games` theirs, `our_games` ours) of each game's mean, theirs (`theirs`) and ours (`ours`), read at the starts of the side's own plies."),
    ("profile_events.csv", "a focus `player`'s `event`: of its `games`, `theirs_in` had it; of `our_games` against it, `ours_in` had it; `first_turn` is the median game turn of their first time; `mean_theirs` and `mean_ours` are the counts a game."),
    ("profile_moments.csv", "a focus `player`'s decisive moments by `kind`, `favours` (them or us), in its wins (`wins_with` of `wins_games` games had one) and losses (`losses_with` of `losses_games`); `total` moments, `median_turn`."),
    ("profile_openings.csv", "the most common `opening` of each `player` (its first own plies, tokens separated by ` / `): `games` and `wins`."),
    ("profile_wins.csv", "how a focus `player`'s wins came: `item` `ending` (`name` is how it ended, `wins` how many) or `first_moment` (`name` is the kind of the first moment that favoured it, `wins` how many wins, `median_turn`)."),
    ("weaknesses_moments.csv", "moments against us in our losses: `kind`, `group` (focus or others), counts in the `early`, `middle` and `end` phases."),
    ("weaknesses_stays_below.csv", "per `metric`: in how many of our `losses` (and of our `wins`, the control) our value stays below the opponent's from some turn to the end (`losses_below`, `wins_below`), and the `median_turn` of that turn in the losses. For metrics where less is better, read it the other way."),
    ("weaknesses_earliest.csv", "the losses where the lead metric (`lead`) fell behind for good earliest: `game_id`, `opponent`, `from_turn`."),
    ("separation.csv", "for each `set` of curves (see below), `metric` and game `turn`: the AUC of the first group against the second (`auc`, empty if a group has fewer than 5 values), the groups' medians (`first_median`, `second_median`) and games (`first_n`, `second_n`), and `separates_from`, the turn from which the groups stay apart (empty if never); `lower_is_better` says how the AUC is read."),
    ("spending.csv", "mean of the game's resource spent per own turn, by the categories of its spending hook: `group`, `phase`, `plies` (own plies behind it), `category` and `mean_spent`."),
    ("spending_kept.csv", "the share of the resource in hand left unspent at the end of an own turn, in percent (`kept_pct`, mean over turns): `group`, `phase`, `plies`."),
    ("games.csv", "the games: `label` (the opponent), `game_id` and our `result` (W, L, D)."),
    ("profile_versions.csv", "each focus `player`'s versions, one row for each agent id it played with in the games (`agent_id`, empty if the index does not name it): `games`, its results against us (`won`, `lost`, `drawn`) and the first and last day of its games (`first_seen`, `last_seen`); a player with several rows changed its code."),
    ("versions.csv", "the agent id of each tracked top `player` in the leaderboard snapshot of each `date` (the latest of the day): `snapshot`, `rank`, `agent_id` and `changed`, true if the agent id differs from the one in the previous day's snapshot."),
    ("economy_exchange.csv", "the exchange of units by `group` and `phase`: over `games` (sides with a ply in the phase), the mean of what the side removed of the enemy's (`killed`), what it lost (`lost`) and their difference (`balance`), as the training cost of the units, each read right after the side's last own ply of the phase."),
    ("economy_efficiency.csv", "efficiency by `group`, `name` and game `turn`: the `median` over `games` sides of the cumulative gain over the cumulative resource spent (see `README.md` for each name's definition); `games` is 0 and `median` empty when no side has spent anything."),
    ("economy_mix.csv", "the investment mix by `group` and `phase`: over `plies` own plies, the resource spent on each `role` per ply (`spent_per_ply`) and its share of all spending in the phase (`share_pct`, empty if nothing was spent)."),
    ("economy_upkeep.csv", "the upkeep burden by `group` and `phase`: over `plies` own plies, the mean of upkeep as a percentage of gross income (`burden_pct`), at the starts of the side's own plies; plies without gross income are left out of the mean."),
    ("economy_tempo.csv", "tempo by `group`: of its `games` sides, `contact_games` made contact (the metric of first contact above zero) with the median turn `contact_turn`; `decisive_games` had a decisive moment in their favour, median turn `decisive_turn`; of the `both_games` with both, `to_decisive` is the median of the turns from first contact to the first decisive moment (negative when it came first)."),
    ("economy_lead.csv", "the value of a lead in our games: for a lead `metric` (ours less theirs, read so that more is an advantage) at a game `turn`, in each `bin` of lead size (relative to the median size of the leads at that turn: `up to` it, or `more than` it): our `games` and how many we won (`wins`)."),
    ("economy_curves.csv", "the economy curves turn by turn, for each `set` (see below) and `series`: the medians of the first and second group (`first_median`, `second_median`) over `first_n` and `second_n` games, for each game `turn` that has a value."),
    ("signals.md", "the strongest findings picked by code (no interpretation), each with its chart and data file."),
];

fn sources(a: &Analysis) -> (Csv, Csv) {
    let mut kinds = Csv::new(&["kind", "batch", "games"]);
    for k in &a.sources.by_kind {
        kinds.row(&[
            k.kind.clone(),
            k.batch.clone().unwrap_or_default(),
            text(k.games),
        ]);
    }
    let mut left = Csv::new(&["game_id", "why"]);
    for l in &a.sources.left_out {
        left.row(&[l.game_id.map_or(String::new(), text), l.why.clone()]);
    }
    (kinds, left)
}

fn opponents(a: &Analysis) -> (Csv, Csv) {
    let mut rows = Csv::new(&[
        "player",
        "rank",
        "games",
        "won",
        "lost",
        "drawn",
        "seat0_won",
        "seat0_lost",
        "seat0_drawn",
        "seat1_won",
        "seat1_lost",
        "seat1_drawn",
        "median_turns",
    ]);
    let mut ends = Csv::new(&["player", "end", "games"]);
    for o in &a.opponents {
        let mut row = vec![
            o.player.clone(),
            o.rank.map_or(String::new(), text),
            text(o.games),
        ];
        row.extend(tally(&o.ours));
        row.extend(tally(&o.by_seat[0]));
        row.extend(tally(&o.by_seat[1]));
        row.push(opt(o.median_turns));
        rows.row(&row);
        for (end, n) in &o.ends {
            ends.row(&[o.player.clone(), end.clone(), text(n)]);
        }
    }
    (rows, ends)
}

/// The profile tables, in the order of `TABLES` (from `profile_results.csv`
/// to `profile_wins.csv`).
fn profiles(a: &Analysis) -> Vec<Csv> {
    let mut results = Csv::new(&[
        "player",
        "rank",
        "games",
        "their_seat",
        "won",
        "lost",
        "drawn",
    ]);
    let mut turn = Csv::new(&[
        "player",
        "metric",
        "turn",
        "games",
        "theirs",
        "our_games",
        "ours",
    ]);
    let mut phase = Csv::new(&[
        "player",
        "metric",
        "phase",
        "games",
        "theirs",
        "our_games",
        "ours",
    ]);
    let mut events = Csv::new(&[
        "player",
        "event",
        "games",
        "theirs_in",
        "our_games",
        "ours_in",
        "first_turn",
        "mean_theirs",
        "mean_ours",
    ]);
    let mut moments = Csv::new(&[
        "player",
        "kind",
        "favours",
        "wins_with",
        "wins_games",
        "losses_with",
        "losses_games",
        "total",
        "median_turn",
    ]);
    let mut openings = Csv::new(&["player", "opening", "games", "wins"]);
    let mut wins = Csv::new(&["player", "item", "name", "wins", "median_turn"]);
    for p in &a.profiles {
        let rank = p.rank.map_or(String::new(), text);
        for (seat, t) in p.vs_us.iter().enumerate() {
            let mut row = vec![p.player.clone(), rank.clone(), text(p.games), text(seat)];
            row.extend(tally(t));
            results.row(&row);
        }
        for m in &p.metrics {
            for c in &m.at_turn {
                turn.row(&[
                    p.player.clone(),
                    m.key.clone(),
                    text(c.turn),
                    text(c.games),
                    opt(c.theirs),
                    text(c.our_games),
                    opt(c.ours),
                ]);
            }
            for c in &m.by_phase {
                phase.row(&[
                    p.player.clone(),
                    m.key.clone(),
                    c.phase.clone(),
                    text(c.games),
                    opt(c.theirs),
                    text(c.our_games),
                    opt(c.ours),
                ]);
            }
        }
        for e in &p.events {
            events.row(&[
                p.player.clone(),
                e.name.clone(),
                text(e.games),
                text(e.theirs_in),
                text(e.our_games),
                text(e.ours_in),
                opt(e.first_turn),
                opt(e.mean_theirs),
                opt(e.mean_ours),
            ]);
        }
        for m in &p.moments {
            moments.row(&[
                p.player.clone(),
                m.kind.clone(),
                m.favours.clone(),
                text(m.in_wins.with),
                text(m.in_wins.games),
                text(m.in_losses.with),
                text(m.in_losses.games),
                text(m.total),
                opt(m.median_turn),
            ]);
        }
        for o in &p.openings {
            openings.row(&[
                p.player.clone(),
                o.opening.clone(),
                text(o.games),
                text(o.wins),
            ]);
        }
        for (end, n) in &p.wins.ends {
            wins.row(&[
                p.player.clone(),
                "ending".to_string(),
                end.clone(),
                text(n),
                String::new(),
            ]);
        }
        for m in &p.wins.first_moments {
            wins.row(&[
                p.player.clone(),
                "first_moment".to_string(),
                m.kind.clone(),
                text(m.wins),
                opt(m.median_turn),
            ]);
        }
    }
    vec![results, turn, phase, events, moments, openings, wins]
}

fn weaknesses(a: &Analysis) -> [Csv; 3] {
    let w = &a.weaknesses;
    let mut moments = Csv::new(&["kind", "group", "early", "middle", "end"]);
    for m in &w.against_us {
        moments.row(&[
            m.kind.clone(),
            m.group.clone(),
            text(m.by_phase[0]),
            text(m.by_phase[1]),
            text(m.by_phase[2]),
        ]);
    }
    let mut below = Csv::new(&[
        "metric",
        "losses",
        "losses_below",
        "wins",
        "wins_below",
        "median_turn",
    ]);
    for s in &w.stays_below {
        below.row(&[
            s.key.clone(),
            text(s.losses),
            text(s.below),
            text(s.wins),
            text(s.wins_below),
            opt(s.median_turn),
        ]);
    }
    let mut earliest = Csv::new(&["game_id", "opponent", "from_turn", "lead"]);
    for e in &w.earliest {
        earliest.row(&[
            text(e.game_id),
            e.opponent.clone(),
            text(e.turn),
            w.lead.clone(),
        ]);
    }
    [moments, below, earliest]
}

fn separation(a: &Analysis) -> Csv {
    let mut csv = Csv::new(&[
        "set",
        "metric",
        "lower_is_better",
        "turn",
        "auc",
        "first_median",
        "second_median",
        "first_n",
        "second_n",
        "separates_from",
    ]);
    for set in sets(a) {
        for c in &set.curves {
            for (i, auc) in c.auc.iter().enumerate() {
                csv.row(&[
                    set.slug.clone(),
                    c.key.clone(),
                    text(c.lower_is_better),
                    text(i + 1),
                    opt(*auc),
                    opt(c.lines[0][i].median),
                    opt(c.lines[1][i].median),
                    text(c.lines[0][i].games),
                    text(c.lines[1][i].games),
                    c.separates_from.map_or(String::new(), text),
                ]);
            }
        }
    }
    csv
}

/// Every set of curves.
fn sets(a: &Analysis) -> Vec<&CurveSet> {
    [&a.curves.top_vs_us, &a.curves.wins_vs_losses]
        .into_iter()
        .chain(&a.curves.players)
        .collect()
}

fn spending(a: &Analysis) -> (Csv, Csv) {
    let mut spent = Csv::new(&["group", "phase", "plies", "category", "mean_spent"]);
    let mut kept = Csv::new(&["group", "phase", "plies", "kept_pct"]);
    let groups: Vec<&SpendGroup> = a
        .spending
        .groups
        .iter()
        .chain(&a.spending.players)
        .collect();
    for g in groups {
        for cell in &g.by_phase {
            for (name, v) in a.spending.categories.iter().zip(&cell.spent) {
                spent.row(&[
                    g.label.clone(),
                    cell.phase.clone(),
                    text(cell.plies),
                    name.clone(),
                    num(*v),
                ]);
            }
            kept.row(&[
                g.label.clone(),
                cell.phase.clone(),
                text(cell.plies),
                opt(cell.kept),
            ]);
        }
    }
    (spent, kept)
}

fn versions_tables(a: &Analysis) -> (Csv, Csv) {
    let mut profile = Csv::new(&[
        "player",
        "agent_id",
        "games",
        "won",
        "lost",
        "drawn",
        "first_seen",
        "last_seen",
    ]);
    for p in &a.profiles {
        for v in &p.versions {
            let mut row = vec![
                p.player.clone(),
                v.agent_id.map_or(String::new(), text),
                text(v.games),
            ];
            row.extend(tally(&v.vs_us));
            row.push(v.first_seen.clone().unwrap_or_default());
            row.push(v.last_seen.clone().unwrap_or_default());
            profile.row(&row);
        }
    }
    let mut days = Csv::new(&["player", "date", "snapshot", "rank", "agent_id", "changed"]);
    for r in &a.versions.rows {
        days.row(&[
            r.player.clone(),
            r.date.clone(),
            r.snapshot.clone(),
            text(r.rank),
            r.agent_id.map_or(String::new(), text),
            text(r.changed),
        ]);
    }
    (profile, days)
}

/// The economy tables, in the order of `TABLES`.
fn economy_tables(a: &Analysis) -> Vec<Csv> {
    let e = &a.economy;
    let mut exchange = Csv::new(&["group", "phase", "games", "killed", "lost", "balance"]);
    for r in &e.exchange {
        exchange.row(&[
            r.group.clone(),
            r.phase.clone(),
            text(r.games),
            opt(r.killed),
            opt(r.lost),
            opt(r.balance),
        ]);
    }
    let mut efficiency = Csv::new(&["group", "name", "turn", "games", "median"]);
    for r in &e.efficiency {
        efficiency.row(&[
            r.group.clone(),
            r.name.clone(),
            text(r.turn),
            text(r.games),
            opt(r.median),
        ]);
    }
    let mut mix = Csv::new(&[
        "group",
        "phase",
        "plies",
        "role",
        "spent_per_ply",
        "share_pct",
    ]);
    for r in &e.mix {
        mix.row(&[
            r.group.clone(),
            r.phase.clone(),
            text(r.plies),
            r.role.clone(),
            num(r.spent_per_ply),
            opt(r.share),
        ]);
    }
    let mut upkeep = Csv::new(&["group", "phase", "plies", "burden_pct"]);
    for r in &e.upkeep {
        upkeep.row(&[
            r.group.clone(),
            r.phase.clone(),
            text(r.plies),
            opt(r.burden),
        ]);
    }
    let mut tempo = Csv::new(&[
        "group",
        "games",
        "contact_games",
        "contact_turn",
        "decisive_games",
        "decisive_turn",
        "both_games",
        "to_decisive",
    ]);
    for r in &e.tempo {
        tempo.row(&[
            r.group.clone(),
            text(r.games),
            text(r.contact_games),
            opt(r.contact_turn),
            text(r.decisive_games),
            opt(r.decisive_turn),
            text(r.both_games),
            opt(r.to_decisive),
        ]);
    }
    let mut lead = Csv::new(&["metric", "turn", "bin", "games", "wins"]);
    for r in &e.lead {
        lead.row(&[
            r.metric.clone(),
            text(r.turn),
            r.bin.clone(),
            text(r.games),
            text(r.wins),
        ]);
    }
    let mut curves = Csv::new(&[
        "set",
        "series",
        "turn",
        "first_median",
        "second_median",
        "first_n",
        "second_n",
    ]);
    for set in e.sets.iter().chain(&e.players) {
        for c in &set.curves {
            for (a, b) in c.lines[0].iter().zip(&c.lines[1]) {
                if a.games + b.games == 0 {
                    continue;
                }
                curves.row(&[
                    set.slug.clone(),
                    c.key.clone(),
                    text(a.turn),
                    opt(a.median),
                    opt(b.median),
                    text(a.games),
                    text(b.games),
                ]);
            }
        }
    }
    vec![exchange, efficiency, mix, upkeep, tempo, lead, curves]
}

fn games(a: &Analysis) -> Csv {
    let mut csv = Csv::new(&["label", "game_id", "result"]);
    for l in &a.games {
        for (id, r) in &l.games {
            csv.row(&[l.label.clone(), text(id), r.clone()]);
        }
    }
    csv
}

/// `README.md`: what the folder holds and each file's columns.
fn readme(a: &Analysis, charts: usize) -> String {
    let mut out = String::from("# Report data\n\nGenerated by `codingame-report` from the games listed in `sources.csv`; every number in the report's text comes from these files. Long tables, one row per item: a `player` or `group` column names whom a row is about; empty cells mean no data. Medians are over games, each side read right after its own ply of the turn.\n\n");
    for w in &a.sources.warnings {
        let _ = writeln!(out, "**Warning:** {w}\n");
    }
    for n in &a.sources.notes {
        let _ = writeln!(out, "{n}\n");
    }
    match &a.sources.snapshot {
        Some(s) => {
            let _ = writeln!(out, "Ranks come from the leaderboard snapshot {s}.\n");
        }
        None => out.push_str("No leaderboard snapshot: players are unranked.\n\n"),
    }
    out.push_str("## Tables\n\n");
    for (name, what) in TABLES {
        let _ = writeln!(out, "- `{name}`: {what}");
    }
    out.push_str("\n## Turn by turn\n\n");
    out.push_str("`turns-GROUP.csv`, one row for each side, metric (every metric of the traces) or event, and game turn that at least 3 games reach: `group,side,kind,name,turn,n,median,p25,p75,mean`. `kind` is `metric` or `event`; `n` the games; `p25` and `p75` the quartiles; for an event the value of a game is 1 if the side does it at that turn and 0 if not, so the `mean` is the share of games. Groups:\n\n");
    for g in &a.data {
        let _ = writeln!(
            out,
            "- `turns-{}.csv`: {} ({} games)",
            g.slug, g.label, g.games
        );
    }
    out.push_str("\n## Charts\n\n");
    let _ = writeln!(
        out,
        "{charts} SVG files, one metric or event each, named `SET-METRIC.svg` or `SET-event-NAME.svg`. The line is the median (an event: the share of games, in percent) at each game turn, the band the middle half of the games; blue is the first group of the set, orange the second; a dashed line marks the turn from which the groups stay apart. The sets (also the `set` column of `separation.csv`; first group, second group):\n"
    );
    for set in sets(a) {
        let _ = writeln!(
            out,
            "- `{}`: {} (n = {}), {} (n = {})",
            set.slug, set.labels[0], set.games[0], set.labels[1], set.games[1]
        );
    }
    out.push_str("\nThe sets `player-NAME` hold, for one focus player, the three metrics that tell our wins from our losses best.\n");
    out.push_str("\n## Economy or fighting\n\n");
    out.push_str("Is a strategy better because of its economy or because of its fighting? `economy_*.csv` hold the numbers; `signals.md` sets them side by side. The charts of the sets below (`SET-SERIES.svg`, also in `economy_curves.csv`; there is also one set `economy-player-NAME` for each focus player, in the CSV only) show, turn by turn, the exchange (the value of the enemy's units a side removed less the value of its own it lost, and each part), the efficiencies and the upkeep burden.\n\n");
    for set in a.economy.sets.iter().chain(&a.economy.players) {
        let _ = writeln!(
            out,
            "- `{}`: {} (n = {}), {} (n = {})",
            set.slug, set.labels[0], set.games[0], set.labels[1], set.games[1]
        );
    }
    if !a.economy.decisive.is_empty() {
        let _ = writeln!(
            out,
            "\nTempo counts as decisive the moments of the kinds: {}.",
            a.economy.decisive.join(", ")
        );
    }
    if let Some(set) = a.economy.sets.first() {
        out.push_str("\nThe series, with their definitions:\n\n");
        for c in &set.curves {
            let _ = writeln!(out, "- `{}`: {}", c.key, c.meaning);
        }
    }
    out
}

/// The tables, `README.md` and `signals.md`, as file name and text.
/// `charts` is the number of charts written beside them.
pub fn report_files(a: &Analysis, charts: usize) -> Vec<(String, String)> {
    let (kinds, left) = sources(a);
    let (opps, ends) = opponents(a);
    let mut profile = profiles(a).into_iter();
    let mut next = || profile.next().expect("a profile table");
    let [moments, below, earliest] = weaknesses(a);
    let (spent, kept) = spending(a);
    let (version_profile, version_days) = versions_tables(a);
    let mut tables = vec![kinds, left, opps, ends];
    tables.extend((0..7).map(|_| next()));
    tables.extend([
        moments,
        below,
        earliest,
        separation(a),
        spent,
        kept,
        games(a),
        version_profile,
        version_days,
    ]);
    tables.extend(economy_tables(a));
    let mut files: Vec<(String, String)> = TABLES
        .iter()
        .map(|t| t.0)
        .filter(|n| n.ends_with(".csv"))
        .map(str::to_string)
        .zip(tables.into_iter().map(|c| c.0))
        .collect();
    files.push(("README.md".to_string(), readme(a, charts)));
    files.push(("signals.md".to_string(), signals(a)));
    files
}

#[cfg(test)]
mod tests;
