use wpilib_macros::{unit, unit_conversion};

unit!(Hour, f64);
unit!(Minute, f64);
unit!(Second, f64);
unit!(Milisecond, f64);
unit!(Microsecond, u64);

unit_conversion!(Second f64, Milisecond f64, second_to_millisecond);
unit_conversion!(Second f64, Microsecond u64, second_to_microsecond);
unit_conversion!(Milisecond f64, Microsecond u64, millisecond_to_microsecond);
unit_conversion!(Hour f64, Second f64, hour_to_second);
unit_conversion!(Minute f64, Second f64, minute_to_second);
unit_conversion!(Hour f64, Minute f64, hour_to_minute);
unit_conversion!(Minute f64, Milisecond f64, minute_to_millisecond);
unit_conversion!(Minute f64, Microsecond u64, minute_to_microsecond);

pub fn second_to_millisecond(second: f64) -> f64 {
    second * 1000.0
}
pub fn second_to_microsecond(second: f64) -> u64 {
    (second * 1_000_000.0) as u64
}
pub fn millisecond_to_microsecond(millisecond: f64) -> u64 {
    (millisecond * 1000.0) as u64
}
pub fn hour_to_second(hour: f64) -> f64 {
    hour * 3600.0
}
pub fn minute_to_second(minute: f64) -> f64 {
    minute * 60.0
}
pub fn hour_to_minute(hour: f64) -> f64 {
    hour * 60.0
}
pub fn minute_to_millisecond(minute: f64) -> f64 {
    minute * 60000.0
}
pub fn minute_to_microsecond(minute: f64) -> u64 {
    (minute * 60_000_000.0) as u64
}
