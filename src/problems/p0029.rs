use std::collections::HashSet;

use crate::util::decimal::{from_u64, multiply_by_u32, to_string};

const MIN_BASE: u32 = 2;
const MAX_BASE: u32 = 100;
const MIN_EXPONENT: u32 = 2;
const MAX_EXPONENT: u32 = 100;

pub fn solve() -> String {
    distinct_power_count(MIN_BASE, MAX_BASE, MIN_EXPONENT, MAX_EXPONENT).to_string()
}

fn distinct_power_count(
    min_base: u32,
    max_base: u32,
    min_exponent: u32,
    max_exponent: u32,
) -> usize {
    let mut terms = HashSet::new();

    for base in min_base..=max_base {
        let mut digits = from_u64(u64::from(base));

        for exponent in 2..=max_exponent {
            multiply_by_u32(&mut digits, base);

            if exponent >= min_exponent {
                terms.insert(to_string(&digits));
            }
        }
    }

    terms.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_two_through_five() {
        assert_eq!(distinct_power_count(2, 5, 2, 5), 15);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "9183");
    }
}
