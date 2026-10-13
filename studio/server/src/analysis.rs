//! The analyses ([ADR 0032](../../../docs/adr/0032-analysis-engine.md)): a
//! game's analysis program run natively, one request per thread, and the three
//! routes that start, read and stop one.
//!
//! The program calls `emit` with each line it writes (studio/README.md, "The
//! analysis program"); `emit` keeps the latest report for `GET`, and says
//! false once the analysis was stopped, which ends the search.
//!
//! - At most [`MAX_ANALYSES`] exist at once; starting one more stops and
//!   removes the oldest.
//! - One that no request has read for [`ANALYSIS_IDLE`] is stopped and
//!   removed (a closed page frees the processor). Every analysis request
//!   sweeps the idle ones, and the thread of an analysis checks its own
//!   reads at each report, so that nothing needs a timer thread.
//! - A stop (`DELETE`, eviction, idle) takes effect at the program's next
//!   call of `emit`, that is its next report: the search is not interrupted
//!   between reports, so a large `report_every` delays it.
//! - Locks: the list of analyses and an analysis's progress are never held
//!   together, and none is held while the search runs, so a `DELETE` racing
//!   the thread's last `emit` cannot deadlock.

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Map, Value};

use crate::api::Response;
use crate::state::{lock, State};

/// The most analyses that exist at once.
pub const MAX_ANALYSES: usize = 4;

/// An analysis that no request has read for this long is stopped and removed.
pub const ANALYSIS_IDLE: Duration = Duration::from_secs(10);

/// What tells the time to the analyses: replaced in tests.
pub type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;

/// What an analysis has reported so far.
struct Progress {
    status: &'static str,
    report: Option<Value>,
    error: Option<String>,
    last_read: Instant,
}

struct Analysis {
    id: String,
    progress: Mutex<Progress>,
    stopped: AtomicBool,
}

impl Analysis {
    /// Keeps what the program wrote in `line`; false when the analysis is
    /// over (done, or an error) and the program should stop.
    fn record(&self, line: &str) -> bool {
        let mut progress = lock(&self.progress);
        if progress.status != "running" {
            return false;
        }
        let parsed: Option<Value> = serde_json::from_str(line).ok();
        match parsed.as_ref().and_then(Value::as_object) {
            Some(object) if object.contains_key("error") => {
                progress.status = "error";
                progress.error = Some(match &object["error"] {
                    Value::String(message) => message.clone(),
                    other => other.to_string(),
                });
                false
            }
            Some(object) if object.get("report").is_some_and(Value::is_object) => {
                let report = object["report"].clone();
                let done = report.get("done") == Some(&Value::Bool(true));
                progress.report = Some(report);
                if done {
                    progress.status = "done";
                }
                !done
            }
            _ => {
                progress.status = "error";
                progress.error = Some("the analysis program wrote an unexpected line".to_string());
                false
            }
        }
    }

    /// Marks a program that ended without a final report or an error line.
    fn ended(&self, crashed: bool) {
        let mut progress = lock(&self.progress);
        if progress.status == "running" && !self.stopped.load(Ordering::Acquire) {
            progress.status = "error";
            progress.error = Some(
                if crashed {
                    "the analysis program crashed"
                } else {
                    "the analysis program ended without a final report"
                }
                .to_string(),
            );
        }
    }

    fn last_read(&self) -> Instant {
        lock(&self.progress).last_read
    }
}

struct Shared {
    /// The analyses, the oldest first.
    list: Mutex<Vec<Arc<Analysis>>>,
    idle: Duration,
    clock: Clock,
    /// The analysis threads that have not ended.
    threads: AtomicUsize,
}

/// The analyses of a server. Cloning shares them.
#[derive(Clone)]
pub struct Analyses {
    shared: Arc<Shared>,
}

/// Counts a thread while it lives.
struct Alive(Arc<Shared>);

impl Alive {
    fn new(shared: &Arc<Shared>) -> Alive {
        shared.threads.fetch_add(1, Ordering::AcqRel);
        Alive(shared.clone())
    }
}

impl Drop for Alive {
    fn drop(&mut self) {
        self.0.threads.fetch_sub(1, Ordering::AcqRel);
    }
}

impl Analyses {
    /// Analyses that expire after `idle` without a read, by `clock`.
    pub fn new(idle: Duration, clock: Clock) -> Analyses {
        Analyses {
            shared: Arc::new(Shared {
                list: Mutex::new(Vec::new()),
                idle,
                clock,
                threads: AtomicUsize::new(0),
            }),
        }
    }

    fn now(&self) -> Instant {
        (self.shared.clock)()
    }

    /// How many analysis threads are still running.
    pub fn threads(&self) -> usize {
        self.shared.threads.load(Ordering::Acquire)
    }

    /// The ids of the analyses that exist, the oldest first.
    pub fn ids(&self) -> Vec<String> {
        lock(&self.shared.list)
            .iter()
            .map(|analysis| analysis.id.clone())
            .collect()
    }

