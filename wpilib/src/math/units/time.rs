use wpilib_macros::{unit, unit_conversion};

unit!(Hour, f64);
unit!(Second, f64);
unit!(Milisecond, f64);
unit!(Microsecond, u64);

unit_conversion!(Second f64, Milisecond f64, second_to_millisecond);
unit_conversion!(Second f64, Microsecond u64, second_to_microsecond);
unit_conversion!(Milisecond f64, Microsecond u64, millisecond_to_microsecond);
unit_conversion!(Hour f64, Second f64, hour_to_second);

pub fn second_to_millisecond(second: f64) -> f64 {
    second * 1000.0
}
pub fn second_to_microsecond(second: f64) -> u64 {
    (second * 1000_000.0) as u64
}
pub fn millisecond_to_microsecond(millisecond: f64) -> u64 {
    (millisecond * 1000.0) as u64
}
pub fn hour_to_second(hour: f64) -> f64 {
    hour * 3600.0
}
