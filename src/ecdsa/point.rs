extern crate num;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use ecdsa::group::Group;
use ecdsa::prime_field::PrimeField;
use std::num::from_str_radix;
use std::num::Zero;
use num::Integer;

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

    fn negate(&self, p: &Point) -> Point {
        match p.clone() {
            Infinity => Infinity,
            Finite(x,y) => Finite(x, self.field.modulo(&-y)),
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

    pub fn multiply_by_scalar(&self, p1: &Point, i: &BigInt) -> Point {
        if (i < &Zero::zero()) { fail!("Scalar is negative.") };
        let mut result = Infinity;
        let mut v = p1.clone();
        let mut i = i.clone();
        while i > Zero::zero() {
            if i.is_odd() {
                result = self.add(&result, &v)
            }
            v = self.double(&v);
            i = i >> 1;
        }
        result
    }
    
    fn point_satisfies_equation(&self, p: &Point) -> bool {
        match p.clone() {
            Infinity => false,
            Finite(x,y) => 
                self.field.square(&y) == 
                self.field.modulo(&(x * x * x + self.param_a * x * self.param_b))
        }
    }
}

fn group() -> Group {
    Group {
        name: "secp256k1".to_owned(), 
        generator: Finite( 
            from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
            from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap()),
        field: PrimeField{prime: from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap()},
        param_a: Zero::zero(),
        param_b: 7u.to_bigint().unwrap(),
        order: from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap(),
    }
}

#[test]
fn check_adding_to_infinity() {
    let g = group();
    assert!(g.add(&g.generator, &Infinity) == g.generator);
}

#[test]
fn check_adding_from_infinity() {
    let g = group();
    assert!(g.add(&Infinity, &g.generator) == g.generator);
}

#[test]
fn check_adding_to_self() {
    let g = group();
    assert!(g.add(&g.generator, &g.generator) == g.double(&g.generator));
}

#[test]
fn check_negate() {
    let g = group();
    assert!(g.negate(&Infinity) == Infinity);
    match (g.negate(&g.generator), g.generator.clone()) {
        (Finite(x,y),Finite(gx,gy)) => {    
            assert!(x == gx);
            assert!(y == g.field.modulo(&-gy));
        },
        _ => fail!("Group does not have finite generator or generator inverse."),
    }
}

#[test]
fn check_multiply_by_scalar() {
    let k = 2u.to_bigint().unwrap();
    let g = group();
    assert!(g.multiply_by_scalar(&g.generator, &k) != Infinity);
}

#[test]
fn check_coords() {
    assert!(Infinity.coords() == None);
    let g = group();
    assert!(g.generator.coords().unwrap() == 
            (from_str_radix::<BigInt>("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
             from_str_radix::<BigInt>("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap()));
}
