const LIMIT: u32 = 1_000;

pub fn solve() -> String {
    sum_multiples_below(LIMIT).to_string()
}

fn sum_multiples_below(limit: u32) -> u32 {
    (1..limit).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_below_10() {
        assert_eq!(sum_multiples_below(10), 23);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "233168");
    }
}
