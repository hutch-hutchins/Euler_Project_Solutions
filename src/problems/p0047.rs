use crate::util::factors::distinct_prime_factor_counts_through;

const TARGET: usize = 4;

pub fn solve() -> String {
    first_consecutive_distinct_prime_factors(TARGET).to_string()
}

fn first_consecutive_distinct_prime_factors(target: usize) -> usize {
    let mut limit = 1_000;

    loop {
        let counts = distinct_prime_factor_counts_through(limit);

        if let Some(start) = first_matching_run(&counts, target) {
            return start;
        }

        limit *= 2;
    }
}

fn first_matching_run(counts: &[usize], target: usize) -> Option<usize> {
    let mut run_length = 0;

    for (n, &count) in counts.iter().enumerate().skip(2) {
        if count == target {
            run_length += 1;
            if run_length == target {
                return Some(n + 1 - target);
            }
        } else {
            run_length = 0;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(first_consecutive_distinct_prime_factors(2), 14);
        assert_eq!(first_consecutive_distinct_prime_factors(3), 644);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "134043");
    }
}
