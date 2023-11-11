//! Czas is a library for converting [`chrono`] timestamps into localized text.  
//! For example, `2020-01-01 01:23:45` would be converted (in Polish) to `pierwszego stycznia dwa tysiące dwudziestego roku o pierwszej dwadzieścia trzy i czterdzieści pięć sekundy`.  
//! The library provides the public [`ToLocalizedText`] trait, which can be implemented against any struct to provide your own translations in any language/format.  
//! The library comes with one struct implementation of this trait, [`Czas`], which supports localization in Polish.
pub use crate::errors::CzasError;
pub use crate::mapping::polish::{
    date_to_polish_genitive, hours_to_polish_locative, minutes_to_polish_nominative,
    month_to_polish_genitive, seconds_to_polish_nominative, year_to_polish_genetive,
};
use chrono::{Datelike, NaiveDateTime, Timelike};

pub mod errors;
pub mod mapping;

/// Trait for converting [`chrono::NaiveDateTime`] timestamps into localized text
pub trait ToLocalizedText {
    /// Translates a [`chrono::NaiveDateTime`] timestamp into it's localized text equivalent
    ///
    /// # Errors
    ///
    /// Will return a [`CzasError`] if the timestamp is invalid
    fn from_naive_date_time(timestamp: NaiveDateTime) -> Result<String, CzasError>;
    /// Parsed an ISO 8601 string timestamp into it's localized text equivalent
    ///
    /// # Errors
    ///
    /// Will return a [`CzasError`] if the timestamp is invalid
    fn from_string(timestamp: &str) -> Result<String, CzasError>;
    /// Get the current date and time in it's localized text equivalent
    ///
    /// # Errors
    ///
    /// Will return a [`CzasError`] if the system time cannot be read
    fn now() -> Result<String, CzasError>;
}

/// Default implementation of [`ToLocalizedText`] in Polish
pub struct Czas {}

impl ToLocalizedText for Czas {
    /// Translates a [`chrono::NaiveDateTime`] timestamp into it's Polish text equivalent
    ///
    /// # Errors
    ///
    /// Will return a [`CzasError`] if the timestamp is invalid   
    fn from_naive_date_time(timestamp: NaiveDateTime) -> Result<String, CzasError> {
        let second = seconds_to_polish_nominative(timestamp.second())?;
        let minute = minutes_to_polish_nominative(timestamp.minute())?;
        let hour = hours_to_polish_locative(timestamp.hour())?;
        let day = date_to_polish_genitive(timestamp.day())?;
        let month = month_to_polish_genitive(timestamp.month())?;
        let year = year_to_polish_genetive(timestamp.year());

        let mut base_string = format!("{day} {month} {year} roku o {hour}");
        if !minute.is_empty() {
            base_string = format!("{base_string} {minute}");
        };
        if !second.is_empty() {
            base_string = format!("{base_string} i {second}");
        }
        Ok(base_string)
    }
    /// Parsed a n ISO 8601 string timestamp into it's Polish text equivalent
    ///
    /// # Errors
    ///
    /// Will return a [`CzasError`] if the timestamp is invalid
    fn from_string(timestamp: &str) -> Result<String, CzasError> {
        let timestamp = chrono::NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")?;
        Self::from_naive_date_time(timestamp)
    }
    /// Get the current date and time in it's Polish text equivalent
    ///
    /// # Errors
    ///
    /// Will return a [`CzasError`] if the system time cannot be read
    fn now() -> Result<String, CzasError> {
        let timestamp = chrono::Local::now().naive_local();
        Self::from_naive_date_time(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let timestamp = chrono::NaiveDateTime::from_timestamp_millis(1_699_643_146_776).unwrap();

        let result = Czas::from_naive_date_time(timestamp).unwrap();

        assert_eq!(
            result,
            String::from("dziesiątego listopada dwa tysiące dwudziestego trzeciego roku o dziewiętnastej pięć i czterdzieści sześć sekund")
        );
    }

    #[test]
    fn from_string_works_with_valid_timestamp() {
        let result = Czas::from_string("2020-01-01 01:23:45").unwrap();

        assert_eq!(
            result,
            String::from("pierwszego stycznia dwa tysiące dwudziestego roku o pierwszej dwadzieścia trzy i czterdzieści pięć sekund")
        );
    }

    #[test]
    fn from_string_with_no_minutes_removes_it() {
        let result = Czas::from_string("2020-01-01 01:00:01").unwrap();

        assert_eq!(
            result,
            String::from(
                "pierwszego stycznia dwa tysiące dwudziestego roku o pierwszej i jeden sekunda"
            )
        );
    }

    #[test]
    fn from_string_with_no_seconds_removes_it() {
        let result = Czas::from_string("2020-01-01 01:01:00").unwrap();

        assert_eq!(
            result,
            String::from("pierwszego stycznia dwa tysiące dwudziestego roku o pierwszej jeden")
        );
    }

    #[test]
    fn from_string_with_no_minutes_and_no_seconds_removes_it() {
        let result = Czas::from_string("2020-01-01 00:00:00").unwrap();

        assert_eq!(
            result,
            String::from("pierwszego stycznia dwa tysiące dwudziestego roku o północy")
        );
    }

    #[test]
    fn from_string_returns_error_with_invalid_timestamp() {
        let result = Czas::from_string("hello world");

        assert!(result.is_err());
    }
}
