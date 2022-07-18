
use ndarray::{Array1,arr1};
use crate::xorsketch::XorSketch;
use crate::utils::MyUnionFind;



pub fn choose2(n: u64) -> u64 {
    n*(n-1)/2
}

#[derive(Debug)]
pub struct Edge {
    pub v1: u64,
    pub v2: u64,
}

impl Edge {
    pub fn new(endpoint1: u64, endpoint2: u64) -> Edge {
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
        //choose2(n) - choose2(n-self.v1) + self.v2 - self.v1
    }

    pub fn index_to_edge(index: u64, n: u64) -> Edge {
        let m = choose2(n);
        let i = n - ((1 + ((8*(m-index-1)+1) as f64).sqrt() as u64)/2) -1;
        //let i = n - ((1 + ((8*(m-index-2)+1) as f64).sqrt() as u64)/2) -1;
        let j = index + i - (m - (n-i) * (n-i-1)/2)+1;
        //let j = index + i - (m - (n-i) * (n-i-1)/2);
        Edge::new(i,j)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Supernode {
    pub cubesketches: Array1<XorSketch>
}

impl Supernode{
    pub fn new(n:u64, repetition_factor: f64, sketch_seed: u64) -> Supernode {
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

    pub fn query(&mut self, round:usize) -> Option<u64> {
        self.cubesketches[round].query()
    }

}

#[derive(Debug)]
pub struct GraphSketch {
    pub vertex_sketches: Array1<Supernode>,
    pub n: u64,
    pub failure_exp: u64,
    pub sketch_seed: u64,
}

impl GraphSketch{
    pub fn new(n:u64,failure_exp:u64,sketch_seed:u64) -> GraphSketch{
        let mut supernodes: Vec<Supernode> = Vec::new();
        let repetitions: f64 = (failure_exp as f64) *((n as f64).log2().ceil());
        for _ in 0..n {
            supernodes.push(Supernode::new(n,repetitions,sketch_seed));
        }
        let mut vertex_sketches: Array1::<Supernode> = arr1(&supernodes);
        GraphSketch{
            vertex_sketches,
            n,
            failure_exp,
            sketch_seed,
        }
    }
    pub fn update(&mut self, e: Edge){
        let v1 = e.v1 as usize;
        let v2 = e.v2 as usize;
        let index = e.edge_to_index(self.n);
        self.vertex_sketches[v1].update(index);
        self.vertex_sketches[v2].update(index);
    }

    pub fn query(&mut self) -> MyUnionFind{
        let mut components = MyUnionFind::new(self.n as usize);
        let mut round = 0;
        for i in 0..self.n{
            let check_index = self.vertex_sketches[i as usize].query(round);
            let mut index = 0;
            match(check_index){
                Some(x) => index = x,
                None => continue,
            }
            let e = Edge::index_to_edge(index, self.n);
            if !components.same(e.v1 as usize, e.v2 as usize){
                components.merge(e.v1 as usize, e.v2 as usize);
            }
        }
        components
    }
}


//let repetitions: i64 = (c as i64) *((n as f64).log2().ceil() as i64);