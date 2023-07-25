use super::{Pose2d, Rotation2d, Translation2d};
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform2d {
    pub translation: Translation2d,
    pub rotation: Rotation2d,
}

impl Transform2d {
    #[must_use]
    pub fn new(initial: Pose2d, last: Pose2d) -> Self {
        let translation = last
            .translation
            .minus(&initial.translation)
            .rotate_by(&initial.rotation.unary_minus());
        let rotation = last.rotation.minus(&initial.rotation);
        Self {
            translation,
            rotation,
        }
    }

    #[must_use]
    pub const fn new_trans_rot(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new_trans_rot(self.translation.times(scalar), self.rotation.times(scalar))
    }

    #[must_use]
    pub fn divide(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        Self::new(
            Pose2d::default(),
            Pose2d::default().transform_by(*self).transform_by(*other),
        )
    }

    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        self.plus(&other.inverse())
    }

    #[must_use]
    pub fn inverse(&self) -> Self {
        Self::new_trans_rot(
            self.translation
                .unary_minus()
                .rotate_by(&self.rotation.unary_minus()),
            self.rotation.unary_minus(),
        )
    }
}

impl Default for Transform2d {
    fn default() -> Self {
        Self::new(Pose2d::default(), Pose2d::default())
    }
}

impl ops::Add for Transform2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.plus(&other)
    }
}

impl ops::AddAssign for Transform2d {
    fn add_assign(&mut self, other: Self) {
        *self = self.plus(&other);
    }
}

impl ops::Sub for Transform2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.minus(&other)
    }
}

impl ops::SubAssign for Transform2d {
    fn sub_assign(&mut self, other: Self) {
        *self = self.minus(&other);
    }
}

impl ops::Neg for Transform2d {
    type Output = Self;
    fn neg(self) -> Self {
        self.inverse()
    }
}

impl ops::Mul<f64> for Transform2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        self.times(scalar)
    }
}

impl ops::Mul<Transform2d> for f64 {
    type Output = Transform2d;
    fn mul(self, transform: Transform2d) -> Transform2d {
        transform.times(self)
    }
}

impl ops::MulAssign<f64> for Transform2d {
    fn mul_assign(&mut self, scalar: f64) {
        *self = self.times(scalar);
    }
}

impl ops::Div<f64> for Transform2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        self.divide(scalar)
    }
}

impl ops::DivAssign<f64> for Transform2d {
    fn div_assign(&mut self, scalar: f64) {
        *self = self.divide(scalar);
    }
}
