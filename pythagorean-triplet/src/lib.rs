#![feature(test)]
extern crate test;
use test::Bencher;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut ret_set = HashSet::new();
    let max_a = ((sum as f64).sqrt()*2.0) as u32;
    
    for a in 1..=max_a {
        for b in a+1..=((sum-a)/2) {
            let c = ((a.pow(2) + b.pow(2)) as f64).sqrt();

            if c.fract() == 0.0 {
                let triple_sum = a + b + c as u32;
                if sum%triple_sum == 0 {
                    let mult = sum/triple_sum;
                    ret_set.insert([a * mult, b * mult, c as u32 * mult]);
                }
            }
        }
    }

    ret_set
}

pub fn find_slow(sum: u32) -> HashSet<[u32; 3]> {
    let mut ret_set = HashSet::new();
    for a in 1..=(((sum/3) - 2u32) as f64) as u32 {
        for b in a+1..=((((sum-a)/2) - 1u32) as f64) as u32 {
            let c = ((a.pow(2) + b.pow(2)) as f64).sqrt();
            if c.fract() == 0.0 {
                if a + b + c as u32 == sum {
                    ret_set.insert([a, b, c as u32]);
                }
            }
        }
    }

    ret_set
}



#[bench]
fn bench_pythag(b: &mut Bencher){
    b.iter(|| {
        find(30000);
    });
}

#[bench]
#[ignore]
fn bench_pythag_slow(b: &mut Bencher){
    b.iter(|| {
        find_slow(3);
    });
}
