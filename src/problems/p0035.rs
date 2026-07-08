use crate::util::{digits::rotations, primes::primes_below};

const LIMIT: usize = 1_000_000;

pub fn solve() -> String {
    circular_prime_count_below(LIMIT).to_string()
}

fn circular_prime_count_below(limit: usize) -> usize {
    let primes = primes_below(limit);
    let mut is_prime = vec![false; limit];

    for &prime in &primes {
        is_prime[prime] = true;
    }

    primes
        .into_iter()
        .filter(|&prime| {
            rotations(prime as u64)
                .into_iter()
                .all(|n| n < limit as u64 && is_prime[n as usize])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_below_100() {
        assert_eq!(circular_prime_count_below(100), 13);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "55");
    }
}
