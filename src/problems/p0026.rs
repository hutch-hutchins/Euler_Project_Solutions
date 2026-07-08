const LIMIT: usize = 1_000;

pub fn solve() -> String {
    denominator_with_longest_cycle_below(LIMIT).to_string()
}

fn denominator_with_longest_cycle_below(limit: usize) -> usize {
    (2..limit)
        .max_by_key(|&denominator| recurring_cycle_length(denominator))
        .expect("range should not be empty")
}

fn recurring_cycle_length(denominator: usize) -> usize {
    let mut seen_at = vec![None; denominator];
    let mut remainder = 1 % denominator;
    let mut position = 0;

    while remainder != 0 && seen_at[remainder].is_none() {
        seen_at[remainder] = Some(position);
        remainder = (remainder * 10) % denominator;
        position += 1;
    }

    match seen_at[remainder] {
        Some(first_seen) => position - first_seen,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_seventh_cycle() {
        assert_eq!(recurring_cycle_length(7), 6);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "983");
    }
}
