extern crate num;
use num::bigint::BigUint;
use ecdsa::group::Group;

pub enum Point {
    Finite(BigUint,BigUint),
    Infinite
}

impl Point {
    fn coords(self) -> Option<(BigUint,BigUint)> {
        match self {
            Finite(x,y) => { Some((x,y))},
            Infinite => { None }
        }
    }
}
