extern crate siphasher;

use siphasher::sip::SipHasher;
use std::hash::{Hash,Hasher};


fn main() {
    try_hashing(5);
}

fn try_hashing(hash_input: u64) {
    let mut siphash = SipHasher::new_with_keys(10, 20);
    hash_input.hash(&mut siphash);
    println!("{}", siphash.finish());
}

/*
pub fn get_r(&self, bucket_id: u64, g: &GraphSketchState) -> i128 {
    let mut siphash = SipHasher::new_with_keys(g.sketch_seed, bucket_id);
    bucket_id.hash(&mut siphash);
    modulo(siphash.finish() as i128, g.p)
}
*/