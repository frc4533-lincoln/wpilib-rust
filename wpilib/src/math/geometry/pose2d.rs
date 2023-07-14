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
    pub fn new() -> Self {
        Self {
            translation: Translation2d::new(),
            rotation: Rotation2d::new(),
        }
    }

    pub fn new_trans_rot(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn new_xy_rot(x: impl Into<Meter>, y: impl Into<Meter>, rotation: Rotation2d) -> Self {
        Self::new_trans_rot(Translation2d::new_xy(x, y), rotation)
    }

    pub fn plus(&self, other: Transform2d) -> Self {
        self.transform_by(other)
    }

    pub fn minus(&self, other: &Self) -> Transform2d {
        let pose = self.relative_to(other);
        Transform2d::new_trans_rot(pose.translation, pose.rotation)
    }

    pub fn times(&self, scalar: f64) -> Self {
        Self::new_trans_rot(self.translation.times(scalar), self.rotation.times(scalar))
    }

    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    pub fn transform_by(&self, other: Transform2d) -> Self {
        Self::new_trans_rot(
            self.translation
                .plus(&other.translation.rotate_by(&self.rotation)),
            other.rotation.plus(&self.rotation),
        )
    }

    pub fn relative_to(&self, other: &Self) -> Self {
        let transform = Transform2d::new_pose_pose(*other, *self);
        Self::new_trans_rot(transform.translation, transform.rotation)
    }

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
            Translation2d::new_xy(dx * s - dy * c, dx * c + dy * s),
            Rotation2d::new_xy(cos_theta, sin_theta),
        );
        self.plus(transform)
    }

    pub fn log(&self, end: &Self) -> Twist2d {
        let transform: Self = end.relative_to(self);
        let dtheta: f64 = transform.rotation.value.into();
        let half_dtheta: f64 = dtheta / 2.0;

        let cos_minus_one = transform.rotation.cos - 1.0;

        let halftheta_by_tan_of_halfdtheta: f64; //this name 😪
        if cos_minus_one.abs() < 1e-9 {
            halftheta_by_tan_of_halfdtheta = 1.0 - 1.0 / 12.0 * dtheta * dtheta;
        } else {
            halftheta_by_tan_of_halfdtheta =
                -(half_dtheta * transform.rotation.sin) / (cos_minus_one);
        }
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
        Twist2d::new_dv(dx, dy, dtheta)
    }

    //pls work 🥺🙏
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

    pub fn interpolate(&self, end_value: &Self, t: f64) -> Self {
        if t < 0.0 {
            *self
        } else if t >= 1.0 {
            *end_value
        } else {
            let twist = self.log(end_value);
            let scaled_twist = Twist2d::new_dv(
                f64::from(twist.dx) * t,
                f64::from(twist.dy) * t,
                f64::from(twist.dtheta) * t,
            );
            self.exp(scaled_twist)
        }
    }
}
