use std::collections::HashSet;

use crate::util::digits::is_pandigital_1_to_9;

pub fn solve() -> String {
    pandigital_product_sum().to_string()
}

fn pandigital_product_sum() -> u64 {
    let mut products = HashSet::new();

    for multiplicand in 1..100 {
        for multiplier in 1..10_000 {
            let product = multiplicand * multiplier;
            let identity = format!("{multiplicand}{multiplier}{product}");

            if identity.len() > 9 {
                break;
            }

            if is_pandigital_1_to_9(&identity) {
                products.insert(product);
            }
        }
    }

    products.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_identity_is_pandigital() {
        assert!(is_pandigital_1_to_9("391867254"));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "45228");
    }
}
