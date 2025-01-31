mod lagrange_polynomial_okvs;
mod okvs;

pub use okvs::OKVS;
use std::collections::HashSet;

use crate::data::point::Point;

pub fn encode(data: &HashSet<Point>) -> impl OKVS {
    lagrange_polynomial_okvs::LagrangePolynomialOKVS::encode(data)
}
