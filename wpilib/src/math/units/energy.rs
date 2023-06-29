use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion, unit_dimensional_analysis};

unit!(Joule, f64);
unit!(Volt, f64);
unit!(Amp, f64);
unit!(Watt, f64);
unit!(WattHour, f64);
unit!(Ohm, f64);

unit_conversion!(Joule f64, WattHour f64, joule_to_watt_hour);

pub fn joule_to_watt_hour(joule: f64) -> f64 {
    joule / 3600.0
}

unit_dimensional_analysis!(Volt * Amp = Watt);

unit_dimensional_analysis!(Watt * Second = Joule);

impl Watt {
    pub fn to_watt_hour(&self, seconds: &Second) -> WattHour {
        WattHour::new(self.value / (3600.0 / seconds.value()))
    }
}

impl WattHour {
    pub fn to_watt(&self, seconds: &Second) -> Watt {
        Watt::new(self.value * (3600.0 / seconds.value()))
    }
}
