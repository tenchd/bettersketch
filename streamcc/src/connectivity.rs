
use ndarray::{Array1,arr1};
use ndarray::Dim;
use crate::xorsketch::XorSketch;


pub fn choose2(n: u64) -> u64 {
    n*(n-1)/2
}

#[derive(Debug)]
pub struct Edge {
    pub v1: u64,
    pub v2: u64,
}

impl Edge {
    pub fn make(endpoint1: u64, endpoint2: u64) -> Edge {
        assert!(endpoint1 != endpoint2);
        if endpoint1 < endpoint2{
            Edge {
                v1: endpoint1,
                v2: endpoint2,
            }
        } else {
            Edge {
                v1: endpoint2,
                v2: endpoint1,
            }
        }
    }

    pub fn edge_to_index(&self, n: u64) -> u64 {
        choose2(n) - choose2(n-self.v1) + self.v2 - self.v1 - 1
    }

    pub fn index_to_edge(index: u64, n: u64) -> Edge {
        let m = choose2(n);
        let i = n - ((1 + ((8*(m-index-1)+1) as f64).sqrt() as u64)/2) -1;
        let j = index + i - (m - (n-i) * (n-i-1)/2)+1;
        Edge::make(i,j)
    }
}

#[derive(Debug)]
pub struct Supernode {
    pub cubesketches: Array1<XorSketch>
}

impl Supernode{
    pub fn make(& self, n:u64, repetition_factor: f64, sketch_seed: u64) -> Supernode {
        let rounds: u64 = (n as f64).log2().ceil() as u64;
        let vec_length: u64 = choose2(n);
        let mut sketches: Vec<XorSketch> = Vec::new();
            for _ in 0..rounds {
                sketches.push(XorSketch::new(vec_length, repetition_factor, sketch_seed));
            }
        let mut cubesketches: Array1::<XorSketch> = arr1(&sketches);
        Supernode{
            cubesketches,
        }
    }

    pub fn update(&mut self, index:u64){
        for i in 0..self.cubesketches.len(){
            self.cubesketches[i].update(index);
        }
    }

}


//let repetitions: i64 = (c as i64) *((n as f64).log2().ceil() as i64);