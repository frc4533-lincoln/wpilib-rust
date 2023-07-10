use crate::math::units::angular_velocity::{
    DegreePerSecond, RadianPerSecond, RotationPerMinute, RotationPerSecond,
};
use crate::math::units::time::{Minute, Second};
use wpilib_macros::{unit, unit_conversion, unit_dimensional_analysis};

crate::crate_namespace!();

unit!(Degree, f64);
unit!(Radian, f64);
unit!(Rotation, f64);

unit_conversion!(Degree f64, Radian f64, degree_to_radian);
unit_conversion!(Degree f64, Rotation f64, degree_to_rotation);
unit_conversion!(Radian f64, Rotation f64, radian_to_rotation);

unit_dimensional_analysis!(DegreePerSecond * Second = Degree);
unit_dimensional_analysis!(RadianPerSecond * Second = Radian);
unit_dimensional_analysis!(RotationPerSecond * Second = Rotation);
unit_dimensional_analysis!(RotationPerMinute * Minute = Rotation);

#[must_use]
pub fn degree_to_radian(degree: f64) -> f64 {
    degree.to_radians()
}
#[must_use]
pub fn degree_to_rotation(degree: f64) -> f64 {
    degree / 360.0
}
#[must_use]
pub fn radian_to_rotation(radian: f64) -> f64 {
    degree_to_rotation(radian.to_degrees())
}

impl Degree {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> DegreePerSecond {
        DegreePerSecond::new(self.value() * seconds.value())
    }
}

impl Radian {
    #[must_use]
    pub fn per_second(self, seconds: Second) -> RadianPerSecond {
        RadianPerSecond::new(self.value() * seconds.value())
    }
}

impl Rotation {
    #[must_use]
    pub fn per_minute(self, minutes: Second) -> RotationPerMinute {
        RotationPerMinute::new(self.value() * minutes.value())
    }

    #[must_use]
    pub fn per_second(self, seconds: Second) -> RotationPerSecond {
        RotationPerSecond::new(self.value() * seconds.value())
    }
}
