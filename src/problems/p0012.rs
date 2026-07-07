use crate::util::factors::divisor_count;

const MIN_DIVISORS: u64 = 500;

pub fn solve() -> String {
    first_triangular_number_with_more_than(MIN_DIVISORS).to_string()
}

fn first_triangular_number_with_more_than(min_divisors: u64) -> u64 {
    let mut index = 1;
    let mut triangle = 1;

    while divisor_count(triangle) <= min_divisors {
        index += 1;
        triangle += index;
    }

    triangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_more_than_five_divisors() {
        assert_eq!(first_triangular_number_with_more_than(5), 28);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "76576500");
    }
}
