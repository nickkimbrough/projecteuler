#![feature(test)]

use std::collections::HashMap;
extern crate test;

/*
    Initial Thoughts

    The actual calculation of a pentagonal number doesn't seem to be too heavy
    of an operation. It wouldn't be hard to populate a dictionary of the first
    n pentagonal numbers, and then do some analysis on that.

    Given P_j, we should be able to determine what values from the dictionary
    would cause a probable P_k to cut down on brute forcing. I will give this
    a try. I will go into this assuming that an unsigned usize int will hold it.

    Final Thoughts

    Wow, it got the correct answer on my first attempt, I am surprised as I only
    got one value for D here. This program takes a while to execute. I'm going
    to refactor it to analyze it a bit more.
*/
fn main() {
    let d = calculate_answer().unwrap();
    println!("Answer: {d}");
}

fn get_pentagonal_number(n: usize) -> usize {
    return n * (3 * n - 1) / 2;
}

fn calculate_answer() -> Option<usize> {
    // Get the first 10,000 pentagonal numbers.
    // Key is pentagonal value, value is n
    let mut pentagonal_numbers: HashMap<usize, usize> = HashMap::new();
    for i in 1..10_000 {
        pentagonal_numbers.insert(get_pentagonal_number(i), i);
    }

    // Key is D, value is j,k
    for pentagonal_number in pentagonal_numbers.iter() {
        for inner_pentagonal_number in pentagonal_numbers.iter() {
            // Skip the same pentagonal numbers
            if pentagonal_number.1 == inner_pentagonal_number.1 {
                continue;
            }

            // Skip where the difference would be negative
            if pentagonal_number.0 > inner_pentagonal_number.0 {
                continue;
            }

            let sum = pentagonal_number.0 + inner_pentagonal_number.0;
            let difference = inner_pentagonal_number.0 - pentagonal_number.0;
            if pentagonal_numbers.contains_key(&sum) && pentagonal_numbers.contains_key(&difference)
            {
                return Some(inner_pentagonal_number.0.abs_diff(*pentagonal_number.0));
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(5482660, calculate_answer().unwrap());
    }

    #[bench]
    fn bench_calculate_answer(b: &mut Bencher) {
        b.iter(|| calculate_answer());
    }
}
