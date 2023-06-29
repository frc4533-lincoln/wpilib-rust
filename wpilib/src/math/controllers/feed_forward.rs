use simba::scalar::ComplexField;
use crate::math::units::angle::Radian;
use crate::math::units::angular_acceleration::RadianPerSecondSquared;
use crate::math::units::angular_velocity::RadianPerSecond;
use crate::math::units::energy::Volt;

pub enum FeedForward {
    Simple{
        k_s: f64,
        k_v: f64,
        k_a: f64,
    },
    Static{
        k_v: f64,
        k_a: f64,
    },
    Elevator{
        k_s: f64,
        k_g: f64,
        k_v: f64,
        k_a: f64,
    },
    Arm{
        k_s: f64,
        k_g: f64,
        k_v: f64,
        k_a: f64,
    },
}

impl FeedForward::Simple {
    pub fn new(k_s: f64, k_v: f64, k_a: f64) -> FeedForward {
        FeedForward::Simple{
            k_s,
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return self.k_s * num::signum(*velocity) + self.k_v * velocity + self.k_a * acceleration;
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        return v_a_calculate(velocity, 0.0);
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return (max_voltage.value() - self.k_s - self.k_a * acceleration) / self.k_v;
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        return (max_voltage.value() - self.k_s * num::signum(*velocity) - velocity * self.k_v) / self.k_a;
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        return max_acceleration(-max_voltage, velocity);
    }
}

impl FeedForward::Static {
    pub fn new(k_v: f64, k_a: f64) -> FeedForward {
        FeedForward::Static{
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return self.k_v * velocity + self.k_a * acceleration;
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        return v_a_calculate(velocity, 0.0);
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return (max_voltage.value() - self.k_a * acceleration) / self.k_v;
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        return (max_voltage.value() * num::signum(*velocity) - velocity * self.k_v) / self.k_a;
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        return max_acceleration(-max_voltage, velocity);
    }
}

impl FeedForward::Elevator {
    pub fn new(k_s: f64, k_g: f64, k_v: f64, k_a: f64) -> FeedForward {
        FeedForward::Elevator{
            k_s,
            k_g,
            k_v,
            k_a,
        }
    }

    pub fn v_a_calculate(&mut self, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return self.k_s * num::signum(*velocity) + self.k_g + self.k_v * velocity + self.k_a * acceleration;
    }

    pub fn v_calculate(&mut self, velocity: impl Into<RadianPerSecond>) -> f64 {
        return v_a_calculate(RadianPerSecond::from(velocity), RadianPerSecondSquared::from(0.0));
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return (max_voltage.value() - self.k_s - self.k_g - self.k_a * acceleration) / self.k_v;
    }

    pub fn min_velocity(&mut self, max_voltage: Volt, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return (-max_voltage.value() + self.k_s - self.k_g - self.k_a * acceleration) / self.k_v;
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        return (max_voltage.value() - self.k_s * num::signum(*velocity) - self.k_g - velocity * self.k_v) / self.k_a;
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, velocity: impl Into<RadianPerSecond>) -> f64 {
        return max_acceleration(-max_voltage, velocity);
    }
}

impl FeedForward::Arm {
    pub fn new(k_s: f64, k_g: f64, k_v: f64, k_a: f64) -> FeedForward {
        FeedForward::Arm{
            k_s,
            k_g,
            k_v,
            k_a,
        }
    }

    pub fn p_v_a_calculate(&mut self, position: impl Into<Radian>, velocity: impl Into<RadianPerSecond>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return self.k_s * num::signum(*velocity) + self.k_g.cos(position) + self.k_v * velocity + self.k_a * acceleration;
    }

    pub fn calculate(&mut self, position: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        return p_v_a_calculate(position, velocity, 0.0 as RadianPerSecondSquared);
    }

    pub fn max_velocity(&mut self, max_voltage: Volt, angle: impl Into<Radian>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return (max_voltage.value() - self.k_s - ComplexField::cos(angle) * self.k_g - acceleration * self.k_a) / self.k_v;
    }

    pub fn min_velocity(&mut self, max_voltage: Volt, angle: impl Into<Radian>, acceleration: impl Into<RadianPerSecondSquared>) -> f64 {
        return (-max_voltage.value() + self.k_s - ComplexField::cos(angle) * self.k_g - acceleration * self.k_a) / self.k_v;
    }

    pub fn max_acceleration(&mut self, max_voltage: Volt, angle: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        return (max_voltage.value() - self.k_s * num::signum(*velocity) - ComplexField::cos(angle) * self.k_g - velocity * self.k_v) / self.k_a;
    }

    pub fn min_acceleration(&mut self, max_voltage: Volt, angle: impl Into<Radian>, velocity: impl Into<RadianPerSecond>) -> f64 {
        return max_acceleration(-max_voltage, angle, velocity);
    }
}