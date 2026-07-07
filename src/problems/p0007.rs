use crate::util::primes::nth_prime;

const ORDINAL: usize = 10_001;

pub fn solve() -> String {
    nth_prime(ORDINAL).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_sixth_prime() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "104743");
    }
}
