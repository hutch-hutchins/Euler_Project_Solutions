use crate::util::primes::primes_below;

const LIMIT: usize = 2_000_000;

pub fn solve() -> String {
    sum_primes_below(LIMIT).to_string()
}

fn sum_primes_below(limit: usize) -> u64 {
    primes_below(limit)
        .into_iter()
        .map(|prime| prime as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_below_10() {
        assert_eq!(sum_primes_below(10), 17);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "142913828922");
    }
}
