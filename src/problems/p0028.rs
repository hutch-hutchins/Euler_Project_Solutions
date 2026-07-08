const SIZE: u64 = 1_001;

pub fn solve() -> String {
    spiral_diagonal_sum(SIZE).to_string()
}

fn spiral_diagonal_sum(size: u64) -> u64 {
    assert!(size % 2 == 1, "spiral size must be odd");

    let mut sum = 1;

    for side in (3..=size).step_by(2) {
        sum += 4 * side * side - 6 * (side - 1);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_five_by_five_spiral() {
        assert_eq!(spiral_diagonal_sum(5), 101);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "669171001");
    }
}
