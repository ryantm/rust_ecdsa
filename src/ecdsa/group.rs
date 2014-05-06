use ecdsa::point::Point;
use ecdsa::prime_field::PrimeField;
use num::bigint::BigInt;

pub struct Group {
    pub name: ~str,
    pub generator: Point,
    pub field: PrimeField,
    pub param_a: BigInt,
}