    /// Starts `run` on a new thread, giving it the `emit` of a new analysis,
    /// and returns the analysis's id. The oldest analyses beyond
    /// [`MAX_ANALYSES`] are stopped and removed.
    pub fn start(&self, run: impl FnOnce(&mut dyn FnMut(&str) -> bool) + Send + 'static) -> String {
        self.sweep();
        let (analysis, evicted) = {
            let mut list = lock(&self.shared.list);
            let keep = MAX_ANALYSES - 1;
            let excess = list.len().saturating_sub(keep);
            let evicted: Vec<Arc<Analysis>> = list.drain(..excess).collect();
            let id = loop {
                let id = new_id();
                if list.iter().all(|analysis| analysis.id != id) {
                    break id;
                }
            };
            let analysis = Arc::new(Analysis {
                id,
                progress: Mutex::new(Progress {
                    status: "running",
                    report: None,
                    error: None,
                    last_read: self.now(),
                }),
                stopped: AtomicBool::new(false),
            });
            list.push(analysis.clone());
            (analysis, evicted)
        };
        for old in evicted {
            old.stopped.store(true, Ordering::Release);
        }
        let id = analysis.id.clone();
        let alive = Alive::new(&self.shared);
        let analyses = self.clone();
        let thread_analysis = analysis.clone();
        let spawned = thread::Builder::new()
            .name(format!("analysis-{id}"))
            .spawn(move || {
                let _alive = alive;
                analyses.run(&thread_analysis, run);
            });
        if let Err(error) = spawned {
            let mut progress = lock(&analysis.progress);
            progress.status = "error";
            progress.error = Some(format!("cannot start the analysis: {error}"));
        }
        id
    }

    /// The body of an analysis thread.
    fn run(&self, analysis: &Analysis, run: impl FnOnce(&mut dyn FnMut(&str) -> bool)) {
        let mut emit = |line: &str| -> bool {
            if analysis.stopped.load(Ordering::Acquire) {
                return false;
            }
            if !analysis.record(line) {
                return false;
            }
            if self.now().saturating_duration_since(analysis.last_read()) > self.shared.idle {
                self.remove(&analysis.id);
                return false;
            }
            !analysis.stopped.load(Ordering::Acquire)
        };
        let outcome = catch_unwind(AssertUnwindSafe(|| run(&mut emit)));
        analysis.ended(outcome.is_err());
    }

    /// Stops and removes the analyses nobody has read for too long.
    pub fn sweep(&self) {
        let analyses: Vec<Arc<Analysis>> = lock(&self.shared.list).clone();
        let now = self.now();
        for analysis in analyses {
            if now.saturating_duration_since(analysis.last_read()) > self.shared.idle {
                self.remove(&analysis.id);
            }
        }
    }

    /// The state of the analysis `id` for `GET`, which counts as a read.
    pub fn read(&self, id: &str) -> Option<Value> {
        self.sweep();
        let analysis = lock(&self.shared.list)
            .iter()
            .find(|analysis| analysis.id == id)
            .cloned()?;
        let mut progress = lock(&analysis.progress);
        progress.last_read = self.now();
        Some(json!({
            "status": progress.status,
            "report": progress.report,
            "error": progress.error,
        }))
    }

    /// Stops and removes the analysis `id`; false when there is none.
    pub fn remove(&self, id: &str) -> bool {
        let removed = {
            let mut list = lock(&self.shared.list);
            list.iter()
                .position(|analysis| analysis.id == id)
                .map(|at| list.remove(at))
        };
        match removed {
            Some(analysis) => {
                analysis.stopped.store(true, Ordering::Release);
                true
            }
            None => false,
        }
    }
}

/// A new short random id: 16 hex digits.
fn new_id() -> String {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_nanos()),
    );
    format!("{:016x}", hasher.finish())
}

fn unknown_analysis() -> Response {
    Response::error(404, "unknown analysis")
}

/// `POST /api/analysis`: the program's request plus `game`.
pub fn start(state: &State, body: &[u8]) -> Response {
    let mut request: Map<String, Value> = match serde_json::from_slice::<Value>(body) {
        Ok(Value::Object(object)) => object,
        Ok(_) => {
            return Response::error(
                400,
                "invalid request body: the request must be a JSON object",
            )
        }
        Err(error) => {
            return Response::error(400, &format!("invalid request body: {error}"));
        }
    };
    let game_id = match request.remove("game") {
        Some(Value::String(game)) => game,
        _ => return Response::error(400, "invalid request body: game must be a string"),
    };
    let Some(game) = state.game(&game_id) else {
        return Response::error(400, &format!("unknown game {game_id:?}"));
    };
    if !game.has_analysis() {
        return Response::error(400, &format!("the game {game_id:?} has no analysis"));
    }
    let request = Value::Object(request).to_string();
    let analyses = state.analyses().clone();
    let state = state.clone();
    let id = analyses.start(move |emit| {
        // The game was found a moment ago, and games are never unregistered.
        if let Some(game) = state.game(&game_id) {
            game.analyse(&request, emit);
        }
    });
    Response::json(200, &json!({ "id": id }))
}

/// `GET /api/analysis/{id}`.
pub fn get(state: &State, id: &str) -> Response {
    match state.analyses().read(id) {
        Some(view) => Response::json(200, &view),
        None => unknown_analysis(),
    }
}

/// `DELETE /api/analysis/{id}`.
pub fn delete(state: &State, id: &str) -> Response {
    state.analyses().sweep();
    if state.analyses().remove(id) {
        Response::json(200, &json!({ "deleted": true }))
    } else {
        unknown_analysis()
    }
}
