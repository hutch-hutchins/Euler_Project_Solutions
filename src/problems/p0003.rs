const TARGET: u64 = 600_851_475_143;

pub fn solve() -> String {
    largest_prime_factor(TARGET).to_string()
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest = 1;

    while n % 2 == 0 {
        largest = 2;
        n /= 2;
    }

    let mut factor = 3;
    while factor <= n / factor {
        while n % factor == 0 {
            largest = factor;
            n /= factor;
        }
        factor += 2;
    }

    if n > 1 { n } else { largest }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_factorization() {
        assert_eq!(largest_prime_factor(13_195), 29);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "6857");
    }
}
