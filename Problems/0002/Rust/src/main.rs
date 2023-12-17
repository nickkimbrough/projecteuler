#![feature(test)]

extern crate test;

/*
    Trying a new implementation after reading the forums.

    We know that phi is the golden ratio, and that every third number is even.
    Given that, we can raise phi to the third power to calculate only the even
    numbers of the sequence.

    With some rounding applied, this might be the shortest method to get there.
    I also got to learn about testing in Rust! It still only takes 0ns though.
*/
fn main() {
    let sum = calculate_answer();
    println!("{sum}");
}

fn calculate_answer() -> usize {
    pub const PHI: f64 = 1.618033988749894848204586834365638118_f64;
    let multiplier = PHI.powi(3);
    let mut sum = 2;
    let mut i = 2;
    loop {
        let value = (i as f64 * multiplier).round() as u128;

        if value >= 4000000 {
            break;
        }

        i = value;
        sum += value as usize;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4613732, calculate_answer());
    }

    #[bench]
    fn bench_calculate_answer(b: &mut Bencher) {
        b.iter(|| calculate_answer());
    }
}
