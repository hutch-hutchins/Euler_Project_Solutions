use crate::util::polygonal::{hexagonal, is_pentagonal};

const FIRST_AFTER_KNOWN_HEXAGONAL_INDEX: u64 = 144;

pub fn solve() -> String {
    next_triangular_pentagonal_hexagonal().to_string()
}

fn next_triangular_pentagonal_hexagonal() -> u64 {
    let mut n = FIRST_AFTER_KNOWN_HEXAGONAL_INDEX;

    loop {
        let value = hexagonal(n);
        if is_pentagonal(value) {
            return value;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_value() {
        assert!(is_pentagonal(hexagonal(143)));
        assert_eq!(hexagonal(143), 40_755);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "1533776805");
    }
}
