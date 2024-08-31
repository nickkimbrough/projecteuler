#![feature(test)]

use std::collections::HashSet;

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


    Secondary Thoughts:

    I made an incorrect logical assumption, that 5 digits was the maximum input.
    Thinking about it, the maximum input should actually be quite a bit higher
    as referenced above. It should be 295,245. This quickly led me to the answer
    with just a small adjustment to my for loop.


    Performance Thoughts and Benchmark:

    This currently only takes 1/10th of a second on my machine in Rust:
    tests::bench_calculate_answer_method_1 ... bench:  10,605,870.00 ns/iter (+/- 626,060.00)

    Trimming down the function to u32 yielded questionably small results, but
    will result in less memory pressure so is a good change:
    tests::bench_calculate_answer_method_1 ... bench:  10,543,360.00 ns/iter (+/- 1,307,110.00)


    Thoughts After Reading Other Solutions:

    While closer to reality, my assumption wasn't quite right with 295,245 being
    the maximum value, but it got close enough for the answer.
*/

fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> u32 {
    let mut digit_fifth_powers: HashSet<u32> = HashSet::new();
    for n in 2u32..295246 {
        let binding = n.to_string();
        let slices = binding.char_indices();

        let mut sum: u32 = 0;
        for slice in slices {
            let number: u32 = slice.1.to_digit(10).unwrap().into();
            sum += number.pow(5);
        }

        if n == sum {
            digit_fifth_powers.insert(n);
        }
    }

    return digit_fifth_powers.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(443839, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
