use simba::scalar::ComplexField;
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
    pub fn new(k_s: f64, k_v: f64, k_a: f64) -> Self {
        Self{
            k_s,
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let velocity = velocity.into().value();
        let acceleration = acceleration.into().value();
        self.k_s * num::signum(velocity) + self.k_v * velocity + self.k_a * acceleration
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.v_a_calculate(velocity, 0.0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration = acceleration.into().value();
        (max_voltage.value() - self.k_s - self.k_a * acceleration) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        let velocity = velocity.into().value();
        (max_voltage.value() - self.k_s * num::signum(velocity) - velocity * self.k_v) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, velocity)
    }
}

impl Static {
    pub fn new(k_v: f64, k_a: f64) -> Self {
        Self {
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let velocity = velocity.into().value();
        let acceleration = acceleration.into().value();
        return self.k_v * velocity + self.k_a * acceleration;
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.v_a_calculate(velocity, 0.0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration = acceleration.into().value();
        (max_voltage.value() - self.k_a * acceleration) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        let velocity = velocity.into().value();
        (max_voltage.value() * num::signum(velocity) - velocity * self.k_v) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, velocity)
    }
}

impl Elevator {
    pub fn new(k_s: f64, k_g: f64, k_v: f64, k_a: f64) -> Self {
        Self {
            k_s,
            k_g,
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let velocity = velocity.into().value();
        let acceleration = acceleration.into().value();
        self.k_s * num::signum(velocity) + self.k_g + self.k_v * velocity + self.k_a * acceleration
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.v_a_calculate(velocity, 0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration = acceleration.into().value();
        (max_voltage.value() - self.k_s - self.k_g - self.k_a * acceleration) / self.k_v
    }

    pub fn min_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let acceleration = acceleration.into().value();
        (-max_voltage.value() + self.k_s - self.k_g - self.k_a * acceleration) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        let velocity = velocity.into().value();
        (max_voltage.value() - self.k_s * num::signum(velocity) - self.k_g - velocity * self.k_v) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, velocity)
    }
}

impl Arm {
    pub fn new(k_s: f64, k_g: f64, k_v: f64, k_a: f64) -> Self {
        Self {
            k_s,
            k_g,
            k_v,
            k_a,
        }
    }

    pub fn p_v_a_calculate(&mut self, position: impl Into<Radian>, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let position = position.into().value();
        let velocity = velocity.into().value();
        let acceleration = acceleration.into().value();
        let g_cos = ComplexField::cos(position) * self.k_g;
        self.k_s * num::signum(velocity) + g_cos + self.k_v * velocity + self.k_a * acceleration
    }

    pub fn calculate(&mut self, position: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.p_v_a_calculate(position, velocity, 0)
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, angle: impl Into<Radian>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let angle = angle.into().value();
        let acceleration = acceleration.into().value();
        (max_voltage.value() - self.k_s - ComplexField::cos(angle) * self.k_g - acceleration * self.k_a) / self.k_v
    }

    pub fn min_velocity(&mut self, max_voltage: Volt, angle: impl Into<Radian>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        let angle = angle.into().value();
        let acceleration = acceleration.into().value();
        (-max_voltage.value() + self.k_s - ComplexField::cos(angle) * self.k_g - acceleration * self.k_a) / self.k_v
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, angle: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        let angle = angle.into().value();
        let velocity = velocity.into().value();
        (max_voltage.value() - self.k_s * num::signum(velocity) - ComplexField::cos(angle) * self.k_g - velocity * self.k_v) / self.k_a
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, angle: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        self.max_acceleration(-max_voltage, angle, velocity)
    }
}