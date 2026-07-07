const LIMIT: u32 = 4_000_000;

pub fn solve() -> String {
    sum_even_fibonacci_below(LIMIT).to_string()
}

fn sum_even_fibonacci_below(limit: u32) -> u32 {
    let mut sum = 0;
    let mut previous = 1;
    let mut current = 2;

    while current < limit {
        if current % 2 == 0 {
            sum += current;
        }

        let next = previous + current;
        previous = current;
        current = next;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_below_90() {
        assert_eq!(sum_even_fibonacci_below(90), 44);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "4613732");
    }
}
