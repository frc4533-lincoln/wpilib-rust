

use nalgebra::{linalg::QR, ComplexField, Quaternion, Rotation3, Unit, UnitQuaternion, Vector3};

use crate::math::units::angle::Radian;
use crate::math::util::math_util::MathUtil;

use super::Rotation2d;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation3d {
    pub q: UnitQuaternion<f64>,
}

impl Rotation3d {
    pub fn new() -> Self {
        Self {
            q: UnitQuaternion::new_normalize(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
        }
    }
    pub fn new_quaternion(q: Quaternion<f64>) -> Self {
        Self {
            q: UnitQuaternion::new_normalize(q),
        }
    }
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

    pub fn new_rotation_vector(rvec: Vector3<f64>) -> Self {
        Self::new_axis_angle(rvec, rvec.norm())
    }

    pub fn new_axis_angle(axis: Vector3<f64>, angle: impl Into<Radian>) -> Self {
        let axis = Unit::new_normalize(axis);
        Self {
            q: UnitQuaternion::from_axis_angle(&axis, f64::from(angle.into())),
        }
    }

    pub fn new_rotation_matrix(matrix: Rotation3<f64>) -> Self {
        Self {
            q: UnitQuaternion::from_rotation_matrix(&matrix),
        }
    }

    //TODO: UNSURE IF THIS WORKS
    pub fn new_first_last(initial: Vector3<f64>, last: Vector3<f64>) -> Self {
        let dot = initial.dot(&last);
        let norm_product = initial.norm() * last.norm();
        let dot_norm = dot / norm_product;

        if dot_norm > 1.0 - 1E-9 {
            return Self::new();
        } else if dot_norm < -1.0 + 1E-9 {
            let x = Vector3::new(1.0, 0.0, 0.0);
            let qr = QR::new(x);
            let q = qr.q();

            Self {
                q: UnitQuaternion::new(q),
            }
        } else {
            let axis = Vector3::cross(&initial, &last);

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

    pub fn plus(&self, other: &Self) -> Self {
        self.rotate_by(other)
    }

    pub fn minus(&self, other: &Self) -> Self {
        self.rotate_by(&other.unary_minus())
    }

    pub fn unary_minus(&self) -> Self {
        Self {
            q: self.q.inverse(),
        }
    }

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

    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    pub fn rotate_by(&self, other: &Self) -> Self {
        Self {
            q: self.q * other.q,
        }
    }

    pub fn get_x(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        f64::atan2(2.0 * (w * x + y * z), 1.0 - 2.0 * (x * x + y * y)).into()
    }

    pub fn get_y(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        let ratio = 2.0 * (w * y - z * x);
        if ratio.abs() >= 1.0 {
            return f64::copysign(std::f64::consts::PI / 2.0, ratio).into();
        } else {
            return f64::asin(ratio).into();
        }
    }

    pub fn get_z(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        f64::atan2(2.0 * (w * z + x * y), 1.0 - 2.0 * (y * y + z * z)).into()
    }

    pub fn get_axis(&self) -> Vector3<Radian> {
        if let Some(axis) = self.q.axis() {
            return Vector3::new(axis.x.into(), axis.y.into(), axis.z.into());
        } else {
            return Vector3::new(0.0.into(), 0.0.into(), 0.0.into());
        }
    }

    pub fn get_angle(&self) -> Radian {
        self.q.angle().into()
    }

    pub fn interpolate(&self, end_value: Self, t: f64) -> Self {
        self.plus(
            &end_value
                .minus(self)
                .times(MathUtil::clamp_double(t, 0.0, 1.0)),
        )
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
