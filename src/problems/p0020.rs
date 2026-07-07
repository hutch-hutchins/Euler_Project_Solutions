use crate::util::decimal::{digit_sum, from_u64, multiply_by_u32};

const FACTORIAL: u32 = 100;

pub fn solve() -> String {
    factorial_digit_sum(FACTORIAL).to_string()
}

fn factorial_digit_sum(n: u32) -> u64 {
    let mut digits = from_u64(1);

    for factor in 2..=n {
        multiply_by_u32(&mut digits, factor);
    }

    digit_sum(&digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_ten_factorial() {
        assert_eq!(factorial_digit_sum(10), 27);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "648");
    }
}
