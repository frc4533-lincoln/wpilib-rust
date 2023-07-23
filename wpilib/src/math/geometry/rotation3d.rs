use nalgebra::{
    linalg::QR, ArrayStorage, ComplexField, Const, Matrix, MatrixCross, OMatrix, Quaternion,
    Rotation3, Unit, UnitQuaternion, Vector3, U1, U3,
};
use num::clamp;

use crate::math::units::angle::Radian;

use super::Rotation2d;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation3d {
    pub q: UnitQuaternion<f64>,
}

impl Rotation3d {
    #[must_use]
    pub fn new() -> Self {
        Self {
            q: UnitQuaternion::new_normalize(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
        }
    }
    #[must_use]
    pub fn new_quaternion(q: Quaternion<f64>) -> Self {
        Self {
            q: UnitQuaternion::new_normalize(q),
        }
    }
    #[must_use]
    pub fn new_euler_angles(
        roll: impl Into<Radian>,
        pitch: impl Into<Radian>,
        yaw: impl Into<Radian>,
    ) -> Self {
        Self {
            q: UnitQuaternion::from_euler_angles(
                f64::from(roll.into()),
                f64::from(pitch.into()),
                f64::from(yaw.into()),
            ),
        }
    }

    #[must_use]
    pub fn new_rotation_vector(rvec: Vector3<f64>) -> Self {
        Self::new_axis_angle(rvec, rvec.norm())
    }

    #[must_use]
    fn new_axis_angle(axis: Vector3<f64>, angle: impl Into<Radian>) -> Self {
        let axis: Unit<Vector3<f64>> = Unit::new_normalize(axis);
        Self {
            q: UnitQuaternion::from_axis_angle(&axis, f64::from(angle.into())),
        }
    }

    #[must_use]
    pub fn new_rotation_matrix(matrix: Rotation3<f64>) -> Self {
        Self {
            q: UnitQuaternion::from_rotation_matrix(&matrix),
        }
    }

    //TODO: UNSURE IF THIS WORKS
    #[must_use]
    pub fn new_first_last(initial: Vector3<f64>, last: Vector3<f64>) -> Self {
        let dot = initial.dot(&last);
        let norm_product = initial.norm() * last.norm();
        let dot_norm = dot / norm_product;

        if dot_norm > 1.0 - 1E-9 {
            Self::new()
        } else if dot_norm < -1.0 + 1E-9 {
            let x: Matrix<f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>> =
                Vector3::new(1.0, 0.0, 0.0);
            let qr: QR<f64, Const<3>, Const<1>> = QR::new(x);
            let q: OMatrix<f64, Const<3>, Const<1>> = qr.q();

            Self {
                q: UnitQuaternion::new(q),
            }
        } else {
            let axis: MatrixCross<f64, U3, U1, U3, U1> = Vector3::cross(&initial, &last);

            Self {
                q: UnitQuaternion::new_normalize(Quaternion::new(
                    norm_product + dot,
                    axis.x,
                    axis.y,
                    axis.z,
                )),
            }
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
        Self {
            q: self.q.inverse(),
        }
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        if self.q.w >= 0.0 {
            Self::new_axis_angle(
                Vector3::new(self.q.i, self.q.j, self.q.k),
                2.0 * scalar * ComplexField::acos(self.q.w),
            )
        } else {
            Self::new_axis_angle(
                Vector3::new(-self.q.i, -self.q.j, -self.q.k),
                2.0 * scalar * ComplexField::acos(-self.q.w),
            )
        }
    }

    #[must_use]
    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn rotate_by(&self, other: &Self) -> Self {
        Self {
            q: self.q * other.q,
        }
    }

    #[must_use]
    pub fn get_x(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        f64::atan2(
            2.0 * w.mul_add(x, y * z),
            2.0f64.mul_add((-x).mul_add(x, y * y), 1.0),
        )
        .into()
    }

    #[must_use]
    pub fn get_y(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        let ratio = 2.0 * w.mul_add(y, -z * x);
        if ratio.abs() >= 1.0 {
            f64::copysign(std::f64::consts::PI / 2.0, ratio).into()
        } else {
            f64::asin(ratio).into()
        }
    }

    #[must_use]
    pub fn get_z(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        f64::atan2(
            2.0 * w.mul_add(z, x * y),
            2.0f64.mul_add((-y).mul_add(y, z * z), 1.0),
        )
        .into()
    }

    #[must_use]
    pub fn get_axis(&self) -> Vector3<Radian> {
        self.q.axis().map_or_else(
            || Vector3::new(0.0.into(), 0.0.into(), 0.0.into()),
            |axis: Unit<Vector3<f64>>| Vector3::new(axis.x.into(), axis.y.into(), axis.z.into()),
        )
    }

    #[must_use]
    pub fn get_angle(&self) -> Radian {
        self.q.angle().into()
    }

    #[must_use]
    pub fn interpolate(&self, end_value: Self, t: f64) -> Self {
        self.plus(&end_value.minus(self).times(clamp(t, 0.0, 1.0)))
    }
}

impl Default for Rotation3d {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Rotation2d> for Rotation3d {
    fn from(r: Rotation2d) -> Self {
        Self::new_euler_angles(0.0, 0.0, r.value)
    }
}

impl From<Rotation3d> for Rotation2d {
    fn from(r: Rotation3d) -> Self {
        Self::new_angle(r.get_z())
    }
}
