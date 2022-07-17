
use fasthash::xx::Hasher64 as XXHasher;
use crate::fasthash::FastHasher;
use std::hash::{Hash,Hasher};
use ndarray::Array2;
use ndarray::Dim;

#[derive(Debug)]
#[derive(Clone)]
pub struct XorSketch {
    pub plains: Array2<u64>,
    pub checks: Array2<u32>,
    pub sketch_seed: u64,
    vec_length: u64,
    guesses: u64,
    repetitions: u64,
    already_sampled: bool,
}


impl XorSketch {
    pub fn new(vec_length: u64, repetition_factor: f64, sketch_seed: u64) ->  XorSketch {
        let guesses: u64 = (vec_length as f64).log(2.0_f64).ceil() as u64;
        let repetitions: u64 = (guesses as f64 * repetition_factor).ceil() as u64;
        //println!("guesses: {}, repetitions: {}", guesses, repetitions);
        let plains = Array2::<u64>::zeros(Dim([repetitions as usize, guesses as usize]));
        let checks = Array2::<u32>::zeros(Dim([repetitions as usize, guesses as usize])); 
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
        let mut checkhash = XXHasher::with_seed(self.sketch_seed);
        index.hash(&mut checkhash);
        let hashed_index = checkhash.finish() as u32;
        for j in 0..self.repetitions {

            let mut repetition_hash = XXHasher::with_seed(self.sketch_seed);
            j.hash(&mut repetition_hash);
            index.hash(&mut repetition_hash);
            let result = repetition_hash.finish();
            let bit_reach = (result - 1) & (!result);
            let mut reach = ((bit_reach as f64 + 1.0).log(2.0_f64) + 1.0) as u64;
            reach = std::cmp::min(reach, self.guesses);

            for i in 0..reach {
                //let bucket = (i*self.repetitions as u64 + j as u64) as usize;
                
                //bucket.hash(&mut buckethash);
                //if buckethash.finish() & ((1 << i)-1) == 0 {
                    //if bucket == 10 {println!("Got to bucket 10 with index {} which hashes to {}", index, hashed_index)}
                self.plains[[j as usize, i as usize]] ^= index;
                self.checks[[j as usize, i as usize]] ^= hashed_index;
                //}
            }
        }
    }

    pub fn query(&mut self, ) -> Option<u64>{
        self.already_sampled = true;
        for j in 0..self.repetitions {
            for i in 0..self.guesses {
                //let bucket = (i*self.repetitions as u64 + j as u64) as usize;
                if self.plains[[j as usize, i as usize]] < self.vec_length {
                    let mut buckethash = XXHasher::with_seed(self.sketch_seed);
                    self.plains[[j as usize, i as usize]].hash(&mut buckethash);
                    let hashed_value = buckethash.finish() as u32;
                    if hashed_value == self.checks[[j as usize, i as usize]] {
                        println!("nonzero found at position {} in bucket {:?}", self.plains[[j as usize, i as usize]], (j,i));
                        return Some(self.plains[[j as usize, i as usize]]);
                    }
                }  
            }
        }
        None
    }
}