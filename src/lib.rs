//! `rusty_time` is a simple timer library.
//!
//! Use it in a loop like this:
//!
//! ```rust
//! use std::time::{Duration, Instant};
//!
//! use rusty_time::Timer;
//!
//! fn main() {
//!     let mut timer = Timer::new(Duration::from_secs_f32(1.5));
//!
//!     let mut start_time = Instant::now();
//!     loop {
//!         timer.tick(start_time.elapsed());
//!         start_time = Instant::now();
//!         println!(
//!             "Time on timer: {:.2}s ({:.1}%)",
//!             timer.remaining().as_secs_f32(),
//!             timer.percent_left() * 100.0
//!         );
//!         if timer.just_finished() {
//!             break;
//!         }
//!     }
//!     println!("Timer finished!");
//! }
//! ```

use std::time::Duration;

/// A simple timer that is externally driven. [`.tick()`](Timer::tick) must be called with a delta
/// time for the timer to advance.
///
/// Use it in a loop like this:
///
/// ```rust
/// use std::time::{Duration, Instant};
///
/// use rusty_time::Timer;
///
/// fn main() {
///     let mut timer = Timer::new(Duration::from_secs_f32(1.5));
///
///     let mut start_time = Instant::now();
///     loop {
///         timer.tick(start_time.elapsed());
///         start_time = Instant::now();
///         println!(
///             "Time on timer: {:.2}s ({:.1}%)",
///             timer.remaining().as_secs_f32(),
///             timer.percent_left() * 100.0
///         );
///         if timer.just_finished() {
///             break;
///         }
///     }
///     println!("Timer finished!");
/// }
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Timer {
    duration: Duration,
    remaining: Duration,
    finished: bool,
    just_finished: bool,
}

impl Timer {
    /// Create a new timer with the given duration
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            remaining: duration,
            finished: false,
            just_finished: false,
        }
    }

    /// Advance the timer by a specific duration. This _must_ be called for the timer to be useful.
    pub fn tick(&mut self, delta: Duration) {
        self.just_finished = false;
        if self.finished {
            return;
        }
        self.remaining = self.remaining.saturating_sub(delta);
        if self.remaining == Duration::ZERO {
            self.just_finished = true;
            self.finished = true;
        }
    }

    /// Whether the timer hit zero since the last call to [`tick`](Timer::tick).
    pub fn just_finished(&self) -> bool {
        self.just_finished
    }

    /// Whether the timer is at zero.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Reset the timer to the starting duration. Resets `finished` and `just_finished` as well.
    pub fn reset(&mut self) {
        self.set_remaining(self.duration);
    }

    /// Returns percent of timer elapsed (from 0.0 to 1.0)
    pub fn percent(&self) -> f32 {
        if self.duration == Duration::ZERO {
            1.0
        } else {
            (self.duration - self.remaining).as_secs_f32() / self.duration.as_secs_f32()
        }
    }

    /// Returns percent of timer remaining (from 1.0 to 0.0)
    pub fn percent_left(&self) -> f32 {
        1.0 - self.percent()
    }

    /// Returns the full duration of the timer when newly started. See also
    /// [`remaining`](Timer::remaining) and [`elapsed`](Timer::elapsed)
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Sets the starting duration of the timer, but does not change the remaining time on the timer.
    /// See [`reset`](Timer::reset) to reset the remaining time to the starting duration.
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// Returns the time currently remaining on the timer.
    pub fn remaining(&self) -> Duration {
        self.remaining
    }

    /// Sets the time currently remaining on the timer. Sets/unsets that the timer has finished as
    /// appropriate.
    pub fn set_remaining(&mut self, remaining: Duration) {
        if remaining == Duration::ZERO && self.remaining != Duration::ZERO {
            self.just_finished = true;
            self.finished = true;
        } else {
            self.just_finished = false;
            self.finished = false;
        }
        self.remaining = remaining.clamp(Duration::ZERO, self.duration);
    }

    /// Returns the time that has elapsed on the timer.
    pub fn elapsed(&self) -> Duration {
        self.duration - self.remaining
    }

    /// Sets the time currently elapsed on the timer. Sets/unsets that the timer has finished as
    /// appropriate.
    pub fn set_elapsed(&mut self, elapsed: Duration) {
        self.set_remaining(self.duration.saturating_sub(elapsed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_works() {
        let mut timer = Timer::new(Duration::from_millis(500));
        assert_eq!(timer.percent(), 0.0);
        assert_eq!(timer.percent_left(), 1.0);
        timer.tick(Duration::from_millis(300));
        assert_eq!(timer.finished(), false);
        assert_eq!(timer.just_finished(), false);
        timer.tick(Duration::from_millis(300));
        assert_eq!(timer.finished(), true);
        assert_eq!(timer.just_finished(), true);
        assert_eq!(timer.remaining(), Duration::ZERO);
        timer.tick(Duration::from_millis(300));
        assert_eq!(timer.finished(), true);
        assert_eq!(timer.just_finished(), false);
        assert_eq!(timer.remaining(), Duration::ZERO);
        timer.reset();
        assert_eq!(timer.finished, false);
        assert_eq!(timer.remaining, Duration::from_millis(500));
        timer.set_elapsed(Duration::from_millis(250));
        assert_eq!(timer.percent(), 0.5);
        assert_eq!(timer.percent_left(), 0.5);
        assert_eq!(timer.remaining(), Duration::from_millis(250));
    }
}
