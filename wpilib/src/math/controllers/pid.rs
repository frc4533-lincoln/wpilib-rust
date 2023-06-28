use crate::math::controllers::controller::Controller;

pub struct PIDController {
    pub k_p: f64,
    pub k_i: f64,
    pub k_d: f64,
    pub i_min: f64,
    pub i_max: f64,
    pub min_input: f64,
    pub max_input: f64,
    pub min_output: f64,
    pub max_output: f64,
    prev_error: f64,
    total_error: f64,
    set_point: f64,
    enabled: bool,
}

impl PIDController {
    pub fn new(k_p: f64, k_i: f64, k_d: f64) -> PIDController {
        PIDController {
            k_p,
            k_i,
            k_d,
            i_min: -1.0,
            i_max: 1.0,
            min_input: -1.0,
            max_input: 1.0,
            min_output: -1.0,
            max_output: 1.0,
            prev_error: 0.0,
            total_error: 0.0,
            set_point: 0.0,
            enabled: true,
        }
    }
    pub fn set_limits(&mut self, min_input: f64, max_input: f64, min_output: f64, max_output: f64) {
        self.min_input = min_input;
        self.max_input = max_input;
        self.min_output = min_output;
        self.max_output = max_output;
    }

    pub fn set_i_zone(&mut self, i_min: f64, i_max: f64) {
        self.i_min = i_min;
        self.i_max = i_max;
    }
}

impl Controller for PIDController {
    fn calculate(&mut self, measurement: f64, period: f64) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        let error = self.set_point - measurement;
        self.total_error += error * period;
        self.total_error = self.total_error.min(self.i_max).max(self.i_min);
        let d_error = (error - self.prev_error) / period;
        self.prev_error = error;
        let p = self.k_p * error;
        let i = self.k_i * self.total_error;
        let d = self.k_d * d_error;
        let output = p + i + d;
        output.min(self.max_output).max(self.min_output)
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

    fn reset(&mut self) {
        self.prev_error = 0.0;
        self.total_error = 0.0;
    }
}
