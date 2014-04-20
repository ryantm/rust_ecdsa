extern crate num;

mod ecdsa {
    pub mod group;
    pub mod finitepoint;

    fn byte_length(integer:&::num::bigint::BigUint) -> uint {
        bit_length(integer) / 8
    }
    fn bit_length(integer:&::num::bigint::BigUint) -> uint {
        integer.bits()
    }
}

fn main () {}
