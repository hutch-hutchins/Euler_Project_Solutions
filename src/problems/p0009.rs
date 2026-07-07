const TARGET_SUM: u64 = 1_000;

pub fn solve() -> String {
    triplet_product_for_sum(TARGET_SUM)
        .expect("problem statement guarantees one triplet")
        .to_string()
}

fn triplet_product_for_sum(target_sum: u64) -> Option<u64> {
    for a in 1..target_sum {
        for b in (a + 1)..target_sum {
            let c = target_sum - a - b;

            if b >= c {
                break;
            }

            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_sum_12() {
        assert_eq!(triplet_product_for_sum(12), Some(60));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "31875000");
    }
}
