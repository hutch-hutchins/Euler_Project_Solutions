use crate::util::{digits::decimal_digits, primes::primes_below};

const TARGET_FAMILY_SIZE: usize = 8;
const SEARCH_LIMIT: usize = 1_000_000;

pub fn solve() -> String {
    smallest_prime_in_family(TARGET_FAMILY_SIZE).to_string()
}

fn smallest_prime_in_family(target_family_size: usize) -> u64 {
    let primes = primes_below(SEARCH_LIMIT);
    let mut is_prime = vec![false; SEARCH_LIMIT];

    for &prime in &primes {
        is_prime[prime] = true;
    }

    for prime in primes.into_iter().filter(|&prime| prime >= 10) {
        if has_replacement_family(prime as u64, target_family_size, &is_prime) {
            return prime as u64;
        }
    }

    panic!("increase SEARCH_LIMIT");
}

fn has_replacement_family(prime: u64, target_family_size: usize, is_prime: &[bool]) -> bool {
    let digits = decimal_digits(prime);
    let mask_limit = 1_u32 << digits.len();

    for mask in 1..mask_limit {
        if !masked_digits_match(&digits, mask) {
            continue;
        }

        let mut family_size = 0;

        for replacement in 0..=9 {
            if mask & 1 != 0 && replacement == 0 {
                continue;
            }

            let candidate = replace_masked_digits(&digits, mask, replacement);
            if candidate < is_prime.len() as u64 && is_prime[candidate as usize] {
                family_size += 1;
            }
        }

        if family_size >= target_family_size {
            return true;
        }
    }

    false
}

fn masked_digits_match(digits: &[u64], mask: u32) -> bool {
    let mut selected = None;

    for (index, &digit) in digits.iter().enumerate() {
        if mask & (1 << index) == 0 {
            continue;
        }

        match selected {
            Some(previous) if previous != digit => return false,
            Some(_) => {}
            None => selected = Some(digit),
        }
    }

    true
}

fn replace_masked_digits(digits: &[u64], mask: u32, replacement: u64) -> u64 {
    digits.iter().enumerate().fold(0, |value, (index, &digit)| {
        let next_digit = if mask & (1 << index) != 0 {
            replacement
        } else {
            digit
        };
        value * 10 + next_digit
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let primes = primes_below(100_000);
        let mut is_prime = vec![false; 100_000];
        for prime in primes {
            is_prime[prime] = true;
        }

        assert!(has_replacement_family(13, 6, &is_prime));
        assert!(has_replacement_family(56_003, 7, &is_prime));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "121313");
    }
}
