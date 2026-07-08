pub fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1 % modulus;
    base %= modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }

        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_modular_powers() {
        assert_eq!(mod_pow(2, 10, 1_000), 24);
        assert_eq!(mod_pow(10, 10, 1), 0);
    }
}
