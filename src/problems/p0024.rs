const TARGET_ORDINAL: usize = 1_000_000;

pub fn solve() -> String {
    nth_lexicographic_permutation((0..=9).collect(), TARGET_ORDINAL)
}

fn nth_lexicographic_permutation(mut digits: Vec<u8>, ordinal: usize) -> String {
    let mut index = ordinal - 1;
    let mut result = String::new();

    while !digits.is_empty() {
        let block_size = factorial(digits.len() - 1);
        let selected = index / block_size;
        index %= block_size;
        result.push(char::from(b'0' + digits.remove(selected)));
    }

    result
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_permutations_of_three_digits() {
        assert_eq!(nth_lexicographic_permutation(vec![0, 1, 2], 1), "012");
        assert_eq!(nth_lexicographic_permutation(vec![0, 1, 2], 6), "210");
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "2783915460");
    }
}
