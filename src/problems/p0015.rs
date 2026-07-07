const GRID_SIZE: u128 = 20;

pub fn solve() -> String {
    square_grid_routes(GRID_SIZE).to_string()
}

fn square_grid_routes(size: u128) -> u128 {
    binomial(2 * size, size)
}

fn binomial(n: u128, k: u128) -> u128 {
    let k = k.min(n - k);
    let mut result = 1;

    for i in 1..=k {
        result = result * (n - k + i) / i;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_two_by_two_grid() {
        assert_eq!(square_grid_routes(2), 6);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "137846528820");
    }
}
