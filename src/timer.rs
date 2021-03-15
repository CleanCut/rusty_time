use std::time::Duration;
/// A simple timer that is externally driven.  `.update()` must be called with the delta time.
pub struct Timer {
    pub duration: Duration,
    pub time_left: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn from_millis(ms: u64) -> Self {
        let duration = Duration::from_millis(ms);
        Self {
            duration,
            time_left: duration,
            ready: false,
        }
    }

    pub fn reset(&mut self) {
        self.ready = false;
        self.time_left = self.duration;
    }

    pub fn update(&mut self, delta: Duration) {
        if self.ready {
            return;
        }
        if let Some(time_left) = self.time_left.checked_sub(delta) {
            self.time_left = time_left;
        } else {
            self.time_left = Duration::from_millis(0);
            self.ready = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_works() {
        let mut timer = Timer::from_millis(500);
        timer.update(Duration::from_millis(300));
        assert_eq!(timer.ready, false);
        timer.update(Duration::from_millis(300));
        assert_eq!(timer.ready, true);
        assert_eq!(timer.time_left, Duration::from_millis(0));
        timer.reset();
        assert_eq!(timer.ready, false);
        assert_eq!(timer.time_left, Duration::from_millis(500));
    }
}
