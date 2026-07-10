const LIMIT: u128 = 1_000_000;
const MAX_N: u64 = 100;

pub fn solve() -> String {
    combinatoric_selection_count(MAX_N, LIMIT).to_string()
}

fn combinatoric_selection_count(max_n: u64, limit: u128) -> u64 {
    let mut count = 0;

    for n in 1..=max_n {
        for r in 0..=n {
            if binomial_exceeds(n, r, limit) {
                count += 1;
            }
        }
    }

    count
}

fn binomial_exceeds(n: u64, r: u64, limit: u128) -> bool {
    let r = r.min(n - r);
    let mut value = 1_u128;

    for i in 1..=r {
        value = value * u128::from(n - r + i) / u128::from(i);
        if value > limit {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_23_choose_10() {
        assert!(binomial_exceeds(23, 10, LIMIT));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "4075");
    }
}
