use crate::errors::CzasError;

/// Converts a second or minute value to the Polish Nominative/Mianownik form
///
/// # Examples
///
/// ```rust
/// let second = czas::seconds_or_minutes_to_polish_nominative(1).unwrap();
/// assert_eq!(second, "jeden");
/// ```
///
/// # Errors
///
/// Will return a [`CzasError`] if !(0 <= `seconds_or_minutes` <= 59)
pub fn seconds_or_minutes_to_polish_nominative(
    seconds_or_minutes: u32,
) -> Result<String, CzasError> {
    match seconds_or_minutes % 60 {
        0 => Ok(String::new()),
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
/// # Examples
///
/// ```rust
/// let hour = czas::hours_to_polish_locative(1).unwrap();
/// assert_eq!(hour, "pierwszej");
/// ```
///
/// # Errors
///
/// Will return a [`CzasError`] if not !(0 <= `hours` <= 23)
pub fn hours_to_polish_locative(hours: u32) -> Result<String, CzasError> {
    match hours % 24 {
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
/// # Examples
///
/// ```rust
/// let date = czas::date_to_polish_genitive(1).unwrap();
/// assert_eq!(date, "pierwszego");
/// ```
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

/// Converts a month to the Polish Genetive/Dopełniacz form
///
/// # Examples
///
/// ```rust
/// let month = czas::month_to_polish_genitive(1).unwrap();
/// assert_eq!(month, "stycznia");
/// ```
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

fn year_ones_to_polish_genetive(year: i32) -> String {
    match year % 10 {
        1 => "pierwszego".to_string(),
        2 => "drugiego".to_string(),
        3 => "trzeciego".to_string(),
        4 => "czwartego".to_string(),
        5 => "piątego".to_string(),
        6 => "szóstego".to_string(),
        7 => "siódmego".to_string(),
        8 => "ósmego".to_string(),
        9 => "dziewiątego".to_string(),
        _ => String::new(),
    }
}

fn millenium_to_polish_mianownik(year: i32) -> String {
    match year / 1000 {
        1 => "tysiąc".to_string(),
        2 => "dwa tysiące".to_string(),
        3 => "trzy tysiące".to_string(),
        4 => "cztery tysiące".to_string(),
        5 => "pięć tysięcy".to_string(),
        6 => "sześć tysięcy".to_string(),
        7 => "siedem tysięcy".to_string(),
        8 => "osiem tysięcy".to_string(),
        9 => "dziewięć tysięcy".to_string(),
        _ => String::new(), // This should never happen.,
    }
}

fn century_to_polish_mianownik(year: i32) -> String {
    match (year % 1000) / 100 {
        1 => "sto".to_string(),
        2 => "dwiescie".to_string(),
        3 => "trzysta".to_string(),
        4 => "czterysta".to_string(),
        5 => "pięćset".to_string(),
        6 => "sześćset".to_string(),
        7 => "siedemset".to_string(),
        8 => "osiemset".to_string(),
        9 => "dziewiąćset".to_string(),
        _ => String::new(),
    }
}

/// Converts a year to the Polish Genetive/Dopełniacz form
///
/// # Examples
///
/// ```rust
/// let year = czas::year_to_polish_genetive(2022);
/// assert_eq!(year, "dwa tysiące dwudziestego drugiego");
/// ```
///
/// ```rust
/// let year = czas::year_to_polish_genetive(120);
/// assert_eq!(year, "sto dwudziestego");
/// ```
///
/// ```rust
/// let year = czas::year_to_polish_genetive(20);
/// assert_eq!(year, "dwudziestego");
/// ```
#[allow(clippy::must_use_candidate)]
pub fn year_to_polish_genetive(year: i32) -> String {
    let millenium = millenium_to_polish_mianownik(year);
    let century = century_to_polish_mianownik(year);
    let tens = year % 100;

    let tens = match tens {
        1..=9 => year_ones_to_polish_genetive(tens),
        10 => "dziesiątego".to_string(),
        11 => "jedenastego".to_string(),
        12 => "dwunastego".to_string(),
        13 => "trzynastego".to_string(),
        14 => "czternastego".to_string(),
        15 => "piętnastego".to_string(),
        16 => "szesnastego".to_string(),
        17 => "siedemnastego".to_string(),
        18 => "osiemnastego".to_string(),
        19 => "dziewiętnastego".to_string(),
        20 => "dwudziestego".to_string(),
        21..=29 => format!("dwudziestego {}", year_ones_to_polish_genetive(tens - 20)),
        30 => "trzydziestego".to_string(),
        31..=39 => format!("trzydziestego {}", year_ones_to_polish_genetive(tens - 30)),
        40 => "czterdziestego".to_string(),
        41..=49 => format!("czterdziestego {}", year_ones_to_polish_genetive(tens - 40)),
        50 => "pięćdziesiątego".to_string(),
        51..=59 => format!(
            "pięćdziesiątego {}",
            year_ones_to_polish_genetive(tens - 50)
        ),
        60 => "sześćdziesiątego".to_string(),
        61..=69 => format!(
            "sześćdziesiątego {}",
            year_ones_to_polish_genetive(tens - 60)
        ),
        70 => "siedemdziesiątego".to_string(),
        71..=79 => format!(
            "siedemdziesiątego  {}",
            year_ones_to_polish_genetive(tens - 70)
        ),
        80 => "osiemdziesiątego".to_string(),
        81..=89 => format!(
            "osiemdziesiątego {}",
            year_ones_to_polish_genetive(tens - 80)
        ),
        90 => "dziewięćdziesiątego".to_string(),
        91..=99 => format!(
            "dziewiąc dziesiątego {}",
            year_ones_to_polish_genetive(tens - 90)
        ),
        _ => String::new(),
    };

    let mut base_string = String::new();
    if !millenium.is_empty() {
        base_string = format!("{base_string}{millenium} ");
    }
    if !century.is_empty() {
        base_string = format!("{base_string}{century} ");
    }
    if !tens.is_empty() {
        base_string = format!("{base_string}{tens}");
    }

    base_string
}
