extern crate siphasher;

use siphasher::sip::SipHasher;
use std::hash::{Hash,Hasher};


fn main() {
    let mut x = XorSketch::make(100,1,0);
    x.update(10);
    println!("{:?}", x.plains);
    x.update(5);
    println!("{:?}", x.plains);
    x.update(10);
    println!("{:?}", x.plains);
    println!("{}", x.query().expect("Sampling failed."));

}



pub struct XorSketch {
    pub plains: Vec<u64>,
    pub checks: Vec<u32>,
    pub sketch_seed: u64,
    vec_length: u64,
    guesses: u64,
    repetitions: u64,
    already_sampled: bool,
}

impl XorSketch {
    pub fn make(vec_length: u64, repetition_factor: u64, sketch_seed: u64) ->  XorSketch {
        let guesses: u64 = (vec_length as f64).log(2.0_f64).ceil() as u64;
        let repetitions: u64 = guesses * repetition_factor;
        //println!("guesses: {}, repetitions: {}", guesses, repetitions);
        let mut plains: Vec<u64> = Vec::new();
        let mut checks: Vec<u32> = Vec::new();
        for _i in 0..guesses {
            for _j in 0..repetitions {
                plains.push(0);
                checks.push(0);
            }
        }
        //println!("{}", plains.len());
        XorSketch{
            plains,
            checks,
            sketch_seed,
            vec_length,
            guesses,
            repetitions,
            already_sampled: false,
        }
    }

    pub fn update(&mut self, index: u64) {
        let mut siphash = SipHasher::new_with_keys(self.sketch_seed,0);
        index.hash(&mut siphash);
        let hashed_index = siphash.finish() as u32;
        for i in 0..self.guesses {
            for j in 0..self.repetitions {
                let bucket = (i*self.repetitions as u64 + j as u64) as usize;
                let mut buckethash = SipHasher::new_with_keys(self.sketch_seed, bucket as u64);
                    index.hash(&mut buckethash);
                if buckethash.finish() % u64::pow(2,j as u32) == 0 {
                    
                    self.plains[bucket] = self.plains[bucket] ^ index;
                    self.checks[bucket] = self.checks[bucket] ^ hashed_index;
                }
            }
        }
    }

    pub fn query(&mut self, ) -> Option<u64>{
        self.already_sampled = true;
        for i in 0..self.guesses {
            for j in 0..self.repetitions {
                let bucket = (i*self.repetitions as u64 + j as u64) as usize;
                let mut buckethash = SipHasher::new_with_keys(self.sketch_seed, bucket as u64);
                if self.plains[bucket] != 0 && self.plains[bucket] < self.vec_length {
                    self.plains[bucket].hash(&mut buckethash);
                    let hashed_value = buckethash.finish() as u32;
                    if hashed_value == self.checks[bucket] {
                        println!("nonzero found at position {}", self.plains[bucket]);
                        return Some(self.plains[bucket]);
                    }
                }  
            }
        }
        None
    }
}

