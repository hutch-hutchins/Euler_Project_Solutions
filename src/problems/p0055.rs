use crate::util::decimal::{add, from_u64};

const LIMIT: u64 = 10_000;
const MAX_ITERATIONS: usize = 50;

pub fn solve() -> String {
    lychrel_count_below(LIMIT).to_string()
}

fn lychrel_count_below(limit: u64) -> usize {
    (1..limit)
        .filter(|&n| is_lychrel(n, MAX_ITERATIONS))
        .count()
}

fn is_lychrel(n: u64, max_iterations: usize) -> bool {
    let mut digits = from_u64(n);

    for _ in 0..max_iterations {
        let reversed = reversed_digits(&digits);
        digits = add(&digits, &reversed);

        if is_palindrome(&digits) {
            return false;
        }
    }

    true
}

fn reversed_digits(digits: &[u8]) -> Vec<u8> {
    digits.iter().rev().copied().collect()
}

fn is_palindrome(digits: &[u8]) -> bool {
    digits.iter().eq(digits.iter().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(!is_lychrel(47, MAX_ITERATIONS));
        assert!(!is_lychrel(349, MAX_ITERATIONS));
        assert!(is_lychrel(196, MAX_ITERATIONS));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "249");
    }
}
