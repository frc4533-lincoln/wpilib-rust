

use crate::math::units::linear_velocity::{FeetPerSecond, MeterPerSecond};
use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion};

unit!(Meter, f64);
unit!(Feet, f64);
unit!(Inch, f64);
unit!(Centimeter, f64);

unit_conversion!(Meter f64, Feet f64, meter_to_feet);
unit_conversion!(Meter f64, Inch f64, meter_to_inch);
unit_conversion!(Feet f64, Inch f64, foot_to_inch);
unit_conversion!(Meter f64, Centimeter f64, meter_to_centimeter);
unit_conversion!(Centimeter f64, Feet f64, centimeter_to_foot);
unit_conversion!(Centimeter f64, Inch f64, centimeter_to_inch);

pub fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}
pub fn meter_to_inch(meter: f64) -> f64 {
    meter * 3.28084 * 12.0
}
pub fn foot_to_inch(foot: f64) -> f64 {
    foot * 12.0
}
pub fn meter_to_centimeter(meter: f64) -> f64 {
    meter * 100.0
}
pub fn centimeter_to_foot(centimeter: f64) -> f64 {
    meter_to_feet(centimeter / 100.0)
}
pub fn centimeter_to_inch(centimeter: f64) -> f64 {
    meter_to_inch(centimeter / 100.0)
}

impl Meter {
    pub fn per_second(self, seconds: Second) -> MeterPerSecond {
        MeterPerSecond::new(self.value() * seconds.value())
    }
}

impl Feet {
    pub fn per_second(self, seconds: Second) -> FeetPerSecond {
        FeetPerSecond::new(self.value() * seconds.value())
    }
}

impl Inch {
    pub fn to_feet_per_second(self, seconds: Second) -> FeetPerSecond {
        FeetPerSecond::new(self.value() * seconds.value() / 12.0)
    }
}

impl Centimeter {
    pub fn to_meter_per_second(self, seconds: Second) -> MeterPerSecond {
        MeterPerSecond::new(self.value() * seconds.value() / 100.0)
    }
}

// #[cfg(test)]
// mod test {
//     use super::units::distance::*;

//     #[test]
//     fn test_meter_to_feet() {
//         let meter = Meter::new(1.0);
//         let feet = Feet::new(3.28084);
//         assert_eq!(feet, meter);
//         assert_eq!(meter, feet);
//         let double = feet + meter;
//         assert_eq!(double, Feet::new(6.56168));
//         assert_eq!(double, Meter::new(2.0));
//         assert!(double > feet);
//     }
// }
