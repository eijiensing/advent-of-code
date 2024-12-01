mod d1;
use crate::utils::{add_day, prompt_add};

pub fn run_day(year: &str, day: &str) {
    match day {
        // <DAY_ENTRY>
        "1" => d1::run(),
        _ => {
            prompt_add(year, day);
            add_day(year, day);
        }
    }
}
