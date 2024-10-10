use crate::date::is_leap;

pub fn subject() -> String {
    let mut months: [u64; 12] = [31, u64::MAX, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut sunday_firsts = 0;
    let mut day_of_week = 0;
    for year in 1900..=2000 {
        months[1] = if is_leap(year) { 29 } else { 28 };
        for days_in_month in months {
            sunday_firsts += (year != 1900 && day_of_week == 6) as u64;
            day_of_week = (day_of_week + days_in_month) % 7;
        }
    }
    sunday_firsts.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject() {
        assert_eq!(subject(), "171");
    }
}
