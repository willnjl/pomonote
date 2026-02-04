use std::time::{ Instant, SystemTime, UNIX_EPOCH };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    duration: u64,
    start_timestamp: Option<u64>,
    #[serde(skip)]
    start_instant: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            duration: 1500,
            start_timestamp: None,
            start_instant: None,
        }
    }

    pub fn start(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        self.start_timestamp = Some(now);
        self.start_instant = Some(Instant::now());
    }

    pub fn remaining_seconds(&self) -> u64 {
        let elapsed = match (self.start_timestamp, self.start_instant) {
            (Some(start_ts), Some(instant)) => {
                let instant_elapsed = instant.elapsed().as_secs();
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let timestamp_elapsed = now.saturating_sub(start_ts);

                if instant_elapsed.abs_diff(timestamp_elapsed) <= 2 {
                    instant_elapsed
                } else {
                    timestamp_elapsed
                }
            }
            (Some(start_ts), None) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                now.saturating_sub(start_ts)
            }
            _ => {
                return self.duration;
            }
        };
        if elapsed >= self.duration {
            0
        } else {
            self.duration - elapsed
        }
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_seconds() == 0
    }

    pub fn output(&self) -> String {
        let remaining = self.remaining_seconds();
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn restore_instant(&mut self) {
        if let Some(start_ts) = self.start_timestamp {
            if self.start_instant.is_none() {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let elapsed = now.saturating_sub(start_ts);
                self.start_instant = Some(Instant::now());
            }
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
