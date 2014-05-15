extern crate num;
use num::bigint::BigInt;
use num::bigint::ToBigInt;
use std::num::Zero;
use std::num::One;
use num::Integer;
use std::num::from_str_radix;

pub struct PrimeField {
    pub prime: BigInt
}

impl PrimeField {
    pub fn include(&self, integer: &BigInt) -> bool {
        *integer >= Zero::zero() && *integer < self.prime
    }

    pub fn modulo(&self, integer: &BigInt) -> BigInt {
        integer.mod_floor(&self.prime)
    }

    pub fn inverse(&self, n: &BigInt) -> BigInt {
        assert!(!n.is_zero(), "0 has no multiplicative inverse.");
        
        let (mut r, mut newr) : (BigInt, BigInt) = ((*n).clone(), self.prime.clone());
        let (mut s, mut news) : (BigInt, BigInt) = (One::one(), Zero::zero());
        let (mut t, mut newt) : (BigInt, BigInt) = (Zero::zero(), One::one());

        while newr > Zero::zero() {
            let quotient = r / newr;
            
            let temp = r.clone();
            r = newr.clone();
            newr = temp - quotient * newr;

            let temp = s.clone();
            s = news.clone();
            news = temp - quotient * news;

            let temp = t.clone();
            t = newt.clone();
            newt = temp - quotient * newt;
        }
        if r != One::one() { fail!("prime is not invertible") }
        return self.modulo(&s);
    }

    fn power(&self, n:&BigInt, m:&BigInt) -> BigInt {
        let mut result: BigInt = One::one();
        let mut n: BigInt = n.clone();
        let mut m: BigInt = m.clone();
        
        while m > Zero::zero() {
            if m.is_odd() {
                result = self.modulo(&(result * n))
            }
            n = self.modulo(&(n*n));
            m = m >> 1
        }
        result
    }

    pub fn square(&self, n:&BigInt) -> BigInt {
        self.modulo(&(*n * *n))
    }
}

#[test]
#[should_fail]
fn fail_when_0() {
    let p = PrimeField{prime: 1367u.to_bigint().unwrap()};
    p.inverse(&Zero::zero());
}

#[test]
fn inverse_of_1() {
    let p = PrimeField{prime: 1367u.to_bigint().unwrap()};
    assert!(p.inverse(&One::one()) == One::one());
}

#[cfg(test)]
fn check_inversion(prime: BigInt, n: BigInt) {
    let p = PrimeField{prime: prime};
    let inverse = p.inverse(&n);
    assert!(p.include(&inverse));
    assert!(p.modulo(&(inverse * n)) == One::one());
}

#[test]
fn check_prime_minus_1() {
    check_inversion(1367u.to_bigint().unwrap(), 1367u.to_bigint().unwrap() - One::one());
}

#[test]
fn check_44() {
    check_inversion(1367u.to_bigint().unwrap(), 44u.to_bigint().unwrap());
}

#[test]
fn check_large_prime() {
    check_inversion(
        from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap(),
        from_str_radix("d44189175bd60c4f6ead9f5f301fd4a9a5ece4c47ab4518611b4c65077ba7a6b", 16).unwrap());
}

#[test]
fn check_power() {
    let p = PrimeField{prime: from_str("1367").unwrap()};
    assert!(p.power(&5u.to_bigint().unwrap(),&Zero::zero()) == One::one());
    assert!(p.power(&Zero::zero(),&Zero::zero()) == One::one());
}

#[test]
fn check_square() {
    let p = PrimeField{prime: from_str("1367").unwrap()};
    assert!(p.square(&Zero::zero()) == Zero::zero());
    assert!(p.square(&One::one()) == One::one());
    assert!(p.square(&1311u.to_bigint().unwrap()) == 402u.to_bigint().unwrap());
}
