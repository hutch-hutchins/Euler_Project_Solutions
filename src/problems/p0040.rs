const POSITIONS: &[usize] = &[1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

pub fn solve() -> String {
    champernowne_digit_product(POSITIONS).to_string()
}

fn champernowne_digit_product(positions: &[usize]) -> u64 {
    positions
        .iter()
        .map(|&position| u64::from(champernowne_digit(position)))
        .product()
}

fn champernowne_digit(mut position: usize) -> u8 {
    let mut digits_per_number = 1;
    let mut count = 9;
    let mut start = 1;

    while position > digits_per_number * count {
        position -= digits_per_number * count;
        digits_per_number += 1;
        count *= 10;
        start *= 10;
    }

    let number = start + (position - 1) / digits_per_number;
    let digit_index = (position - 1) % digits_per_number;
    number.to_string().as_bytes()[digit_index] - b'0'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(champernowne_digit(1), 1);
        assert_eq!(champernowne_digit(10), 1);
        assert_eq!(champernowne_digit(12), 1);
        assert_eq!(champernowne_digit(15), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "210");
    }
}
