use crate::math::units::distance::{Feet, Meter};
use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion};

unit!(MeterPerSecond, f64);
unit!(KilometerPerHour, f64);
unit!(MilePerHour, f64);
unit!(FeetPerSecond, f64);

unit_conversion!(MeterPerSecond f64, KilometerPerHour f64, meter_per_second_to_kilometer_per_hour);
unit_conversion!(MeterPerSecond f64, MilePerHour f64, meter_per_second_to_mile_per_hour);
unit_conversion!(MeterPerSecond f64, FeetPerSecond f64, meter_per_second_to_feet_per_second);
unit_conversion!(FeetPerSecond f64, MilePerHour f64, feet_per_second_to_mile_per_hour);
unit_conversion!(FeetPerSecond f64, KilometerPerHour f64, feet_per_second_to_kilometer_per_hour);
unit_conversion!(MilePerHour f64, KilometerPerHour f64, mile_per_hour_to_kilometer_per_hour);

pub fn meter_per_second_to_kilometer_per_hour(meter_per_second: f64) -> f64 {
    meter_per_second * 3.6
}

pub fn meter_per_second_to_mile_per_hour(meter_per_second: f64) -> f64 {
    meter_per_second * 2.23694
}

pub fn meter_per_second_to_feet_per_second(meter_per_second: f64) -> f64 {
    meter_per_second * 3.28084
}

pub fn feet_per_second_to_mile_per_hour(feet_per_second: f64) -> f64 {
    meter_per_second_to_mile_per_hour(feet_per_second / 3.28084)
}

pub fn feet_per_second_to_kilometer_per_hour(feet_per_second: f64) -> f64 {
    meter_per_second_to_kilometer_per_hour(feet_per_second / 3.28084)
}

pub fn mile_per_hour_to_kilometer_per_hour(mile_per_hour: f64) -> f64 {
    meter_per_second_to_kilometer_per_hour(mile_per_hour / 2.23694)
}

impl MeterPerSecond {
    pub fn to_meters(&self, seconds: Second) -> Meter {
        Meter::new(self.value * seconds.value())
    }
}

impl FeetPerSecond {
    pub fn to_feet(&self, seconds: Second) -> Feet {
        Feet::new(self.value * seconds.value())
    }
}

impl MilePerHour {
    pub fn to_feet(&self, seconds: Second) -> Feet {
        Feet::new(FeetPerSecond::from(*self).value() * seconds.value())
    }
}

impl KilometerPerHour {
    pub fn to_meters(&self, seconds: Second) -> Meter {
        Meter::new(MeterPerSecond::from(*self).value() * seconds.value())
    }
}
