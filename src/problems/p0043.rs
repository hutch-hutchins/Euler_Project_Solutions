use crate::util::digits::digits_to_number;

const DIVISORS: &[u64] = &[2, 3, 5, 7, 11, 13, 17];

pub fn solve() -> String {
    substring_divisibility_sum().to_string()
}

fn substring_divisibility_sum() -> u64 {
    let mut used = [false; 10];
    let mut digits = Vec::with_capacity(10);
    let mut sum = 0;

    search(&mut digits, &mut used, &mut sum);

    sum
}

fn search(digits: &mut Vec<u64>, used: &mut [bool; 10], sum: &mut u64) {
    if digits.len() == 10 {
        *sum += digits_to_number(digits);
        return;
    }

    for digit in 0..=9 {
        if used[digit] || (digits.is_empty() && digit == 0) {
            continue;
        }

        digits.push(digit as u64);

        if has_valid_suffix(digits) {
            used[digit] = true;
            search(digits, used, sum);
            used[digit] = false;
        }

        digits.pop();
    }
}

fn has_valid_suffix(digits: &[u64]) -> bool {
    if digits.len() < 4 {
        return true;
    }

    let divisor = DIVISORS[digits.len() - 4];
    let start = digits.len() - 3;
    let value = digits[start] * 100 + digits[start + 1] * 10 + digits[start + 2];

    value % divisor == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_number_has_property() {
        let digits = vec![1, 4, 0, 6, 3, 5, 7, 2, 8, 9];
        assert!((4..=10).all(|len| has_valid_suffix(&digits[..len])));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "16695334890");
    }
}
