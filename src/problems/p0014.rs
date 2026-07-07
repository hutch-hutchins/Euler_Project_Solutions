use std::collections::HashMap;

const LIMIT: u64 = 1_000_000;

pub fn solve() -> String {
    longest_collatz_start_below(LIMIT).to_string()
}

fn longest_collatz_start_below(limit: u64) -> u64 {
    let mut memo = HashMap::from([(1, 1)]);
    let mut best_start = 1;
    let mut best_length = 1;

    for start in 2..limit {
        let length = collatz_length(start, &mut memo);
        if length > best_length {
            best_start = start;
            best_length = length;
        }
    }

    best_start
}

fn collatz_length(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(length) = memo.get(&n) {
        return *length;
    }

    let next = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    let length = 1 + collatz_length(next, memo);
    memo.insert(n, length);
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_start_13_has_ten_terms() {
        let mut memo = HashMap::from([(1, 1)]);
        assert_eq!(collatz_length(13, &mut memo), 10);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "837799");
    }
}
