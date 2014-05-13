use num::bigint::BigInt;
use num::bigint::ToBigInt;
use ecdsa::group::Group;
use ecdsa::prime_field::PrimeField;
use std::num::from_str_radix;
use std::num::Zero;
use num::Integer;
use ecdsa::signature::Signature;
use ecdsa::point::Infinity;
use ecdsa::point::Finite;

pub mod group;
pub mod point;
pub mod prime_field;
pub mod signature;

/*
  # Produces an ECDSA signature.
  #
  # This algorithm comes from section 4.1.3 of [SEC1](http://www.secg.org/collateral/sec1_final.pdf).
  #
  # @param group (Group) The curve that is being used.
  # @param private_key (Integer) The private key.  (The number of times to add
  #   the generator point to itself to get the public key.)
  # @param digest (String or Integer)
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
        Finite(rx,ry) => {
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
