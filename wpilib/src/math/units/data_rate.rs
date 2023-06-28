use crate::math::units::data::{Byte, Gigabyte, Kilobyte, Megabyte};
use crate::math::units::time::Second;
use wpilib_macros::{unit, unit_conversion};

unit!(BytesPerSecond, f64);
unit!(KilobytesPerSecond, f64);
unit!(MegabytesPerSecond, f64);
unit!(GigabytesPerHour, f64);

unit_conversion!(BytesPerSecond f64, KilobytesPerSecond f64, byte_per_second_to_kilobyte_per_second);
unit_conversion!(BytesPerSecond f64, MegabytesPerSecond f64, byte_per_second_to_megabyte_per_second);
unit_conversion!(BytesPerSecond f64, GigabytesPerHour f64, byte_per_second_to_gigabyte_per_hour);
unit_conversion!(KilobytesPerSecond f64, MegabytesPerSecond f64, kilobyte_per_second_to_megabyte_per_second);
unit_conversion!(KilobytesPerSecond f64, GigabytesPerHour f64, kilobyte_per_second_to_gigabyte_per_hour);

pub fn byte_per_second_to_kilobyte_per_second(byte_per_second: f64) -> f64 {
    byte_per_second / 1000.0
}

pub fn byte_per_second_to_megabyte_per_second(byte_per_second: f64) -> f64 {
    byte_per_second / 1000_000.0
}

pub fn byte_per_second_to_gigabyte_per_hour(byte_per_second: f64) -> f64 {
    byte_per_second * 0.000036
}

pub fn kilobyte_per_second_to_megabyte_per_second(kilobyte_per_second: f64) -> f64 {
    kilobyte_per_second / 1000.0
}

pub fn kilobyte_per_second_to_gigabyte_per_hour(kilobyte_per_second: f64) -> f64 {
    kilobyte_per_second * 3600.0 / 1000_000.0
}

pub fn megabyte_per_second_to_gigabyte_per_hour(megabyte_per_second: f64) -> f64 {
    megabyte_per_second * 3600.0 / 1000.0
}

impl BytesPerSecond {
    pub fn to_byte(&self, seconds: Second) -> Byte {
        Byte::new(self.value * seconds.value())
    }
}

impl KilobytesPerSecond {
    pub fn to_kilobyte(&self, seconds: Second) -> Kilobyte {
        Kilobyte::new(self.value * seconds.value())
    }
}

impl MegabytesPerSecond {
    pub fn to_megabyte(&self, seconds: Second) -> Megabyte {
        Megabyte::new(self.value * seconds.value())
    }
}

impl GigabytesPerHour {
    pub fn to_gigabyte(&self, seconds: Second) -> Gigabyte {
        Gigabyte::new(self.value / (3600.0 / seconds.value()))
    }
}
