extern crate num;
use num::bigint::BigInt;
use ecdsa::group::Group;

pub enum Point {
    Finite(BigInt,BigInt),
    Infinity
}

impl Point {
    fn coords(self) -> Option<(BigInt,BigInt)> {
        match self {
            Finite(x,y) => { Some((x,y))},
            Infinite => { None }
        }
    }
}
