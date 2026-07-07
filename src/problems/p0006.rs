const LIMIT: u64 = 100;

pub fn solve() -> String {
    sum_square_difference(LIMIT).to_string()
}

fn sum_square_difference(limit: u64) -> u64 {
    let sum = limit * (limit + 1) / 2;
    let sum_of_squares = limit * (limit + 1) * (2 * limit + 1) / 6;
    sum * sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_first_10() {
        assert_eq!(sum_square_difference(10), 2_640);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "25164150");
    }
}
