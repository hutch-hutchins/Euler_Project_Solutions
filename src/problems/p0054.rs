use std::{cmp::Ordering, collections::HashMap};

const POKER_HANDS: &str = include_str!("../../data/p0054_poker.txt");

#[derive(Debug, Eq, PartialEq)]
struct HandScore {
    category: u8,
    tie_breakers: Vec<u8>,
}

impl Ord for HandScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.category
            .cmp(&other.category)
            .then_with(|| self.tie_breakers.cmp(&other.tie_breakers))
    }
}

impl PartialOrd for HandScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
struct Card {
    value: u8,
    suit: u8,
}

pub fn solve() -> String {
    player_one_win_count(POKER_HANDS).to_string()
}

fn player_one_win_count(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| player_one_wins(line))
        .count()
}

fn player_one_wins(line: &str) -> bool {
    let cards: Vec<Card> = line.split_whitespace().map(parse_card).collect();
    assert_eq!(cards.len(), 10);

    score_hand(&cards[..5]) > score_hand(&cards[5..])
}

fn parse_card(card: &str) -> Card {
    let bytes = card.as_bytes();
    assert_eq!(bytes.len(), 2);

    Card {
        value: match bytes[0] {
            b'2'..=b'9' => bytes[0] - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => panic!("invalid card value: {card}"),
        },
        suit: bytes[1],
    }
}

fn score_hand(cards: &[Card]) -> HandScore {
    assert_eq!(cards.len(), 5);

    let mut values: Vec<u8> = cards.iter().map(|card| card.value).collect();
    values.sort_unstable_by(|left, right| right.cmp(left));

    let is_flush = cards.iter().all(|card| card.suit == cards[0].suit);
    let straight_high = straight_high_card(&values);

    let mut counts_by_value = HashMap::new();
    for &value in &values {
        *counts_by_value.entry(value).or_insert(0_u8) += 1;
    }

    let mut groups: Vec<(u8, u8)> = counts_by_value
        .into_iter()
        .map(|(value, count)| (count, value))
        .collect();
    groups.sort_unstable_by(|left, right| right.cmp(left));

    if let (true, Some(high)) = (is_flush, straight_high) {
        return hand_score(8, vec![high]);
    }

    match groups.as_slice() {
        [(4, four), (1, kicker)] => hand_score(7, vec![*four, *kicker]),
        [(3, triple), (2, pair)] => hand_score(6, vec![*triple, *pair]),
        _ if is_flush => hand_score(5, values),
        _ if straight_high.is_some() => {
            hand_score(4, vec![straight_high.expect("straight high exists")])
        }
        [(3, triple), (1, kicker1), (1, kicker2)] => {
            hand_score(3, vec![*triple, *kicker1, *kicker2])
        }
        [(2, high_pair), (2, low_pair), (1, kicker)] => {
            hand_score(2, vec![*high_pair, *low_pair, *kicker])
        }
        [(2, pair), (1, kicker1), (1, kicker2), (1, kicker3)] => {
            hand_score(1, vec![*pair, *kicker1, *kicker2, *kicker3])
        }
        _ => hand_score(0, values),
    }
}

fn straight_high_card(values_desc: &[u8]) -> Option<u8> {
    let mut unique = values_desc.to_vec();
    unique.dedup();

    if unique.len() != 5 {
        return None;
    }

    if unique == [14, 5, 4, 3, 2] {
        return Some(5);
    }

    unique
        .windows(2)
        .all(|window| window[0] == window[1] + 1)
        .then_some(unique[0])
}

fn hand_score(category: u8, tie_breakers: Vec<u8>) -> HandScore {
    HandScore {
        category,
        tie_breakers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_examples() {
        assert!(!player_one_wins("5H 5C 6S 7S KD 2C 3S 8S 8D TD"));
        assert!(player_one_wins("5D 8C 9S JS AC 2C 5C 7D 8S QH"));
        assert!(!player_one_wins("2D 9C AS AH AC 3D 6D 7D TD QD"));
        assert!(player_one_wins("4D 6S 9H QH QC 3D 6D 7H QD QS"));
        assert!(player_one_wins("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D"));
    }

    #[test]
    fn data_file_has_expected_shape() {
        assert_eq!(POKER_HANDS.lines().count(), 1_000);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "376");
    }
}
