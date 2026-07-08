pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

pub fn integer_sqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut low = 1;
    let mut high = n / 2 + 1;
    let mut answer = 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if mid <= n / mid {
            answer = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    answer
}

pub fn is_square(n: u64) -> bool {
    let root = integer_sqrt(n);
    root * root == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greatest_common_divisor() {
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn least_common_multiple() {
        assert_eq!(lcm(21, 6), 42);
    }

    #[test]
    fn square_helpers() {
        assert_eq!(integer_sqrt(15), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert!(is_square(16));
        assert!(!is_square(17));
    }
}
