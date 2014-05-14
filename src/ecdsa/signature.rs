use num::bigint::BigInt;

#[deriving(Eq)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}
