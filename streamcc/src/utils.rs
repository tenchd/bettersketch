
use union_find_rs::prelude::*;

pub struct MyUnionFind{
    sets: DisjointSets<usize>,
}

impl MyUnionFind{
    pub fn new(n:usize) -> MyUnionFind{
        let mut sets: DisjointSets<usize> = DisjointSets::new();
        for i in 0..n{
            sets.make_set(i).unwrap();
        }
        MyUnionFind{
            sets,
        }
    }

    pub fn same(& self, u: usize, v: usize) -> bool{
        self.sets.find_set(&u).unwrap() == self.sets.find_set(&v).unwrap()
    }

    pub fn merge(&mut self, u: usize, v: usize){
        self.sets.union(&u ,&v).unwrap();
    }

    pub fn get(& self, u: usize) -> usize {
        self.sets.find_set(&u).unwrap()
    }
}