extern crate num;
use num::bigint::BigUint;

pub struct Group {
    name: ~str,
    generator: Option<Point>
}


struct FinitePoint {
    group: Option<Group>,
    x: BigUint,
    y: BigUint
}

pub enum Point {
    finite(FinitePoint),
    infinite
}
/*
impl Point {
    fn new(group: Group, x: BigUint, y: BigUint) -> Point {
        Point { group: group, x: x
    }
}*/



