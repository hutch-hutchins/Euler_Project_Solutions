pub fn decimal_digits(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.reverse();
    digits
}

pub fn digit_power_sum(n: u64, power: u32) -> u64 {
    decimal_digits(n)
        .into_iter()
        .map(|digit| digit.pow(power))
        .sum()
}

pub fn digits_to_number(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |value, digit| value * 10 + digit)
}

pub fn rotations(n: u64) -> Vec<u64> {
    let digits = decimal_digits(n);
    let mut values = Vec::with_capacity(digits.len());

    for offset in 0..digits.len() {
        let mut rotated = Vec::with_capacity(digits.len());
        rotated.extend_from_slice(&digits[offset..]);
        rotated.extend_from_slice(&digits[..offset]);
        values.push(digits_to_number(&rotated));
    }

    values
}

pub fn is_palindrome_base(mut n: u64, base: u64) -> bool {
    assert!(base >= 2, "base must be at least 2");

    let original = n;
    let mut reversed = 0;

    while n > 0 {
        reversed = reversed * base + n % base;
        n /= base;
    }

    original == reversed
}

pub fn is_pandigital_1_to_9(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }

    let mut seen = [false; 10];

    for byte in value.bytes() {
        if !(b'1'..=b'9').contains(&byte) {
            return false;
        }

        let digit = (byte - b'0') as usize;
        if seen[digit] {
            return false;
        }
        seen[digit] = true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_decimal_digits() {
        assert_eq!(decimal_digits(0), vec![0]);
        assert_eq!(decimal_digits(12_345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sums_digit_powers() {
        assert_eq!(digit_power_sum(1634, 4), 1634);
    }

    #[test]
    fn converts_digits_to_number() {
        assert_eq!(digits_to_number(&[1, 2, 3, 4, 5]), 12_345);
        assert_eq!(digits_to_number(&[0, 1, 1]), 11);
    }

    #[test]
    fn rotates_digits() {
        assert_eq!(rotations(197), vec![197, 971, 719]);
    }

    #[test]
    fn checks_palindromes_in_arbitrary_base() {
        assert!(is_palindrome_base(585, 10));
        assert!(is_palindrome_base(585, 2));
        assert!(!is_palindrome_base(586, 10));
    }

    #[test]
    fn checks_one_to_nine_pandigital_strings() {
        assert!(is_pandigital_1_to_9("123456789"));
        assert!(!is_pandigital_1_to_9("123456788"));
        assert!(!is_pandigital_1_to_9("012345678"));
    }
}
