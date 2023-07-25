use crate::math::geometry::{Rotation2d, Rotation3d, Translation2d, Translation3d};
use crate::math::units::angle::Degree;
use crate::math::units::distance::Meter;
use approx::assert_relative_eq;
use nalgebra::UnitQuaternion;

#[test]
fn rotation2d() {
    let rot_1 = Rotation2d::new(Degree::new(90.0));
    let rot_2 = Rotation2d::new(Degree::new(180.0));

    // Addition
    assert_eq!(rot_1 + rot_2, Rotation2d::new(Degree::new(270.0)));
    let mut temp = rot_1;
    temp += rot_1;
    assert_eq!(temp, Rotation2d::new(Degree::new(180.0)));

    // Subtraction
    assert_eq!(rot_1 - rot_2, Rotation2d::new(Degree::new(-90.0)));
    let mut temp = rot_1;
    temp -= rot_1;
    assert_eq!(temp, Rotation2d::new(Degree::new(0.0)));

    // Unary minus
    assert_eq!(-rot_1, Rotation2d::new(Degree::new(-90.0)));

    // Multiplication
    assert_eq!(rot_1 * 2.0, Rotation2d::new(Degree::new(180.0)));
    assert_eq!(2.0 * rot_1, Rotation2d::new(Degree::new(180.0)));

    // Division
    assert_eq!(rot_1 / 2.0, Rotation2d::new(Degree::new(45.0)));

    // Equality
    assert_eq!(rot_1, Rotation2d::new(Degree::new(90.0)));
}

#[test]
fn translation2d() {
    let blahaj_1 = Translation2d::new(Meter::new(1.0), Meter::new(1.0));
    let blahaj_2 = Translation2d::new(Meter::new(2.0), Meter::new(2.0));

    // Addition
    assert_eq!(
        blahaj_1 + blahaj_2,
        Translation2d::new(Meter::new(3.0), Meter::new(3.0))
    );
    let mut temp = blahaj_1;
    temp += blahaj_1;
    assert_eq!(temp, Translation2d::new(Meter::new(2.0), Meter::new(2.0)));

    // Subtraction
    assert_eq!(
        blahaj_1 - blahaj_2,
        Translation2d::new(Meter::new(-1.0), Meter::new(-1.0))
    );
    let mut temp = blahaj_1;
    temp -= blahaj_1;
    assert_eq!(temp, Translation2d::new(Meter::new(0.0), Meter::new(0.0)));

    // Multiplication
    assert_eq!(
        blahaj_1 * 2.0,
        Translation2d::new(Meter::new(2.0), Meter::new(2.0))
    );
    assert_eq!(
        2.0 * blahaj_1,
        Translation2d::new(Meter::new(2.0), Meter::new(2.0))
    );

    // Division
    assert_eq!(
        blahaj_1 / 2.0,
        Translation2d::new(Meter::new(0.5), Meter::new(0.5))
    );

    // Equality
    assert_eq!(
        blahaj_1,
        Translation2d::new(Meter::new(1.0), Meter::new(1.0))
    );
}

#[test]
fn rotation3d() {
    let rot_1 = Rotation3d::new(Degree::new(90.0), Degree::new(0.0), Degree::new(0.0));
    let rot_2 = Rotation3d::new(Degree::new(180.0), Degree::new(0.0), Degree::new(0.0));

    // Addition
    let temp = rot_1 + rot_2;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(270.0), Degree::new(0.0), Degree::new(0.0)).q,
    );
    let mut temp = rot_1;
    temp += rot_1;
    quaternian_relative_eq(temp.q, rot_2.q);

    // Subtraction
    let temp = rot_1 - rot_2;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(-90.0), Degree::new(0.0), Degree::new(0.0)).q,
    );
    let mut temp = rot_1;
    temp -= rot_1;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(0.0), Degree::new(0.0), Degree::new(0.0)).q,
    );

    // Unary minus
    let temp = -rot_1;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(-90.0), Degree::new(0.0), Degree::new(0.0)).q,
    );

    // Multiplication
    let temp = rot_1 * 2.0;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(180.0), Degree::new(0.0), Degree::new(0.0)).q,
    );
    let temp = 2.0 * rot_1;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(180.0), Degree::new(0.0), Degree::new(0.0)).q,
    );

    // Division
    let temp = rot_1 / 2.0;
    quaternian_relative_eq(
        temp.q,
        Rotation3d::new(Degree::new(45.0), Degree::new(0.0), Degree::new(0.0)).q,
    );
}

fn quaternian_relative_eq(q1: UnitQuaternion<f64>, q2: UnitQuaternion<f64>) {
    assert_relative_eq!(q1.w, q2.w);
    assert_relative_eq!(q1.i, q2.i);
    assert_relative_eq!(q1.j, q2.j);
    assert_relative_eq!(q1.k, q2.k);
}

#[test]
fn translation3d() {
    let blahaj_1 = Translation3d::new(Meter::new(1.0), Meter::new(1.0), Meter::new(1.0));
    let blahaj_2 = Translation3d::new(Meter::new(2.0), Meter::new(2.0), Meter::new(2.0));

    // Addition
    assert_eq!(
        blahaj_1 + blahaj_2,
        Translation3d::new(Meter::new(3.0), Meter::new(3.0), Meter::new(3.0))
    );
    let mut temp = blahaj_1;
    temp += blahaj_1;
    assert_eq!(temp, blahaj_2);

    // Subtraction
    assert_eq!(
        blahaj_1 - blahaj_2,
        Translation3d::new(Meter::new(-1.0), Meter::new(-1.0), Meter::new(-1.0))
    );
    let mut temp = blahaj_1;
    temp -= blahaj_1;
    assert_eq!(temp, Translation3d::default());

    // Multiplication
    assert_eq!(blahaj_1 * 2.0, blahaj_2);
    assert_eq!(2.0 * blahaj_1, blahaj_2);
    let mut temp = blahaj_1;
    temp *= 2.0;
    assert_eq!(temp, blahaj_2);

    // Division
    assert_eq!(
        blahaj_1 / 2.0,
        Translation3d::new(Meter::new(0.5), Meter::new(0.5), Meter::new(0.5))
    );
    let mut temp = blahaj_1;
    temp /= 2.0;
    assert_eq!(
        temp,
        Translation3d::new(Meter::new(0.5), Meter::new(0.5), Meter::new(0.5))
    );
}
