const START_YEAR: u32 = 1901;
const END_YEAR: u32 = 2000;

pub fn solve() -> String {
    count_sundays_on_first_of_month(START_YEAR, END_YEAR).to_string()
}

fn count_sundays_on_first_of_month(start_year: u32, end_year: u32) -> u32 {
    let mut first_day_weekday = 1;
    let mut count = 0;

    for year in 1900..=end_year {
        for month in 1..=12 {
            if year >= start_year && first_day_weekday == 0 {
                count += 1;
            }

            first_day_weekday = (first_day_weekday + days_in_month(year, month)) % 7;
        }
    }

    count
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => panic!("invalid month: {month}"),
    }
}

fn is_leap_year(year: u32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_rules() {
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), "171");
    }
}
