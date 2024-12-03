use std::error::Error;
use itertools::Itertools;
use crate::util::{parsing, vecstuff};
use crate::util::parsing::{convert_strings_matrix, whitepsace_split};

pub mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_2_input.txt")?;
    let reports = whitepsace_split(lines);
    let reports = convert_strings_matrix::<i32>(&reports)?;
    let safe = count_safe(&reports);
    println!("Number of safe reports:\n{}", safe);

    let dampened_safe = count_dampened_safe(&reports);
    println!("Number of safe dampened reports:\n{}", dampened_safe);

    Ok(())
}

fn count_safe(reports: &Vec<Vec<i32>>) -> usize {
    safeties(reports).iter().filter(|&n| *n).count()
}

fn safeties(reports: &Vec<Vec<i32>>) -> Vec<bool> {
    reports.iter().map(|report| {
        is_safe(report)
    }).collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let ascending = report[0] < report[1];
    for (a, b) in report.into_iter().tuple_windows() {
        let diff = (a - b).abs();
        let currently_ascending = a < b;
        if ascending != currently_ascending {
            return false;
        }
        if !(diff >= 1 && diff <= 3) {
            return false;
        }
    }
    true
}

fn count_dampened_safe(reports: &Vec<Vec<i32>>) -> usize {
    dampened_safeties(reports).iter().filter(|&n| *n).count()
}

fn dampened_safeties(reports: &Vec<Vec<i32>>) -> Vec<bool> {
    reports.iter().map(|report| {
        is_dampened_safe(report)
    }).collect()
}

fn is_dampened_safe(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    report.iter().enumerate().map(|(idx, _)| {
        let sub_report = vecstuff::vec_without(report, idx);
        is_safe(&sub_report)
    }).filter(|&n| n).count() > 0
}

#[cfg(test)]
mod tests {
    use crate::{count_safe, dampened_safeties, safeties};

    #[test]
    fn test_safe() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let safeness = [
            true,
            false,
            false,
            false,
            false,
            true,
        ];
        let num_safe = 2;
        let actual_safeties = safeties(&reports);
        for idx in 0..6 {
            assert_eq!(safeness[idx], actual_safeties[idx]);
        }
        assert_eq!(count_safe(&reports), num_safe);
    }


    #[test]
    fn test_dampened_safe() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let safeness = [
            true,
            false,
            false,
            true,
            true,
            true,
        ];
        let num_safe = 2;
        let actual_safeties = dampened_safeties(&reports);
        for idx in 0..6 {
            assert_eq!(safeness[idx], actual_safeties[idx]);
        }
        assert_eq!(count_safe(&reports), num_safe);
    }
}