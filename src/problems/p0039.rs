const LIMIT: u64 = 1_000;

pub fn solve() -> String {
    perimeter_with_most_right_triangles(LIMIT).to_string()
}

fn perimeter_with_most_right_triangles(limit: u64) -> u64 {
    (1..=limit)
        .max_by_key(|&perimeter| right_triangle_count(perimeter))
        .expect("range should not be empty")
}

fn right_triangle_count(perimeter: u64) -> u64 {
    let mut count = 0;

    for a in 1..=perimeter / 3 {
        for b in a..=(perimeter - a) / 2 {
            let c = perimeter - a - b;
            if a * a + b * b == c * c {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_120() {
        assert_eq!(right_triangle_count(120), 3);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "840");
    }
}
