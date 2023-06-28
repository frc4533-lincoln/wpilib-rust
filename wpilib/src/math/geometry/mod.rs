



use nalgebra::Translation2;

use super::units::{distance::Meter, angle::Radian};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation2d {
    pub inner: Radian
}
impl Rotation2d {
    pub fn new(angle: impl Into<Radian>) -> Self {
        Self {
            inner: angle.into()
        }
    }
    pub fn angle(&self) -> Radian {
        self.inner
    }
    pub fn plus(&self, other: &Self) -> Self {
        Self {
            inner: self.inner + other.inner
        }
    }
    pub fn minus(&self, other: &Self) -> Self {
        Self {
            inner: self.inner - other.inner
        }
    }
    pub fn inverse(&self) -> Self {
        Self {
            inner: -self.inner
        }
    }
    pub fn rotate_by(&self, other: &Rotation2d) -> Self {
        Self {
            inner: self.inner + other.inner
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Translation2d {
    pub inner: Translation2<Meter>
}

impl Translation2d {
    pub fn new(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        Self {
            inner: Translation2::new(x.into(), y.into())
        }
    }
    pub fn x(&self) -> Meter {
        self.inner.x
    }
    pub fn y(&self) -> Meter {
        self.inner.y
    }
    pub fn strength(&self) -> Meter {
        self.inner.vector.norm().into()
    }
    pub fn normalize(&mut self) {
        self.inner.vector.normalize_mut();
    }
}