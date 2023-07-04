use wpilib_macros::{unit, unit_conversion};

unit!(Celsius, f64);
unit!(Fahrenheit, f64);
unit!(Kelvin, f64);

unit_conversion!(Celsius f64, Fahrenheit f64, celsius_to_fahrenheit);
unit_conversion!(Celsius f64, Kelvin f64, celsius_to_kelvin);
unit_conversion!(Fahrenheit f64, Kelvin f64, fahrenheit_to_kelvin);

#[must_use]
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius.mul_add(1.8, 32.0)
}

#[must_use]
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

#[must_use]
pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * 5.0 / 9.0
}
