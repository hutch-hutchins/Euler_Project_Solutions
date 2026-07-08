const TARGET: usize = 200;
const COINS: &[usize] = &[1, 2, 5, 10, 20, 50, 100, 200];

pub fn solve() -> String {
    coin_sum_count(TARGET, COINS).to_string()
}

fn coin_sum_count(target: usize, coins: &[usize]) -> u64 {
    let mut ways = vec![0_u64; target + 1];
    ways[0] = 1;

    for &coin in coins {
        for amount in coin..=target {
            ways[amount] += ways[amount - coin];
        }
    }

    ways[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_five_pence() {
        assert_eq!(coin_sum_count(5, &[1, 2, 5]), 4);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "73682");
    }
}
