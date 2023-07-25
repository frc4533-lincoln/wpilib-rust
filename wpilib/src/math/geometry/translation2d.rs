use nalgebra::{ComplexField, Translation2};
use std::ops;

use super::Rotation2d;
use crate::math::units::distance::Meter;
use crate::math::util::math_util::MathUtil;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation2d {
    pub x: Meter,
    pub y: Meter,
}

impl Translation2d {
    #[must_use]
    pub fn new(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    #[must_use]
    pub fn new_dist_angle(distance: impl Into<Meter>, angle: Rotation2d) -> Self {
        let distance: Meter = distance.into();
        Self {
            x: (f64::from(distance) * angle.cos).into(),
            y: (f64::from(distance) * angle.sin).into(),
        }
    }

    #[must_use]
    pub fn get_distance(&self, other: &Self) -> Meter {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        ComplexField::hypot(delta_x, delta_y).into()
    }

    #[must_use]
    pub fn get_norm(&self) -> Meter {
        ComplexField::hypot(self.x, self.y).into()
    }

    #[must_use]
    pub fn get_angle(&self) -> Rotation2d {
        Rotation2d::new_xy(self.x, self.y)
    }

    #[must_use]
    pub fn rotate_by(&self, other: &Rotation2d) -> Self {
        let x = f64::from(self.x);
        let y = f64::from(self.y);
        Self::new(
            x.mul_add(other.cos, -y * other.sin),
            x.mul_add(other.sin, y * other.cos),
        )
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    #[must_use]
    pub fn unary_minus(&self) -> Self {
        Self::new(-self.x, -self.y)
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new(f64::from(self.x) * scalar, f64::from(self.y) * scalar)
    }

    #[must_use]
    pub fn divide(&self, scalar: f64) -> Self {
        Self::new(f64::from(self.x) / scalar, f64::from(self.y) / scalar)
    }

    //pls work 🥺🙏
    #[must_use]
    pub fn nearest(&self, translations: &[Self]) -> Self {
        let mut nearest = translations[0];
        let mut nearest_distance = self.get_distance(&nearest);
        for translation in translations {
            let distance = self.get_distance(translation);
            if distance < nearest_distance {
                nearest = *translation;
                nearest_distance = distance;
            }
        }
        nearest
    }

    #[must_use]
    pub const fn get_vector(&self) -> Translation2<Meter> {
        Translation2::new(self.x, self.y)
    }

    // #[must_use] pub fn strength(&self) -> Meter {
    //     self.inner.vector.norm().into()
    // }
    // pub fn normalize(&mut self) {
    //     self.inner.vector.normalize_mut();
    // }

    #[must_use]
    pub fn interpolate(&self, other: &Self, t: f64) -> Self {
        Self::new(
            MathUtil::interpolate(f64::from(self.x), f64::from(other.x), t),
            MathUtil::interpolate(f64::from(self.y), f64::from(other.y), t),
        )
    }
}

impl Default for Translation2d {
    fn default() -> Self {
        Self::new(Meter::new(0.0), Meter::new(0.0))
    }
}

impl ops::Add for Translation2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.plus(&other)
    }
}

impl ops::AddAssign for Translation2d {
    fn add_assign(&mut self, other: Self) {
        *self = self.plus(&other);
    }
}

impl ops::Sub for Translation2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.minus(&other)
    }
}

impl ops::SubAssign for Translation2d {
    fn sub_assign(&mut self, other: Self) {
        *self = self.minus(&other);
    }
}

impl ops::Neg for Translation2d {
    type Output = Self;
    fn neg(self) -> Self {
        self.unary_minus()
    }
}

impl ops::Mul<f64> for Translation2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        self.times(scalar)
    }
}

impl ops::Mul<Translation2d> for f64 {
    type Output = Translation2d;
    fn mul(self, translation: Translation2d) -> Translation2d {
        translation.times(self)
    }
}

impl ops::MulAssign<f64> for Translation2d {
    fn mul_assign(&mut self, scalar: f64) {
        *self = self.times(scalar);
    }
}

impl ops::Div<f64> for Translation2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        self.divide(scalar)
    }
}

impl ops::DivAssign<f64> for Translation2d {
    fn div_assign(&mut self, scalar: f64) {
        *self = self.divide(scalar);
    }
}
