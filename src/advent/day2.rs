use crate::get_data_filepath;
use std::{fs, path::Path};

type LevelReport = Vec<i32>;
type LevelReports = Vec<LevelReport>;
type LevelDiffReport = Vec<i32>;
type LevelDiffReports = Vec<LevelDiffReport>;

pub fn run() {
    let data_filepath = get_data_filepath!();
    let levels = get_levels(&data_filepath);

    let answer1 = part1(&levels);
    println!("Part 1: {answer1}");

    let answer2 = part2(&levels);
    println!("Part 2: {answer2}");
}

fn get_levels(data_filepath: &Path) -> LevelReports {
    let data_string = fs::read_to_string(data_filepath).unwrap();
    data_string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn levels_to_diffs(level_reports: &LevelReports) -> LevelDiffReports {
    level_reports.iter().map(|r| level_to_diff(r)).collect()
}

fn level_to_diff(level_report: &LevelReport) -> LevelDiffReport {
    level_report
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

fn all_increasing_or_decreasing(diff_report: &LevelDiffReport) -> bool {
    let first = *diff_report.first().unwrap();
    if first < 0 {
        diff_report.iter().skip(1).all(|&d| d < 0)
    } else if first > 0 {
        diff_report.iter().skip(1).all(|&d| d > 0)
    } else {
        false
    }
}

fn all_in_safe_diff_range(diff_report: &LevelDiffReport) -> bool {
    let safe_range_positive = 1..=3;
    diff_report
        .iter()
        .all(|d| safe_range_positive.contains(&d.abs()))
}

fn diff_report_is_safe_pt1(diff_report: &LevelDiffReport) -> bool {
    let aiod = all_increasing_or_decreasing(diff_report);
    let aisdr = all_in_safe_diff_range(diff_report);
    aiod && aisdr
}

fn report_is_safe_pt1(report: &LevelReport) -> bool {
    let diff = level_to_diff(report);
    diff_report_is_safe_pt1(&diff)
}

fn report_is_safe_pt2(report: &LevelReport) -> bool {
    if report_is_safe_pt1(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.clone();
        report.remove(i);
        if report_is_safe_pt1(&report) {
            return true;
        }
    }

    false
}

fn part1(reports: &LevelReports) -> usize {
    let diff_reports = levels_to_diffs(reports);
    diff_reports
        .iter()
        .filter(|r| diff_report_is_safe_pt1(r))
        .count()
}

fn part2(reports: &LevelReports) -> usize {
    reports.iter().filter(|r| report_is_safe_pt2(r)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_reports() -> LevelReports {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_part1() {
        let test_reports = get_test_reports();
        let answer = part1(&test_reports);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part2() {
        let test_reports = get_test_reports();
        let answer = part2(&test_reports);
        assert_eq!(answer, 4);
    }
}
