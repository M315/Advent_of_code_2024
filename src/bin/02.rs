use std::cmp::Ordering;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
         .map(|line| line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
         .collect()
}

fn safe_report(report: Vec<i32>) -> u32 {
    let direction: Ordering = report[1].cmp(&report[0]);
    for levels in report.windows(2) {
        if levels[0].abs_diff(levels[1]) > 3 || levels[0] == levels[1] || levels[1].cmp(&levels[0]) != direction {
            return 0;
        }
    }
    1
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input).into_iter()
        .map(|report| safe_report(report))
        .sum())
}

fn super_safe_report(report: Vec<i32>) -> u32 {
    if safe_report(report.clone()) == 1 { return 1; }
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if safe_report(new_report) == 1 {
            return 1;
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).into_iter()
        .map(|report| super_safe_report(report))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
