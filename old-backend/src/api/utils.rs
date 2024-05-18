use std::error::Error;

use time::{
    error::Parse, format_description::FormatItem, macros::format_description, Date, Duration,
};
const DATE_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// Parses a string date of the form 'YYYY-MM-DD'.
pub fn parse_date(date: &String) -> Result<time::Date, Parse> {
    Date::parse(date, &DATE_FORMAT)
}

/// Parses a date from a string, subtracts `days` days from it, and re-formats it into a string.
pub fn sub_days(date: &String, days: i32) -> Result<String, Box<dyn Error>> {
    let date_obj = parse_date(date)? - Duration::days(days as i64);
    Ok(date_obj.format(&DATE_FORMAT)?)
}

/// Parses dates from the two inputs, and returns the difference in days between `date_a` and `date_b`.
pub fn day_diff(date_a: &String, date_b: &String) -> Result<i32, Box<dyn Error>> {
    let date_a_obj = parse_date(date_a)?;
    let date_b_obj = parse_date(date_b)?;

    let duration = date_a_obj - date_b_obj;
    Ok(duration.whole_days() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn test_parse_date() {
        let date_string = String::from("2023-11-25");
        let date = parse_date(&date_string).unwrap();
        let expected = Date::from_calendar_date(2023, Month::November, 25).unwrap();
        assert_eq!(expected, date);
    }

    #[test]
    fn test_parse_invalid_date() {
        let date_string = String::from("I'm not a date.");
        let date = parse_date(&date_string);
        assert!(date.is_err());
    }

    #[test]
    fn test_sub_days() {
        let date_string = String::from("2023-11-25");
        let date = sub_days(&date_string, 6).unwrap();
        let expected = String::from("2023-11-19");
        assert_eq!(expected, date);
    }

    #[test]
    fn test_day_diff() {
        let date_a_string = String::from("2023-11-25");
        let date_b_string = String::from("2023-11-20");
        let diff = day_diff(&date_a_string, &date_b_string).unwrap();
        assert_eq!(diff, 5);
    }
}
