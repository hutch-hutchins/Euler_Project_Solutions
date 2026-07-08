use crate::util::primes::is_prime;

const TARGET_COUNT: usize = 11;

pub fn solve() -> String {
    truncatable_prime_sum(TARGET_COUNT).to_string()
}

fn truncatable_prime_sum(target_count: usize) -> u64 {
    let mut sum = 0;
    let mut count = 0;
    let mut candidate = 11;

    while count < target_count {
        if is_truncatable_prime(candidate) {
            sum += candidate;
            count += 1;
        }
        candidate += 2;
    }

    sum
}

fn is_truncatable_prime(n: u64) -> bool {
    if n < 10 || !is_prime(n) {
        return false;
    }

    let mut power_of_10 = 10;
    while power_of_10 < n {
        if !is_prime(n / power_of_10) || !is_prime(n % power_of_10) {
            return false;
        }
        power_of_10 *= 10;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_3797() {
        assert!(is_truncatable_prime(3_797));
    }

    #[test]
    fn single_digit_primes_do_not_count() {
        assert!(!is_truncatable_prime(7));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "748317");
    }
}
