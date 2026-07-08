use crate::util::{digits::digits_to_number, primes::is_prime};

pub fn solve() -> String {
    largest_pandigital_prime().to_string()
}

fn largest_pandigital_prime() -> u64 {
    for digit_count in (1..=9).rev() {
        if digit_count * (digit_count + 1) / 2 % 3 == 0 {
            continue;
        }

        let mut digits: Vec<u64> = (1..=digit_count).collect();
        let mut best = 0;
        find_largest_prime_permutation(&mut digits, 0, &mut best);

        if best > 0 {
            return best;
        }
    }

    0
}

fn find_largest_prime_permutation(digits: &mut [u64], index: usize, best: &mut u64) {
    if index == digits.len() {
        let value = digits_to_number(digits);
        if value > *best && is_prime(value) {
            *best = value;
        }
        return;
    }

    for swap_index in index..digits.len() {
        digits.swap(index, swap_index);
        find_largest_prime_permutation(digits, index + 1, best);
        digits.swap(index, swap_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_four_digit_pandigital_prime() {
        assert!(is_prime(2_143));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "7652413");
    }
}
