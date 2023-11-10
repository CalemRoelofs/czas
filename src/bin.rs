use chrono::prelude::*;
use czas::to_polish_time;

fn main() {
    let timestamp = Local::now().naive_local();
    println!("{}: {}", timestamp, to_polish_time(timestamp).unwrap());
}
