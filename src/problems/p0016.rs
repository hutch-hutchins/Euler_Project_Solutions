use crate::util::decimal::{digit_sum, from_u64, multiply_by_u32};

const POWER: usize = 1_000;

pub fn solve() -> String {
    power_digit_sum(2, POWER).to_string()
}

fn power_digit_sum(base: u32, exponent: usize) -> u64 {
    let mut digits = from_u64(1);

    for _ in 0..exponent {
        multiply_by_u32(&mut digits, base);
    }

    digit_sum(&digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_two_to_the_fifteenth() {
        assert_eq!(power_digit_sum(2, 15), 26);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "1366");
    }
}
