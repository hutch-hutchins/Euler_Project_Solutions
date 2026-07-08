pub fn divisor_count(mut n: u64) -> u64 {
    if n == 1 {
        return 1;
    }

    let mut count = 1;
    let mut exponent = 0;

    while n % 2 == 0 {
        exponent += 1;
        n /= 2;
    }

    if exponent > 0 {
        count *= exponent + 1;
    }

    let mut factor = 3;
    while factor <= n / factor {
        exponent = 0;

        while n % factor == 0 {
            exponent += 1;
            n /= factor;
        }

        if exponent > 0 {
            count *= exponent + 1;
        }

        factor += 2;
    }

    if n > 1 {
        count *= 2;
    }

    count
}

pub fn proper_divisor_sum(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }

    let mut sum = 1;
    let mut factor = 2;

    while factor <= n / factor {
        if n % factor == 0 {
            sum += factor;
            let pair = n / factor;
            if pair != factor {
                sum += pair;
            }
        }
        factor += 1;
    }

    sum
}

pub fn proper_divisor_sums_through(limit: usize) -> Vec<u64> {
    let mut sums = vec![0; limit + 1];

    for divisor in 1..=limit / 2 {
        for multiple in (divisor * 2..=limit).step_by(divisor) {
            sums[multiple] += divisor as u64;
        }
    }

    sums
}

pub fn distinct_prime_factor_counts_through(limit: usize) -> Vec<usize> {
    let mut counts = vec![0; limit + 1];

    for n in 2..=limit {
        if counts[n] == 0 {
            for multiple in (n..=limit).step_by(n) {
                counts[multiple] += 1;
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_divisors() {
        assert_eq!(divisor_count(1), 1);
        assert_eq!(divisor_count(28), 6);
        assert_eq!(divisor_count(36), 9);
    }

    #[test]
    fn sums_proper_divisors() {
        assert_eq!(proper_divisor_sum(1), 0);
        assert_eq!(proper_divisor_sum(220), 284);
        assert_eq!(proper_divisor_sum(284), 220);
    }

    #[test]
    fn sieves_proper_divisor_sums() {
        let sums = proper_divisor_sums_through(10);
        assert_eq!(sums[1], 0);
        assert_eq!(sums[6], 6);
        assert_eq!(sums[10], 8);
    }

    #[test]
    fn counts_distinct_prime_factors() {
        let counts = distinct_prime_factor_counts_through(20);
        assert_eq!(counts[14], 2);
        assert_eq!(counts[15], 2);
        assert_eq!(counts[16], 1);
    }
}
