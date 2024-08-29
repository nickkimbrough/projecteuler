#![feature(test)]

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
*/
fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> u128 {
    for a in 2..101 {
        for b in 2..101 {}
    }
    return 7;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(8675309, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
