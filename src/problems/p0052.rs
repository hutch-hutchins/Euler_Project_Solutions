use crate::util::digits::digit_signature;

pub fn solve() -> String {
    smallest_permuted_multiple().to_string()
}

fn smallest_permuted_multiple() -> u64 {
    let mut n = 1;

    loop {
        let signature = digit_signature(n);

        if (2..=6).all(|multiple| digit_signature(n * multiple) == signature) {
            return n;
        }

        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_125874() {
        assert_eq!(digit_signature(125_874), digit_signature(251_748));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "142857");
    }
}
