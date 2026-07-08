use crate::util::digits::is_palindrome_base;

const LIMIT: u64 = 1_000_000;

pub fn solve() -> String {
    double_base_palindrome_sum_below(LIMIT).to_string()
}

fn double_base_palindrome_sum_below(limit: u64) -> u64 {
    (1..limit)
        .filter(|&n| is_palindrome_base(n, 10) && is_palindrome_base(n, 2))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_585() {
        assert!(is_palindrome_base(585, 10));
        assert!(is_palindrome_base(585, 2));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "872187");
    }
}
