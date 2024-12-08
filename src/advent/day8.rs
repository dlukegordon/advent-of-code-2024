use crate::get_data_filepath;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

type Cell = char;
type Grid = Vec<Vec<Cell>>;
type Position = (usize, usize);
type Positions = Vec<Position>;
type Frequency = char;
type Antennas = HashMap<Frequency, Positions>;
type Input = (Grid, Antennas);
type FrequencyAntinodes = HashSet<Position>;

pub fn run() {
    let input_filepath = get_data_filepath!();
    let input = get_input(&input_filepath);

    let answer1 = part1(&input);
    println!("Part 1: {answer1}");

    let answer2 = part2(&input);
    println!("Part 2: {answer2}");
}

fn get_input(input_filepath: &Path) -> Input {
    let input_str = fs::read_to_string(input_filepath).unwrap();
    get_input_from_str(input_str)
}

fn get_input_from_str(input_str: String) -> Input {
    let grid: Grid = input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_insert_with(Vec::new).push((y, x));
            }
        }
    }

    (grid, antennas)
}

fn part1(input: &Input) -> usize {
    solve(input, false)
}

fn part2(input: &Input) -> usize {
    solve(input, true)
}

fn solve((grid, antennas): &Input, use_harmonics: bool) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut antinodes: FrequencyAntinodes = HashSet::new();
    for positions in antennas.values() {
        let freq_antinodes = get_freq_antinodes(positions, height, width, use_harmonics);
        antinodes.extend(freq_antinodes);
    }

    antinodes.len()
}

fn get_freq_antinodes(
    anten_positions: &Positions,
    height: usize,
    width: usize,
    use_harmonics: bool,
) -> Positions {
    let mut freq_antinodes = vec![];

    for (ant1, ant2) in anten_positions.iter().tuple_combinations() {
        let anti_positions = if use_harmonics {
            get_anten_pair_antinodes_part2(ant1, ant2, height, width)
        } else {
            get_anten_pair_antinodes_part1(ant1, ant2, height, width)
        };
        freq_antinodes.extend(anti_positions);
    }

    freq_antinodes
}

fn get_anten_pair_antinodes_part1(
    (y1, x1): &Position,
    (y2, x2): &Position,
    height: usize,
    width: usize,
) -> Positions {
    let mut antinodes = vec![];

    let dy = *y2 as i64 - *y1 as i64;
    let dx = *x2 as i64 - *x1 as i64;

    let anti_y1 = *y1 as i64 - dy;
    let anti_x1 = *x1 as i64 - dx;

    let anti_y2 = *y2 as i64 + dy;
    let anti_x2 = *x2 as i64 + dx;

    for (y, x) in [(anti_y1, anti_x1), (anti_y2, anti_x2)] {
        if let Some((y_int, x_int)) = check_pos(y, x, height, width) {
            antinodes.push((y_int, x_int));
        }
    }

    antinodes
}

fn get_anten_pair_antinodes_part2(
    (y1, x1): &Position,
    (y2, x2): &Position,
    height: usize,
    width: usize,
) -> Positions {
    let mut antinodes = vec![];
    antinodes.push((*y1, *x1));
    antinodes.push((*y2, *x2));

    let dy = *y2 as i64 - *y1 as i64;
    let dx = *x2 as i64 - *x1 as i64;

    let (mut y, mut x) = (*y1 as i64, *x1 as i64);
    loop {
        y -= dy;
        x -= dx;

        if let Some((y_int, x_int)) = check_pos(y, x, height, width) {
            antinodes.push((y_int, x_int));
        } else {
            break;
        }
    }

    let (mut y, mut x) = (*y2 as i64, *x2 as i64);
    loop {
        y += dy;
        x += dx;

        if let Some((y_int, x_int)) = check_pos(y, x, height, width) {
            antinodes.push((y_int, x_int));
        } else {
            break;
        }
    }

    antinodes
}

fn check_pos(y: i64, x: i64, height: usize, width: usize) -> Option<(usize, usize)> {
    let no_negs = y >= 0 && x >= 0;
    if no_negs {
        let y_int = y as usize;
        let x_int = x as usize;
        let no_too_big = y_int < height && x_int < width;
        if no_too_big {
            return Some((y_int, x_int));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_STR: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const TEST_INPUT_STR2: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test_part1() {
        let input = get_input_from_str(TEST_INPUT_STR.into());
        let answer = part1(&input);
        assert_eq!(answer, 14)
    }

    #[test]
    fn test_get_anten_pair_nodes() {
        let answer = get_anten_pair_antinodes_part1(&(3, 4), &(5, 5), 10, 10);
        assert_eq!(answer, vec![(1, 3), (7, 6)]);

        let answer = get_anten_pair_antinodes_part1(&(5, 5), &(3, 4), 10, 10);
        assert_eq!(answer, vec![(7, 6), (1, 3)]);
    }

    #[test]
    fn test_get_freq_nodes() {
        let (_grid, antennas) = get_input_from_str(TEST_INPUT_STR.into());
        let (height, width) = (12, 12);
        let answer = get_freq_antinodes(antennas.get(&'A').unwrap(), height, width, false);
        assert_eq!(answer.len(), 5);
    }

    #[test]
    fn test_part2() {
        let input = get_input_from_str(TEST_INPUT_STR.into());
        let answer = part2(&input);
        assert_eq!(answer, 34);

        let input = get_input_from_str(TEST_INPUT_STR2.into());
        let answer = part2(&input);
        assert_eq!(answer, 9);
    }
}
