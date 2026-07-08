use crate::util::{math::is_square, primes::is_prime};

pub fn solve() -> String {
    smallest_goldbach_counterexample().to_string()
}

fn smallest_goldbach_counterexample() -> u64 {
    let mut n = 9;

    loop {
        if !is_prime(n) && !matches_goldbach_other_conjecture(n) {
            return n;
        }
        n += 2;
    }
}

fn matches_goldbach_other_conjecture(n: u64) -> bool {
    (2..n)
        .filter(|&candidate| is_prime(candidate))
        .any(|prime| {
            let remainder = n - prime;
            remainder % 2 == 0 && is_square(remainder / 2)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_match_conjecture() {
        assert!(matches_goldbach_other_conjecture(9));
        assert!(matches_goldbach_other_conjecture(15));
        assert!(matches_goldbach_other_conjecture(33));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "5777");
    }
}
