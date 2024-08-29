#![feature(test)]

use num_bigint::BigUint;
use std::collections::HashSet;

extern crate test;

/*
    Initial Thoughts:

    This one should be possible to brute force every answer and enter into a
    collection that only keeps unique values (hashset/map or something?).

    I immediately noticed that any non prime values using multiples of 2 could
    likely be logically deduced, but I'm going to try a brute force first, I
    think in Rust it will probably be fast enough.

    Complexity here is going to be the nested for loops, we'll have 98*98
    iterations. I'm going to start with u128 values but I have a feeling I'm
    going to rather quickly start having to use large number libraries.

    Solution Thoughts:

    Yep, ran into large numbers, but Rust has great support for that. Brute
    forcing worked really well:

    tests::bench_calculate_answer_method_1 ... bench:   4,059,015.00 ns/iter (+/- 1,228,926.00)

    With these speeds, I probably wouldn't bother much more.
*/
fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> usize {
    let mut terms: HashSet<BigUint> = HashSet::new();
    for a in num_iter::range_inclusive(BigUint::from(2u8), BigUint::from(100u8)) {
        for b in 2..101 {
            terms.insert(a.pow(b));
        }
    }
    return terms.iter().count();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(9183, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
