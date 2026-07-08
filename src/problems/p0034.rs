use crate::util::digits::decimal_digits;

pub fn solve() -> String {
    digit_factorial_sum().to_string()
}

fn digit_factorial_sum() -> u64 {
    let factorials = digit_factorials();
    let upper_bound = digit_factorial_search_upper_bound(factorials[9]);

    (10..=upper_bound)
        .filter(|&n| n == sum_digit_factorials(n, &factorials))
        .sum()
}

fn digit_factorials() -> [u64; 10] {
    let mut factorials = [1; 10];

    for n in 1..factorials.len() {
        factorials[n] = factorials[n - 1] * n as u64;
    }

    factorials
}

fn sum_digit_factorials(n: u64, factorials: &[u64; 10]) -> u64 {
    decimal_digits(n)
        .into_iter()
        .map(|digit| factorials[digit as usize])
        .sum()
}

fn digit_factorial_search_upper_bound(nine_factorial: u64) -> u64 {
    let mut digits = 1;

    while 10_u64.pow(digits - 1) <= digits as u64 * nine_factorial {
        digits += 1;
    }

    (digits as u64 - 1) * nine_factorial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_145() {
        assert_eq!(sum_digit_factorials(145, &digit_factorials()), 145);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "40730");
    }
}
