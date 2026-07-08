use crate::util::digits::is_pandigital_1_to_9;

pub fn solve() -> String {
    largest_pandigital_concatenated_product().to_string()
}

fn largest_pandigital_concatenated_product() -> u64 {
    let mut best = 0;

    for base in 1..10_000 {
        let mut concatenated = String::new();
        let mut multiplier = 1;

        while concatenated.len() < 9 {
            concatenated.push_str(&(base * multiplier).to_string());
            multiplier += 1;
        }

        if is_pandigital_1_to_9(&concatenated) {
            best = best.max(concatenated.parse().expect("digits should parse"));
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_192() {
        let concatenated = format!("{}{}{}", 192, 192 * 2, 192 * 3);
        assert_eq!(concatenated, "192384576");
        assert!(is_pandigital_1_to_9(&concatenated));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "932718654");
    }
}
