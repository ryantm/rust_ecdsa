use num::bigint::BigInt;
use ecdsa::signature::Signature;
use ecdsa::point::Infinity;
use ecdsa::point::Finite;
use ecdsa::group::Group;
use ecdsa::point::Point;
use ecdsa::prime_field::PrimeField;
use std::num::Zero;


/* # Verifies the given {Signature} and raises an {InvalidSignatureError} if it
  # is invalid.
  #
  # This algorithm comes from Section 4.1.4 of [SEC1](http://www.secg.org/collateral/sec1_final.pdf).
  #
  # @param public_key (Point)
  # @param digest (String or Integer)
  # @param signature (Signature)
  # @return bool */
fn check_signature(g: Group, public_key: Point, digest: BigInt, signature:Signature) -> bool {
    let field = &g.field;

    //# Step 1: r and s must be in the field and non-zero
    assert!(field.include(&signature.r), "Invalid signature: r is not in the field.");
    assert!(field.include(&signature.s), "Invalid signature: s is not in the field.");
    assert!(signature.r != Zero::zero(), "Invalid signature: r is zero.");
    assert!(signature.s != Zero::zero(), "Invalid signature: s is zero.");
    
    //# Step 2 was already performed when the digest of the message was computed.
    
    //# Step 3: Convert octet string to number and take leftmost bits.
    // TODO normalize digest
    //e = normalize_digest(digest, group.bit_length)
    
    //# Step 4
    let point_field = PrimeField{prime:g.order.clone()};
    let s_inverted = point_field.inverse(&signature.s);
    let u1 = point_field.modulo(&(digest * s_inverted)); // e -> digest
    let u2 = point_field.modulo(&(signature.r * s_inverted));
    
    //# Step 5
    match g.add(&g.multiply_by_scalar(&g.generator, &u1), &g.multiply_by_scalar(&public_key, &u2)) {
        Infinity => fail!("Invalid signature: r is infinity in step 5."),
        Finite(rx,_) => {
    
            //# Steps 6 and 7
            let v = point_field.modulo(&rx);

            //# Step 8
            assert!(v == signature.r, "Invalid signature: v does not equal r.");
        }
    }

    true
}
