use crate::get_data_filepath;
use regex::Regex;
use std::{fs, path::Path};

pub fn run() {
    let data_filepath = get_data_filepath!();
    let data = get_data(&data_filepath);

    let answer1 = part1(&data);
    println!("Part 1: {answer1}");

    let answer2 = part2(&data);
    println!("Part 2: {answer2}");
}

fn get_data(data_filepath: &Path) -> String {
    fs::read_to_string(data_filepath).unwrap()
}

fn part1(data: &str) -> u64 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).unwrap();

    let num_pairs: Vec<(u64, u64)> = regex
        .captures_iter(data)
        .map(|captures| {
            (
                captures[1].parse::<u64>().unwrap(),
                captures[2].parse::<u64>().unwrap(),
            )
        })
        .collect();

    let answer = num_pairs.iter().map(|(x, y)| x * y).sum();
    answer
}

fn part2(data: &str) -> u64 {
    let pattern = r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))";
    let regex = Regex::new(pattern).unwrap();

    let mut mult_enabled = true;
    let mut sum = 0;

    for captures in regex.captures_iter(data) {
        let matched_str = captures[0].parse::<String>().unwrap();
        match matched_str.as_str() {
            m if m.starts_with("mul(") && mult_enabled => {
                let x = captures[2].parse::<u64>().unwrap();
                let y = captures[3].parse::<u64>().unwrap();
                sum += x * y;
            }
            "do()" => mult_enabled = true,
            "don't()" => mult_enabled = false,
            _ => {}
        }
    }

    sum
}
