pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut factor = 3;
    while factor <= n / factor {
        if n % factor == 0 {
            return false;
        }
        factor += 2;
    }

    true
}

pub fn nth_prime(n: usize) -> u64 {
    assert!(n > 0, "prime ordinals start at 1");

    let mut found = 0;
    let mut candidate = 1;

    loop {
        candidate += 1;
        if is_prime(candidate) {
            found += 1;
            if found == n {
                return candidate;
            }
        }
    }
}

pub fn primes_below(limit: usize) -> Vec<usize> {
    if limit <= 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut prime = 2;
    while prime <= (limit - 1) / prime {
        if is_prime[prime] {
            for multiple in (prime * prime..limit).step_by(prime) {
                is_prime[multiple] = false;
            }
        }
        prime += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(value, is_prime)| is_prime.then_some(value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primality_checks() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(29));
        assert!(!is_prime(49));
    }

    #[test]
    fn ordinal_prime() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn sieve_below_10() {
        assert_eq!(primes_below(10), vec![2, 3, 5, 7]);
    }
}
