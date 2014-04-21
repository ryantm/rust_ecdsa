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

impl ::ecdsa::group::Group {
    fn add(&self, p1: Point, p2: Point) -> Point {
        match (p1,p2) {
            (Infinity,p2) => p2,
            (p1,Infinity) => p1,
            _ => Infinity
        }
    }
}

