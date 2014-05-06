extern crate num;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use ecdsa::group::Group;
use ecdsa::prime_field::PrimeField;
use std::num::from_str_radix;
use std::num::Zero;

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

#[test]
fn check_adding_to_infinity() {
    let g = Group{name: ~"secp256k1", 
                  generator: Finite( 
                      from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
                      from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap()),
                  field: PrimeField{prime: from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap()},
                  param_a: Zero::zero()};

    assert!(g.add(&g.generator, &Infinity) == g.generator);
}
/*  describe '#add_to_point' do
    context 'when adding point + infinity' do
      it 'returns the point' do
        expect(group.generator.add_to_point(group.infinity)).to eq group.generator
      end
    end

    context 'when adding infinity + point' do
      it 'returns the point' do
        expect(group.infinity.add_to_point(group.generator)).to eq group.generator
      end
    end

    context 'when adding the generator to itself' do
      it 'returns the double' do
        expect(group.generator.add_to_point(group.generator)).to eq group.generator.double
      end
    end
  end
*/
