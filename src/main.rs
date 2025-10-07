use clap::Parser;
use num::bigint::BigUint;
use std::collections::VecDeque;

const ABOUT_MESSAGE: &str = "Count of bnuuies total following the Finobacci sequence.
One pair of offspring (k) per pair.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// Total duration of experiment in months
    n: usize,

    /// Lifetime of bnuuies in months
    m: usize,
}

fn bnuuy_pair_count(n: usize, m: usize) -> BigUint {
    let zero = || BigUint::ZERO;
    let one = || BigUint::try_from(1).unwrap();
    //no new bnuuies until after 2nd month
    if n <= 2 {
        return one();
    }

    let mut age_counts: VecDeque<BigUint> = VecDeque::with_capacity(m);
    age_counts.push_front(one());

    let mut gen_z: BigUint = zero();
    for _ in 1..n {
        age_counts.remove(m - 1);
        age_counts.push_front(gen_z);
        let mut sum_all = zero();
        for addend in age_counts.iter() {
            sum_all += addend;
        }
        gen_z = sum_all - age_counts.get(0).unwrap();
    }

    let mut count: BigUint = zero();
    for addend in age_counts.iter() {
        count += addend;
    }
    count
}

fn main() {
    let args = Cli::parse();
    let count = bnuuy_pair_count(args.n, args.m);
    println!("{}", count);
}

#[test]
fn count_6_3() {
    let (n, m) = (6, 3);
    let count = bnuuy_pair_count(n, m);
    assert_eq!(4, count);
}
