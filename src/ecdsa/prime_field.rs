extern crate num;
use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::bigint::{Zero,Integer};

struct PrimeField {
    prime: BigUint
}

impl PrimeField {
    fn include(&self, integer: BigUint) -> bool {
        //e.is_a?(Integer) && e >= 0 && e < prime
        integer < self.prime
    }

    fn modulo(&self, integer: BigUint) -> BigUint {
        
        //num::mod_floor(self.prime, integer)
        integer.mod_floor(self.prime)
    }

    fn inverse(&self, n: BigUint) -> BigUint {
        assert!(!n.is_zero(), "0 has no multiplicative inverse.");

        let t : BigUint  = Zero::zero();
        let newt : BigUint  = Zero::zero();
        let r = n;
        let newr = self.prime;

        while !newr.is_zero() {
            let quotient = r / newr;
            let temp = t;
            let t = newt;
            let newt = temp - quotient * newt;

            let temp = r;
            let r = newt;
            let newr = temp - quotient * newr;
        }
        if r > 1u.to_biguint().unwrap() { fail!("prime is not invertible") }
        if r < Zero::zero() { let t = t + n; }
        return t;
    }
}
