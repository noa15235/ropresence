//! Exponential backoff used to throttle Discord reconnection attempts (#25),
//! so a closed Discord doesn't make us hammer the IPC socket.

use std::time::{Duration, Instant};

pub struct Backoff {
    base: Duration,
    max: Duration,
    current: Duration,
    last_attempt: Option<Instant>,
}

impl Backoff {
    pub fn new() -> Self {
        Self {
            base: Duration::from_secs(3),
            max: Duration::from_secs(30),
            current: Duration::from_secs(3),
            last_attempt: None,
        }
    }

    /// Whether enough time has elapsed to attempt another connection.
    pub fn ready(&self) -> bool {
        match self.last_attempt {
            None => true,
            Some(t) => t.elapsed() >= self.current,
        }
    }

    /// Record the outcome of an attempt: reset on success, grow on failure.
    pub fn record(&mut self, success: bool) {
        self.last_attempt = Some(Instant::now());
        if success {
            self.current = self.base;
        } else {
            self.current = (self.current * 2).min(self.max);
        }
    }

    /// Reset the backoff (e.g. when the client id changes).
    pub fn reset(&mut self) {
        self.current = self.base;
        self.last_attempt = None;
    }
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new()
    }
}
