#![feature(test)]

use std::vec;

extern crate test;

/*
    Initial Thoughts:

    This should be a simple logic loop I believe. If there must be two digits in
    the numerator and denominator, that limits the total combinations to be 99-9
    or 90. If cancelling out 0's in the ones place is trivial, that will further
    reduce our options for each set to 81 since there are 9 multiples of ten in
    the set. These can be ignored because if the other number is removed, a
    divide by zero error would occur. 81^2 is 6561, so we have that many
    iterations to consider.

    We can quickly filter these iterations with some guard checks. If there
    isn't a number they share in common, we will move on to the next iteration.


    Psuedo Code:

    Let the equation be a/b = c. A Vec[u8] containing only valid numbers from
    above will be built. A Vec<Vec<[u8; 2]>> will be created to hold the
    valid digit cancelling fractions. Then there will be a for loop over each
    item for a, and an inner for loop over each item for b. Within the inner
    loop, we will check to see if they share a number in common, if they do we
    will continue. Then, we will perform a comparison to see if the fractions
    are identically equal with their common number cancelled out. If they are,
    add it to the list.

    After the loops, ensure that we found exactly four examples, and then
    multiply each fraction and find the least common denominator and return it.


    Solution Thoughts:

    It worked about like I thought it would. One annoyance was typing out the
    set of numbers for a and b. It probably doesn't save a ton of time and these
    could probably be replaced with something like for a in 11..99 instead.


    Performance:

    tests::bench_calculate_answer_method_1 ... bench:   1,074,140.00 ns/iter (+/- 110,421.00)

    This is 0.001 seconds, so it doesn't make sense to further refine it.
*/

fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> usize {
    let mut digit_cancelling_fractions = Vec::<[usize; 2]>::with_capacity(4);

    let valid_fraction_parts: Vec<usize> = vec![
        11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34, 35,
        36, 37, 38, 39, 41, 42, 43, 44, 45, 46, 47, 48, 49, 51, 52, 53, 54, 55, 56, 57, 58, 59, 61,
        62, 63, 64, 65, 66, 67, 68, 69, 71, 72, 73, 74, 75, 76, 77, 78, 79, 81, 82, 83, 84, 85, 86,
        87, 88, 89, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    ];

    for a in valid_fraction_parts.clone() {
        for b in valid_fraction_parts.clone() {
            if a >= b {
                continue;
            }

            let b_numbers = a.to_string();

            for a_number in a.to_string().chars() {
                if b_numbers.contains(a_number) {
                    let a_cancelled: usize = match Some(a.to_string().replace(a_number, ""))
                        .filter(|x: &String| x.len() > 0)
                    {
                        None => continue,
                        Some(x) => x.parse::<usize>().unwrap(),
                    };
                    let b_cancelled: usize = match Some(b.to_string().replace(a_number, ""))
                        .filter(|x: &String| x.len() > 0)
                    {
                        None => continue,
                        Some(x) => x.parse::<usize>().unwrap(),
                    };

                    if a as f64 / b as f64 == a_cancelled as f64 / b_cancelled as f64 {
                        digit_cancelling_fractions.push([a, b])
                    }
                }
            }
        }
    }

    if digit_cancelling_fractions.len() != 4 {
        panic!("Somehow didn't get 4 results!");
    }

    let final_numerator: usize = digit_cancelling_fractions
        .iter()
        .map(|x| x[0])
        .product::<usize>();
    let final_denominator: usize = digit_cancelling_fractions
        .iter()
        .map(|x| x[1])
        .product::<usize>();
    let gcd = greatest_common_divisor_iterative(final_numerator as i64, final_denominator as i64);
    let reduced_denominator = final_denominator / gcd as usize;

    return reduced_denominator;
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/greatest_common_divisor.rs
pub fn greatest_common_divisor_iterative(mut a: i64, mut b: i64) -> i64 {
    while a != 0 {
        let remainder = b % a;
        b = a;
        a = remainder;
    }
    b.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(100, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
