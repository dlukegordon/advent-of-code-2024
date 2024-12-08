use crate::get_data_filepath;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::{fs, path::Path, str::FromStr, string::ToString};

type Obstacles = Vec<Vec<bool>>;
type Position = (usize, usize);
type Data = (Obstacles, Position, Direction);
type VisitedDirections = FxHashMap<Position, Direction>;

#[derive(Clone, Debug)]
struct State {
    obstacles: Obstacles,
    guard_pos: Position,
    guard_dir: Direction,
    visited_dirs: VisitedDirections,
}

enum NextStateResult {
    Next(State),
    Finish(State),
    Loop,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right_turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_char = match self {
            Direction::Up => " ^ ",
            Direction::Right => " > ",
            Direction::Down => " v ",
            Direction::Left => " < ",
        };
        write!(f, "{}", dir_char)
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            ">" => Ok(Direction::Right),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

fn obstacles_to_strs(obstacles: &Obstacles) -> Vec<Vec<String>> {
    let mut strs = vec![];
    for row in obstacles {
        let mut row_strs = vec![];
        for cell in row {
            if *cell {
                row_strs.push(" # ".into());
            } else {
                row_strs.push(" . ".into());
            }
        }
        strs.push(row_strs);
    }
    strs
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut state_strs = obstacles_to_strs(&self.obstacles);

        for (y, x) in self.visited_dirs.keys() {
            state_strs[*y][*x] = " X ".into();
        }

        state_strs[self.guard_pos.0][self.guard_pos.1] = self.guard_dir.to_string();

        for row in &state_strs {
            for s in row {
                write!(f, "{s}",)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

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
    get_data_from_str(data_str)
}

fn get_data_from_str(data_str: String) -> Data {
    let (mut guard_y, mut guard_x): Position = (0, 0);
    let mut guard_dir: Direction = Direction::Up;

    let obstacles: Obstacles = data_str
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => true,
                    '^' => {
                        guard_y = y;
                        guard_x = x;
                        guard_dir = c.to_string().parse().unwrap();
                        false
                    }
                    _ => false,
                })
                .collect()
        })
        .collect();

    (obstacles, (guard_y, guard_x), guard_dir)
}

fn get_next_pos(obstacles: &Obstacles, pos: &Position, dir: &Direction) -> Option<Position> {
    let y_len = obstacles.len() as i32;
    let x_len = obstacles[0].len() as i32;
    let offset = dir.offset();

    let next_y = pos.0 as i32 + offset.0;
    if next_y < 0 || next_y > y_len - 1 {
        return None;
    }

    let next_x = pos.1 as i32 + offset.1;
    if next_x < 0 || next_x > x_len - 1 {
        return None;
    }

    Some((next_y as usize, next_x as usize))
}

fn has_obstacle(obstacles: &Obstacles, pos: &Position) -> bool {
    obstacles[pos.0][pos.1]
}

fn get_next_state(starting_state: &State) -> NextStateResult {
    let mut state = starting_state.clone();

    loop {
        let visited = state.visited_dirs.get(&state.guard_pos);
        match visited {
            Some(last_dir) => {
                if *last_dir == state.guard_dir {
                    return NextStateResult::Loop;
                }
            }
            None => {
                state
                    .visited_dirs
                    .insert(state.guard_pos, state.guard_dir.clone());
            }
        }

        let next_pos_result = get_next_pos(&state.obstacles, &state.guard_pos, &state.guard_dir);

        if next_pos_result.is_none() {
            return NextStateResult::Finish(state);
        }

        let next_pos = next_pos_result.unwrap();

        if has_obstacle(&state.obstacles, &next_pos) {
            state.guard_dir = state.guard_dir.right_turn();
            continue;
        }

        state.guard_pos = next_pos;
        return NextStateResult::Next(state);
    }
}

fn guard_walk((obstacles, (guard_y, guard_x), guard_dir): &Data) -> Option<VisitedDirections> {
    let mut state = State {
        obstacles: obstacles.clone(),
        guard_pos: (*guard_y, *guard_x),
        guard_dir: guard_dir.clone(),
        visited_dirs: FxHashMap::default(),
    };

    loop {
        let next_state = get_next_state(&state);
        match next_state {
            NextStateResult::Next(next_state) => {
                state = next_state;
            }
            NextStateResult::Finish(last_state) => {
                state = last_state;
                break;
            }
            NextStateResult::Loop => return None,
        }
    }

    Some(state.visited_dirs)
}

fn part1(data: &Data) -> usize {
    match guard_walk(data) {
        Some(visited_dirs) => visited_dirs.len(),
        None => panic!("Entered loop on part1"),
    }
}

fn part2((obstacles, (guard_y, guard_x), guard_dir): &Data) -> usize {
    let original_path =
        guard_walk(&(obstacles.clone(), (*guard_y, *guard_x), guard_dir.clone())).unwrap();

    let num_in_path = original_path.len();

    original_path
        .keys()
        .collect::<Vec<_>>()
        .par_iter()
        .enumerate()
        .filter(|(i, (y, x))| {
            // println!("trying {i}/{num_in_path} pos {:?}", (y, x));

            if obstacles[*y][*x] {
                return false;
            }
            if (y, x) == (guard_y, guard_x) {
                return false;
            }

            let mut new_obstacles = obstacles.clone();
            new_obstacles[*y][*x] = true;

            let walk = guard_walk(&(new_obstacles, (*guard_y, *guard_x), guard_dir.clone()));
            if walk.is_none() {
                return true;
            }

            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_STR: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let data = get_data_from_str(TEST_DATA_STR.into());
        let answer = part1(&data);
        assert_eq!(answer, 41)
    }

    #[test]
    fn test_part2() {
        let data = get_data_from_str(TEST_DATA_STR.into());
        let answer = part2(&data);
        assert_eq!(answer, 6)
    }
}
