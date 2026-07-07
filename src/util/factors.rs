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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_divisors() {
        assert_eq!(divisor_count(1), 1);
        assert_eq!(divisor_count(28), 6);
        assert_eq!(divisor_count(36), 9);
    }
}
