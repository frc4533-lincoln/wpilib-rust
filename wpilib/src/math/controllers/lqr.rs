use crate::math::controllers::controller::Controller;

#[derive(Debug, Clone, Copy)]
pub struct LQRController {
    pub k: f64,
    pub r: f64,
    pub q: f64,
    pub prev_error: f64,
    pub set_point: f64,
    pub enabled: bool,
}

impl LQRController {
    pub fn new(k: f64, r: f64, q: f64) -> LQRController {
        LQRController {
            k,
            r,
            q,
            prev_error: 0.0,
            set_point: 0.0,
            enabled: true,
        }
    }
}

impl Controller for LQRController {
    fn calculate(&mut self, _measurement: f64, _period: f64) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        // TODO: Implement LQR controller
        return 0.0;
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
    }
}
