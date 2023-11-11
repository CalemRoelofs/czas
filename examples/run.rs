use chrono::prelude::*;
use czas::{Czas, ToLocalizedText};

fn main() {
    let timestamp = Local::now().naive_local();
    println!(
        "{}: {}",
        timestamp,
        Czas::from_naive_date_time(timestamp).unwrap()
    );
}
