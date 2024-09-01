#![feature(test)]

use std::collections::HashSet;

extern crate test;

/*
    Initial Thoughts:

    The multiplicand, multiplier, and product have to equal nine digits, so
    there's a limiting factor there. If we think about how to make the shortest
    multiplicand and multiplier, it's probably using squares. For example,
    10 X 10 = 100. Using this logic, I can find the first square that has it's
    equation set have ten digits to find a limit.

    99 * 99 = 9801 is 8 digits, so we need more.
    100 * 100 = 10000 is 11 digits, so we know the limit is somewhere around
    that. I'll start with 10,000 and see if I can get the answer.


    Psuedo Code:

    Let's setup the equation as a X b = n.

    I'll initialize a HashSet[u32] to contain n.

    I'll have a for loop (n) from 1 to 10,000 for n, and inside of it, I'll have
    another for loop (a) going from 1 to n. Inside of that loop, I will check to
    see if n is divisible by a. If it is, I will evaluate the equation.

    If the equation evaluates, I'll add it to the HashSet.

    Then, I'll sum them all up.


    Post Solution Thoughts:

    My psuedo code was almost perfect, it worked like a charm. I had a few bugs
    in my logic that I quickly fixed. This seems quite fast too at 0.08 seconds:

    tests::bench_calculate_answer_method_1 ... bench:  79,402,990.00 ns/iter (+/- 11,287,205.00)

    If I were to  improve this, maybe I would exclude primes from n to not waste
    time there since they aren't divisible.
*/
fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> u32 {
    let mut unusual_products: HashSet<u32> = HashSet::new();

    'n: for n in 1u32..10001 {
        'a: for a in 1u32..n {
            if n % a == 0 {
                let b = n / a;
                let result = format!("{}{}{}", a, b, n);

                for i in 1..10 {
                    if !result.contains(&i.to_string()) {
                        continue 'a;
                    }
                }

                unusual_products.insert(n);
                continue 'n;
            }
        }
    }

    return unusual_products.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(45228, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
