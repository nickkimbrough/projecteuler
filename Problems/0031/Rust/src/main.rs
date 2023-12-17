#![feature(test)]
extern crate test;

/*
    Initial Thoughts

    I feel like there is an underlying mathematical formula here to calculate
    this. I might try simplifying it a bit on paper to see. I can already see
    an almost binary pattern where £2 is the first bit, £1 the second, etc.

    I feel like factorials might be something here.

    Thoughts After Research:

    It looks like we can use a generating function as discussed in this
    mathematics stack exchange post:
    https://math.stackexchange.com/q/15521/952561

    While this example is for a US Dollar, I think I can make this work for
    UK currency.

    Final Thoughts:

    This little function worked very well, and could easily be modified to
    calculate the number of ways any number of things could be combined to
    equal some sum. This could be very useful in the future. Apparently this
    field is called Combinatorics.
*/
fn main() {
    let d = calculate_answer();
    println!("Answer: {d}");
}

fn calculate_answer() -> usize {
    let mut c = [0; 201];
    c[0] = 1;

    // For each coin type
    for k in [1, 2, 5, 10, 20, 50, 100, 200] {
        // For each value, inclusive (201 not 200)
        for i in 0..(201 - k) {
            c[i + k] += c[i];
        }
    }

    return c[200];
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(73682, calculate_answer());
    }

    #[bench]
    fn bench_calculate_answer(b: &mut Bencher) {
        b.iter(|| calculate_answer());
    }
}
