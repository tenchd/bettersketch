#![feature(test)]


extern crate siphasher;
extern crate ndarray;
extern crate fasthash;
extern crate test;

mod xorsketch;
mod connectivity;

use xorsketch::XorSketch;


// fn main() {
//     for j in 0..20 {
//         let mut x = XorSketch::new(100,0.5_f64,j);
//         for i in 0..10 {
//             x.update(2*i);
//         }
//         println!("{}", x.query().expect("Sampling failed."));
//     }
// }






fn main() {
    println!("IGNORE ME");
}

fn speed_updates(vec_length: u64) {
    let mut x = XorSketch::new(vec_length,0.5_f64,1);
    for i in 0..1000000 {
        x.update(i);
    }
    println!("{}", x.query().expect("Sampling failed."))
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_sketch_updates(b: &mut Bencher) {
        b.iter(|| speed_updates(1000000));
    }
}