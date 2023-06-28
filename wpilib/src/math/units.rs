
use wpilib_macros::{unit, unit_conversion};


///// distance /////
unit!(Meter, f64);
unit!(Feet, f64);
unit!(Inch, f64);
unit!(CentiMeter, f64);

unit_conversion!(Meter f64, Feet f64, meter_to_feet);
unit_conversion!(Meter f64, Inch f64, meter_to_inch);
unit_conversion!(Feet f64, Inch f64, foot_to_inch);
unit_conversion!(Meter f64, CentiMeter f64, meter_to_centimeter);
unit_conversion!(CentiMeter f64, Feet f64, centimeter_to_foot);
unit_conversion!(CentiMeter f64, Inch f64, centimeter_to_inch);

pub fn meter_to_feet(meter: f64) -> f64 {
    meter * 3.28084
}
pub fn meter_to_inch(meter: f64) -> f64 {
    meter * 3.28084 * 12.0
}
pub fn foot_to_inch(foot: f64) -> f64 {
    foot * 12.0
}
pub fn meter_to_centimeter(meter: f64) -> f64 {
    meter * 100.0
}
pub fn centimeter_to_foot(centimeter: f64) -> f64 {
    meter_to_feet(centimeter / 100.0)
}
pub fn centimeter_to_inch(centimeter: f64) -> f64 {
    meter_to_inch(centimeter / 100.0)
}
////////////////////

///// angle /////
unit!(Degree, f64);
unit!(Radian, f64);
unit!(Rotation, f64);

unit_conversion!(Degree f64, Radian f64, degree_to_radian);
unit_conversion!(Degree f64, Rotation f64, degree_to_rotation);
unit_conversion!(Radian f64, Rotation f64, radian_to_rotation);

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}
pub fn degree_to_rotation(degree: f64) -> f64 {
    degree / 360.0
}
pub fn radian_to_rotation(radian: f64) -> f64 {
    degree_to_rotation(radian * 180.0 / std::f64::consts::PI)
}
////////////////////

///// time /////
unit!(Second, f64);
unit!(Milisecond, f64);
unit!(Microsecond, u64);

unit_conversion!(Second f64, Milisecond f64, second_to_milisecond);
unit_conversion!(Second f64, Microsecond u64, second_to_microsecond);
unit_conversion!(Milisecond f64, Microsecond u64, milisecond_to_microsecond);

pub fn second_to_milisecond(second: f64) -> f64 {
    second * 1000.0
}
pub fn second_to_microsecond(second: f64) -> u64 {
    (second * 1000_000.0) as u64
}
pub fn milisecond_to_microsecond(milisecond: f64) -> u64 {
    (milisecond * 1000.0) as u64
}
////////////////////

///// weight /////
unit!(Kilogram, f64);
unit!(Gram, f64);
unit!(Pound, f64);
unit!(Ounce, f64);

unit_conversion!(Kilogram f64, Gram f64, kilogram_to_gram);
unit_conversion!(Kilogram f64, Pound f64, kilogram_to_pound);
unit_conversion!(Kilogram f64, Ounce f64, kilogram_to_ounce);
unit_conversion!(Gram f64, Pound f64, gram_to_pound);
unit_conversion!(Gram f64, Ounce f64, gram_to_ounce);
unit_conversion!(Pound f64, Ounce f64, pound_to_ounce);

pub fn kilogram_to_gram(kilogram: f64) -> f64 {
    kilogram * 1000.0
}
pub fn kilogram_to_pound(kilogram: f64) -> f64 {
    kilogram * 2.20462
}
pub fn kilogram_to_ounce(kilogram: f64) -> f64 {
    kilogram * 35.274
}
pub fn gram_to_pound(gram: f64) -> f64 {
    kilogram_to_pound(gram / 1000.0)
}
pub fn gram_to_ounce(gram: f64) -> f64 {
    kilogram_to_ounce(gram / 1000.0)
}
pub fn pound_to_ounce(pound: f64) -> f64 {
    kilogram_to_ounce(pound / 2.20462)
}
////////////////////



















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