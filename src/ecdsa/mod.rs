use num::bigint::BigInt;
use ecdsa::group::Group;
use ecdsa::prime_field::PrimeField;
use std::num::from_str_radix;
use std::num::Zero;
use std::num::One;
use ecdsa::signature::Signature;
use ecdsa::point::Infinity;
use ecdsa::point::Finite;

pub mod group;
pub mod point;
pub mod prime_field;
pub mod signature;
pub mod verify;

/*
  # Produces an ECDSA signature.
  #
  # This algorithm comes from section 4.1.3 of [SEC1](http://www.secg.org/collateral/sec1_final.pdf).
  #
  # @param group (Group) The curve that is being used.
  # @param private_key (Integer) The private key.  (The number of times to add
  #   the generator point to itself to get the public key.)
  # @param digest (Integer)
  #   A digest of the message to be signed, usually generated with a hashing algorithm
  #   like SHA2.  The same algorithm must be used when verifying the signature.
  # @param temporary_key (Integer)
  #   A temporary private key.
  #   This is also known as "k" in some documents.
  #   Warning: Never use the same `temporary_key` value twice for two different messages
  #   or else it will be easy for someone to calculate your private key.
  #   The `temporary_key` should be generated with a secure random number generator.
  # @return (Signature or nil)  Usually this method returns a {Signature}, but
  #   there is a very small chance that the calculated "s" value for the
  #   signature will be 0, in which case the method returns nil.  If that happens,
  #   you should generate a new temporary key and try again.
*/
pub fn sign(g: Group, private_key: BigInt, digest: BigInt, temporary_key: BigInt) -> Option<Signature> {
    // Second part of step 1: Select ephemeral elliptic curve key pair
    // temporary_key was already selected for us by the caller
    let r_point = g.multiply_by_scalar(&g.generator, &temporary_key);
    
    match r_point {
        Infinity => return None,
        Finite(rx,_) => {
            // Steps 2 and 3p
            let point_field = PrimeField {prime: g.order};
            let r = point_field.modulo(&rx);
            
            // Step 4, calculating the hash, was already performed by the caller.

            // Step 5
            // TODO normalize digest
            //    e = normalize_digest(digest, group.bit_length)
            
            // Step 6
            let s = point_field.modulo(&(point_field.inverse(&temporary_key) * (digest + r * private_key)));
            if s == Zero::zero() {
                return None;
            }

            Some(Signature {r:r,s:s})
        }
    }
}

/*
// TODO finish normalize digest
fn normalize_digest(digest: &[u8], bit_length: int) -> BigInt {
    let digest_bit_length = digest.len() * 8;
    let num = BigInt::parse_bytes(digest, 8);
    match num {
        Some(num) => {

            if digest_bit_length <= bit_length {
                num
            } else {
                num >> (digest_bit_length - bit_length)
            }
        },
        None => fail!("invalid octet string")            
    }
}
*/
/*
  def self.normalize_digest(digest, bit_length)
    if digest.is_a?(String)
      digest = digest.dup.force_encoding('BINARY')
      digest_bit_length = digest.size * 8
      num = Format::IntegerOctetString.decode(digest)

      if digest_bit_length <= bit_length
        num
      else
        num >> (digest_bit_length - bit_length)
      end
    elsif digest.is_a?(Integer)
      digest
    else
      raise ArgumentError, 'Digest must be a string or integer.'
    end
  end
*/


fn group() -> Group {
    Group {
        name: "secp112r1".to_owned(), 
        generator: Finite( 
            from_str_radix("09487239995A5EE76B55F9C2F098", 16).unwrap(),
            from_str_radix("A89CE5AF8724C0A23E0E0FF77500", 16).unwrap()),
        field: PrimeField{prime: from_str_radix("DB7C2ABF62E35E668076BEAD208B",16).unwrap()},
        param_a: from_str_radix("DB7C2ABF62E35E668076BEAD2088",16).unwrap(),
        param_b: from_str_radix("659EF8BA043916EEDE8911702B22",16).unwrap(),
        order: from_str_radix("DB7C2ABF62E35E7628DFAC6561C5", 16).unwrap(),
        
    }
}

#[test]
fn check_sign_returns_none_if_s_is_zero() {
    let g = group();
    let private_key: BigInt = One::one();
    let temporary_key: BigInt = One::one();
    let r_point = g.multiply_by_scalar(&g.generator, &temporary_key);
    match r_point {
        Infinity => fail!("Inifity unexpected"),
        Finite(rx, _) => {
            let prime_field = PrimeField{prime:g.order.clone()};
            let r = prime_field.modulo(&rx);
            let e = -(r * private_key);
            assert!(sign(g,private_key,e, temporary_key) == None);
        }
    }
}
