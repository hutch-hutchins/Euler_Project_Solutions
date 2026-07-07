const DIGITS: u32 = 3;

pub fn solve() -> String {
    largest_palindrome_product(DIGITS).to_string()
}

fn largest_palindrome_product(digits: u32) -> u64 {
    let min = 10_u64.pow(digits - 1);
    let max = 10_u64.pow(digits) - 1;
    let mut best = 0;

    for a in (min..=max).rev() {
        if a * max <= best {
            break;
        }

        for b in (min..=a).rev() {
            let product = a * b;
            if product <= best {
                break;
            }

            if is_palindrome(product) {
                best = product;
                break;
            }
        }
    }

    best
}

fn is_palindrome(n: u64) -> bool {
    n == reverse_digits(n)
}

fn reverse_digits(mut n: u64) -> u64 {
    let mut reversed = 0;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_two_digit_products() {
        assert_eq!(largest_palindrome_product(2), 9_009);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "906609");
    }
}
