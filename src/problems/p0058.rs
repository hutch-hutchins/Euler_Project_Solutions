use crate::util::primes::is_prime;

const THRESHOLD_NUMERATOR: u64 = 1;
const THRESHOLD_DENOMINATOR: u64 = 10;

pub fn solve() -> String {
    spiral_side_length_below_prime_ratio(THRESHOLD_NUMERATOR, THRESHOLD_DENOMINATOR).to_string()
}

fn spiral_side_length_below_prime_ratio(
    threshold_numerator: u64,
    threshold_denominator: u64,
) -> u64 {
    let mut prime_count = 0;
    let mut diagonal_count = 1;
    let mut side_length = 1;

    loop {
        side_length += 2;
        let square = side_length * side_length;
        let step = side_length - 1;

        for corner in 0..4 {
            if is_prime(square - corner * step) {
                prime_count += 1;
            }
        }

        diagonal_count += 4;

        if prime_count * threshold_denominator < diagonal_count * threshold_numerator {
            return side_length;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ratio_below_sixty_percent() {
        assert_eq!(spiral_side_length_below_prime_ratio(6, 10), 5);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "26241");
    }
}
