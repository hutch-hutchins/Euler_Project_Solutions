use std::collections::HashMap;

use crate::util::primes::{is_prime, primes_below};

const TARGET_SET_SIZE: usize = 5;
const PRIME_LIMIT: usize = 10_000;

pub fn solve() -> String {
    lowest_prime_pair_set_sum(TARGET_SET_SIZE).to_string()
}

fn lowest_prime_pair_set_sum(target_size: usize) -> u64 {
    let primes: Vec<u64> = primes_below(PRIME_LIMIT)
        .into_iter()
        .map(|prime| prime as u64)
        .filter(|&prime| prime != 2 && prime != 5)
        .collect();
    let mut cache = HashMap::new();
    let mut best = u64::MAX;

    search(&[], &primes, 0, target_size, &mut best, &mut cache);

    assert_ne!(best, u64::MAX, "increase PRIME_LIMIT");
    best
}

fn search(
    selected: &[u64],
    candidates: &[u64],
    current_sum: u64,
    target_size: usize,
    best: &mut u64,
    cache: &mut HashMap<(u64, u64), bool>,
) {
    if selected.len() == target_size {
        *best = (*best).min(current_sum);
        return;
    }

    let remaining = target_size - selected.len();
    if candidates.len() < remaining {
        return;
    }

    for (index, &candidate) in candidates.iter().enumerate() {
        if current_sum + candidate >= *best {
            break;
        }

        let lower_bound = current_sum
            + candidate
            + candidates[index + 1..]
                .iter()
                .take(remaining - 1)
                .sum::<u64>();
        if lower_bound >= *best {
            break;
        }

        if !selected
            .iter()
            .all(|&prime| are_compatible(prime, candidate, cache))
        {
            continue;
        }

        let mut next_selected = selected.to_vec();
        next_selected.push(candidate);

        let next_candidates: Vec<u64> = candidates[index + 1..]
            .iter()
            .copied()
            .filter(|&next| are_compatible(candidate, next, cache))
            .collect();

        search(
            &next_selected,
            &next_candidates,
            current_sum + candidate,
            target_size,
            best,
            cache,
        );
    }
}

fn are_compatible(left: u64, right: u64, cache: &mut HashMap<(u64, u64), bool>) -> bool {
    let key = if left < right {
        (left, right)
    } else {
        (right, left)
    };

    if let Some(&compatible) = cache.get(&key) {
        return compatible;
    }

    let compatible = is_prime(concat(left, right)) && is_prime(concat(right, left));
    cache.insert(key, compatible);
    compatible
}

fn concat(left: u64, right: u64) -> u64 {
    let mut scale = 10;
    while scale <= right {
        scale *= 10;
    }

    left * scale + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_four_prime_set() {
        assert_eq!(lowest_prime_pair_set_sum(4), 792);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "26033");
    }
}
