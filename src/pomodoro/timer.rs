use std::time::{Duration, Instant};

#[derive(PartialEq, Eq, Debug)]
enum Status {
    Active,
    Paused,
}

#[derive(Debug)]
pub struct Timer {
    finish_at: Instant,
    paused_at: Instant,
    status: Status,
}

impl Timer {
    pub fn new_paused(duration: Duration) -> Self {
        let now = Instant::now();
        Self {
            finish_at: now + duration,
            paused_at: now,
            status: Status::Paused,
        }
    }

    pub fn time_left(&self) -> Duration {
        match self.status {
            Status::Active => {
                let now = Instant::now();
                if now > self.finish_at {
                    Duration::ZERO
                } else {
                    self.finish_at - now
                }
            }
            Status::Paused => self.finish_at - self.paused_at,
        }
    }

    pub fn pause(&mut self) {
        if self.status == Status::Active {
            self.paused_at = Instant::now();
            self.status = Status::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.status == Status::Paused {
            let delta = self.finish_at - self.paused_at;
            self.finish_at = Instant::now() + delta;
            self.status = Status::Active;
        }
    }

    pub fn is_running(&self) -> bool {
        self.status == Status::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let seconds = 80;
        let mut timer = Timer::new_paused(Duration::from_secs(seconds));

        std::thread::sleep(Duration::from_millis(200));

        assert_eq!(timer.time_left().as_secs(), 80);

        timer.resume();

        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(timer.time_left().as_secs(), 79);
    }
}
