use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
    Initial Thoughts:

    My initial thoughts are really to just brute force this efficiently and hope
    that Rust's inherit performance and my decent CPU can get me through.
    Perhaps I could use Rayon to introduce multithreading if needed. I will
    try that.

    That being said, I do recognize there is probably a mathematical shortcut
    here that I'm currently unaware of. Maybe something like calculating
    powers until a least common denominator is found or something close and
    seeing which number has more exponentiations remaining.

    Part Two:

    Well, Rust can't save us from this, the math is massive and too much to
    brute force. I'm going to have do some research.

    After reading a bit, I think now I'm going to try fractional exponents. I
    got this idea from Math Stack Exchange here:
    https://math.stackexchange.com/a/4539696/952561

    I will compare two numbers and divide the larger exponent by the smaller
    exponent. Then I will be able to compare much easier by raising to a float.

    Since I am not getting the actual resulting values, I will need to do one
    of the following:
    1 - Compare all numbers by dividing all exponents by the smallest one.
      This might leave very large exponents still that my machine won't be able
      to calculate.
    2 - Compare two numbers at a time and do some type of sorting algorithm
      This will require a lot more comparisons but won't have numbers as large.

    After looking at the exponents, the spread is too wide for 1, so 2 it is.

    After thinking about 2 a bit, the type of sorting algorithm I think I want
    is the 'bubble sort'. I'm going to use an open source implementation that
    I found here:
    https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/bubble_sort.rs

    I will modify it to do the exponentiation comparison and take a non
    generic type for my use.
*/
fn main() {
    let input_file = "src/0099_base_exp.txt";
    let mut numbers: Vec<[u32; 3]> = vec![];
    if let Ok(lines) = read_lines(input_file) {
        let mut line_number: u32 = 1;
        for line in lines {
            if let Ok(ip) = line {
                let base = ip.split(',').nth(0).unwrap().parse::<u32>().unwrap();
                let exponent = ip.split(',').nth(1).unwrap().parse::<u32>().unwrap();
                numbers.push([base, exponent, line_number]);

                line_number += 1;
            }
        }
    }

    bubble_sort::<Vec<[u32; 3]>>(&mut numbers);

    let answer = numbers.iter().last().unwrap()[2];
    println!("Answer: Line {answer}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/bubble_sort.rs
pub fn bubble_sort<T: Ord>(arr: &mut Vec<[u32; 3]>) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            // This is mainly where I made my modification.
            let base_1 = arr[i][0] as f64;
            let mut exponent_1: f64 = arr[i][1] as f64;
            let base_2 = arr[i + 1][0] as f64;
            let mut exponent_2: f64 = arr[i + 1][1] as f64;

            let smaller_exponent = match exponent_1 < exponent_2 {
                true => exponent_1,
                false => exponent_2,
            };

            exponent_1 = exponent_1 / smaller_exponent;
            exponent_2 = exponent_2 / smaller_exponent;

            let product_1 = base_1.powf(exponent_1);
            let product_2 = base_2.powf(exponent_2);

            if product_1 > product_2 {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}
