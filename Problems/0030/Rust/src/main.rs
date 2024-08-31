#![feature(test)]

extern crate test;

/*
    Initial Thoughts:

    As numbers with 5 digits only contain 99,999, I think the best option here
    will be to just loop through all of them and do a calculation with an
    efficient function, then if they qualify, add them to a colleciton.

    Then I'll just use something functional like the map method to get a sum of
    them all and return that.

    Let's think about the maximum sizes I'll be working with so I can use
    the most efficient variables:

    9^5 is 59049. Multiply this by 5 and you'll get 295,245. That's the maximum
    value for any one number. Multiply this by 99,999 and it becomes
    29,524,204,755. This is the absolute maximum number if EVERY number met the
    rule, which it will not. This is one more digit than u32, but a u64 will
    handily cover it. We can go smaller later once we get our answer.
*/

fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> u64 {
    return 128;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(57, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
