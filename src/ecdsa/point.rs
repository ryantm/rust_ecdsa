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
        let ref field = self.field;
        match (p1,p2) {
            (Infinity,p2) => p2,
            (p1,Infinity) => p1,
            (Finite(x, y), Finite(ox, oy)) => {
                if x == ox && y == field.modulo(&-oy) {
                    Infinity
                } else if x != ox {
                    let gamma = field.modulo(&(oy - y)) * field.inverse(&(ox - x));
                    let sum_x = field.modulo(&(gamma * gamma - x - ox));
                    let sum_y = field.modulo(&(gamma * (x - sum_x) - y));
                    Finite(sum_x, sum_y)
                } else {
                    Infinity //Todo add rule 5
                }
            }
        }
    }
}

