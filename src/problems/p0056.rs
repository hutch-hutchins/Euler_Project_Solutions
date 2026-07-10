use crate::util::decimal::{digit_sum, from_u64, multiply_by_u32};

const LIMIT: u32 = 100;

pub fn solve() -> String {
    maximum_power_digit_sum(LIMIT).to_string()
}

fn maximum_power_digit_sum(limit: u32) -> u64 {
    let mut best = 0;

    for base in 1..limit {
        let mut digits = from_u64(1);

        for _exponent in 1..limit {
            multiply_by_u32(&mut digits, base);
            best = best.max(digit_sum(&digits));
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_range() {
        assert_eq!(maximum_power_digit_sum(10), 45);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "972");
    }
}
