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

    pub fn ready(&self) -> bool {
        match self.last_attempt {
            None => true,
            Some(t) => t.elapsed() >= self.current,
        }
    }

    pub fn record(&mut self, success: bool) {
        self.last_attempt = Some(Instant::now());
        if success {
            self.current = self.base;
        } else {
            self.current = (self.current * 2).min(self.max);
        }
    }

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
