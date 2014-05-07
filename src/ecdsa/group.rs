use ecdsa::point::Point;
use ecdsa::point::Finite;
use ecdsa::prime_field::PrimeField;
use num::bigint::BigInt;
use std::num::from_str_radix;
use std::num::Zero;

pub struct Group {
    pub name: ~str,
    pub generator: Point,
    pub field: PrimeField,
    pub param_a: BigInt,
}
