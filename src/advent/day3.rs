use crate::get_data_filepath;
use regex::Regex;
use std::{fs, path::Path};

pub fn run() {
    let data_filepath = get_data_filepath!();
    let data = get_data(&data_filepath);

    let answer1 = part1(&data);
    println!("Part 1: {answer1}");
}

fn get_data(data_filepath: &Path) -> String {
    fs::read_to_string(data_filepath).unwrap()
}

fn part1(data: &str) -> i64 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(pattern).unwrap();

    // let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    // let data = &data.replace("\n", " ");
    // let matches: Vec<String> = regex
    //     .find_iter(data)
    //     .map(|mat| mat.as_str().to_string())
    //     .collect();
    // for m in matches {
    //     println!("{m}");
    // }

    let num_pairs: Vec<(i64, i64)> = regex
        .captures_iter(data)
        .map(|captures| {
            (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .collect();

    // for m in num_pairs.clone() {
    //     println!("{m:?}");
    // }
    let answer = num_pairs.iter().map(|(x, y)| x * y).sum();
    answer
}
