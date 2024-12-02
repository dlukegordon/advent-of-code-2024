use std::{fs, iter};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let (vec1, vec2) = get_lists();
    part1(&vec1, &vec2);
    part2(&vec1, &vec2);
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let (vec1, vec2): (Vec<i32>, Vec<i32>) = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .unzip();

    (vec1, vec2)
}

fn part1(vec1: &Vec<i32>, vec2: &Vec<i32>) {
    let mut vec1 = vec1.clone();
    let mut vec2 = vec2.clone();
    vec1.sort();
    vec2.sort();

    let answer: i32 = iter::zip(vec1, vec2).map(|(n1, n2)| (n1 - n2).abs()).sum();

    println!("{}", answer);
}

fn part2(vec1: &Vec<i32>, vec2: &Vec<i32>) {
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

    println!("{}", answer);
}
