mod advent;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <exercise>");
        process::exit(1);
    }

    match args[1].as_str() {
        "day1" => advent::day1::run(),
        "day2" => advent::day2::run(),
        "day3" => advent::day3::run(),
        "day4" => advent::day4::run(),
        "day5" => advent::day5::run(),
        "day6" => advent::day6::run(),
        "day7" => advent::day7::run(),
        "day8" => advent::day8::run(),
        "day9" => advent::day9::run(),
        "day10" => advent::day10::run(),
        _ => {
            eprintln!("Unknown exercise: {}", args[1]);
            std::process::exit(1);
        }
    }
}
