use crate::get_data_filepath;
use std::{collections::HashSet, fs};

pub fn run() {
    let input_filepath = get_data_filepath!();
    let input = fs::read_to_string(input_filepath).unwrap();

    let answer1 = part1(&input);
    println!("Part 1: {answer1}");

    let answer2 = part2(&input);
    println!("Part 2: {answer2}");
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut trailheads: Vec<(usize, usize)> = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                trailheads.push((y, x));
            }
        }
    }

    trailheads
        .iter()
        .map(|(y, x)| search_trails_part1(&map, &HashSet::new(), *y, *x).len())
        .sum()
}

fn search_trails_part1(
    map: &[Vec<usize>],
    found_trailends: &HashSet<(usize, usize)>,
    y: usize,
    x: usize,
) -> HashSet<(usize, usize)> {
    let current_height = map[y][x];
    if current_height == 9 && !found_trailends.contains(&(y, x)) {
        let mut next_found_trailends = found_trailends.clone();
        next_found_trailends.insert((y, x));
        return next_found_trailends;
    }

    let next_height = current_height + 1;
    let mut found_trailends: HashSet<(usize, usize)> = HashSet::new();

    for (y_offset, x_offset) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let next_y: i32 = y as i32 + y_offset;
        let next_x: i32 = x as i32 + x_offset;
        let y_in_bounds = next_y >= 0 && next_y < map.len() as i32;
        let x_in_bounds = next_x >= 0 && next_x < map[0].len() as i32;

        if y_in_bounds && x_in_bounds {
            let next_x_usize = next_x as usize;
            let next_y_usize = next_y as usize;

            if map[next_y_usize][next_x_usize] == next_height {
                let next_found_trailends =
                    search_trails_part1(map, &found_trailends, next_y_usize, next_x_usize);

                found_trailends = found_trailends
                    .union(&next_found_trailends)
                    .cloned()
                    .collect();
            }
        }
    }

    found_trailends
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut trailheads: Vec<(usize, usize)> = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                trailheads.push((y, x));
            }
        }
    }

    trailheads
        .iter()
        .map(|(y, x)| search_trails_part2(&map, *y, *x))
        .sum()
}

fn search_trails_part2(map: &[Vec<usize>], y: usize, x: usize) -> usize {
    let current_height = map[y][x];
    if current_height == 9 {
        return 1;
    }

    let next_height = current_height + 1;
    let mut num_trails = 0;

    for (y_offset, x_offset) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let next_y: i32 = y as i32 + y_offset;
        let next_x: i32 = x as i32 + x_offset;
        let y_in_bounds = next_y >= 0 && next_y < map.len() as i32;
        let x_in_bounds = next_x >= 0 && next_x < map[0].len() as i32;

        if y_in_bounds && x_in_bounds {
            let next_x_usize = next_x as usize;
            let next_y_usize = next_y as usize;

            if map[next_y_usize][next_x_usize] == next_height {
                num_trails += search_trails_part2(map, next_y_usize, next_x_usize);
            }
        }
    }

    num_trails
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let answer = part1(TEST_INPUT);
        assert_eq!(answer, 36);
    }

    #[test]
    fn test_part2() {
        let answer = part2(TEST_INPUT);
        assert_eq!(answer, 81);
    }
}
