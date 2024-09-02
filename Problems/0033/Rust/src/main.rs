#![feature(test)]

use std::collections::HashSet;

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
    above will be built. A Vec[Vec[u8; 2]; 4] will be created to hold the
    valid digit cancelling fractions. Then there will be a for loop over each
    item for a, and an inner for loop over each item for b. Within the inner
    loop, we will check to see if they share a number in common, if they do we
    will continue. Then, we will perform a comparison to see if the fractions
    are identically equal with their common number cancelled out. If they are,
    add it to the list.

    After the loops, ensure that we found exactly four examples, and then
    multiply each fraction and find the least common denominator and return it.
*/

fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(42, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
