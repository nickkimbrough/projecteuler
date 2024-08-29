#![feature(test)]

extern crate test;

/*
    Initial Thoughts:

    I think I'll need to write an efficient algorithm for checking the formula's
    values with a set of given inputs. Since there are 1000 variations of a and
    1001 variations of b, I think that means there are 1,001,000 different
    possible combinations here. n could grow to a very large number as well.

    My first attempt is going to be a brute force, but there may be some
    mathematical shortcuts that I could take here to eliminate some of the
    combinations that will arise after I start analyzing the data.


    Solution Thoughts:

    Brute force worked better than I expected. I ran into some logical pitfalls
    that I didn't think of at first. Primarily that a and b could be negative.
    There are a few ways I can think of to further optimize this function:

    1. Precalculate the primary numbers and add them to a vec so we aren't
    checking for prime over and over.
    2. Figure out some patterns and math shortcuts as previously mentioned.


    Benchmarking:

    Results show this only takes less than a second, so I don't think further
    optimization is really necessary.

    tests::bench_calculate_answer_method_1 ... bench:  76,688,170.00 ns/iter (+/- 34,849,216.00)


    Forum Thoughts:

    As I expected, there are a few mathematical shortcuts that can be figured
    out:

    - B has to be a prime for the function to even return a prime.
      - This means we could just create a vec of prime numbers within the range
      given for B and drastically reduce the cyclomatic complexity of the
      function.


    What I Learned (Or Re-Learned!):

    - In Rust, the upper end on a range is exclusive.
    - max_by_key is great for filtering based on a specific value inside of a vec.
*/

fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> i128 {
    let mut sequences: Vec<[i128; 3]> = Vec::new();

    for a in -999i128..1000i128 {
        for b in -1000i128..1001i128 {
            let mut n: i128 = 0;
            loop {
                let x: i128 = n.pow(2) + a * n + b;

                if x < 0 || (x >= 0 && !prime_check(x.try_into().unwrap())) {
                    let sequence = [n, a, b];
                    sequences.push(sequence);
                    n += 1;
                    break;
                }

                n += 1;
            }
        }
    }
    let max_sequence = sequences.iter().max_by_key(|x| x[0]).unwrap();
    return max_sequence[1] * max_sequence[2];
}

// Rather than write my own prime check again, I'm going to grab it from
// the excellent 'The Algorithms' Rust repo at
// https://github.com/TheAlgorithms/Rust/blob/master/src/math/prime_check.rs
pub fn prime_check(num: usize) -> bool {
    if (num > 1) & (num < 4) {
        return true;
    } else if (num < 2) || (num % 2 == 0) {
        return false;
    }

    let stop: usize = (num as f64).sqrt() as usize + 1;
    for i in (3..stop).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(-59231, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
