extern crate num;
use num::bigint::BigUint;
use num::bigint::ToBigUint;
//use num::bigint::{Zero};
use std::num::Zero;
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

        let (t, newt) = (0u.to_biguint().unwrap(), 1u.to_biguint().unwrap());
        let (r, newr) = (self.prime.clone(), (*n).clone());

        while !newr.is_zero() {
            let quotient = r / newr;
            let (t, newt) = (newt.clone(), t - quotient * newt);
            let (r, newr) = (newr.clone(), r - quotient * newr);
        }
        println!("finished loop!");
        if r > 1u.to_biguint().unwrap() { fail!("prime is not invertible") }
        if t < Zero::zero() { let t = t + (*n).clone(); }
        return t.clone();
    }
}

#[test]
#[should_fail]
fn fail_when_0() {
    let p = PrimeField{prime: 1367u.to_biguint().unwrap()};
    p.inverse(&0u.to_biguint().unwrap());
}

#[test]
fn inverse_of_1() {
    let p = PrimeField{prime: 1367u.to_biguint().unwrap()};
    assert!(p.inverse(&1u.to_biguint().unwrap()) == 1u.to_biguint().unwrap());
}

/*
    it 'when given 1 returns 1' do
      expect(field.inverse(1)).to eq 1
    end

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
