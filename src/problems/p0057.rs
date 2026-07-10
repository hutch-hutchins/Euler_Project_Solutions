use crate::util::decimal::{add, from_u64, multiply_by_u32};

const EXPANSIONS: usize = 1_000;

pub fn solve() -> String {
    square_root_convergent_count(EXPANSIONS).to_string()
}

fn square_root_convergent_count(expansions: usize) -> usize {
    let mut numerator = from_u64(3);
    let mut denominator = from_u64(2);
    let mut count = 0;

    for _ in 0..expansions {
        if numerator.len() > denominator.len() {
            count += 1;
        }

        let mut doubled_denominator = denominator.clone();
        multiply_by_u32(&mut doubled_denominator, 2);
        let next_numerator = add(&numerator, &doubled_denominator);
        let next_denominator = add(&numerator, &denominator);

        numerator = next_numerator;
        denominator = next_denominator;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_eight_expansions() {
        assert_eq!(square_root_convergent_count(8), 1);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "153");
    }
}
