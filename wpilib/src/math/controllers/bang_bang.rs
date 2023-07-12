use crate::math::controllers::controller::Controller;
use crate::math::units::time::Millisecond;

#[derive(Debug, Clone, Copy)]
pub struct BangBangController {
    pub min_input: f64,
    pub max_input: f64,
    pub min_output: f64,
    pub max_output: f64,
    pub set_point: f64,
    pub tolerance: f64,
    pub enabled: bool,
}

impl BangBangController {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min_input: -1.0,
            max_input: 1.0,
            min_output: -1.0,
            max_output: 1.0,
            set_point: 0.0,
            tolerance: 0.0,
            enabled: true,
        }
    }

    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }
}

impl Controller for BangBangController {
    fn calculate(&mut self, measurement: f64, _period: impl Into<Millisecond>) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        if measurement.clamp(self.min_input, self.max_input) < self.set_point {
            self.max_output
        } else {
            self.min_output
        }
    }

    fn set_set_point(&mut self, set_point: f64) {
        self.set_point = set_point;
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn get_enabled(&self) -> bool {
        self.enabled
    }

    fn get_set_point(&self) -> f64 {
        self.set_point
    }

    fn set_limits(&mut self, min_input: f64, max_input: f64, min_output: f64, max_output: f64) {
        self.min_input = min_input;
        self.max_input = max_input;
        self.min_output = min_output;
        self.max_output = max_output;
    }

    fn get_limits(&self) -> (f64, f64, f64, f64) {
        (
            self.min_input,
            self.max_input,
            self.min_output,
            self.max_output,
        )
    }

    fn reset(&mut self) {
        self.set_point = 0.0;
    }
}
