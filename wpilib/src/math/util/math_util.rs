use nalgebra::ComplexField;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MathUtil {}

impl MathUtil {
    pub fn clamp_int(value: i32, low: i32, high: i32) -> i32 {
        low.max(value.min(high))
    }

    pub fn clamp_double(value: f64, low: f64, high: f64) -> f64 {
        low.max(value.min(high))
    }

    pub fn apply_deadband(value: f64, deadband: f64, max_magnitude: f64) -> f64 {
        if ComplexField::abs(value) > deadband {
            if max_magnitude / deadband > 1.0e12 {
                if value > 0.0 {
                    return value - deadband;
                } else {
                    return value + deadband;
                }
            }
            if value > 0.0 {
                max_magnitude * (value - deadband) / (max_magnitude - deadband)
            } else {
                max_magnitude * (value + deadband) / (max_magnitude - deadband)
            }
        } else {
            0.0
        }
    }

    pub fn apply_deadband_no_max(value: f64, deadband: f64) -> f64 {
        Self::apply_deadband(value, deadband, 1.0)
    }

    pub fn input_modulus(input: f64, minimum_input: f64, maximum_input: f64) -> f64 {
        let modulus = maximum_input - minimum_input;

        let num_max = ((input - minimum_input) / modulus) as i32;
        let input = input - (num_max as f64) * modulus;

        let num_min = ((input - maximum_input) / modulus) as i32;
        let input = input - (num_min as f64) * modulus;

        input
    }

    pub fn angle_modulus(angle_radians: f64) -> f64 {
        Self::input_modulus(angle_radians, -std::f64::consts::PI, std::f64::consts::PI)
    }

    pub fn interpolate(start_value: f64, end_value: f64, t: f64) -> f64 {
        start_value + (end_value - start_value) * Self::clamp_double(t, 0.0, 1.0)
    }

    pub fn is_near(expected: f64, actual: f64, tolerance: f64) -> bool {
        if tolerance < 0.0 {
            panic!("Tolerance must be a non-negative number!");
        }
        ComplexField::abs(expected - actual) < tolerance
    }

    pub fn is_near_min_max(expected: f64, actual: f64, tolerance: f64, min: f64, max: f64) -> bool {
        if tolerance < 0.0 {
            panic!("Tolerance must be a non-negative number!");
        }
        // Max error is exactly halfway between the min and max
        let error_bound = (max - min) / 2.0;
        let error = Self::input_modulus(expected - actual, -error_bound, error_bound);
        ComplexField::abs(error) < tolerance
    }
}
