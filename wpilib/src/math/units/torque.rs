use wpilib_macros::{unit, unit_conversion};
crate::crate_namespace!();

unit!(NewtonMeter, f64);
unit!(NewtonCentimeter, f64);
unit!(KilogramMeter, f64);
unit!(FootPound, f64);
unit!(InchPound, f64);

unit_conversion!(NewtonMeter f64, NewtonCentimeter f64, newton_meter_to_newton_centimeter);
unit_conversion!(NewtonMeter f64, KilogramMeter f64, newton_meter_to_kilogram_meter);
unit_conversion!(NewtonMeter f64, FootPound f64, newton_meter_to_foot_pound);
unit_conversion!(NewtonMeter f64, InchPound f64, newton_meter_to_inch_pound);
unit_conversion!(NewtonCentimeter f64, KilogramMeter f64, newton_centimeter_to_kilogram_meter);
unit_conversion!(NewtonCentimeter f64, FootPound f64, newton_centimeter_to_foot_pound);
unit_conversion!(NewtonCentimeter f64, InchPound f64, newton_centimeter_to_inch_pound);
unit_conversion!(KilogramMeter f64, FootPound f64, kilogram_meter_to_foot_pound);

#[must_use]
pub fn newton_meter_to_newton_centimeter(newton_meter: f64) -> f64 {
    newton_meter * 100.0
}
#[must_use]
pub fn newton_meter_to_kilogram_meter(newton_meter: f64) -> f64 {
    newton_meter * 0.101_972
}
#[must_use]
pub fn newton_meter_to_foot_pound(newton_meter: f64) -> f64 {
    newton_meter * 0.737_562
}
#[must_use]
pub fn newton_meter_to_inch_pound(newton_meter: f64) -> f64 {
    newton_meter * 8.85075
}
#[must_use]
pub fn newton_centimeter_to_kilogram_meter(newton_centimeter: f64) -> f64 {
    newton_meter_to_kilogram_meter(newton_centimeter / 100.0)
}
#[must_use]
pub fn newton_centimeter_to_foot_pound(newton_centimeter: f64) -> f64 {
    newton_meter_to_foot_pound(newton_centimeter / 100.0)
}
#[must_use]
pub fn newton_centimeter_to_inch_pound(newton_centimeter: f64) -> f64 {
    newton_meter_to_inch_pound(newton_centimeter / 100.0)
}
#[must_use]
pub fn kilogram_meter_to_foot_pound(kilogram_meter: f64) -> f64 {
    kilogram_meter * 7.23301
}
