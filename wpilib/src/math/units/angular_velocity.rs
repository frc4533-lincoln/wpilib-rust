use wpilib_macros::{unit, unit_conversion};
crate::crate_namespace!();

unit!(DegreePerSecond, f64);
unit!(RadianPerSecond, f64);
unit!(RotationPerSecond, f64);
unit!(RotationPerMinute, f64);

unit_conversion!(DegreePerSecond f64, RadianPerSecond f64, degree_per_second_to_radian_per_second);
unit_conversion!(DegreePerSecond f64, RotationPerSecond f64, degree_per_second_to_rotation_per_second);
unit_conversion!(DegreePerSecond f64, RotationPerMinute f64, degree_per_second_to_rotation_per_minute);
unit_conversion!(RadianPerSecond f64, RotationPerSecond f64, radian_per_second_to_rotation_per_second);
unit_conversion!(RadianPerSecond f64, RotationPerMinute f64, radian_per_second_to_rotation_per_minute);
unit_conversion!(RotationPerSecond f64, RotationPerMinute f64, rotation_per_second_to_rotation_per_minute);

#[must_use]
pub fn degree_per_second_to_radian_per_second(degree_per_second: f64) -> f64 {
    degree_per_second.to_radians()
}

#[must_use]
pub fn degree_per_second_to_rotation_per_second(degree_per_second: f64) -> f64 {
    degree_per_second / 360.0
}

#[must_use]
pub fn degree_per_second_to_rotation_per_minute(degree_per_second: f64) -> f64 {
    degree_per_second / 360.0 * 60.0
}

#[must_use]
pub fn radian_per_second_to_rotation_per_second(radian_per_second: f64) -> f64 {
    degree_per_second_to_rotation_per_second(radian_per_second.to_degrees())
}

#[must_use]
pub fn radian_per_second_to_rotation_per_minute(radian_per_second: f64) -> f64 {
    degree_per_second_to_rotation_per_minute(radian_per_second.to_degrees())
}

#[must_use]
pub fn rotation_per_second_to_rotation_per_minute(rotation_per_second: f64) -> f64 {
    rotation_per_second * 60.0
}
