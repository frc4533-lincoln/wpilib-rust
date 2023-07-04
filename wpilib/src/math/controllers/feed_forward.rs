use crate::math::units::angle::Radian;
use crate::math::units::angular_acceleration::RadianPerSecondSquared;
use crate::math::units::angular_velocity::RadianPerSecond;
use crate::math::units::energy::Volt;


#[derive(Debug, Clone, Copy)]
pub struct Simple{
    k_s: f64,
    k_v: f64,
    k_a: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct Static{
    k_v: f64,
    k_a: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct Elevator{
    k_s: f64,
    k_g: f64,
    k_v: f64,
    k_a: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct Arm{
    k_s: f64,
    k_g: f64,
    k_v: f64,
    k_a: f64,
}

impl Simple {
    #[must_use] pub const fn new(k_s: f64, k_v: f64, k_a: f64) -> Self {
        Self{
            k_s,
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let velocity: f64 = velocity.into().value();
        let acceleration: f64 = acceleration.into().value();
        self.k_a.mul_add(acceleration, self.k_s.mul_add(num::signum(velocity), self.k_v * velocity))
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.v_a_calculate(velocity, 0.0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration: f64 = acceleration.into().value();
        self.k_a.mul_add(-acceleration, max_voltage.value() - self.k_s) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        let velocity: f64 = velocity.into().value();
        velocity.mul_add(-self.k_v, self.k_s.mul_add(-num::signum(velocity), max_voltage.value())) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, velocity)
    }
}

impl Static {
    #[must_use] pub const fn new(k_v: f64, k_a: f64) -> Self {
        Self {
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let velocity: f64 = velocity.into().value();
        let acceleration: f64 = acceleration.into().value();
        self.k_v.mul_add(velocity, self.k_a * acceleration)
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.v_a_calculate(velocity, 0.0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration: f64 = acceleration.into().value();
        self.k_a.mul_add(-acceleration, max_voltage.value()) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        let velocity: f64 = velocity.into().value();
        max_voltage.value().mul_add(num::signum(velocity), -velocity * self.k_v) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, velocity)
    }
}

impl Elevator {
    #[must_use] pub const fn new(k_s: f64, k_g: f64, k_v: f64, k_a: f64) -> Self {
        Self {
            k_s,
            k_g,
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let velocity: f64 = velocity.into().value();
        let acceleration: f64 = acceleration.into().value();
        self.k_a.mul_add(acceleration, self.k_v.mul_add(velocity, self.k_s.mul_add(num::signum(velocity), self.k_g)))
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.v_a_calculate(velocity, 0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration: f64 = acceleration.into().value();
        self.k_a.mul_add(-acceleration, max_voltage.value() - self.k_s - self.k_g) / self.k_v
    }

    pub fn min_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration: f64 = acceleration.into().value();
        self.k_a.mul_add(-acceleration, -max_voltage.value() + self.k_s - self.k_g) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        let velocity: f64 = velocity.into().value();
        velocity.mul_add(-self.k_v, self.k_s.mul_add(-num::signum(velocity), max_voltage.value()) - self.k_g) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, velocity)
    }
}

impl Arm {
    #[must_use] pub const fn new(k_s: f64, k_g: f64, k_v: f64, k_a: f64) -> Self {
        Self {
            k_s,
            k_g,
            k_v,
            k_a,
        }
    }

    pub fn p_v_a_calculate(&mut self, position: impl Into<Radian>, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let position: f64 = position.into().value();
        let velocity: f64 = velocity.into().value();
        let acceleration: f64 = acceleration.into().value();
        let g_cos: f64 = position.cos() * self.k_g;
        self.k_a.mul_add(acceleration, self.k_v.mul_add(velocity, self.k_s.mul_add(num::signum(velocity), g_cos)))
    }

    pub fn calculate(&mut self, position: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.p_v_a_calculate(position, velocity, 0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, angle: impl Into<Radian>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let angle: f64 = angle.into().value();
        let acceleration: f64 = acceleration.into().value();
        acceleration.mul_add(-self.k_a, angle.cos().mul_add(-self.k_g, max_voltage.value() - self.k_s)) / self.k_v
    }

    pub fn min_velocity(&mut self, max_voltage: Volt, angle: impl Into<Radian>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let angle: f64 = angle.into().value();
        let acceleration: f64 = acceleration.into().value();
        acceleration.mul_add(-self.k_a, angle.cos().mul_add(-self.k_g, -max_voltage.value() + self.k_s)) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, angle: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        let angle: f64 = angle.into().value();
        let velocity: f64 = velocity.into().value();
        velocity.mul_add(-self.k_v, angle.cos().mul_add(-self.k_g, self.k_s.mul_add(-num::signum(velocity), max_voltage.value()))) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, angle: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, angle, velocity)
    }
}