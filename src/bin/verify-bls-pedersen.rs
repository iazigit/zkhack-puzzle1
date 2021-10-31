use bls_pedersen::bls::verify;
use bls_pedersen::sol::solve;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};
use ark_bls12_381::{G1Affine};
use std::io::Cursor;
use ark_serialize::CanonicalDeserialize;
use ark_bls12_381::Fq as F;
use ark_ff::{PrimeField, FpParameters};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    let (pk, ms, sigs) = puzzle_data();

    /* Your solution here! */
    let modulus = <F as PrimeField>::Params::MODULUS;
    println!("{}", modulus);
    
    let username = "iazid#3291".as_bytes();
    let b2hash = blake2s_simd::blake2s(username);
    println!("{:?}", b2hash.to_hex());

    let sig = solve();
 
    let tmp =  G1Affine::deserialize(&mut Cursor::new(sig)).unwrap();
    println!("{:?}", tmp);
    
    verify(pk, &username, tmp);
    println!("soulved");


    for (m, sig) in ms.iter().zip(sigs.iter()) {
  //      verify(pk, m, *sig); 
    }
    
    
}
