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
    let mut range = 1000000;
    let mut reps = 1;    
    if vec_length < 1000000 {
        range = vec_length;
        reps = 1000000/range;
    }
    for _j in 0..reps{
        for i in 0..range {
            x.update(i);
        }
    }
    x.update(1);
    
    println!("{}", x.query().expect("Sampling failed."))
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_sketch_updates3(b: &mut Bencher) {
        b.iter(|| speed_updates(1000));
    }
    
    #[bench]
    fn bench_sketch_updates4(b: &mut Bencher) {
        b.iter(|| speed_updates(10000));
    }

    #[bench]
    fn bench_sketch_updates5(b: &mut Bencher) {
        b.iter(|| speed_updates(100000));
    }

    #[bench]
    fn bench_sketch_updates6(b: &mut Bencher) {
        b.iter(|| speed_updates(1000000));
    }

    #[bench]
    fn bench_sketch_updates7(b: &mut Bencher) {
        b.iter(|| speed_updates(10000000));
    }

    #[bench]
    fn bench_sketch_updates8(b: &mut Bencher) {
        b.iter(|| speed_updates(100000000));
    }

    #[bench]
    fn bench_sketch_updates9(b: &mut Bencher) {
        b.iter(|| speed_updates(1000000000));
    }

    #[bench]
    fn bench_sketch_updates10(b: &mut Bencher) {
        b.iter(|| speed_updates(10000000000));
    }
    
    #[bench]
    fn bench_sketch_updates11(b: &mut Bencher) {
        b.iter(|| speed_updates(100000000000));
    }

    #[bench]
    fn bench_sketch_updates12(b: &mut Bencher) {
        b.iter(|| speed_updates(1000000000000));
    }

    #[bench]
    fn bench_sketch_updates13(b: &mut Bencher) {
        b.iter(|| speed_updates(10000000000000));
    }

    #[bench]
    fn bench_sketch_updates14(b: &mut Bencher) {
        b.iter(|| speed_updates(100000000000000));
    }

    #[bench]
    fn bench_sketch_updates15(b: &mut Bencher) {
        b.iter(|| speed_updates(1000000000000000));
    }

    #[bench]
    fn bench_sketch_updates16(b: &mut Bencher) {
        b.iter(|| speed_updates(10000000000000000));
    }

    #[bench]
    fn bench_sketch_updates17(b: &mut Bencher) {
        b.iter(|| speed_updates(10000000000000000));
    }

    #[bench]
    fn bench_sketch_updates18(b: &mut Bencher) {
        b.iter(|| speed_updates(1000000000000000000));
    }
}
