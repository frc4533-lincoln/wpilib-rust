use num::clamp;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct SlewRateLimiter {
    pub positive_rate_limit: f64,
    pub negative_rate_limit: f64,
    pub last_value: f64,
    pub previous_timestamp: Instant,
}

impl SlewRateLimiter {
    #[must_use]
    pub fn new(positive_rate_limit: f64, negative_rate_limit: f64, initial_value: f64) -> Self {
        Self {
            positive_rate_limit,
            negative_rate_limit,
            last_value: initial_value,
            previous_timestamp: Instant::now(),
        }
    }

    pub fn calculate(&mut self, input: f64) -> f64 {
        let timestamp = Instant::now();
        let delta_time: Duration = timestamp - self.previous_timestamp;
        self.last_value += clamp(
            input - self.last_value,
            -self.negative_rate_limit * delta_time.as_secs_f64(),
            self.positive_rate_limit * delta_time.as_secs_f64(),
        );
        self.previous_timestamp = timestamp;
        self.last_value
    }

    pub fn reset(&mut self, value: f64) {
        self.last_value = value;
        self.previous_timestamp = Instant::now();
    }
}
