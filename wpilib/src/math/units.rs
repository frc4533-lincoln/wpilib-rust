

wpilib_macros::unit!(Meter, f64);
wpilib_macros::unit!(Feet, f64);


wpilib_macros::unit_conversion!(Meter f64, Feet f64, meter_to_feet);

pub fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}