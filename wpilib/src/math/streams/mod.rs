use std::ops::{Add, Div, Mul, Sub};

type Supplier = fn() -> f64;

pub enum Stream {
    Mono(Supplier),
    Aggregate(Box<Stream>, Box<Stream>, fn(f64, f64) -> f64),
    Composite(Box<Stream>, Box<dyn Fn(f64) -> f64>),
}

impl Stream {
    pub fn get(&self) -> f64 {
        match self {
            Stream::Mono(f) => f(),
            Stream::Aggregate(f, g, op) => op(f.to_owned().get(), g.to_owned().get()),
            Stream::Composite(f, op) => op(f.to_owned().get()),
        }
    }

    pub fn map(self, op: impl Fn(f64) -> f64 + 'static) -> Self {
        Stream::Composite(Box::new(self), Box::new(op))
    }
}

impl Add for Stream {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x + y)
    }
}

impl Sub for Stream {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x - y)
    }
}

impl Mul for Stream {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x * y)
    }
}

impl Div for Stream {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Stream::Aggregate(Box::new(self), Box::new(rhs), |x, y| x / y)
    }
}

// TODO scalar multiplication

// impl Mul<f64> for Stream {
//     type Output = Self;

//     fn mul(self, rhs: f64) -> Self::Output {
//         self.map(|x| rhs * x)
//     }
// }
