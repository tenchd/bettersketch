extern crate siphasher;

use siphasher::sip::SipHasher;
use std::hash::{Hash,Hasher};


fn main() {
    let mut x = XorSketch::make(4,5,0);
    x.update(10);
    println!("{:?}", x.plains);
    x.update(5);
    println!("{:?}", x.plains);
    x.update(10);
    println!("{:?}", x.plains);
}


fn try_hashing(hash_input: u64) {
    let mut siphash = SipHasher::new_with_keys(10, 20);
    hash_input.hash(&mut siphash);
    println!("{}", siphash.finish());
}


pub struct XorSketch {
    pub plains: Vec<u64>,
    pub checks: Vec<u32>,
    pub sketch_seed: u64,
    vec_length: u64,
    repetitions: u32,
    already_sampled: bool,
}

impl XorSketch {
    pub fn make(vec_length: u64, repetitions: u32, sketch_seed: u64) ->  XorSketch {
        let mut plains: Vec<u64> = Vec::new();
        let mut checks: Vec<u32> = Vec::new();
        for _i in 0..vec_length {
            for _j in 0..repetitions {
                plains.push(0);
                checks.push(0);
            }
        }
        XorSketch{
            plains,
            checks,
            sketch_seed,
            vec_length,
            repetitions,
            already_sampled: false,
        }
    }

    pub fn update(&mut self, index: u64) {
        let mut siphash = SipHasher::new_with_keys(0,0);
        index.hash(&mut siphash);
        let hashed_index = siphash.finish() as u32;
        for i in 0..self.vec_length {
            for j in 0..self.repetitions {
                let mut buckethash = SipHasher::new_with_keys(i,j as u64);
                    index.hash(&mut buckethash);
                if buckethash.finish() % u64::pow(2,j) == 0 {
                    let bucket = (i*self.repetitions as u64 + j as u64) as usize;
                    self.plains[bucket] = self.plains[bucket] ^ index;
                    self.checks[bucket] = self.checks[bucket] ^ hashed_index;
                }
            }
        }
    }



}


/*
pub fn get_r(&self, bucket_id: u64, g: &GraphSketchState) -> i128 {
    let mut siphash = SipHasher::new_with_keys(g.sketch_seed, bucket_id);
    bucket_id.hash(&mut siphash);
    modulo(siphash.finish() as i128, g.p)
}
*/