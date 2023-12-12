use is_prime::*;

/*
    Initial Thoughts:

    I think I can build out some sort of iterator that starts at one then goes
    right, up, left, down in that order the appropriate amount of times to build
    the next square. We will build a list of numbers added along the way.

    With every number that is added, we will calculate if they are prime or not
    and keep a running total. Once it falls below 10%, we will return.


    Second Thoughts:

    Right after typing the above thoughts out I had another idea. The total
    number of new digits in each square surely follows some sort of pattern. I
    can likely use ranges and some math to calculate what numbers the outer
    shell of the square contains given N where N is the length of the side. I
    can then iterate over N to ∞. For each iteration, i'll check the outer shell
    for prime numbers, and add them to a running count of prime numbers found.
    At the end of every iteration, I'll calculate the running percentage, and
    if it falls below 10, I will return.
*/
fn main() {
    let mut n: i128 = 1;
    let mut prime_number_count: i128 = 0 as i128;
    let mut total_number_count: i128 = 1;
    let mut last_number: i128 = 1;

    loop {
        let mut corners: Vec<i128> = vec![];
        corners.push(last_number + n + 1);
        corners.push(last_number + ((n + 1) * 2));
        corners.push(last_number + ((n + 1) * 3));
        corners.push(last_number + ((n + 1) * 4));
        last_number = last_number + ((n + 1) * 4);
        n += 2;

        for corner in corners {
            total_number_count += 1;
            if is_prime(corner.to_string().as_str()) {
                prime_number_count += 1;
            }
        }

        let prime_ratio = prime_number_count as f64 * 100 as f64 / total_number_count as f64;

        println!("Side Length: {n}. Ratio: {prime_ratio}%");
        if prime_ratio < 10.0 {
            println!("Ratio fell below 10%! Exiting...");
            break;
        }
    }
}
