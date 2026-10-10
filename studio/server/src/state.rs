//! What every request shares.

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use studio_game::StudioGame;

use crate::history::History;
use crate::releases::Releases;
use crate::session::Session;
use crate::speed::Speed;

/// Sessions untouched for this long are dropped.
const SESSION_IDLE: Duration = Duration::from_secs(2 * 60 * 60);

/// Looking up a session sweeps the idle ones at most this often.
const SWEEP_EVERY: Duration = Duration::from_secs(60);

/// A session shared between requests and its bot thread.
pub type SharedSession = Arc<Mutex<Session>>;

/// Locks a mutex even if a thread panicked while holding it: a poisoned
/// session is still better shown than a server that fails forever.
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

struct Inner {
    games: Vec<Box<dyn StudioGame>>,
    web: PathBuf,
    releases: Releases,
    speed: Speed,
    history: History,
    sessions: Mutex<HashMap<String, SharedSession>>,
    last_sweep: Mutex<Instant>,
}

/// The server's state. Cloning shares it: bot threads hold a clone.
#[derive(Clone)]
pub struct State {
    inner: Arc<Inner>,
}

impl State {
    /// A state serving the games of `repo` (the repository root), the front
    /// end built in `web`, and keeping compiled bots and measurements in
    /// `data`.
    pub fn new(repo: PathBuf, web: PathBuf, data: PathBuf) -> State {
        let games: Vec<Box<dyn StudioGame>> = vec![Box::new(uttt_studio::Uttt)];
        State {
            inner: Arc::new(Inner {
                games,
                web,
                releases: Releases::new(repo, data.join("bin")),
                speed: Speed::new(data.join("speed.json")),
                history: History::new(data.join("history")),
                sessions: Mutex::new(HashMap::new()),
                last_sweep: Mutex::new(Instant::now()),
            }),
        }
    }

    pub fn games(&self) -> &[Box<dyn StudioGame>] {
        &self.inner.games
    }

    pub fn game(&self, id: &str) -> Option<&dyn StudioGame> {
        self.inner
            .games
            .iter()
            .map(|game| game.as_ref())
            .find(|game| game.info().id == id)
    }

    pub fn releases(&self) -> &Releases {
        &self.inner.releases
    }

    pub fn speed(&self) -> &Speed {
        &self.inner.speed
    }

    pub fn history(&self) -> &History {
        &self.inner.history
    }

    pub fn web_dir(&self) -> &Path {
        &self.inner.web
    }

    /// Adds a session, dropping the idle ones first.
    pub fn insert(&self, session: Session) -> SharedSession {
        self.sweep();
        let id = session.id.clone();
        let shared = Arc::new(Mutex::new(session));
        lock(&self.inner.sessions).insert(id, shared.clone());
        shared
    }

    /// The session `id`, marked as used now.
    pub fn session(&self, id: &str) -> Option<SharedSession> {
        self.sweep_if_due();
        let shared = lock(&self.inner.sessions).get(id).cloned()?;
        lock(&shared).touched = Instant::now();
        Some(shared)
    }

    /// Forgets a session and stops its bots.
    pub fn remove(&self, id: &str) -> bool {
        let Some(shared) = lock(&self.inner.sessions).remove(id) else {
            return false;
        };
        lock(&shared).stop();
        true
    }

    /// Sweeps unless it did less than a minute ago, so that an abandoned
    /// game's bots stop without waiting for a new game to be created.
    fn sweep_if_due(&self) {
        {
            let mut last = lock(&self.inner.last_sweep);
            if last.elapsed() < SWEEP_EVERY {
                return;
            }
            *last = Instant::now();
        }
        self.sweep();
    }

    /// Drops the sessions untouched for two hours.
    pub fn sweep(&self) {
        let shared: Vec<(String, SharedSession)> = lock(&self.inner.sessions)
            .iter()
            .map(|(id, shared)| (id.clone(), shared.clone()))
            .collect();
        for (id, shared) in shared {
            if lock(&shared).touched.elapsed() > SESSION_IDLE {
                self.remove(&id);
            }
        }
    }

    /// A new short random id: 16 hex digits.
    pub fn new_id(&self) -> String {
        loop {
            let mut hasher = RandomState::new().build_hasher();
            hasher.write_u128(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |elapsed| elapsed.as_nanos()),
            );
            let id = format!("{:016x}", hasher.finish());
            if !lock(&self.inner.sessions).contains_key(&id) {
                return id;
            }
        }
    }
}
