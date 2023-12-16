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
    (at most) the smallest number n such that 10n − 1 is divisible by d.
*/

fn main() {
    let mut n: usize = 0;
    let mut max_period: usize = 0;

    let mut i: usize = 1;
    while i <= 1000 {
        if i % 5 != 0 {
            let mut p = 1;
            while p < i && (10_usize.pow(p as u32)) % i != 1 {
                p += 1;
            }
            if p > max_period {
                max_period = p;
                n = i;
            }
        }
        i += 2;
    }

    print!("blah");
}
