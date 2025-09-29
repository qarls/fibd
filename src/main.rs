use clap::Parser;
const ABOUT_MESSAGE: &str = "Count of bnuuies total following the Finobacci sequence.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// Months
    n: usize,
    /// Number of bnuuy pairs per litter per month
    k: usize,
}

fn bnuuy_pair_count(n: usize, k: usize) -> usize {
    //no new bnuuies until after 2nd month
    if n <= 2 {
        return 1;
    }

    //these represent current, F_{n-1} and F_{n-2}
    let mut count = 1;
    let mut f_one = 1;
    let mut f_two: usize;

    for _ in 2..n {
        f_two = f_one;
        f_one = count;
        count += f_two * k;
    }

    return count;
}

fn main() {
    let args = Cli::parse();
    let count = bnuuy_pair_count(args.n, args.k);
    println!("{}", count);
}

#[test]
fn count_5_3() {
    let (n, k) = (5, 3);
    let count = bnuuy_pair_count(n, k);
    assert_eq!(19, count);
}
