use std::ops::Mul;

use super::stream::Stream;

pub struct PIDConstants {
    k_p: f64,
    k_i: f64,
    k_d: f64,
}

pub fn pid(error: Stream, constants: PIDConstants) -> Stream {
    let p = constants.k_p * error.clone();
    let i = constants.k_i * error.clone().integrate(None);
    let d = constants.k_d * error.clone().differentiate(None);
    p + i + d
}
