use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<u64, u64> {
    input.split_whitespace()
         .map(|line| (line.parse().unwrap(), 1))
         .collect()
}

fn stone_len(mut stone: u64) -> usize {
    let mut len = 0;
    while stone != 0 {
        len += 1;
        stone /= 10;
    }
    len
}

fn split_stone(mut stone: u64, len: usize) -> Vec<u64> {
    let mut half: u64 = 0;
    let mut mult = 1;
    for _ in 0..len / 2 {
        half += (stone % 10) * mult;
        mult *= 10;
        stone /= 10;
    }
    vec![stone, half]
}

fn apply_rule(stone: u64, memo: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if let Some(result) = memo.get(&stone) { return result.clone(); }
    if stone == 0 { return vec![1]; }
    let len = stone_len(stone);
    if len % 2 == 0 {
        split_stone(stone, len)
    } else {
        vec![stone * 2024]
    }
}

fn step(stones: HashMap<u64, u64>, memo: &mut HashMap<u64, Vec<u64>>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    stones.into_iter()
        .map(|(stone, instances)| apply_rule(stone, memo).into_iter().map(|new_stone| (new_stone, instances)).collect::<Vec<(u64, u64)>>())
        .flatten()
        .for_each(|(stone, instances)| *new_stones.entry(stone).or_insert(0) += instances);
    new_stones
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = parse(input);
    let mut memo = HashMap::new();
    for _ in 0..25 { stones = step(stones, &mut memo); }
    stones.values().sum::<u64>().into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = parse(input);
    let mut memo = HashMap::new();
    for _ in 0..75 { stones = step(stones, &mut memo); }
    stones.values().sum::<u64>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
