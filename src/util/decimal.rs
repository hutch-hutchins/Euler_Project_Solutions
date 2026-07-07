pub fn from_u64(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }

    digits
}

pub fn add_assign_str(digits: &mut Vec<u8>, decimal: &str) {
    let mut carry = 0_u8;

    for (index, ch) in decimal.chars().rev().enumerate() {
        let digit = ch
            .to_digit(10)
            .unwrap_or_else(|| panic!("invalid decimal digit: {ch}")) as u8;

        if index == digits.len() {
            digits.push(0);
        }

        let value = digits[index] + digit + carry;
        digits[index] = value % 10;
        carry = value / 10;
    }

    let mut index = decimal.len();
    while carry > 0 {
        if index == digits.len() {
            digits.push(0);
        }

        let value = digits[index] + carry;
        digits[index] = value % 10;
        carry = value / 10;
        index += 1;
    }

    trim_leading_zeroes(digits);
}

pub fn multiply_by_u32(digits: &mut Vec<u8>, multiplier: u32) {
    if multiplier == 0 {
        digits.clear();
        digits.push(0);
        return;
    }

    let mut carry = 0_u32;

    for digit in digits.iter_mut() {
        let value = u32::from(*digit) * multiplier + carry;
        *digit = (value % 10) as u8;
        carry = value / 10;
    }

    while carry > 0 {
        digits.push((carry % 10) as u8);
        carry /= 10;
    }

    trim_leading_zeroes(digits);
}

pub fn digit_sum(digits: &[u8]) -> u64 {
    digits.iter().map(|digit| u64::from(*digit)).sum()
}

pub fn to_string(digits: &[u8]) -> String {
    digits
        .iter()
        .rev()
        .map(|digit| char::from(b'0' + *digit))
        .collect()
}

fn trim_leading_zeroes(digits: &mut Vec<u8>) {
    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_decimal_strings() {
        let mut digits = from_u64(0);
        add_assign_str(&mut digits, "999");
        add_assign_str(&mut digits, "1");
        assert_eq!(to_string(&digits), "1000");
    }

    #[test]
    fn multiply_decimal_digits() {
        let mut digits = from_u64(123);
        multiply_by_u32(&mut digits, 45);
        assert_eq!(to_string(&digits), "5535");
    }

    #[test]
    fn sums_digits() {
        assert_eq!(digit_sum(&from_u64(32_768)), 26);
    }
}
