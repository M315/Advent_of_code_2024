use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Operator {
    result: u64,
    values: Vec<u64>,
}

impl Operator {
    fn from_string(result: &str, values: Vec<&str>) -> Self {
        let result = result.parse::<u64>().unwrap();
        let values = values.into_iter()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Operator{ result, values }
    }

    fn eval(&self, operations: &Vec<char>) -> Option<u64> {
        let start: u64 = *self.values.first().unwrap();
        let res = self.values.iter()
            .skip(1)
            .zip(operations.iter())
            .fold(start, |acc, (val, op)| match op {
                '+' => acc + *val,
                '*' => acc * *val,
                '|' => concatenate_digits(acc, *val),
                _ => panic!("Invalid operation!"),
            });
        Some(res)
    }

    fn valid(&self, valid_operations: Vec<char>) -> bool {
        for op in (0..self.values.len()).map(|i| valid_operations.iter().cloned()).multi_cartesian_product() {
            if let Some(res) = self.eval(&op) {
                if res == self.result {
                    return true;
                }
            }
        }
        false
    }
}

fn concatenate_digits(mut a: u64, b: u64) -> u64 {
    let mut c = b;
    while c != 0 {
        c /= 10;
        a *= 10;
    }
    a + b
}

fn parse(input: &str) -> Vec<Operator> {
    input.lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(r, v)| (r, v.split_whitespace().collect::<Vec<&str>>()))
        .map(|(result, values)| Operator::from_string(result, values))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input).into_iter()
        .filter(|op| op.valid(vec!['+', '*']))
        .map(|op| op.result)
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse(input).into_iter()
        .filter(|op| op.valid(vec!['+', '*', '|']))
        .map(|op| op.result)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
