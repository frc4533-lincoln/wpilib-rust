
use nalgebra::{Matrix3, Vector3, Quaternion, Rotation3, ComplexField};

use crate::math::units::{distance::Meter, angle::Radian};

use super::{Rotation3d, Transform3d, Translation3d, Twist3d};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pose3d {
    pub translation: Translation3d,
    pub rotation: Rotation3d,
}

impl Pose3d {
    pub fn new () -> Self {
        Self {
            translation: Translation3d::new(),
            rotation: Rotation3d::new(),
        }
    }

    pub fn new_trans_rot (translation: Translation3d, rotation: Rotation3d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn new_xyz_rot (x: impl Into<Meter>, y: impl Into<Meter>, z: impl Into<Meter>, rotation: Rotation3d) -> Self {
        Self::new_trans_rot(
            Translation3d::new_xyz(x, y, z),
            rotation)
    }

    pub fn plus (&self, other: Transform3d) -> Self {
        self.transform_by(other)
    }

    pub fn minus (&self, other: &Self) -> Transform3d {
        let pose = self.relative_to(other);
        Transform3d::new_trans_rot(pose.translation, pose.rotation)
    }

    pub fn times (&self, scalar: f64) -> Self {
        Self::new_trans_rot(self.translation.times(scalar), self.rotation.times(scalar))
    }

    pub fn div (&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    pub fn transform_by (&self, other: Transform3d) -> Self {
        Self::new_trans_rot(
            self.translation.plus(&other.translation.rotate_by(&self.rotation)),
            other.rotation.plus(&self.rotation),
        )
    }

    pub fn relative_to (&self, other: &Self) -> Self {
        let transform = Transform3d::new_pose_pose(*other, *self);
        Self::new_trans_rot(transform.translation, transform.rotation)
    }

    // pub fn exp (&self, twist: Twist3d) -> Self {
    //     let u = Vector3::new(twist.dx, twist.dy, twist.dz);
    //     let rvec = Vector3::new(twist.rx, twist.ry, twist.rz);
    //     let omega = self.rotation_vector_to_matrix(rvec);
    //     let omgega_sq = omega * omega;
    //     let theta = rvec.norm();
    //     let theta_sq = theta * theta;

    //     let a: f64;
    //     let b: f64;
    //     let c: f64;
    //     if theta.abs() < 1E-7 {
    //         a = 1.0 - theta_sq / 6.0 + theta_sq * theta_sq / 120.0;
    //         b = 0.5 - theta_sq / 24.0 + theta_sq * theta_sq / 720.0;
    //         c = 1.0 / 6.0 - theta_sq / 120.0 + theta_sq * theta_sq / 5040.0;
    //     } else {
    //         a = theta.sin() / theta;
    //         b = (1.0 - theta.cos()) / theta_sq;
    //         c = (1.0 - a) / theta_sq;
    //     }

    //     let r = Matrix3::identity() + a.into() * omega + b.into() * omgega_sq;
    //     let v = Matrix3::identity() + b.into() * omega + c.into() * omgega_sq;
    //     let translation_component: Matrix3<f64> = v * u;

    //     let transform = Transform3d::new_trans_rot(
    //         Translation3d::new_xyz(
    //             translation_component[0],
    //             translation_component[1],
    //             translation_component[2],
    //         ),
    //         Rotation3d::new_quaternion(Quaternion::from(r)),
    //     );
    //     self.plus(transform)

    // }

    // pub fn log (&self, end: &Self) -> Twist3d { 
    //     let transform = end.relative_to(self);
    //     let rvec = Vector3::fromransform.rotation.q.into());transform.rotation.q;

    //     let omega = self.rotation_vector_to_matrix(rvec);
    //     let theta = rvec.norm();
    //     let theta_sq = theta * theta;

    //     let c: f64;
    //     if theta.abs() < 1E-7 {
    //         c = 1.0 / 12.0 + theta_sq / 720.0 + theta_sq * theta_sq / 30240.0;
    //     } else {
    //         let a = theta.sin() / theta;
    //         let b = (1.0 - theta.cos()) / theta_sq;
    //         c = (1.0 - a / (2.0 * b)) / theta_sq;
    //     }

    //     let v_inv = Matrix3::identity() - 0.5.into() * omega + c.into() * omega * omega;

    //     let twist_translation = v_inv * Vector3::new(
    //         transform.translation.x,
    //         transform.translation.y,
    //         transform.translation.z,
    //     );

    //     Twist3d::new_dv(
    //         twist_translation[0],
    //         twist_translation[1],
    //         twist_translation[2],
    //         rvec[0],
    //         rvec[1],
    //         rvec[2],
    //     )
    // }

    fn rotation_vector_to_matrix (&self, rotation: Vector3<Radian>) -> Matrix3<Radian> {
        nalgebra::Matrix3::new(
            0.0.into(), -rotation[2], rotation[1],
            rotation[2], 0.0.into(), -rotation[0],
            -rotation[1], rotation[0], 0.0.into(),
        )       
    }
}
