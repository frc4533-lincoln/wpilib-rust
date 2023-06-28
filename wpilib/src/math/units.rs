
wpilib_macros::unit!(Meter, f64);
wpilib_macros::unit!(Feet, f64);
wpilib_macros::unit!(Inch, f64);


wpilib_macros::unit_conversion!(Meter f64, Feet f64, meter_to_feet);
wpilib_macros::unit_conversion!(Meter f64, Inch f64, meter_to_inch);
wpilib_macros::unit_conversion!(Feet f64, Inch f64, foot_to_inch);

pub fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}
pub fn meter_to_inch(meter: f64) -> f64 {
    meter * 3.28084 * 12.0
}
pub fn foot_to_inch(foot: f64) -> f64 {
    foot * 12.0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_meter_to_feet() {
        let meter = Meter::new(1.0);
        let feet = Feet::new(3.28084);
        assert_eq!(feet, meter);
        assert_eq!(meter, feet);
        let double = feet + meter;
        assert!(double == Feet::new(6.56168));
        assert!(double == Meter::new(2.0));
        assert!(double > feet);
    }
}