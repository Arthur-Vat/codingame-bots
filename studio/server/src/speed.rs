//! How fast a release searches, for "think time".
//!
//! A computer seat gets a think time in milliseconds, but a takeback needs
//! the bot to replay exactly, so it plays a fixed number of iterations. The
//! rate converts one to the other. It is measured once per release on this
//! computer by letting the release play itself, and cached in a file.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use cg_arena::live::{BotSettings, LiveBot, LiveGame};
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::GameSetup;
use cg_arena::runner::{bot_seed, BotSpec};
use studio_game::StudioGame;

use crate::releases::Compiled;
use crate::state::lock;

/// The iterations per answer in the measuring game.
pub const MEASURE_ITERS: u64 = 3000;

/// No answer of the measuring game may take longer than this.
const MEASURE_LIMIT: Duration = Duration::from_secs(120);

pub struct Speed {
    file: PathBuf,
    cache: Mutex<HashMap<String, f64>>,
    /// Measurements run one at a time: two at once would slow each other.
    measuring: Mutex<()>,
}

impl Speed {
    pub fn new(file: PathBuf) -> Speed {
        let cache = fs::read_to_string(&file)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        Speed {
            file,
            cache: Mutex::new(cache),
            measuring: Mutex::new(()),
        }
    }

    fn key(release: &str, compiled: &Compiled) -> String {
        format!("{release}:{}", compiled.hash)
    }

    /// The cached rate (iterations per millisecond) of a release, if any.
    pub fn cached(&self, release: &str, compiled: &Compiled) -> Option<f64> {
        lock(&self.cache)
            .get(&Self::key(release, compiled))
            .copied()
    }

    /// The release's rate, measured if it is not cached. `progress` is
    /// called with the number of answers of the measuring game so far.
    pub fn rate(
        &self,
        game: &dyn StudioGame,
        release: &str,
        compiled: &Compiled,
        progress: &dyn Fn(usize),
    ) -> Result<f64, String> {
        if let Some(rate) = self.cached(release, compiled) {
            return Ok(rate);
        }
        let _one_at_a_time = lock(&self.measuring);
        if let Some(rate) = self.cached(release, compiled) {
            return Ok(rate);
        }
        let rate = measure(game, release, &compiled.path, progress)
            .map_err(|error| format!("cannot measure the speed of {release}: {error}"))?;
        let snapshot = {
            let mut cache = lock(&self.cache);
            cache.insert(Self::key(release, compiled), rate);
            cache.clone()
        };
        // A cache that cannot be written only costs a new measurement later.
        if let Some(dir) = self.file.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = serde_json::to_string_pretty(&snapshot).map(|text| fs::write(&self.file, text));
        Ok(rate)
    }
}

/// The iterations for `think_ms` at `rate` iterations per millisecond.
pub fn iterations(think_ms: u64, rate: f64) -> u64 {
    ((think_ms as f64 * rate).round() as u64).max(1)
}

/// The median of `samples`; 0 when there are none.
pub fn median(samples: &mut [f64]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    samples.sort_by(f64::total_cmp);
    let middle = samples.len() / 2;
    if samples.len() % 2 == 1 {
        samples[middle]
    } else {
        (samples[middle - 1] + samples[middle]) / 2.0
    }
}

/// Plays one game of the release against itself with [`MEASURE_ITERS`]
/// iterations per answer and seed 1, and returns the iterations per
/// millisecond from the median answer time, not counting each seat's first
/// answer (it includes the start-up).
fn measure(
    game: &dyn StudioGame,
    release: &str,
    binary: &Path,
    progress: &dyn Fn(usize),
) -> Result<f64, String> {
    let setup = GameSetup {
        seed: 1,
        opening_plies: 0,
    };
    let spec = BotSpec {
        name: release.to_string(),
        program: binary.to_string_lossy().into_owned(),
        args: Vec::new(),
    };
    let mut bots = Vec::new();
    for seat in 0..2 {
        let settings = BotSettings {
            seed: bot_seed(setup.seed, seat, release),
            time_scale: 1.0,
            fixed_iters: Some(MEASURE_ITERS),
            show_stderr: false,
        };
        bots.push(LiveBot::spawn(&spec, &settings).map_err(|error| error.to_string())?);
    }
    let mut live = LiveGame::new(game.new_referee(&setup), setup);
    let mut samples = Vec::new();
    let mut answered = 0;
    loop {
        let seats = live.to_act();
        if seats.is_empty() {
            break;
        }
        let mut turn = Vec::new();
        for seat in seats {
            let first = bots[seat].answers() == 0;
            let (lines, ms) = bots[seat]
                .ask(
                    &live.input_for(seat),
                    live.answer_lines(seat),
                    MEASURE_LIMIT,
                )
                .map_err(|error| error.to_string())?;
            if !first {
                samples.push(ms);
            }
            turn.push(RecordedAnswer { seat, lines, ms });
        }
        live.play(turn).map_err(|invalid| {
            format!(
                "seat {} answered against the rules: {}",
                invalid.seat, invalid.reason
            )
        })?;
        answered += 1;
        progress(answered);
    }
    let median_ms = median(&mut samples);
    if samples.is_empty() {
        return Err("the game was too short to measure".to_string());
    }
    Ok(MEASURE_ITERS as f64 / median_ms.max(0.001))
}
