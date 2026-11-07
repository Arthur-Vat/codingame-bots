//! Versions of a player: a player who changes its code gets a new agent id
//! (ADR 0057, decision 3). Two views: the agent ids a player appeared with
//! in the games, with the results of each (`version_rows`, in the profiles),
//! and the agent id of each top player in the leaderboard snapshots by day,
//! with a flag where it differs from the previous day's ([`versions`]).

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::{rank_key, Options, Tally};

/// One row of a leaderboard snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnapshotRow {
    pub rank: u32,
    pub nickname: String,
    /// The agent id, when the snapshot holds it.
    pub agent_id: Option<u64>,
}

/// A leaderboard snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// The snapshot's file name.
    pub name: String,
    /// The day it was taken, `YYYY-MM-DD`.
    pub date: String,
    pub rows: Vec<SnapshotRow>,
}

/// One game of a player: the agent id it played with, the day, and its
/// result against us (`Some(None)` for a draw; `None` when we were not the
/// opponent).
pub(super) struct Seen {
    pub agent_id: Option<u64>,
    pub date: Option<String>,
    pub theirs: Option<Option<bool>>,
}

/// A player's games with one agent id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VersionRow {
    pub agent_id: Option<u64>,
    pub games: usize,
    /// The player's results in the games against us.
    pub vs_us: Tally,
    /// The first and last day of its games with this agent id.
    pub first_seen: Option<String>,
    pub last_seen: Option<String>,
}

/// The versions of one player, from its games: one row for each agent id,
/// the earliest first seen first (those without a date or id last).
pub(super) fn version_rows(seen: &[Seen]) -> Vec<VersionRow> {
    let mut rows: BTreeMap<Option<u64>, VersionRow> = BTreeMap::new();
    for game in seen {
        let row = rows.entry(game.agent_id).or_insert_with(|| VersionRow {
            agent_id: game.agent_id,
            games: 0,
            vs_us: Tally::default(),
            first_seen: None,
            last_seen: None,
        });
        row.games += 1;
        if let Some(result) = game.theirs {
            row.vs_us.add(result);
        }
        if let Some(date) = &game.date {
            if row.first_seen.as_ref().is_none_or(|d| date < d) {
                row.first_seen = Some(date.clone());
            }
            if row.last_seen.as_ref().is_none_or(|d| date > d) {
                row.last_seen = Some(date.clone());
            }
        }
    }
    let mut rows: Vec<VersionRow> = rows.into_values().collect();
    rows.sort_by(|a, b| {
        (a.first_seen.is_none(), &a.first_seen, a.agent_id).cmp(&(
            b.first_seen.is_none(),
            &b.first_seen,
            b.agent_id,
        ))
    });
    rows
}

/// A top player's agent id in the snapshot of a day.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TrackedAgent {
    pub player: String,
    pub date: String,
    pub snapshot: String,
    pub rank: u32,
    pub agent_id: Option<u64>,
    /// The agent id differs from the one of the last earlier day on which the player was listed.
    pub changed: bool,
}

/// A change of agent id between two days' snapshots.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Change {
    pub player: String,
    pub from_date: String,
    pub to_date: String,
    pub from_agent: u64,
    pub to_agent: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Versions {
    /// The days with a snapshot, oldest first (a day with several snapshots
    /// counts its latest).
    pub days: Vec<String>,
    /// The agent id of each tracked player at each day it is listed, the
    /// players by rank in the newest snapshot, then by day.
    pub rows: Vec<TrackedAgent>,
    pub changes: Vec<Change>,
}

/// The agent ids of the top players by day. The tracked players are those
/// named in `options.players`, or else those at rank `options.top` or better
/// in any snapshot.
pub(super) fn versions(snapshots: &[Snapshot], options: &Options) -> Versions {
    let mut by_day: BTreeMap<&str, &Snapshot> = BTreeMap::new();
    for snapshot in snapshots {
        let slot = by_day.entry(&snapshot.date).or_insert(snapshot);
        if snapshot.name > slot.name {
            *slot = snapshot;
        }
    }
    let tracked: BTreeSet<&str> = if options.players.is_empty() {
        by_day
            .values()
            .flat_map(|s| s.rows.iter())
            .filter(|r| (r.rank as usize) <= options.top)
            .map(|r| r.nickname.as_str())
            .collect()
    } else {
        options.players.iter().map(String::as_str).collect()
    };
    let newest = by_day.values().next_back();
    let rank_in_newest = |name: &str| {
        newest.and_then(|s| s.rows.iter().find(|r| r.nickname == name).map(|r| r.rank))
    };
    let mut players: Vec<&str> = tracked.into_iter().collect();
    players.sort_by_key(|p| rank_key(rank_in_newest(p), p));
    let mut out = Versions {
        days: by_day.keys().map(|d| (*d).to_string()).collect(),
        ..Versions::default()
    };
    for player in players {
        let mut previous: Option<(&str, Option<u64>)> = None;
        for (day, snapshot) in &by_day {
            let Some(row) = snapshot.rows.iter().find(|r| r.nickname == player) else {
                continue;
            };
            let changed = match (previous, row.agent_id) {
                (Some((_, Some(old))), Some(new)) => old != new,
                _ => false,
            };
            if let (true, Some((from_date, Some(from_agent))), Some(to_agent)) =
                (changed, previous, row.agent_id)
            {
                out.changes.push(Change {
                    player: player.to_string(),
                    from_date: from_date.to_string(),
                    to_date: (*day).to_string(),
                    from_agent,
                    to_agent,
                });
            }
            out.rows.push(TrackedAgent {
                player: player.to_string(),
                date: (*day).to_string(),
                snapshot: snapshot.name.clone(),
                rank: row.rank,
                agent_id: row.agent_id,
                changed,
            });
            previous = Some((day, row.agent_id));
        }
    }
    out
}

#[cfg(test)]
mod tests;
