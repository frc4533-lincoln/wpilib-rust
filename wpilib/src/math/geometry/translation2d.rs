use nalgebra::{ComplexField, Translation2};

use super::Rotation2d;
use crate::math::units::distance::Meter;
use crate::math::util::math_util::MathUtil;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation2d {
    pub x: Meter,
    pub y: Meter,
}

impl Translation2d {
    pub fn new() -> Self {
        Self {
            x: 0.0.into(),
            y: 0.0.into(),
        }
    }
    pub fn new_xy(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn new_dist_angle(distance: impl Into<Meter>, angle: Rotation2d) -> Self {
        let distance: Meter = distance.into();
        Self {
            x: (f64::from(distance) * angle.cos).into(),
            y: (f64::from(distance) * angle.sin).into(),
        }
    }

    pub fn get_distance(&self, other: &Self) -> Meter {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        ComplexField::hypot(delta_x, delta_y).into()
    }

    pub fn get_norm(&self) -> Meter {
        ComplexField::hypot(self.x, self.y).into()
    }

    pub fn get_angle(&self) -> Rotation2d {
        Rotation2d::new_xy(self.x, self.y)
    }

    pub fn rotate_by(&self, other: &Rotation2d) -> Self {
        let x = f64::from(self.x);
        let y = f64::from(self.y);
        Self::new_xy(x * other.cos - y * other.sin, x * other.sin + y * other.cos)
    }

    pub fn plus(&self, other: &Self) -> Self {
        Self::new_xy(self.x + other.x, self.y + other.y)
    }

    pub fn minus(&self, other: &Self) -> Self {
        Self::new_xy(self.x - other.x, self.y - other.y)
    }

    pub fn unary_minus(&self) -> Self {
        Self::new_xy(-self.x, -self.y)
    }

    pub fn times(&self, scalar: f64) -> Self {
        Self::new_xy(f64::from(self.x) * scalar, f64::from(self.y) * scalar)
    }

    pub fn div(&self, scalar: f64) -> Self {
        Self::new_xy(f64::from(self.x) / scalar, f64::from(self.y) / scalar)
    }

    //pls work 🥺🙏
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

    pub fn get_vector(&self) -> Translation2<Meter> {
        Translation2::new(self.x, self.y)
    }

    // #[must_use] pub fn strength(&self) -> Meter {
    //     self.inner.vector.norm().into()
    // }
    // pub fn normalize(&mut self) {
    //     self.inner.vector.normalize_mut();
    // }

    pub fn interpolate(&self, other: &Self, t: f64) -> Self {
        Self::new_xy(
            MathUtil::interpolate(f64::from(self.x), f64::from(other.x), t),
            MathUtil::interpolate(f64::from(self.y), f64::from(other.y), t),
        )
    }
}
