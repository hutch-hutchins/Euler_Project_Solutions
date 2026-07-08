use crate::util::factors::proper_divisor_sums_through;

const LIMIT: usize = 28_123;

pub fn solve() -> String {
    non_abundant_sum_total(LIMIT).to_string()
}

fn non_abundant_sum_total(limit: usize) -> u64 {
    let sums = proper_divisor_sums_through(limit);
    let abundant_numbers: Vec<usize> = (1..=limit).filter(|&n| sums[n] > n as u64).collect();
    let mut can_be_written = vec![false; limit + 1];

    for (index, &left) in abundant_numbers.iter().enumerate() {
        for &right in &abundant_numbers[index..] {
            let sum = left + right;
            if sum > limit {
                break;
            }
            can_be_written[sum] = true;
        }
    }

    (1..=limit)
        .filter(|&n| !can_be_written[n])
        .map(|n| n as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_is_first_abundant_number() {
        let sums = proper_divisor_sums_through(12);
        assert!(sums[12] > 12);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "4179871");
    }
}
