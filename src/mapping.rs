use crate::errors::CzasError;

/// Converts a second or minute value to the Polish Nominative/Mianownik form
///
/// # Errors
///
/// Will return a [`CzasError`] if !(0 <= `seconds_or_minutes` <= 59)
pub fn seconds_or_minutes_to_polish_nominative(
    seconds_or_minutes: u32,
) -> Result<String, CzasError> {
    match seconds_or_minutes {
        0 => Ok("".to_string()),
        1 => Ok("jeden".to_string()),
        2 => Ok("dwa".to_string()),
        3 => Ok("trzy".to_string()),
        4 => Ok("cztery".to_string()),
        5 => Ok("pięć".to_string()),
        6 => Ok("sześć".to_string()),
        7 => Ok("siedem".to_string()),
        8 => Ok("osiem".to_string()),
        9 => Ok("dziewięć".to_string()),
        10 => Ok("dziesięć".to_string()),
        11 => Ok("jedenaście".to_string()),
        12 => Ok("dwanaście".to_string()),
        13 => Ok("trzynaście".to_string()),
        14 => Ok("czternaście".to_string()),
        15 => Ok("piętnaście".to_string()),
        16 => Ok("szesnaście".to_string()),
        17 => Ok("siedemnaście".to_string()),
        18 => Ok("osiemnaście".to_string()),
        19 => Ok("dziewiętnaście".to_string()),
        20 => Ok("dwadzieścia".to_string()),
        21..=29 => Ok(format!(
            "dwadzieścia {}",
            seconds_or_minutes_to_polish_nominative(seconds_or_minutes - 20)?
        )),
        30 => Ok("trzydzieści".to_string()),
        31..=39 => Ok(format!(
            "trzydzieści {}",
            seconds_or_minutes_to_polish_nominative(seconds_or_minutes - 30)?
        )),
        40 => Ok("czterdzieści".to_string()),
        41..=49 => Ok(format!(
            "czterdzieści {}",
            seconds_or_minutes_to_polish_nominative(seconds_or_minutes - 40)?
        )),
        50 => Ok("pięćdziesiąt".to_string()),
        51..=59 => Ok(format!(
            "pięćdziesiąt {}",
            seconds_or_minutes_to_polish_nominative(seconds_or_minutes - 50)?
        )),
        _ => Err(CzasError::Error),
    }
}

/// Converts an hour value to the Polish Locative/Miejscownik form
///
/// # Errors
///
/// Will return a [`CzasError`] if not !(0 <= `hours` <= 23)
pub fn hours_to_polish_locative(hours: u32) -> Result<String, CzasError> {
    match hours {
        0 => Ok("północy".to_string()),
        1 => Ok("pierwszej".to_string()),
        2 => Ok("drugiej".to_string()),
        3 => Ok("trzeciej".to_string()),
        4 => Ok("czwartej".to_string()),
        5 => Ok("piątej".to_string()),
        6 => Ok("szóstej".to_string()),
        7 => Ok("siódmej".to_string()),
        8 => Ok("ósmej".to_string()),
        9 => Ok("dziewiątej".to_string()),
        10 => Ok("dziesiątej".to_string()),
        11 => Ok("jedenastej".to_string()),
        12 => Ok("dwunastej".to_string()),
        13 => Ok("trzynastej".to_string()),
        14 => Ok("czternastej".to_string()),
        15 => Ok("piętnastej".to_string()),
        16 => Ok("szesnastej".to_string()),
        17 => Ok("siedemnastej".to_string()),
        18 => Ok("osiemnastej".to_string()),
        19 => Ok("dziewiętnastej".to_string()),
        20 => Ok("dwudziestej".to_string()),
        21..=23 => Ok(format!(
            "dwudziestej {}",
            hours_to_polish_locative(hours - 20)?
        )),
        _ => Err(CzasError::Error),
    }
}

/// Converts a day of the month to the Polish Genetive/Dopełniacz form
///
/// # Errors
///
/// Will return a [`CzasError`] if !(0 <= `date` <= 31)
pub fn date_to_polish_genitive(date: u32) -> Result<String, CzasError> {
    match date {
        1 => Ok("pierwszego".to_string()),
        2 => Ok("drugiego".to_string()),
        3 => Ok("trzeciego".to_string()),
        4 => Ok("czwartego".to_string()),
        5 => Ok("piątego".to_string()),
        6 => Ok("szóstego".to_string()),
        7 => Ok("siódmego".to_string()),
        8 => Ok("ósmego".to_string()),
        9 => Ok("dziewiątego".to_string()),
        10 => Ok("dziesiątego".to_string()),
        11 => Ok("jedenastego".to_string()),
        12 => Ok("dwunastego".to_string()),
        13 => Ok("trzynastego".to_string()),
        14 => Ok("czternastego".to_string()),
        15 => Ok("piętnastego".to_string()),
        16 => Ok("szesnastego".to_string()),
        17 => Ok("siedemnastego".to_string()),
        18 => Ok("osiemnastego".to_string()),
        19 => Ok("dziewiętnastego".to_string()),
        20 => Ok("dwudziestego".to_string()),
        21..=29 => Ok(format!(
            "dwudziestego {}",
            date_to_polish_genitive(date - 20)?
        )),
        30 => Ok("trzydziestego".to_string()),
        31 => Ok("trzydziestego pierwszego".to_string()),
        _ => Err(CzasError::Error),
    }
}

/// Converts month to the Polish Genetive/Dopełniacz form
///
/// # Errors
///
/// Will return a [`CzasError`] if !(1 <= `month` <= 12)
pub fn month_to_polish_genitive(month: u32) -> Result<String, CzasError> {
    match month {
        1 => Ok("stycznia".to_string()),
        2 => Ok("lutego".to_string()),
        3 => Ok("marca".to_string()),
        4 => Ok("kwietnia".to_string()),
        5 => Ok("maja".to_string()),
        6 => Ok("czerwca".to_string()),
        7 => Ok("lipca".to_string()),
        8 => Ok("sierpnia".to_string()),
        9 => Ok("września".to_string()),
        10 => Ok("października".to_string()),
        11 => Ok("listopada".to_string()),
        12 => Ok("grudnia".to_string()),
        _ => Err(CzasError::Error),
    }
}
