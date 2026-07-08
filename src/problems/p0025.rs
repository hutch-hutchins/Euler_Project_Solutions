use crate::util::decimal::{add, from_u64};

const TARGET_DIGITS: usize = 1_000;

pub fn solve() -> String {
    first_fibonacci_index_with_digits(TARGET_DIGITS).to_string()
}

fn first_fibonacci_index_with_digits(target_digits: usize) -> usize {
    let mut previous = from_u64(1);
    let mut current = from_u64(1);
    let mut index = 2;

    while current.len() < target_digits {
        let next = add(&previous, &current);
        previous = current;
        current = next;
        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_three_digits() {
        assert_eq!(first_fibonacci_index_with_digits(3), 12);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "4782");
    }
}
