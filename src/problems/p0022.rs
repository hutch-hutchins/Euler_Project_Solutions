const NAMES: &str = include_str!("../../data/p0022_names.txt");

pub fn solve() -> String {
    total_name_scores(NAMES).to_string()
}

fn total_name_scores(input: &str) -> u64 {
    let mut names = parse_names(input);
    names.sort_unstable();

    names
        .iter()
        .enumerate()
        .map(|(index, name)| (index as u64 + 1) * alphabetical_value(name))
        .sum()
}

fn parse_names(input: &str) -> Vec<&str> {
    input
        .trim()
        .split(',')
        .map(|name| name.trim_matches('"'))
        .collect()
}

fn alphabetical_value(name: &str) -> u64 {
    name.bytes().map(|byte| u64::from(byte - b'A' + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_colin_score() {
        assert_eq!(alphabetical_value("COLIN"), 53);
    }

    #[test]
    fn data_file_has_expected_shape() {
        let names = parse_names(NAMES);
        assert_eq!(names.len(), 5_163);
        assert!(names.contains(&"COLIN"));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "871198282");
    }
}
