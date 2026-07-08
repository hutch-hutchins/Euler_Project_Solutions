use crate::util::modular::mod_pow;

const LIMIT: u64 = 1_000;
const DIGITS: u32 = 10;

pub fn solve() -> String {
    self_power_series_last_digits(LIMIT, DIGITS)
}

fn self_power_series_last_digits(limit: u64, digits: u32) -> String {
    let modulus = 10_u64.pow(digits);
    let sum = (1..=limit)
        .map(|n| mod_pow(n, n, modulus))
        .fold(0, |accumulator, term| (accumulator + term) % modulus);

    format!("{sum:0width$}", width = digits as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_first_ten_terms() {
        assert_eq!(self_power_series_last_digits(10, 10), "0405071317");
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "9110846700");
    }
}
