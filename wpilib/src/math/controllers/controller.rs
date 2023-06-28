pub trait Controller {
    fn calculate(&mut self, measurement: f64, period: f64) -> f64;
    fn set_set_point(&mut self, set_point: f64);
    fn set_enabled(&mut self, enabled: bool);
    fn get_enabled(&self) -> bool;
    fn get_set_point(&self) -> f64;
    fn reset(&mut self);
}