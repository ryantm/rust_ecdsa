extern crate num;

mod ECDSA {
    fn byte_length(integer:&::num::bigint::BigUint) -> uint {
        integer.bits()
    }
}

fn main () {}
