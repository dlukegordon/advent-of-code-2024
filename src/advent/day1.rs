use crate::get_data_filepath;
use std::{fs, iter, path::Path};

type List = Vec<i32>;

pub fn run() {
    let data_filepath = get_data_filepath!();
    let (vec1, vec2) = get_lists(&data_filepath);
    part1(&vec1, &vec2);
    part2(&vec1, &vec2);
}

fn get_lists(data_filepath: &Path) -> (List, List) {
    let data_string = fs::read_to_string(data_filepath).unwrap();
    data_string
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip()
}

fn part1(vec1: &List, vec2: &List) {
    let mut vec1 = vec1.clone();
    let mut vec2 = vec2.clone();
    vec1.sort();
    vec2.sort();

    let answer: i32 = iter::zip(vec1, vec2).map(|(n1, n2)| (n1 - n2).abs()).sum();

    println!("Part 1: {answer}");
}

fn part2(vec1: &List, vec2: &List) {
    let answer: i32 = vec1
        .iter()
        .map(|n1| {
            let num_appears: i32 = vec2
                .iter()
                .filter(|n2| n1 == *n2)
                .count()
                .try_into()
                .unwrap();
            n1 * num_appears
        })
        .sum();

    println!("Part 2: {answer}");
}
