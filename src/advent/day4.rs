use crate::get_data_filepath;
use regex::Regex;
use std::{fs, path::Path};

type DataLine = Vec<char>;
type DataLines = Vec<DataLine>;
type Data = Vec<DataLine>;

pub fn run() {
    let data_filepath = get_data_filepath!();
    let data = get_data(&data_filepath);

    let answer1 = part1(&data);
    println!("Part 1: {answer1}");

    let answer2 = part2(&data);
    println!("Part 2: {answer2}");
}

fn get_data(data_filepath: &Path) -> Data {
    let data_str = fs::read_to_string(data_filepath).unwrap();
    string_to_2d_vec(&data_str)
}

fn string_to_2d_vec(input: &str) -> Data {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_horizontals(data: &Data) -> DataLines {
    let mut horizs = vec![];

    for row in data {
        horizs.push(row.clone());
        horizs.push(row.iter().cloned().rev().collect());
    }

    horizs
}

fn get_verticals(data: &Data) -> DataLines {
    let mut verts = vec![];
    let num_cols = data[0].len();

    for x in 0..num_cols {
        let mut v_line = vec![];
        for row in data {
            v_line.push(row[x]);
        }
        verts.push(v_line.clone());
        verts.push(v_line.iter().cloned().rev().collect());
    }

    verts
}

fn get_diagonals(data: &Data) -> DataLines {
    let mut diagonals = vec![];
    let num_rows: i32 = data.len().try_into().unwrap();
    let num_cols: i32 = data[0].len().try_into().unwrap();

    // Top-left to bottom-right diagonals
    for d in 0..(num_rows + num_cols - 1) {
        let mut diagonal = Vec::new();
        for i in 0..=d {
            let row = i;
            let col = d - i;
            if row < num_rows && col < num_cols {
                diagonal.push(data[row as usize][col as usize]);
            }
        }
        if !diagonal.is_empty() {
            diagonals.push(diagonal.clone());
            diagonals.push(diagonal.iter().cloned().rev().collect())
        }
    }

    // Top-right to bottom-left diagonals
    for d in 0..(num_rows + num_cols - 1) {
        let mut diagonal = Vec::new();
        for i in 0..=d {
            let row = i;
            let col = num_cols - 1 - (d - i);
            if row < num_rows && col < num_cols && col >= 0 {
                diagonal.push(data[row as usize][col as usize]);
            }
        }
        if !diagonal.is_empty() {
            diagonals.push(diagonal.clone());
            diagonals.push(diagonal.iter().cloned().rev().collect())
        }
    }

    diagonals
}

fn count_occurences(line: &DataLine) -> u64 {
    let line_str: String = line.iter().collect();
    let regex = Regex::new(r"XMAS").unwrap();
    regex.find_iter(&line_str).count().try_into().unwrap()
}

fn pattern_at_point(data: &Data, row: usize, col: usize) -> bool {
    let tlbr: String = vec![
        data[row][col],
        data[row + 1][col + 1],
        data[row + 2][col + 2],
    ]
    .iter()
    .collect();

    let trbl: String = vec![
        data[row][col + 2],
        data[row + 1][col + 1],
        data[row + 2][col],
    ]
    .iter()
    .collect();

    let tlbr_contains = tlbr == "MAS" || tlbr == "SAM";
    let trbl_contains = trbl == "MAS" || trbl == "SAM";
    tlbr_contains && trbl_contains
}

fn part1(data: &Data) -> u64 {
    let horizontals = get_horizontals(data);
    let verticals = get_verticals(data);
    let diagonals = get_diagonals(data);
    let all_paths = [horizontals, verticals, diagonals].concat();

    all_paths.iter().map(count_occurences).sum()
}

fn part2(data: &Data) -> u64 {
    let num_rows = data.len();
    let num_cols = data[0].len();
    let mut num_occurences = 0;

    for row in 0..num_rows - 2 {
        for col in 0..num_cols - 2 {
            if pattern_at_point(data, row, col) {
                num_occurences += 1;
            }
        }
    }

    num_occurences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = string_to_2d_vec(
            "MMMSXXMASM\n\
             MSAMXMSMSA\n\
             AMXSXMAAMM\n\
             MSAMASMSMX\n\
             XMASAMXAMM\n\
             XXAMMXXAMA\n\
             SMSMSASXSS\n\
             SAXAMASAAA\n\
             MAMMMXMMMM\n\
             MXMXAXMASX\n",
        );
        assert_eq!(part1(&data), 18);
    }

    #[test]
    fn test_part2() {
        let data = string_to_2d_vec(
            ".M.S......\n\
             ..A..MSMS.\n\
             .M.S.MAA..\n\
             ..A.ASMSM.\n\
             .M.S.M....\n\
             ..........\n\
             S.S.S.S.S.\n\
             .A.A.A.A..\n\
             M.M.M.M.M.\n\
             ..........\n",
        );
        assert_eq!(part2(&data), 9);
    }
}
