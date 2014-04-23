extern crate num;
use num::bigint::BigUint;

struct PrimeField {
    prime: BigUint
}

impl PrimeField {
    fn include(&self, integer: BigUint) -> bool {
        //e.is_a?(Integer) && e >= 0 && e < prime
        integer < self.prime
    }

    fn mod_floor(&self, integer: BigUint) -> BigUint {
        integer.mod_floor(self)
    }
}
