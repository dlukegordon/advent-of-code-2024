use crate::get_data_filepath;
use std::{fs, iter, ops::Range};

type DataPart1 = Vec<Option<usize>>;
type FileList = Vec<(Range<usize>, u64)>;
type FreeList = Vec<Range<usize>>;

pub fn run() {
    let input_filepath = get_data_filepath!();
    let input_str = fs::read_to_string(input_filepath).unwrap();

    let data_part1 = parse_data_part1(&input_str);
    let answer1 = part1(&data_part1);
    println!("Part 1: {answer1}");

    let answer2 = part2(&input_str);
    println!("Part 2: {answer2}");
}

fn parse_data_part1(input_str: &str) -> DataPart1 {
    let input_str = input_str.trim();
    let mut unpacked = vec![];

    for (i, num_blocks) in input_str.chars().enumerate() {
        let num_blocks = num_blocks.to_digit(10).unwrap() as usize;
        let id_num = if i % 2 == 0 { Some(i / 2) } else { None };
        unpacked.extend(iter::repeat(id_num).take(num_blocks));
    }

    unpacked
}

fn compact_part1(input: &DataPart1) -> DataPart1 {
    let mut defragged = input.clone();

    let mut head = 0usize;
    let mut tail = input.len() - 1;

    loop {
        while defragged[head].is_some() {
            head += 1;
        }
        while defragged[tail].is_none() {
            tail -= 1;
        }

        if head >= tail {
            break;
        }

        defragged.swap(head, tail);
    }

    defragged
}

fn get_checksum_part1(defragged: &DataPart1) -> usize {
    defragged
        .iter()
        .enumerate()
        .map(|(i, id)| i * id.unwrap_or(0))
        .sum()
}

fn part1(data: &DataPart1) -> usize {
    let compacted = compact_part1(data);
    get_checksum_part1(&compacted)
}

fn part2(input_str: &str) -> u64 {
    let input_str = input_str.trim();
    let mut file_list: FileList = vec![];
    let mut free_list: FreeList = vec![];
    let mut current_index = 0;

    for (i, space_size) in input_str.chars().enumerate() {
        let space_size = space_size.to_digit(10).unwrap() as usize;
        let space_range = current_index..current_index + space_size;
        current_index = space_range.end;

        if i % 2 == 0 {
            file_list.push((space_range, i as u64 / 2));
        } else {
            free_list.push(space_range);
        };
    }

    for file in file_list.iter_mut().rev() {
        if let Some((i, free)) = free_list
            .iter_mut()
            .enumerate()
            .find(|(_i, f)| f.end <= file.0.start && f.len() >= file.0.len())
        {
            let free_start = free.start;
            *free = free_start + file.0.len()..free.end;
            *file = (free_start..free_start + file.0.len(), file.1);
            if free.len() == 0 {
                free_list.remove(i);
            }
        }
    }

    file_list
        .into_iter()
        .map(|f| f.0.sum::<usize>() as u64 * f.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_STR: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let data = parse_data_part1(TEST_INPUT_STR);
        let answer = part1(&data);
        assert_eq!(answer, 1928);
    }

    #[test]
    fn test_part2() {
        let answer = part2(TEST_INPUT_STR);
        assert_eq!(answer, 2858);
    }
}
