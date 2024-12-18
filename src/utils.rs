use std::{io::Write, process::Command};

pub fn prompt_add(year: &str, day: &str) {
    print!("Do you want to add day {day} of year {year}? [Y/n] ");
    std::io::stdout().flush().unwrap();
    let mut user_confirm = String::new();
    std::io::stdin().read_line(&mut user_confirm).unwrap();
    if std::env::consts::OS == "windows"
        && !(user_confirm.to_lowercase() == "\r\n" || user_confirm.to_lowercase() == "y\r\n")
    {
        std::process::exit(0);
    }
    if !(user_confirm.to_lowercase() == "\n" || user_confirm.to_lowercase() == "y\n") {
        std::process::exit(0);
    }
}

pub fn add_day(year: &str, day: &str) {
    let slash = if std::env::consts::OS == "windows" {
        "\\"
    } else {
        "/"
    };
    let puzzles_mod_contents =
        std::fs::read_to_string(format!("src/puzzles/y{year}/mod.rs").replace("/", slash)).unwrap();
    let mut new_puzzles_mod_contents: Vec<String> = puzzles_mod_contents
        .clone()
        .lines()
        .map(String::from)
        .collect();
    for (index, line) in puzzles_mod_contents.lines().enumerate() {
        if line.contains("<DAY_ENTRY>") {
            new_puzzles_mod_contents.insert(index + 1, format!("\"{day}\" => d{day}::run(),"))
        }
    }
    new_puzzles_mod_contents.insert(0, format!("mod d{day};"));
    std::fs::write(
        format!("src/puzzles/y{year}/mod.rs").replace("/", slash),
        new_puzzles_mod_contents.join("\n"),
    )
    .unwrap();
    let template_contents = std::fs::read_to_string("src/puzzles/day_template".replace("/", slash))
        .unwrap()
        .replace("<YEAR>", year)
        .replace("<DAY>", day);

    std::fs::write(
        format!("src/puzzles/y{year}/d{day}.rs").replace("/", slash),
        template_contents,
    )
    .unwrap();
    let input_year_path = format!("inputs/y{year}").replace("/", slash);
    if !std::path::Path::new(&input_year_path).exists() {
        std::fs::create_dir(input_year_path).unwrap();
    }
    std::fs::write(format!("inputs/y{year}/d{day}.txt").replace("/", slash), "").unwrap();

    Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("Failed to run cargo fmt");

    println!("Created day in src/puzzles/y{year}/d{day}.rs run it with the same command.");
    println!("Start by copying the input from https://adventofcode.com/20{year}/day/{day}/input and pasting it in inputs/y{year}/d{day}.txt")
}

pub fn add_year(year: &str) {
    let slash = if std::env::consts::OS == "windows" {
        "\\"
    } else {
        "/"
    };
    let year_path = format!("src/puzzles/y{year}").replace("/", slash);
    if !std::path::Path::new(&year_path).exists() {
        std::fs::create_dir(&year_path).unwrap();
    }
    let template_contents =
        std::fs::read_to_string("src/puzzles/year_template".replace("/", slash)).unwrap();
    std::fs::write(format!("{year_path}/mod.rs"), template_contents).unwrap();
    let main_contents = std::fs::read_to_string("src/main.rs".replace("/", slash)).unwrap();
    let mut new_main_contents: Vec<String> =
        main_contents.clone().lines().map(String::from).collect();
    for (index, line) in main_contents.lines().enumerate() {
        if line.contains("<YEAR_ENTRY>") {
            new_main_contents.insert(
                index + 1,
                format!("\"{year}\" => y{year}::run_day(year, day),"),
            )
        }
    }
    let updated_main_contents = new_main_contents.join("\n");
    std::fs::write("src/main.rs", updated_main_contents).unwrap();
    let mut puzzles_mod_contents: Vec<String> =
        std::fs::read_to_string("src/puzzles/mod.rs".replace("/", slash))
            .unwrap()
            .clone()
            .lines()
            .map(String::from)
            .collect();
    puzzles_mod_contents.insert(0, format!("pub mod y{year};"));
    std::fs::write(
        "src/puzzles/mod.rs".replace("/", slash),
        puzzles_mod_contents.join("\n"),
    )
    .unwrap();
}
