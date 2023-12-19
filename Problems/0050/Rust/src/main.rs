#![feature(test)]

use rayon::prelude::*;
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    vec,
};
extern crate test;

/*
    Initial Thoughts

    For this, I should generate a bunch of prime numbers and then do some
    analysis on the ones below a million. In order to generate the primes,
    I will need an algorithm. As I have already solved this in other languages,
    I am going to use the excellent [The Algorithms](https://github.com/TheAlgorithms)
    repository to get a Rust implementation.

    Intermediate Thoughts

    Well, I got the answer using this brute force method, without too much
    trouble. However, on my CPU it took quite a while. I'm going to play with
    Rayon and see if I can enable some multithreading here.

    Final Thoughts (for now)

    I learned a bit about implementing parallelism here. I had to rewrite the
    function to return a collection of collections and then flatten them. The
    result uses 100% of my CPU but operates much faster, here are the benchmark
    results:

    test tests::bench_calculate_answer_method_1 ... bench: 18,245,118,860 ns/iter (+/- 1,569,764,366)
    test tests::bench_calculate_answer_method_2 ... bench: 1,863,273,900 ns/iter (+/- 38,839,092)

    As you can see, we went from an average of 18.25 seconds to an average of 1.86
    seconds. This is an important reminder to remember to use the hardware
    available! Especially with the number of cores these days.

    I know as soon as I look at the forum here there are going to be math
    shortcuts that I could likely use to shorten this up. I may implement that
    if the fancy takes me.

    Final Final Thoughts:

    Fancy took me. The [top comment](https://projecteuler.net/thread=50#1215)
    mentioned summing up consecutive primes until right before it goes over
    a million. Then, subtracting from that list to find it.

    I found that if I subtracted the smallest primes I would find the answer. I
    can't guarantee this would work in all cases, but the performance is much
    faster at 0.044 seconds.

    test tests::bench_calculate_answer_method_3 ... bench:  44,747,210 ns/iter (+/- 5,522,292)
*/
fn main() {
    let d = calculate_answer_method_1();
    println!("Method 1 Answer: {d}");

    let e = calculate_answer_method_2();
    println!("Method 1 Answer: {e}");

    let f = calculate_answer_method_3();
    println!("Method 1 Answer: {f}");
}

fn calculate_answer_method_1() -> u32 {
    let max_prime_size = 999_999;

    let primes = prime_numbers(max_prime_size);

    let mut consecutive_primes: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in primes.iter() {
        //.skip_while(|x| x < &&952_usize) {
        let mut j: usize = 0;
        'outer: loop {
            let mut sum = 0;
            for k in 0..primes.len() {
                if primes[k + j] >= *i {
                    j = 0;
                    break 'outer;
                }
                if k + j > primes.len() - 1 {
                    j = 0;
                    break 'outer;
                }
                sum += primes[k + j];
                if sum == *i {
                    match consecutive_primes.entry(*i) {
                        Entry::Vacant(e) => {
                            e.insert(vec![k + 1]);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(k + 1);
                        }
                    }
                    break;
                } else if sum > *i {
                    break;
                }
            }
            j += 1;
        }
    }

    let answer = consecutive_primes
        .par_iter()
        .max_by(|a, b| a.1.iter().max().unwrap().cmp(b.1.iter().max().unwrap()))
        .map(|(k, _v)| k)
        .unwrap();

    return *answer as u32;
}

fn calculate_answer_method_2() -> u32 {
    let max_prime_size = 999_999;

    let primes = prime_numbers(max_prime_size);

    let consecutive_primes: HashMap<usize, Vec<usize>> = primes
        .par_iter()
        .map(|x| {
            let mut consecutive_primes: HashMap<usize, Vec<usize>> = HashMap::new();
            //let mut consecutive_primes: Vec<[usize; 2]> = vec![];
            let mut j: usize = 0;
            'outer: loop {
                let mut sum = 0;
                for k in 0..primes.len() {
                    if primes[k + j] >= *x {
                        j = 0;
                        break 'outer;
                    }
                    if k + j > primes.len() - 1 {
                        j = 0;
                        break 'outer;
                    }
                    sum += primes[k + j];
                    if sum == *x {
                        match consecutive_primes.entry(*x) {
                            Entry::Vacant(e) => {
                                e.insert(vec![k + 1]);
                            }
                            Entry::Occupied(mut e) => {
                                e.get_mut().push(k + 1);
                            }
                        }
                        break;
                    } else if sum > *x {
                        break;
                    }
                }
                j += 1;
            }
            return consecutive_primes;
        })
        .flatten()
        .collect::<HashMap<usize, Vec<usize>>>();

    let answer = consecutive_primes
        .par_iter()
        .max_by(|a, b| a.1.iter().max().unwrap().cmp(b.1.iter().max().unwrap()))
        .map(|(k, _v)| k)
        .unwrap();

    return *answer as u32;
}

fn calculate_answer_method_3() -> u32 {
    let max_prime_size = 997_661;
    let primes = prime_numbers(max_prime_size);
    let mut primes_summed = 0;

    for prime in primes.iter() {
        if primes_summed + prime >= 1_000_000 {
            break;
        }
        primes_summed += prime;
    }

    let mut primes_to_remove: VecDeque<usize> = VecDeque::from(primes.clone());

    while !primes.contains(&primes_summed) && primes_to_remove.len() > 1 {
        let lowest_prime = primes_to_remove.pop_front().unwrap();
        primes_summed = primes_summed - lowest_prime;
        if primes.contains(&primes_summed) {
            return primes_summed.try_into().unwrap();
        }
    }

    panic!("No answer!");
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/math/prime_numbers.rs
pub fn prime_numbers(max: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    if max >= 2 {
        result.push(2)
    }
    for i in (3..max + 1).step_by(2) {
        let stop: usize = (i as f64).sqrt() as usize + 1;
        let mut status: bool = true;

        for j in (3..stop).step_by(2) {
            if i % j == 0 {
                status = false;
                break;
            }
        }
        if status {
            result.push(i)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(997651, calculate_answer_method_1());
        assert_eq!(997651, calculate_answer_method_2());
        assert_eq!(997651, calculate_answer_method_3());
    }

    #[bench]
    fn bench_calculate_answer_method_1(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_1());
    }
    #[bench]
    fn bench_calculate_answer_method_2(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_2());
    }
    #[bench]
    fn bench_calculate_answer_method_3(b: &mut Bencher) {
        b.iter(|| calculate_answer_method_3());
    }
}
