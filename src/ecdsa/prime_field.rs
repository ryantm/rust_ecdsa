extern crate num;
use num::bigint::BigInt;
use num::bigint::BigUint;
use num::bigint::Plus;
use num::bigint::ToBigUint;
//use num::bigint::{Zero};
use std::num::Zero;
use std::num::One;
use num::Integer;

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
        integer.mod_floor(&self.prime)
    }

/*
function inverse(a, n)
    t := 0;     newt := 1;    
    r := n;     newr := a;    
    while newr ≠ 0
        quotient := r div newr
        (t, newt) := (newt, t - quotient * newt) 
        (r, newr) := (newr, r - quotient * newr)
    if r > 1 then return "a is not invertible"
    if t < 0 then t := t + n
    return t
*/

    fn inverse(&self, n: &BigUint) -> BigUint {
        assert!(!n.is_zero(), "0 has no multiplicative inverse.");
        
        let mut r : BigInt = BigInt::from_biguint(Plus, (*n).clone());
        let mut newr : BigInt = BigInt::from_biguint(Plus, self.prime.clone());
        let mut s : BigInt = One::one();
        let mut news : BigInt = Zero::zero();
        let mut t : BigInt = Zero::zero();
        let mut newt : BigInt = One::one();

        /*let mut rs : (BigUint, BigUint) = ((*n).clone(), self.prime.clone());
        let mut ss : (BigUint, BigUint) = (One::one(), Zero::zero());
        let mut ts : (BigUint, BigUint) = (Zero::zero(), One::one());
         */

        while newr > Zero::zero() {
            let quotient = r / newr;
            println!("q:{}", quotient);

            let temp = r.clone();
            r = newr.clone();
            newr = temp - quotient * newr;
            
            println!("did first");
            let temp = s.clone();
            s = news.clone();
            news = temp - quotient * news;

            let temp = t.clone();
            t = newt.clone();
            newt = temp - quotient * newt;

            /*
            (r, newr) = (newr.clone(), r - quotient * newr);
            (s, news) = (news.clone(), s - quotient * news);
            (t, newt) = (newt.clone(), t - quotient * newt);
             */

            /*let swap = |(current, new): (BigUint, BigUint)| -> (BigUint, BigUint) {
                (new.clone(), current - quotient * new)
            }*/
            /*rs = (rs.val1().clone(), rs.val0() - quotient * rs.val1());
            ss = (ss.val1().clone(), ss.val0() - quotient * ss.val1());
            ts = (ts.val1().clone(), ts.val0() - quotient * ts.val1());*/
            println!("r:{} s:{} t:{}", r, s, t);
            println!("newr:{} news:{} newt:{}", newr, news, newt);
        }
        println!("finished loop!");
        if r > One::one() { fail!("prime is not invertible") }
        //if t < Zero::zero() { let t = t + (*n).clone(); }
        return self.modulo(s.to_biguint().unwrap());
    }
}

#[test]
#[should_fail]
fn fail_when_0() {
    let p = PrimeField{prime: 1367u.to_biguint().unwrap()};
    p.inverse(~Zero::zero());
}

#[test]
fn inverse_of_1() {
    let p = PrimeField{prime: 1367u.to_biguint().unwrap()};
    assert!(p.inverse(~One::one()) == One::one());
}

#[cfg(test)]
fn check_inversion(n: BigUint) {
    let p = PrimeField{prime: 1367u.to_biguint().unwrap()};
    let inverse = p.inverse(&n);
    assert!(p.include(inverse.clone()));
    assert!(p.modulo(inverse * n) == One::one());
}

#[test]
fn check_prime_minus_1() {
    check_inversion(1367u.to_biguint().unwrap() - One::one());
}

/*

    def check_inversion(n)
      inverse = field.inverse(n)
      expect(field).to include inverse
      expect(field.mod(inverse * n)).to eq 1
    end

    it 'when given prime - 1 returns the inverse' do
      check_inversion prime - 1
    end

    it 'when given 44 returns the inverse' do
      check_inversion 44
    end

    context 'for large primes' do
      let(:prime) { 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F }

      it 'still works' do
        check_inversion 0xd4418917_5bd60c4f_6ead9f5f_301fd4a9_a5ece4c4_7ab45186_11b4c650_77ba7a6b
      end
    end
  end*/
