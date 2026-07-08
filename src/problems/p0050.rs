use crate::util::primes::primes_below;

const LIMIT: usize = 1_000_000;

pub fn solve() -> String {
    consecutive_prime_sum_below(LIMIT).to_string()
}

fn consecutive_prime_sum_below(limit: usize) -> usize {
    let primes = primes_below(limit);
    let mut is_prime = vec![false; limit];

    for &prime in &primes {
        is_prime[prime] = true;
    }

    let mut prefix_sums = Vec::with_capacity(primes.len() + 1);
    prefix_sums.push(0);

    for &prime in &primes {
        prefix_sums.push(prefix_sums.last().expect("prefix sum exists") + prime);
    }

    let mut best_sum = 0;
    let mut best_length = 0;

    for start in 0..prefix_sums.len() {
        for end in start + best_length + 1..prefix_sums.len() {
            let sum = prefix_sums[end] - prefix_sums[start];
            if sum >= limit {
                break;
            }

            if is_prime[sum] {
                best_sum = sum;
                best_length = end - start;
            }
        }
    }

    best_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(consecutive_prime_sum_below(100), 41);
        assert_eq!(consecutive_prime_sum_below(1_000), 953);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "997651");
    }
}
