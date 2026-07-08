use std::collections::HashSet;

use crate::util::polygonal::pentagonal;

const SEARCH_LIMIT: u64 = 10_000;

pub fn solve() -> String {
    minimal_pentagonal_difference().to_string()
}

fn minimal_pentagonal_difference() -> u64 {
    let pentagonals: Vec<u64> = (1..=SEARCH_LIMIT).map(pentagonal).collect();
    let pentagonal_set: HashSet<u64> = pentagonals.iter().copied().collect();
    let mut best = u64::MAX;

    for k in 1..pentagonals.len() {
        let pk = pentagonals[k];

        for j in (0..k).rev() {
            let pj = pentagonals[j];
            let difference = pk - pj;

            if difference >= best {
                break;
            }

            if pentagonal_set.contains(&difference) && pentagonal_set.contains(&(pk + pj)) {
                best = difference;
            }
        }
    }

    assert_ne!(best, u64::MAX, "increase SEARCH_LIMIT");
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::polygonal::is_pentagonal;

    #[test]
    fn sample_sum_is_pentagonal() {
        assert!(is_pentagonal(pentagonal(4) + pentagonal(7)));
        assert!(!is_pentagonal(pentagonal(7) - pentagonal(4)));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "5482660");
    }
}
