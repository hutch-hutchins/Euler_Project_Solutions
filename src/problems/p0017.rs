const LIMIT: u32 = 1_000;
const BELOW_TWENTY: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn solve() -> String {
    total_letter_count_through(LIMIT).to_string()
}

fn total_letter_count_through(limit: u32) -> usize {
    (1..=limit).map(letter_count).sum()
}

fn letter_count(n: u32) -> usize {
    match n {
        0 => 0,
        1..=19 => BELOW_TWENTY[n as usize].len(),
        20..=99 => TENS[(n / 10) as usize].len() + letter_count(n % 10),
        100..=999 => {
            let hundreds = BELOW_TWENTY[(n / 100) as usize].len() + "hundred".len();
            let remainder = n % 100;

            if remainder == 0 {
                hundreds
            } else {
                hundreds + "and".len() + letter_count(remainder)
            }
        }
        1_000 => "onethousand".len(),
        _ => panic!("unsupported number: {n}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(total_letter_count_through(5), 19);
        assert_eq!(letter_count(342), 23);
        assert_eq!(letter_count(115), 20);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "21124");
    }
}
