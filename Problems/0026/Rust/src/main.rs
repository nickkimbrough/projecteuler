#![feature(test)]
/*
    Initial Thoughts:

    Recurring cycles are going to be tricky to detect. I'm thinking of turning
    the decimal fraction part into a string and doing some sort of growing
    window search to find matching repeating cycles. The windows will always
    be next to each other, but it won't always contain the entire decimal
    fraction part.

    I'm thinking about how many decimal fraction part places I need to calculate
    and what would constitute a recurring cycle. Now that I think about it more,
    I think I need a reptition of three to confirm a cycle.

    This is getting a bit confusing, I'm going to go do some research, there's
    probably a math shortcut.

    Math Shortcut:

    https://en.wikipedia.org/wiki/Repeating_decimal#A_shortcut

    Particularly this part:
    It follows that any repeating decimal with period n, and k digits after the
    decimal point that do not belong to the repeating part, can be written as a
    (not necessarily reduced) fraction whose denominator is (10n − 1)10k.

    Conversely the period of the repeating decimal of a fraction c/d will be
    (at most) the smallest number n such that 10^n − 1 is divisible by d.

    Implementation:

    Implementing this was a bit difficult, I ran into overflow so I had to start
    using BigUints.
*/
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};

extern crate test;

fn main() {
    let answer = calculate_answer();
    print!("The Answer Is {answer}");
}

fn calculate_answer() -> BigUint {
    let mut answer: BigUint = Zero::zero();
    let mut max_period: BigUint = Zero::zero();

    let mut d: u32 = 1;
    while d < 1000 {
        if d % 5 != 0 {
            let mut p: u32 = 1;
            while p < d && (10_i32.to_biguint().unwrap().pow(p) % d != One::one()) {
                p += 1;
            }
            if p.to_biguint().unwrap() > max_period {
                max_period = p.to_biguint().unwrap();
                answer = d.to_biguint().unwrap();
            }
        }
        d += 2;
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(983.to_biguint().unwrap(), calculate_answer());
    }

    #[bench]
    fn bench_calculate_answer(b: &mut Bencher) {
        b.iter(|| calculate_answer());
    }
}
