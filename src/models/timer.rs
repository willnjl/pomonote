use std::time::{ Instant, SystemTime, UNIX_EPOCH };
use serde::{ Deserialize, Serialize };

/// Timer struct for Pomodoro sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    /// Total duration of the timer in seconds
    duration: u64,
    /// Start time as Unix timestamp (seconds since epoch)
    start_timestamp: Option<u64>,
    /// Instant for accurate elapsed time calculation (not serialized)
    #[serde(skip)]
    start_instant: Option<Instant>,
}

impl Timer {
    /// Create a new timer set to 25 minutes (1500 seconds) but not started
    pub fn new() -> Self {
        Self {
            duration: 1500, // 25 minutes in seconds
            start_timestamp: None,
            start_instant: None,
        }
    }

    /// Start the countdown timer
    pub fn start(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        self.start_timestamp = Some(now);
        self.start_instant = Some(Instant::now());
    }

    /// Get the remaining time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        match (self.start_timestamp, self.start_instant) {
            // If we have both, use Instant for accuracy
            (Some(_), Some(instant)) => {
                let elapsed = instant.elapsed().as_secs();
                if elapsed >= self.duration {
                    0
                } else {
                    self.duration - elapsed
                }
            }
            // If we only have timestamp (loaded from disk), calculate from SystemTime
            (Some(start_ts), None) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let elapsed = now.saturating_sub(start_ts);
                if elapsed >= self.duration {
                    0
                } else {
                    self.duration - elapsed
                }
            }
            // Not started
            _ => self.duration,
        }
    }

    /// Check if the timer has finished
    pub fn is_finished(&self) -> bool {
        self.remaining_seconds() == 0
    }

    /// Output the timer in mm:ss format
    pub fn output(&self) -> String {
        let remaining = self.remaining_seconds();
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Restore the Instant after deserialization
    pub fn restore_instant(&mut self) {
        if self.start_timestamp.is_some() && self.start_instant.is_none() {
            self.start_instant = Some(Instant::now());
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
