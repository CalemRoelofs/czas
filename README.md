# Czas - Convert timestamps into localized text

Czas is a library for converting [chrono](https://docs.rs/chrono/latest/chrono/) timestamps into localized text.  
For example, `2020-01-01 01:23:45` would be converted (in Polish) to `pierwszego stycznia dwa tysiące dwudziestego roku o pierwszej dwadzieścia trzy i czterdzieści pięć sekund`.  
The library provides the public `ToLocalizedText` trait, which can be implemented against any struct to provide your own translations in any language/format.  
The library comes with one struct implementation of this trait, `Czas`, which supports localization in Polish.  

## Documentation

Crate documentation can be found on [docs.rs](https://docs.rs/czas/latest/czas/)  

## Usage

### Library

Add `czas` as a dependency in your `Cargo.toml`:  
```toml
[dependencies]
czas = "*"
```

Then use it in your code:
```rust
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
```

### Binary

```shell
# Install locally from this repo
$ cargo install path .

# Or from crates.io directly
$ cargo install czas

# Then run it
$ czas-teraz
2023-11-11 14:33:50.152787900: jedenastego listopada dwa tysiące dwudziestego trzeciego roku o czternastej trzydzieści trzy i pięćdziesiąt sekundy  
```

## Contributing  

PRs are welcome!   
If you wish to change something with the existing implementation or add a new localization, please consider opening an issue for discussion.  

## License  

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT License](https://opensource.org/licenses/MIT)

at your option.

