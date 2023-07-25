use super::{Rotation2d, Transform2d, Translation2d, Twist2d};

use crate::math::units::distance::Meter;

use nalgebra::ComplexField;

// type Transform2d = Pose2d;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pose2d {
    pub translation: Translation2d,
    pub rotation: Rotation2d,
}

impl Pose2d {
    #[must_use]
    pub const fn new(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn new_xy_rot(x: impl Into<Meter>, y: impl Into<Meter>, rotation: Rotation2d) -> Self {
        Self::new(Translation2d::new(x, y), rotation)
    }

    #[must_use]
    pub fn plus(&self, other: Transform2d) -> Self {
        self.transform_by(other)
    }

    #[must_use]
    pub fn minus(&self, other: &Self) -> Transform2d {
        let pose = self.relative_to(other);
        Transform2d::new_trans_rot(pose.translation, pose.rotation)
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new(self.translation.times(scalar), self.rotation.times(scalar))
    }

    #[must_use]
    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn transform_by(&self, other: Transform2d) -> Self {
        Self::new(
            self.translation
                .plus(&other.translation.rotate_by(&self.rotation)),
            other.rotation.plus(&self.rotation),
        )
    }

    #[must_use]
    pub fn relative_to(&self, other: &Self) -> Self {
        let transform = Transform2d::new(*other, *self);
        Self::new(transform.translation, transform.rotation)
    }

    #[must_use]
    pub fn exp(&self, twist: Twist2d) -> Self {
        let dx: f64 = twist.dx.into();
        let dy: f64 = twist.dy.into();
        let dtheta: f64 = twist.dtheta.into();

        let sin_theta = dtheta.sin();
        let cos_theta = dtheta.cos();

        let s: f64;
        let c: f64;

        if dtheta.abs() < 1e-9 {
            s = 1.0 - 1.0 / 6.0 * dtheta * dtheta;
            c = 0.5 * dtheta;
        } else {
            s = sin_theta / dtheta;
            c = (1.0 - cos_theta) / dtheta;
        }
        let transform = Transform2d::new_trans_rot(
            Translation2d::new(dx.mul_add(s, -dy * c), dx.mul_add(c, dy * s)),
            Rotation2d::new_xy(cos_theta, sin_theta),
        );
        self.plus(transform)
    }

    #[must_use]
    pub fn log(&self, end: &Self) -> Twist2d {
        let transform: Self = end.relative_to(self);
        let dtheta: f64 = transform.rotation.value.into();
        let half_dtheta: f64 = dtheta / 2.0;

        let cos_minus_one = transform.rotation.cos - 1.0;

        let halftheta_by_tan_of_halfdtheta = if cos_minus_one.abs() < 1e-9 {
            (1.0 / 12.0 * dtheta).mul_add(-dtheta, 1.0)
        } else {
            -(half_dtheta * transform.rotation.sin) / (cos_minus_one)
        };
        let translation_part = transform
            .translation
            .rotate_by(&Rotation2d::new_xy(
                halftheta_by_tan_of_halfdtheta,
                -half_dtheta,
            ))
            .times(ComplexField::hypot(
                halftheta_by_tan_of_halfdtheta,
                half_dtheta,
            ));

        let dx = translation_part.x;
        let dy = translation_part.y;
        Twist2d::new(dx, dy, dtheta)
    }

    //pls work 🥺🙏
    #[must_use]
    pub fn nearest(&self, poses: &[Self]) -> Self {
        let mut nearest = poses[0];
        let mut nearest_distance = self.translation.get_distance(&nearest.translation);
        for pose in poses {
            let distance = self.translation.get_distance(&pose.translation);
            if distance < nearest_distance {
                nearest = *pose;
                nearest_distance = distance;
            }
        }
        nearest
    }

    #[must_use]
    pub fn interpolate(&self, end_value: &Self, t: f64) -> Self {
        if t < 0.0 {
            *self
        } else if t >= 1.0 {
            *end_value
        } else {
            let twist = self.log(end_value);
            let scaled_twist = Twist2d::new(
                f64::from(twist.dx) * t,
                f64::from(twist.dy) * t,
                f64::from(twist.dtheta) * t,
            );
            self.exp(scaled_twist)
        }
    }
}

impl Default for Pose2d {
    fn default() -> Self {
        Self {
            translation: Translation2d::default(),
            rotation: Rotation2d::default(),
        }
    }
}
