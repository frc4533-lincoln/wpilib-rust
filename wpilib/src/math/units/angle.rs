use wpilib_macros::{unit, unit_conversion};
use crate::math::units::angular_velocity::{DegreePerSecond, RadianPerSecond, RotationPerMinute, RotationPerSecond};
use crate::math::units::time::Second;

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

impl Degree {
    pub fn per_second(self, seconds: Second) -> DegreePerSecond {
        DegreePerSecond::new(self.value() * seconds.value())
    }
}

impl Radian {
    pub fn per_second(self, seconds: Second) -> RadianPerSecond {
        RadianPerSecond::new(self.value() * seconds.value())
    }
}

impl Rotation {
    pub fn per_minute(self, minutes: Second) -> RotationPerMinute {
        RotationPerMinute::new(self.value() * minutes.value())
    }

    pub fn per_second(self, seconds: Second) -> RotationPerSecond {
        RotationPerSecond::new(self.value() * seconds.value())
    }
}