mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
use crate::utils::{add_day, prompt_add};

pub fn run_day(year: &str, day: &str) {
    match day {
        // <DAY_ENTRY>
        "6" => d6::run(),
        "5" => d5::run(),
        "4" => d4::run(),
        "3" => d3::run(),
        "2" => d2::run(),
        "1" => d1::run(),
        _ => {
            prompt_add(year, day);
            add_day(year, day);
        }
    }
}
