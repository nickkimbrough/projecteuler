#![feature(test)]

extern crate test;

/*
    Initial Thoughts:

*/
fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");
}

fn calculate_answer_method_1() -> usize {
    return 7;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(7, calculate_answer_method_1());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
}
