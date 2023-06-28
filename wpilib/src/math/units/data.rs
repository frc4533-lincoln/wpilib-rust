use crate::math::units::data_rate::{
    BytesPerSecond, GigabytesPerHour, KilobytesPerSecond, MegabytesPerSecond,
};
use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion};

unit!(Byte, f64);
unit!(Kilobyte, f64);
unit!(Megabyte, f64);
unit!(Gigabyte, f64);

unit_conversion!(Byte f64, Kilobyte f64, byte_to_kilobyte);
unit_conversion!(Byte f64, Megabyte f64, byte_to_megabyte);
unit_conversion!(Byte f64, Gigabyte f64, byte_to_gigabyte);
unit_conversion!(Kilobyte f64, Megabyte f64, kilobyte_to_megabyte);
unit_conversion!(Kilobyte f64, Gigabyte f64, kilobyte_to_gigabyte);
unit_conversion!(Megabyte f64, Gigabyte f64, megabyte_to_gigabyte);

pub fn byte_to_kilobyte(byte: f64) -> f64 {
    byte / 1000.0
}

pub fn byte_to_megabyte(byte: f64) -> f64 {
    byte / 1000_000.0
}

pub fn byte_to_gigabyte(byte: f64) -> f64 {
    byte / 1000_000_000.0
}

pub fn kilobyte_to_megabyte(kilobyte: f64) -> f64 {
    kilobyte / 1000.0
}

pub fn kilobyte_to_gigabyte(kilobyte: f64) -> f64 {
    kilobyte / 1000_000.0
}

pub fn megabyte_to_gigabyte(megabyte: f64) -> f64 {
    megabyte / 1000.0
}

impl Byte {
    pub fn per_second(self, seconds: Second) -> BytesPerSecond {
        BytesPerSecond::new(self.value() * seconds.value())
    }
}

impl Kilobyte {
    pub fn per_second(self, seconds: Second) -> KilobytesPerSecond {
        KilobytesPerSecond::new(self.value() * seconds.value())
    }
}

impl Megabyte {
    pub fn per_second(self, seconds: Second) -> MegabytesPerSecond {
        MegabytesPerSecond::new(self.value() * seconds.value())
    }
}

impl Gigabyte {
    pub fn per_hour(self, seconds: Second) -> GigabytesPerHour {
        GigabytesPerHour::new(self.value() * seconds.value() * 3600.0)
    }
}
