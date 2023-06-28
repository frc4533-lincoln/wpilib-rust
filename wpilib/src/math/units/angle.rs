use wpilib_macros::{unit, unit_conversion};

unit!(Degree, f64);
unit!(Radian, f64);
unit!(Rotation, f64);

unit_conversion!(Degree f64, Radian f64, degree_to_radian);
unit_conversion!(Degree f64, Rotation f64, degree_to_rotation);
unit_conversion!(Radian f64, Rotation f64, radian_to_rotation);

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}
pub fn degree_to_rotation(degree: f64) -> f64 {
    degree / 360.0
}
pub fn radian_to_rotation(radian: f64) -> f64 {
    degree_to_rotation(radian * 180.0 / std::f64::consts::PI)
}
