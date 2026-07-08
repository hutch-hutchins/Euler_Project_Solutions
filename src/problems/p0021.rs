use crate::util::factors::{proper_divisor_sum, proper_divisor_sums_through};

const LIMIT: usize = 10_000;

pub fn solve() -> String {
    amicable_sum_below(LIMIT).to_string()
}

fn amicable_sum_below(limit: usize) -> u64 {
    let sums = proper_divisor_sums_through(limit);
    let mut total = 0;

    for a in 2..limit {
        let b = sums[a];
        if b != a as u64 && proper_divisor_sum(b) == a as u64 {
            total += a as u64;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_pair_is_amicable() {
        let sums = proper_divisor_sums_through(300);
        assert_eq!(sums[220], 284);
        assert_eq!(sums[284], 220);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "31626");
    }
}
