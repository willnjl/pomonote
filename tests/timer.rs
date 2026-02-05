use pomonote::models::timer::Timer;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod timer_tests {
    use super::*;

    #[test]
    fn test_timer_new() {
        let timer = Timer::new();
        assert_eq!(timer.remaining_seconds(), 1500);
        assert!(!timer.is_finished());
        assert_eq!(timer.output(), "25:00");
    }

    #[test]
    fn test_timer_start() {
        let mut timer = Timer::new();
        timer.start();
        thread::sleep(Duration::from_millis(100));
        assert!(timer.remaining_seconds() <= 1500);
        assert!(!timer.is_finished());
    }

    #[test]
    fn test_timer_is_finished() {
        let mut timer = Timer::new();
        timer.start();
        // Simulate elapsed time by waiting
        thread::sleep(Duration::from_secs(2));
        assert!(timer.remaining_seconds() < 1500);
        assert!(!timer.is_finished());
    }

    #[test]
    fn test_timer_output_formatting() {
        let timer = Timer::new();
        assert_eq!(timer.output(), "25:00");

        let mut timer = Timer::new();
        timer.start();
        thread::sleep(Duration::from_secs(1));
        let output = timer.output();
        // Should be close to 24:59
        assert!(output.starts_with("24:"));
    }

    #[test]
    fn test_timer_remaining_seconds_decreases() {
        let mut timer = Timer::new();
        let initial = timer.remaining_seconds();
        timer.start();
        thread::sleep(Duration::from_millis(1500));
        let after = timer.remaining_seconds();
        assert!(after < initial);
    }
}
