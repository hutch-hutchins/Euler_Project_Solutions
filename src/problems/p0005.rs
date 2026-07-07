use crate::util::math::lcm;

const MAX_DIVISOR: u64 = 20;

pub fn solve() -> String {
    smallest_multiple_through(MAX_DIVISOR).to_string()
}

fn smallest_multiple_through(max_divisor: u64) -> u64 {
    (1..=max_divisor).fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_through_10() {
        assert_eq!(smallest_multiple_through(10), 2_520);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "232792560");
    }
}
