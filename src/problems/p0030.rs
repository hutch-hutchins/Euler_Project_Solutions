use crate::util::digits::digit_power_sum;

const POWER: u32 = 5;

pub fn solve() -> String {
    sum_numbers_equal_to_digit_power_sum(POWER).to_string()
}

fn sum_numbers_equal_to_digit_power_sum(power: u32) -> u64 {
    let upper_bound = digit_power_search_upper_bound(power);

    (2..=upper_bound)
        .filter(|&n| n == digit_power_sum(n, power))
        .sum()
}

fn digit_power_search_upper_bound(power: u32) -> u64 {
    let mut digits = 1;
    let ninth_power = 9_u64.pow(power);

    while 10_u64.pow(digits - 1) <= digits as u64 * ninth_power {
        digits += 1;
    }

    (digits as u64 - 1) * ninth_power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_fourth_powers() {
        assert_eq!(sum_numbers_equal_to_digit_power_sum(4), 19_316);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "443839");
    }
}
