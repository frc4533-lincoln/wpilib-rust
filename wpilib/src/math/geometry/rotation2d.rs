use crate::math::units::{angle::Radian, distance::Meter};
use std::fmt::Display;
use std::ops;

use nalgebra::ComplexField;
use num::clamp;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation2d {
    pub value: Radian,
    pub sin: f64,
    pub cos: f64,
}
impl Rotation2d {
    #[must_use]
    pub fn new(angle: impl Into<Radian>) -> Self {
        let value: Radian = angle.into();
        Self {
            value,
            sin: value.sin().into(),
            cos: value.cos().into(),
        }
    }
    pub fn new_xy(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        let x = x.into();
        let y = y.into();
        let magnitude = x.hypot(y);
        let sin;
        let cos;
        if magnitude > 1e-6 {
            sin = f64::from(y) / magnitude;
            cos = f64::from(x) / magnitude;
        } else {
            sin = 0.0;
            cos = 1.0;
        }
        let value = sin.atan2(cos);
        Self {
            value: value.into(),
            sin,
            cos,
        }
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        self.rotate_by(other)
    }
    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        self.rotate_by(&other.unary_minus())
    }
    #[must_use]
    pub fn unary_minus(&self) -> Self {
        Self::new(-self.value)
    }
    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new(f64::from(self.value) * scalar)
    }
    #[must_use]
    pub fn divide(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }
    #[must_use]
    pub fn rotate_by(&self, other: &Self) -> Self {
        Self::new(self.value + other.value)
    }

    #[must_use]
    pub fn get_tan(&self) -> f64 {
        self.sin / self.cos
    }

    #[must_use]
    pub fn interpolate(&self, end_value: &Self, t: f64) -> Self {
        self.plus(&end_value.minus(self).times(clamp(t, 0.0, 1.0)))
    }
}

impl Default for Rotation2d {
    fn default() -> Self {
        Self::new(0)
    }
}

impl ops::Add for Rotation2d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.plus(&rhs)
    }
}

impl ops::AddAssign for Rotation2d {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.plus(&rhs);
    }
}

impl ops::Sub for Rotation2d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(&rhs)
    }
}

impl ops::SubAssign for Rotation2d {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.minus(&rhs);
    }
}

impl ops::Neg for Rotation2d {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.unary_minus()
    }
}

impl ops::Mul<f64> for Rotation2d {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        self.times(rhs)
    }
}

impl ops::Mul<Rotation2d> for f64 {
    type Output = Rotation2d;
    fn mul(self, rhs: Rotation2d) -> Self::Output {
        rhs.times(self)
    }
}

impl ops::MulAssign<f64> for Rotation2d {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.times(rhs);
    }
}

impl ops::Div<f64> for Rotation2d {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self.divide(rhs)
    }
}

impl ops::DivAssign<f64> for Rotation2d {
    fn div_assign(&mut self, rhs: f64) {
        *self = self.divide(rhs);
    }
}

impl Display for Rotation2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rad", self.value)
    }
}
