use ecdsa::point::Point;
use ecdsa::prime_field::PrimeField;

pub struct Group {
    name: ~str,
    generator: Option<Point>,
    pub field: PrimeField
}


