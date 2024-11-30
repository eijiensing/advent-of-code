mod puzzles;
mod utils;
use puzzles::*;
use utils::prompt_add;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Enter the year and day run or add it.");
        println!("Usage: cargo run <year> <day>");
        return;
    }

    // accept 2021 aswell as 21
    let year = if args[1].len() == 4 {
        &args[1][2..]
    } else {
        args[1].as_str()
    };

    let day = &args[2];

    match year {
        // <YEAR_ENTRY>
        "23" => y23::run_day(year, day),
        _ => {
            prompt_add(year, day);
            utils::add_year(year);
            utils::add_day(year, day);
        }
    }
}
