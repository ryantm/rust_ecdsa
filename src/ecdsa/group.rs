use ecdsa::point::Point;
use ecdsa::prime_field::PrimeField;
use num::bigint::BigInt;

pub struct Group {
    name: ~str,
    generator: Option<Point>,
    pub field: PrimeField,
    pub param_a: BigInt,
}


