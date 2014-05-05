extern crate num;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use ecdsa::group::Group;

#[deriving(Eq,Clone)]
pub enum Point {
    Finite(BigInt,BigInt),
    Infinity
}

impl Point {
    fn coords(self) -> Option<(BigInt,BigInt)> {
        match self {
            Finite(x,y) => { Some((x,y))},
            Infinity => { None }
        }
    }
}

impl ::ecdsa::group::Group {
    fn add(&self, p1: &Point, p2: &Point) -> Point {
        let ref field = self.field;
        match (p1.clone(),p2.clone()) {
            (Infinity,p2) => p2,
            (p1,Infinity) => p1,
            (Finite( x, y), Finite(ox, oy)) => {
                if x == ox && y == field.modulo(&-oy) {
                    Infinity
                } else if x != ox {
                    let gamma = field.modulo(&(oy - y)) * field.inverse(&(ox - x));
                    let sum_x = field.modulo(&(gamma * gamma - x - ox));
                    let sum_y = field.modulo(&(gamma * (x - sum_x) - y));
                    Finite(sum_x, sum_y)
                } else if p1 == p2 {
                    self.double(p1)
                } else {
                    fail!("Failed to add p1 and p2");
                }
            }
        }
    }

    fn double(&self, p1: &Point) -> Point {
        match p1.clone() {
            Infinity => p1.clone(),
            Finite(x, y) => {
                let ref field = self.field;
                let gamma = field.modulo(&((3u.to_bigint().unwrap() * x * x * self.param_a) * field.inverse(&(2u.to_bigint().unwrap() * y))));
                let new_x = field.modulo(&(gamma * gamma - 2u.to_bigint().unwrap() * x));
                let new_y = field.modulo(&(gamma * (x - new_x) - y));
                Finite(new_x, new_y)
            }
        }
    }
}

