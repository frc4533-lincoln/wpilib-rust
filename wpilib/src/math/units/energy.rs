use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion};

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

impl Volt {
    pub fn to_watt(&self, amp: &Amp) -> Watt {
        Watt::new(self.value * amp.value)
    }
}

impl Amp {
    pub fn to_watt(&self, volt: &Volt) -> Watt {
        Watt::new(self.value * volt.value)
    }

    pub fn to_voltage(&self, watt: &Watt) -> Volt {
        Volt::new(watt.value / self.value)
    }
}

impl Watt {
    pub fn to_voltage(&self, amp: &Amp) -> Volt {
        Volt::new(self.value / amp.value)
    }

    pub fn to_amp(&self, volt: &Volt) -> Amp {
        Amp::new(self.value / volt.value)
    }

    pub fn to_watt_hour(&self, seconds: &Second) -> WattHour {
        WattHour::new(self.value / (3600.0 / seconds.value()))
    }
}

impl WattHour {
    pub fn to_watt(&self, seconds: &Second) -> Watt {
        Watt::new(self.value * (3600.0 / seconds.value()))
    }
}
