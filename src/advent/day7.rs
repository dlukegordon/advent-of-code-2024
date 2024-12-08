use crate::get_data_filepath;
use itertools::Itertools;
use rayon::prelude::*;
use std::{fs, path::Path};

type Num = usize;
type Lhs = Num;
type Rhs = Vec<Num>;
type UnfinishedEquation = (Lhs, Rhs);
type Input = Vec<UnfinishedEquation>;
type Operator = String;
type Operators = Vec<Operator>;

pub fn run() {
    let input_filepath = get_data_filepath!();
    let input = get_input(&input_filepath);

    let answer1 = part1(&input);
    println!("Part 1: {answer1}");

    let answer2 = part2(&input);
    println!("Part 2: {answer2}");
}

fn get_input(data_filepath: &Path) -> Input {
    let data_str = fs::read_to_string(data_filepath).unwrap();
    get_input_from_str(data_str)
}

fn get_input_from_str(input_str: String) -> Input {
    input_str
        .lines()
        .map(|line| {
            let mut splitted = line.split(':');
            let lhs = splitted.next().unwrap().parse::<Num>().unwrap();
            let rhs_str = splitted.next().unwrap();
            let rhs: Rhs = rhs_str
                .split_whitespace()
                .map(|s| s.parse::<Num>().unwrap())
                .collect();

            (lhs, rhs)
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    input
        .par_iter()
        .filter(|ueq| eq_could_be_true_part1(ueq))
        .map(|(lhs, _rhs)| lhs)
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .par_iter()
        .filter(|ueq| eq_could_be_true_part2(ueq))
        .map(|(lhs, _rhs)| lhs)
        .sum()
}

fn eq_could_be_true_part1(ueq: &UnfinishedEquation) -> bool {
    let (_lhs, rhs) = ueq;
    possible_operators_part1(rhs.len() - 1)
        .par_iter()
        .any(|ops| eq_is_true(ueq, ops))
}

fn eq_could_be_true_part2(ueq: &UnfinishedEquation) -> bool {
    let (_lhs, rhs) = ueq;
    possible_operators_part2(rhs.len() - 1)
        .par_iter()
        .any(|ops| eq_is_true(ueq, ops))
}

fn eq_is_true((lhs, rhs): &UnfinishedEquation, ops: &Operators) -> bool {
    let mut result = rhs[0];

    for i in 0..ops.len() {
        let op = &ops[i];
        let num = rhs[i + 1];

        match op.as_str() {
            "+" => result += num,
            "*" => result *= num,
            "||" => result = concat_nums(result, num),
            _ => panic!("received invalid op: {}", op),
        };

        match result {
            result if &result > lhs => return false,
            result if &result == lhs && i == ops.len() - 1 => {
                return true;
            }
            _ => {}
        }
    }

    false
}

fn concat_nums(a: Num, b: Num) -> Num {
    format!("{}{}", a, b).parse().unwrap()
}

fn possible_operators_part1(len: usize) -> Vec<Operators> {
    let valid_operators = vec!["+".into(), "*".into()];
    possible_operators(len, valid_operators)
}

fn possible_operators_part2(len: usize) -> Vec<Operators> {
    let valid_operators = vec!["+".into(), "*".into(), "||".into()];
    possible_operators(len, valid_operators)
}

fn possible_operators(len: usize, valid_operators: Vec<Operator>) -> Vec<Operators> {
    (0..len)
        .map(|_| valid_operators.to_vec())
        .multi_cartesian_product()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_STR: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let data = get_input_from_str(TEST_DATA_STR.into());
        let answer = part1(&data);
        assert_eq!(answer, 3749)
    }

    #[test]
    fn test_part2() {
        let data = get_input_from_str(TEST_DATA_STR.into());
        let answer = part2(&data);
        assert_eq!(answer, 11387)
    }

    #[test]
    fn test_eq_could_be_true_part2() {
        let mut ueq = (156, vec![15, 6]);
        assert!(eq_could_be_true_part2(&ueq));

        ueq = (7290, vec![6, 8, 6, 15]);
        assert!(eq_could_be_true_part2(&ueq));
    }

    #[test]
    fn test_eq_is_true() {
        let lhs: Lhs = 44209555513;
        let rhs: Rhs = vec![44, 209, 55, 49, 6, 13];
        let ops: Operators = vec![
            "||".to_string(),
            "||".to_string(),
            "||".to_string(),
            "+".to_string(),
            "||".to_string(),
        ];
        assert!(eq_is_true(&(lhs, rhs), &ops));
    }
}
