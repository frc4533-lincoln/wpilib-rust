use std::ops::Mul;

use nalgebra::Quaternion;
use nalgebra::Rotation3;
use nalgebra::ComplexField;
use nalgebra::Vector3;
// use nalgebra::Matrix3;
// use nalgebra::linalg::QR;

use crate::math::units::angle::Radian;
use crate::math::util::math_util::MathUtil;

use super::Rotation2d;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rotation3d {
    pub q: Quaternion<f64>
}

impl Rotation3d {
    pub fn new() -> Self {
        Self {
            q: Quaternion::new(0.0, 0.0, 0.0, 1.0)
        }
    }
    pub fn new_quaternion(q: Quaternion<f64>) -> Self {
        Self {
            q: Quaternion::normalize(&q) 
        }
    }
    pub fn new_euler_angles(roll: Radian, pitch: Radian, yaw: Radian) -> Self {
        let roll = f64::from(roll);
        let pitch = f64::from(pitch);
        let yaw = f64::from(yaw);
        
        let cr = ComplexField::cos(roll * 0.5);
        let sr = ComplexField::sin(roll * 0.5);

        let cp = ComplexField::cos(pitch * 0.5);
        let sp = ComplexField::sin(pitch * 0.5);

        let cy = ComplexField::cos(yaw * 0.5);
        let sy = ComplexField::sin(yaw * 0.5);

        let q = Quaternion::new(
            cr * cp * cy + sr * sp * sy,
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
        );
        
        Self {
            q
        }
    }

    //TODO
    // pub fn new_rotation_vector(rvec: Vector3<f64>) -> Self {
    //     Self::new_first_last
    // }

    pub fn new_axis_angle(axis: Vector3<f64>, angle: Radian) -> Self {
        let norm = axis.norm();
        if norm == 0.0 {
            return Self::new();
        }
        
        let v = axis.mul(1.0 / norm).mul(ComplexField::sin(f64::from(angle) / 2.0));
        Self::new_quaternion(Quaternion::new(ComplexField::cos(f64::from(angle) / 2.0), v[(0, 0)], v[(1, 0)], v[(2, 0)]))
    }

    pub fn new_rotation_matrix(matrix: Rotation3<f64>) -> Self {
        let r = matrix;

        //TODO: require rotation matrix to be special orthogonal 

        let trace = r[(0, 0)] + r[(1, 1)] + r[(2, 2)];
        let w: f64;
        let x: f64;
        let y: f64;
        let z: f64;
        
        if trace > 0.0 {
            let s = 0.5 / ComplexField::sqrt(trace + 1.0);
            w = 0.25 / s;
            x = (r[(2, 1)] - r[(1, 2)]) * s;
            y = (r[(0, 2)] - r[(2, 0)]) * s;
            z = (r[(1, 0)] - r[(0, 1)]) * s;
        } else {
            if r[(0, 0)] > r[(1, 1)] && r[(0, 0)] > r[(2, 2)]{
                let s = 2.0 * ComplexField::sqrt(1.0 + r[(0, 0)] - r[(1, 1)] - r[(2, 2)]);
                w = (r[(2, 1)] - r[(1, 2)]) / s;
                x = 0.25 * s;
                y = (r[(0, 1)] + r[(1, 0)]) / s;
                z = (r[(0, 2)] + r[(2, 0)]) / s;
            } else if r[(1, 1)] > r[(2, 2)] {
                let s = 2.0 * ComplexField::sqrt(1.0 + r[(1, 1)] - r[(0, 0)] - r[(2, 2)]);
                w = (r[(0, 2)] - r[(2, 0)]) / s;
                x = (r[(0, 1)] + r[(1, 0)]) / s;
                y = 0.25 * s;
                z = (r[(1, 2)] + r[(2, 1)]) / s;
            } else {
                let s = 2.0 * ComplexField::sqrt(1.0 + r[(2, 2)] - r[(0, 0)] - r[(1, 1)]);
                w = (r[(1, 0)] - r[(0, 1)]) / s;
                x = (r[(0, 2)] + r[(2, 0)]) / s;
                y = (r[(1, 2)] + r[(2, 1)]) / s;
                z = 0.25 * s; 
            }
        }
        Self::new_quaternion(Quaternion::new(w, x, y, z))
    }

    //TODO
    // pub fn new_first_last(initial: Vector3<f64>, last: Vector3<f64>) -> Self {
    //     let dot = initial.dot(&last);
    //     let norm_product = initial.norm() * last.norm();
    //     let dot_norm = dot / norm_product;

    //     if dot_norm  > 1.0 - 1E-9 {
    //         return Self::new();
    //     } else if dot_norm < -1.0 + 1E-9 {

    //     }
    // }

    pub fn plus(&self, other: &Self) -> Self {
        self.rotate_by(other)
    }

    pub fn minus(&self, other: &Self) -> Self {
        self.rotate_by(&other.unary_minus())
    }

    pub fn unary_minus(&self) -> Self {
        if let Some(invert) = self.q.try_inverse() {
            Self::new_quaternion(invert)
        } else {
            return Self::new_quaternion(self.q);
        }
    }

    pub fn times(&self, scalar: f64) -> Self {
        if self.q.w >= 0.0 {
            Self::new_axis_angle(
                Vector3::new(self.q.i, self.q.j, self.q.k),
                (2.0 * scalar * ComplexField::acos(self.q.w)).into()  
            )
        } else {
            Self::new_axis_angle(
                Vector3::new(-self.q.i, -self.q.j, -self.q.k),
                (2.0 * scalar * ComplexField::acos(-self.q.w)).into()
            )
        }
    }

    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    pub fn rotate_by(&self, other: &Self) -> Self {
        Self::new_quaternion(self.q * other.q)
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
        let norm = ComplexField::sqrt(self.q.i * self.q.i + self.q.j * self.q.j + self.q.k * self.q.k);
        if norm == 0.0 {
            return Vector3::new(0.0.into(), 0.0.into(), 0.0.into());
        } else {
            return Vector3::new((self.q.i / norm).into(), (self.q.j / norm).into(), (self.q.k / norm).into());
        }
    }

    pub fn get_angle(&self) -> Radian {
        let norm = ComplexField::sqrt(self.q.i * self.q.i + self.q.j * self.q.j + self.q.k * self.q.k);
        (2.0 * f64::atan2(norm, self.q.w)).into()
    }

    pub fn interpolate(&self, end_value: Self, t: f64) -> Self {
        self.plus(&end_value.minus(self).times(MathUtil::clamp_double(t, 0.0, 1.0)))
    }

}

impl From<Rotation2d> for Rotation3d {
    fn from(r: Rotation2d) -> Self {
        Self::new_euler_angles(0.0.into(), 0.0.into(), r.value)
    }
}

impl From<Rotation3d> for Rotation2d {
    fn from(r: Rotation3d) -> Self {
        Self::new_angle(r.get_z())
    }
}