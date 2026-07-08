pub fn decimal_digits(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.reverse();
    digits
}

pub fn digit_power_sum(n: u64, power: u32) -> u64 {
    decimal_digits(n)
        .into_iter()
        .map(|digit| digit.pow(power))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_decimal_digits() {
        assert_eq!(decimal_digits(0), vec![0]);
        assert_eq!(decimal_digits(12_345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sums_digit_powers() {
        assert_eq!(digit_power_sum(1634, 4), 1634);
    }
}
