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
}
