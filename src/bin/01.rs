use std::collections::HashMap;
advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<(u32, u32)> {
    input.lines()
         .map(|line| line.split_once(' ').unwrap())
         .map(|(l, r)| (l.trim().parse::<u32>().unwrap(), r.trim().parse::<u32>().unwrap()))
         .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let initial_list = parse(input);
    let mut left: Vec<u32> = initial_list.clone().into_iter().map(|(n, _)| n).collect();
    let mut right: Vec<u32> = initial_list.into_iter().map(|(_, n)| n).collect();

    left.sort_unstable();
    right.sort_unstable();

    Some(left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| u32::abs_diff(l, r))
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let initial_list = parse(input);
    let right: HashMap<u32, u32> = initial_list.clone()
        .into_iter()
        .map(|(_, n)| n)
        .fold(HashMap::<u32, u32>::new(), |mut acc, n| {
            acc.entry(n).and_modify(|count| *count += 1).or_insert(1);
            acc
        });

    Some(initial_list.into_iter()
        .map(|(n, _)| n * *right.get(&n).unwrap_or(&0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
