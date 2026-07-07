const TRIANGLE: &str = "\
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

pub fn solve() -> String {
    maximum_path_sum(TRIANGLE).to_string()
}

fn maximum_path_sum(input: &str) -> u64 {
    let mut triangle = parse_triangle(input);

    for row in (0..triangle.len() - 1).rev() {
        for col in 0..triangle[row].len() {
            triangle[row][col] += triangle[row + 1][col].max(triangle[row + 1][col + 1]);
        }
    }

    triangle[0][0]
}

fn parse_triangle(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse().expect("triangle values should be integers"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_triangle() {
        let input = "\
3
7 4
2 4 6
8 5 9 3";
        assert_eq!(maximum_path_sum(input), 23);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "1074");
    }
}
