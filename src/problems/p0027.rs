use crate::util::primes::is_prime;

const COEFFICIENT_LIMIT: i64 = 1_000;

pub fn solve() -> String {
    best_coefficient_product(COEFFICIENT_LIMIT).to_string()
}

fn best_coefficient_product(limit: i64) -> i64 {
    let mut best_product = 0;
    let mut best_length = 0;

    for a in (-(limit - 1))..limit {
        for b in (-limit)..=limit {
            let length = consecutive_prime_count(a, b);
            if length > best_length {
                best_length = length;
                best_product = a * b;
            }
        }
    }

    best_product
}

fn consecutive_prime_count(a: i64, b: i64) -> u64 {
    let mut n = 0;

    loop {
        let value = n * n + a * n + b;
        if value < 2 || !is_prime(value as u64) {
            return n as u64;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(consecutive_prime_count(1, 41), 40);
        assert_eq!(consecutive_prime_count(-79, 1601), 80);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "-59231");
    }
}
