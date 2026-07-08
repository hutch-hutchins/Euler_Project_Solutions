use crate::util::math::{integer_sqrt, is_square};

pub fn triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

pub fn hexagonal(n: u64) -> u64 {
    n * (2 * n - 1)
}

pub fn is_triangle(value: u64) -> bool {
    let discriminant = 8 * value + 1;
    is_square(discriminant) && integer_sqrt(discriminant) % 2 == 1
}

pub fn is_pentagonal(value: u64) -> bool {
    let discriminant = 24 * value + 1;
    is_square(discriminant) && (1 + integer_sqrt(discriminant)) % 6 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_polygonal_numbers() {
        assert_eq!(triangle(7), 28);
        assert_eq!(pentagonal(7), 70);
        assert_eq!(hexagonal(7), 91);
    }

    #[test]
    fn recognizes_polygonal_numbers() {
        assert!(is_triangle(55));
        assert!(!is_triangle(56));
        assert!(is_pentagonal(70));
        assert!(!is_pentagonal(71));
    }
}
