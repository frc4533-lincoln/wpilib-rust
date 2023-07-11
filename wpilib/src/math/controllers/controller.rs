use crate::math::units::time::Millisecond;

pub trait Controller {
    /// Returns the control output.
    fn calculate(&mut self, measurement: f64, period: impl Into<Millisecond>) -> f64;
    /// Sets the set point.
    fn set_set_point(&mut self, set_point: f64);
    /// Enables or disables the controller.
    fn set_enabled(&mut self, enabled: bool);
    /// Returns whether the controller is enabled.
    fn get_enabled(&self) -> bool;
    /// Returns the set point.
    fn get_set_point(&self) -> f64;
    /// Sets the input and output limits.
    fn set_limits(&mut self, min_input: f64, max_input: f64, min_output: f64, max_output: f64);
    /// Gets the input and output limits.
    /// in the order min_input, max_input, min_output, max_output
    fn get_limits(&self) -> (f64, f64, f64, f64);
    /// Resets the controller.
    fn reset(&mut self);
}
