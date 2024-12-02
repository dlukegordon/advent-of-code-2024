use crate::get_data_filepath;
use std::{fs, path::Path};

type LevelReport = Vec<i32>;
type LevelReports = Vec<LevelReport>;
type LevelDiffReport = Vec<i32>;
type LevelDiffReports = Vec<LevelDiffReport>;

pub fn run() {
    let data_filepath = get_data_filepath!();
    let levels = get_levels(&data_filepath);
    part1(&levels);
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
    level_reports
        .iter()
        .map(|report| report.windows(2).map(|pair| pair[1] - pair[0]).collect())
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

fn diff_report_is_safe(diff_report: &LevelDiffReport) -> bool {
    let aiod = all_increasing_or_decreasing(diff_report);
    let aisdr = all_in_safe_diff_range(diff_report);
    aiod && aisdr
}

fn num_safe_reports(diff_reports: &LevelDiffReports) -> usize {
    diff_reports
        .iter()
        .filter(|r| diff_report_is_safe(r))
        .count()
}

fn part1(reports: &LevelReports) {
    let diff_reports = levels_to_diffs(reports);
    let num_safe_reports = num_safe_reports(&diff_reports);
    println!("Part 1: {num_safe_reports}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_reports() {
        let test_reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let test_diffs = levels_to_diffs(&test_reports);
        let num_safe_reports = num_safe_reports(&test_diffs);
        assert_eq!(num_safe_reports, 2);
    }
}
