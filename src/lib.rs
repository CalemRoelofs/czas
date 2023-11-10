use crate::errors::CzasError;
use crate::mapping::{
    date_to_polish_genitive, hours_to_polish_locative, month_to_polish_genitive,
    seconds_or_minutes_to_polish_nominative,
};
use chrono::{Datelike, NaiveDateTime, Timelike};

pub mod errors;
pub mod mapping;

/// Translates a [`chrono::NaiveDateTime`] timestamp into it's Polish text equivalent
///
/// # Errors
///
/// Will return a [`CzasError`] if the timestamp is invalid
pub fn to_polish_time(timestamp: NaiveDateTime) -> Result<String, CzasError> {
    let second = seconds_or_minutes_to_polish_nominative(timestamp.second())?;
    let minute = seconds_or_minutes_to_polish_nominative(timestamp.minute())?;
    let hour = hours_to_polish_locative(timestamp.hour())?;
    let day = date_to_polish_genitive(timestamp.day())?;
    let month = month_to_polish_genitive(timestamp.month())?;

    Ok(format!(
        "{day} {month} o {hour} {minute} i {second} sekundy"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let timestamp = chrono::NaiveDateTime::from_timestamp_millis(1_699_643_146_776).unwrap();

        let result = to_polish_time(timestamp).unwrap();

        assert_eq!(
            result,
            String::from("dziesiątego listopada o dziewiętnastej pięć czterdzieści sześć")
        );
    }
}
