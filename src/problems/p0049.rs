use std::collections::{HashMap, HashSet};

use crate::util::{digits::digit_signature, primes::primes_below};

const KNOWN_SEQUENCE_START: usize = 1_487;

pub fn solve() -> String {
    other_prime_permutation_sequence()
}

fn other_prime_permutation_sequence() -> String {
    for sequence in prime_permutation_sequences() {
        if sequence[0] != KNOWN_SEQUENCE_START {
            return format!("{}{}{}", sequence[0], sequence[1], sequence[2]);
        }
    }

    panic!("expected another prime permutation sequence");
}

fn prime_permutation_sequences() -> Vec<[usize; 3]> {
    let mut groups: HashMap<[u8; 10], Vec<usize>> = HashMap::new();

    for prime in primes_below(10_000)
        .into_iter()
        .filter(|&prime| prime >= 1_000)
    {
        groups
            .entry(digit_signature(prime as u64))
            .or_default()
            .push(prime);
    }

    let mut sequences = Vec::new();

    for group in groups.values_mut() {
        group.sort_unstable();
        let members: HashSet<usize> = group.iter().copied().collect();

        for (left_index, &left) in group.iter().enumerate() {
            for &middle in &group[left_index + 1..] {
                let right = 2 * middle - left;
                if members.contains(&right) {
                    sequences.push([left, middle, right]);
                }
            }
        }
    }

    sequences.sort_unstable();
    sequences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_known_sequence() {
        assert!(prime_permutation_sequences().contains(&[1_487, 4_817, 8_147]));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "296962999629");
    }
}
