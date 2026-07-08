use crate::util::math::gcd;

pub fn solve() -> String {
    digit_cancelling_denominator().to_string()
}

fn digit_cancelling_denominator() -> u64 {
    let mut numerator_product = 1;
    let mut denominator_product = 1;

    for numerator in 10..100 {
        for denominator in numerator + 1..100 {
            if numerator % 10 == 0 && denominator % 10 == 0 {
                continue;
            }

            if is_digit_cancelling_fraction(numerator, denominator) {
                numerator_product *= numerator;
                denominator_product *= denominator;
            }
        }
    }

    let common = gcd(numerator_product, denominator_product);
    denominator_product / common
}

fn is_digit_cancelling_fraction(numerator: u64, denominator: u64) -> bool {
    let numerator_digits = [numerator / 10, numerator % 10];
    let denominator_digits = [denominator / 10, denominator % 10];

    for numerator_index in 0..2 {
        for denominator_index in 0..2 {
            if numerator_digits[numerator_index] != denominator_digits[denominator_index] {
                continue;
            }

            let reduced_numerator = numerator_digits[1 - numerator_index];
            let reduced_denominator = denominator_digits[1 - denominator_index];

            if reduced_denominator != 0
                && numerator * reduced_denominator == denominator * reduced_numerator
            {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_fraction() {
        assert!(is_digit_cancelling_fraction(49, 98));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "100");
    }
}
