const CIPHER: &str = include_str!("../../data/p0059_cipher.txt");

pub fn solve() -> String {
    decrypted_ascii_sum(CIPHER).to_string()
}

fn decrypted_ascii_sum(input: &str) -> u64 {
    let cipher = parse_cipher(input);
    let (_key, plain_text) = decrypt_best_english_text(&cipher);
    plain_text.iter().map(|byte| u64::from(*byte)).sum()
}

fn parse_cipher(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(|value| value.parse().expect("cipher values should be bytes"))
        .collect()
}

fn decrypt_best_english_text(cipher: &[u8]) -> ([u8; 3], Vec<u8>) {
    let mut best_score = i64::MIN;
    let mut best_key = [b'a'; 3];
    let mut best_plain_text = Vec::new();

    for first in b'a'..=b'z' {
        for second in b'a'..=b'z' {
            for third in b'a'..=b'z' {
                let key = [first, second, third];
                let plain_text = decrypt(cipher, key);
                let score = english_score(&plain_text);

                if score > best_score {
                    best_score = score;
                    best_key = key;
                    best_plain_text = plain_text;
                }
            }
        }
    }

    (best_key, best_plain_text)
}

fn decrypt(cipher: &[u8], key: [u8; 3]) -> Vec<u8> {
    cipher
        .iter()
        .enumerate()
        .map(|(index, byte)| byte ^ key[index % key.len()])
        .collect()
}

fn english_score(text: &[u8]) -> i64 {
    if text
        .iter()
        .any(|byte| !matches!(*byte, b'\n' | b'\r' | b'\t' | 32..=126))
    {
        return i64::MIN / 2;
    }

    let lower = String::from_utf8_lossy(text).to_ascii_lowercase();
    let common_words = [" the ", " and ", " of ", " to ", " in ", " that ", " is "];
    let word_score: i64 = common_words
        .iter()
        .map(|word| lower.matches(word).count() as i64)
        .sum::<i64>()
        * 100;
    let space_score = text.iter().filter(|&&byte| byte == b' ').count() as i64;
    let uncommon_score = text
        .iter()
        .filter(|&&byte| !(byte.is_ascii_alphanumeric() || b" .,;:'\"?!()-\n\r".contains(&byte)))
        .count() as i64;

    word_score + space_score - uncommon_score * 20
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_key() {
        let cipher = parse_cipher(CIPHER);
        let (key, plain_text) = decrypt_best_english_text(&cipher);
        assert_eq!(&key, b"exp");
        assert!(String::from_utf8_lossy(&plain_text).contains("Euler's most celebrated papers"));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "129448");
    }
}
