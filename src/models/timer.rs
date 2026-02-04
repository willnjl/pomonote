use std::time::Instant;

/// Timer struct for Pomodoro sessions
#[derive(Debug, Clone)]
pub struct Timer {
    /// Total duration of the timer in seconds
    duration: u64,
    /// Start time of the timer (None if not started)
    start_time: Option<Instant>,
}

impl Timer {
    /// Create a new timer set to 25 minutes (1500 seconds) but not started
    pub fn new() -> Self {
        Self {
            duration: 1500, // 25 minutes in seconds
            start_time: None,
        }
    }

    /// Start the countdown timer
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Get the remaining time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        match self.start_time {
            Some(start) => {
                let elapsed = start.elapsed().as_secs();
                if elapsed >= self.duration {
                    0
                } else {
                    self.duration - elapsed
                }
            }
            None => self.duration, // Return full duration if not started
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
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
