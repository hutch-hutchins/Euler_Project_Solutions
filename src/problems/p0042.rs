use crate::util::polygonal::is_triangle;

const WORDS: &str = include_str!("../../data/p0042_words.txt");

pub fn solve() -> String {
    triangle_word_count(WORDS).to_string()
}

fn triangle_word_count(input: &str) -> usize {
    parse_words(input)
        .into_iter()
        .filter(|word| is_triangle(word_value(word)))
        .count()
}

fn parse_words(input: &str) -> Vec<&str> {
    input
        .trim()
        .split(',')
        .map(|word| word.trim_matches('"'))
        .collect()
}

fn word_value(word: &str) -> u64 {
    word.bytes().map(|byte| u64::from(byte - b'A' + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_sky() {
        assert_eq!(word_value("SKY"), 55);
        assert!(is_triangle(word_value("SKY")));
    }

    #[test]
    fn data_file_has_expected_shape() {
        let words = parse_words(WORDS);
        assert_eq!(words.len(), 1_786);
        assert!(words.contains(&"SKY"));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "162");
    }
}
