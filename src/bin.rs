//! Prints the current time and date in Polish
//!
//! # Usage
//!
//! ```shell
//! czas-teraz
//! ```
use chrono::prelude::*;
use czas::{Czas, ToLocalizedText};

/// Prints the current time and date in Polish
///
/// Usage: `czas-teraz`
fn main() {
    let timestamp = Local::now().naive_local();
    println!(
        "{}: {}",
        timestamp,
        Czas::from_naive_date_time(timestamp).unwrap()
    );
}
