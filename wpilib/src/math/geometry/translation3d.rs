use nalgebra::{ComplexField, Quaternion};
use std::ops;

use crate::math::units::distance::Meter;
use crate::math::util::math_util::MathUtil;

use super::{Rotation3d, Translation2d};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation3d {
    pub x: Meter,
    pub y: Meter,
    pub z: Meter,
}

impl Translation3d {
    #[must_use]
    pub fn new(x: impl Into<Meter>, y: impl Into<Meter>, z: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    #[must_use]
    pub fn new_dist_angle(dist: impl Into<Meter>, angle: Rotation3d) -> Self {
        let rectangle = Self::new(dist, 0.0, 0.0).rotate_by(&angle);
        Self {
            x: rectangle.x,
            y: rectangle.y,
            z: rectangle.z,
        }
    }

    #[must_use]
    fn get_distance(&self, other: &Self) -> Meter {
        ComplexField::sqrt(
            (other.x - self.x).square() + (other.y - self.y).square() + (other.z - self.z).square(),
        )
    }

    #[must_use]
    pub fn get_norm(&self) -> Meter {
        ComplexField::sqrt(self.x.square() + self.y.square() + self.z.square())
    }

    #[must_use]
    pub fn rotate_by(&self, other: &Rotation3d) -> Self {
        let p = Quaternion::new(0.0, self.x.into(), self.y.into(), self.z.into());
        let mut qprime: Quaternion<f64> = other.q.quaternion() * p;
        //TODO: inversion quaternion meanie
        other.q.try_inverse().map_or_else(
            || panic!("ROTATED BY ZERO QUATERNION 😪"),
            |invert| {
                qprime *= invert;
            },
        );

        Self::new(qprime.i, qprime.j, qprime.k)
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    #[must_use]
    pub fn unary_minus(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new(
            f64::from(self.x) * scalar,
            f64::from(self.y) * scalar,
            f64::from(self.z) * scalar,
        )
    }

    #[must_use]
    pub fn divide(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn interpolate(&self, end_value: Self, t: f64) -> Self {
        Self::new(
            MathUtil::interpolate(self.x.into(), end_value.x.into(), t),
            MathUtil::interpolate(self.y.into(), end_value.y.into(), t),
            MathUtil::interpolate(self.z.into(), end_value.z.into(), t),
        )
    }
}

impl Default for Translation3d {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<Translation2d> for Translation3d {
    fn from(translation: Translation2d) -> Self {
        Self::new(translation.x, translation.y, 0.0)
    }
}

impl From<Translation3d> for Translation2d {
    fn from(translation: Translation3d) -> Self {
        Self::new(translation.x, translation.y)
    }
}

impl ops::Add<Translation3d> for Translation3d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.plus(&other)
    }
}

impl ops::AddAssign<Translation3d> for Translation3d {
    fn add_assign(&mut self, other: Self) {
        *self = self.plus(&other);
    }
}

impl ops::Sub<Translation3d> for Translation3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.minus(&other)
    }
}

impl ops::SubAssign<Translation3d> for Translation3d {
    fn sub_assign(&mut self, other: Self) {
        *self = self.minus(&other);
    }
}

impl ops::Mul<f64> for Translation3d {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        self.times(scalar)
    }
}

impl ops::Mul<Translation3d> for f64 {
    type Output = Translation3d;

    fn mul(self, translation: Translation3d) -> Translation3d {
        translation.times(self)
    }
}

impl ops::MulAssign<f64> for Translation3d {
    fn mul_assign(&mut self, scalar: f64) {
        *self = self.times(scalar);
    }
}

impl ops::Div<f64> for Translation3d {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        self.divide(scalar)
    }
}

impl ops::DivAssign<f64> for Translation3d {
    fn div_assign(&mut self, scalar: f64) {
        *self = self.divide(scalar);
    }
}

impl ops::Neg for Translation3d {
    type Output = Self;

    fn neg(self) -> Self {
        self.unary_minus()
    }
}
